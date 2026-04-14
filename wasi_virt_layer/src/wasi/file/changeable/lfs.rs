use crate::__private::wasip1;
use crate::wasi::file::Wasip1DynamicLFS;
use crate::{
    memory::{WasmAccess, WasmPathAccess, WasmPathComponent},
    wasi::file::{
        DefaultAddInfo, FilestatWithoutDevice, WasiAddInfo, Wasip1LFS,
        changeable::inode::{DirMap, Inode, InodeData, InodeId, InodeMetadata},
        stdio::StdIO,
    },
};
use alloc::{collections::BTreeMap, string::String, vec::Vec};
use smallstr::SmallString;

#[cfg(feature = "threads")]
use dashmap::DashMap;

#[cfg(not(feature = "threads"))]
use core::cell::UnsafeCell;
use std::borrow::Borrow;
use std::ops::Deref;

/// A local file system that allows runtime modifications
#[derive(Debug)]
pub struct ChangeableLFS<StdIo: StdIO + 'static, AddInfo: WasiAddInfo + 'static = DefaultAddInfo> {
    #[cfg(feature = "threads")]
    pub inodes: DashMap<InodeId, Inode<AddInfo>>,
    #[cfg(not(feature = "threads"))]
    pub inodes: UnsafeCell<BTreeMap<InodeId, Inode<AddInfo>>>,
    #[cfg(feature = "threads")]
    pub preopens: DashMap<InodeId, String>,
    #[cfg(not(feature = "threads"))]
    pub preopens: UnsafeCell<BTreeMap<InodeId, String>>,
    __marker: core::marker::PhantomData<StdIo>,
}

impl<StdIo: StdIO + 'static, AddInfo: WasiAddInfo + 'static> ChangeableLFS<StdIo, AddInfo> {
    /// Creates a new ChangeableLFS with a root directory at Inode 0
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "threads")]
            inodes: DashMap::from_iter(inodes_map),
            #[cfg(not(feature = "threads"))]
            inodes: UnsafeCell::new(inodes_map),
            #[cfg(feature = "threads")]
            preopens: DashMap::new(),
            #[cfg(not(feature = "threads"))]
            preopens: UnsafeCell::new(BTreeMap::new()),
            __marker: core::marker::PhantomData,
        }
    }
}

impl<StdIo: StdIO + 'static, AddInfo: WasiAddInfo + 'static> Wasip1LFS
    for ChangeableLFS<StdIo, AddInfo>
{
    type Inode = InodeId;

    fn fd_write_raw<Wasm: WasmAccess>(
        &self,
        inode: Self::Inode,
        data: *const u8,
        data_len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        let inode_entry = self.inodes.get_mut(&inode).ok_or(wasip1::ERRNO_BADF)?;
        if let InodeData::File(ref mut vec) = inode_entry.data {
            let mut buf = alloc::vec![0u8; data_len];
            Wasm::memcpy_to(&mut buf, data);
            vec.extend_from_slice(&buf);
            let mtim = inode_entry.meta.add_info.modification_time();
            inode_entry.meta.add_info.set_modification_time(mtim + 1); // Simplistic time update
            Ok(data_len)
        } else {
            Err(wasip1::ERRNO_ISDIR)
        }
    }

    fn fd_write_stdout_raw<Wasm: WasmAccess>(
        &self,
        data: *const u8,
        data_len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        #[cfg(not(feature = "multi_memory"))]
        {
            StdIo::write_direct::<Wasm>(data, data_len)
        }
        #[cfg(feature = "multi_memory")]
        {
            let mut buf = alloc::vec![0u8; data_len];
            Wasm::memcpy_to(&mut buf, data);
            StdIo::write(&buf)
        }
    }

    fn fd_write_stderr_raw<Wasm: WasmAccess>(
        &self,
        data: *const u8,
        data_len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        #[cfg(not(feature = "multi_memory"))]
        {
            StdIo::ewrite_direct::<Wasm>(data, data_len)
        }
        #[cfg(feature = "multi_memory")]
        {
            let mut buf = alloc::vec![0u8; data_len];
            Wasm::memcpy_to(&mut buf, data);
            StdIo::ewrite(&buf)
        }
    }

    fn is_dir(&self, inode: Self::Inode) -> bool {
        matches!(
            self.inodes.get(&inode).map(|i| &i.data),
            Some(InodeData::Dir(_))
        )
    }

    fn fd_readdir_raw<Wasm: WasmAccess>(
        &self,
        inode: Self::Inode,
        buf: *mut u8,
        buf_len: usize,
        cookie: wasip1::Dircookie,
    ) -> Result<(wasip1::Size, wasip1::Dircookie), wasip1::Errno> {
        let dir_map = match self.inodes.get(&inode).map(|i| &i.data) {
            Some(InodeData::Dir(map)) => map,
            _ => return Err(wasip1::ERRNO_NOTDIR),
        };

        let mut current_cookie = cookie as usize;
        let mut total_written = 0;
        let mut out_ptr = buf;
        let mut rem_len = buf_len;

        for (i, (name, &child_inode)) in dir_map.iter().enumerate() {
            if i < current_cookie {
                continue;
            }
            let child = self.inodes.get(&child_inode).ok_or(wasip1::ERRNO_NOENT)?;
            let name_bytes = name.as_bytes();
            let entry = wasip1::Dirent {
                d_next: (i + 1) as u64,
                d_ino: child_inode as u64,
                d_namlen: name_bytes.len() as u32,
                d_type: child.meta.filetype,
            };

            let entry_size = core::mem::size_of::<wasip1::Dirent>();

            if rem_len < entry_size {
                break;
            }

            // Write dirent
            let entry_slice =
                unsafe { core::slice::from_raw_parts(&entry as *const _ as *const u8, entry_size) };
            Wasm::memcpy(out_ptr, entry_slice);

            out_ptr = unsafe { out_ptr.add(entry_size) };
            rem_len -= entry_size;
            total_written += entry_size;

            // Write name
            let write_name_len = core::cmp::min(name_bytes.len(), rem_len);
            if write_name_len > 0 {
                Wasm::memcpy(out_ptr, &name_bytes[..write_name_len]);
                out_ptr = unsafe { out_ptr.add(write_name_len) };
                rem_len -= write_name_len;
                total_written += write_name_len;
            }

            current_cookie = i + 1;
            if rem_len == 0 {
                break;
            }
        }

        Ok((total_written, current_cookie as u64))
    }

    fn path_filestat_get_raw<Wasm: WasmAccess>(
        &self,
        inode: Self::Inode,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno> {
        let mut target_inode = self
            .get_inode_for_path::<Wasm>(inode, path_ptr, path_len)
            .ok_or(wasip1::ERRNO_NOENT)?;

        if flags & wasip1::LOOKUPFLAGS_SYMLINK_FOLLOW == wasip1::LOOKUPFLAGS_SYMLINK_FOLLOW {
            target_inode = self
                .resolve_inode(target_inode, 0)
                .ok_or(wasip1::ERRNO_LOOP)?;
        }

        self.fd_filestat_get_raw::<Wasm>(target_inode)
    }

    fn fd_prestat_get_raw<Wasm: WasmAccess>(
        &self,
        inode: Self::Inode,
    ) -> Result<wasip1::Prestat, wasip1::Errno> {
        let name = self.preopens.get(&inode).ok_or(wasip1::ERRNO_BADF)?;
        Ok(wasip1::Prestat {
            tag: 0,
            u: wasip1::PrestatU {
                dir: wasip1::PrestatDir {
                    pr_name_len: name.len() as usize,
                },
            },
        })
    }

    fn fd_prestat_dir_name_raw<Wasm: WasmAccess>(
        &self,
        inode: Self::Inode,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        let name = self.preopens.get(&inode).ok_or(wasip1::ERRNO_BADF)?;
        let copy_len = core::cmp::min(name.len(), dir_path_len);
        if copy_len > 0 {
            Wasm::memcpy(dir_path_ptr, &name.as_bytes()[..copy_len]);
        }
        Ok(())
    }

    fn fd_filestat_get_raw<Wasm: WasmAccess>(
        &self,
        inode: Self::Inode,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno> {
        let node = self.inodes.get(&inode).ok_or(wasip1::ERRNO_BADF)?;
        Ok(FilestatWithoutDevice {
            ino: inode as u64,
            filetype: node.meta.filetype,
            nlink: node.meta.nlink,
            size: node.meta.size(&node.data),
            atim: node.meta.add_info.access_time(),
            mtim: node.meta.add_info.modification_time(),
            ctim: node.meta.add_info.creation_time(),
        })
    }

    fn fd_pread_raw<Wasm: WasmAccess>(
        &self,
        inode: Self::Inode,
        buf: *mut u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        let mut node = self.inodes.get_mut(&inode).ok_or(wasip1::ERRNO_BADF)?;
        if let InodeData::File(ref file_data) = node.data {
            if offset >= file_data.len() {
                return Ok(0);
            }
            let available = file_data.len() - offset;
            let read_len = core::cmp::min(buf_len, available);
            Wasm::memcpy(buf, &file_data[offset..offset + read_len]);
            let atim = node.meta.add_info.access_time();
            node.meta.add_info.set_access_time(atim + 1);
            Ok(read_len)
        } else {
            Err(wasip1::ERRNO_ISDIR)
        }
    }

    fn fd_read_stdin_raw<Wasm: WasmAccess>(
        &self,
        buf: *mut u8,
        buf_len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        #[cfg(not(feature = "multi_memory"))]
        {
            StdIo::read_direct::<Wasm>(buf, buf_len)
        }

        #[cfg(feature = "multi_memory")]
        {
            let mut internal_buf = alloc::vec![0u8; buf_len];
            let read = StdIo::read(&mut internal_buf).map_err(|_| wasip1::ERRNO_IO)?;
            Wasm::memcpy(buf, &internal_buf[..read]);
            Ok(read)
        }
    }

    fn path_open_raw<Wasm: WasmAccess>(
        &self,
        dir_ino: Self::Inode,
        _: wasip1::Fdflags,
        path_ptr: *const u8,
        path_len: usize,
        o_flags: wasip1::Oflags,
        fs_rights_base: wasip1::Rights,
        _: wasip1::Rights,
        _: wasip1::Fdflags,
    ) -> Result<Self::Inode, wasip1::Errno> {
        if let Some(inode) = self.get_inode_for_path::<Wasm>(dir_ino, path_ptr, path_len) {
            if o_flags & wasip1::OFLAGS_EXCL == wasip1::OFLAGS_EXCL {
                return Err(wasip1::ERRNO_EXIST);
            }
            if o_flags & wasip1::OFLAGS_DIRECTORY == wasip1::OFLAGS_DIRECTORY && !self.is_dir(inode)
            {
                return Err(wasip1::ERRNO_NOTDIR);
            }
            if o_flags & wasip1::OFLAGS_TRUNC == wasip1::OFLAGS_TRUNC {
                if let Some(node) = self.inodes.get_mut(&inode) {
                    if let InodeData::File(ref mut vec) = node.data {
                        vec.clear();
                    }
                }
            }
            Ok(inode)
        } else {
            if o_flags & wasip1::OFLAGS_CREAT == wasip1::OFLAGS_CREAT {
                let path = WasmPathAccess::<Wasm>::new(path_ptr, path_len);
                let components: Vec<_> = path.components().collect();

                if components.is_empty() {
                    return Err(wasip1::ERRNO_NOENT);
                }

                // For simplicity, we assume creation only happens in `dir_ino` directly
                // and the path is just a single Normal component.
                // Full path traversal for creation is complex and not fully implemented here.
                if components.len() == 1 {
                    if let WasmPathComponent::Normal(name) = &components[0] {
                        let mut s = SmallString::<[u8; 32]>::new();
                        for j in 0..name.len() {
                            s.push(name.get(j) as char);
                        }

                        let is_dir = o_flags & wasip1::OFLAGS_DIRECTORY == wasip1::OFLAGS_DIRECTORY;
                        let filetype = if is_dir {
                            wasip1::FILETYPE_DIRECTORY
                        } else {
                            wasip1::FILETYPE_REGULAR_FILE
                        };

                        let data = if is_dir {
                            let mut map = DirMap::new();
                            map.insert(SmallString::from_str("."), 0); // Self (will update)
                            map.insert(SmallString::from_str(".."), dir_ino);
                            InodeData::Dir(map)
                        } else {
                            InodeData::File(Vec::new())
                        };

                        let new_inode = Inode {
                            meta: InodeMetadata::new(filetype, fs_rights_base),
                            data,
                        };

                        let new_id = self.allocate_inode(new_inode);

                        // Update self reference if dir
                        if is_dir {
                            if let Some(Inode {
                                data: InodeData::Dir(map),
                                ..
                            }) = self.inodes.get_mut(&new_id)
                            {
                                map.insert(SmallString::from_str("."), new_id);
                            }
                        }

                        // Link in parent
                        if let Some(Inode {
                            data: InodeData::Dir(dir_map),
                            ..
                        }) = self.inodes.get_mut(&dir_ino)
                        {
                            dir_map.insert(s, new_id);
                        }

                        return Ok(new_id);
                    }
                }
                Err(wasip1::ERRNO_NOENT)
            } else {
                Err(wasip1::ERRNO_NOENT)
            }
        }
    }
}

impl<StdIo: StdIO + 'static, AddInfo: WasiAddInfo + 'static> Wasip1DynamicLFS
    for ChangeableLFS<StdIo, AddInfo>
{
    fn pre_open_inodes<'a>(&'a self) -> impl IntoIterator<Item = (Self::Inode, impl Deref<Target = impl Borrow<str>> + 'a)> + 'a {
        #[cfg(feature = "threads")]
        {
            self.preopens.iter().map(|entry| {
                (*entry.key(), entry)
            })
        }
        #[cfg(not(feature = "threads"))]
        {
            unsafe { &*self.preopens.get() }
                .iter()
                .map(|(inode, name)| (*inode, name.as_str()))
        }
    }
}
