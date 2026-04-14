use crate::__private::wasip1;
use crate::{
    memory::{WasmAccess, WasmPathAccess, WasmPathComponent},
    wasi::file::{
        FilestatWithoutDevice, Wasip1LFS,
        changeable::inode::{DirMap, Inode, InodeData, InodeId, InodeMetadata},
        stdio::StdIO,
    },
};
use alloc::{collections::BTreeMap, vec::Vec, string::String};
use smallstr::SmallString;

/// A local file system that allows runtime modifications
#[derive(Debug)]
pub struct ChangeableLFS<StdIo: StdIO + 'static> {
    pub inodes: BTreeMap<InodeId, Inode>,
    pub next_inode_id: InodeId,
    pub preopens: BTreeMap<InodeId, String>,
    __marker: core::marker::PhantomData<StdIo>,
}

impl<StdIo: StdIO + 'static> ChangeableLFS<StdIo> {
    /// Creates a new ChangeableLFS with a root directory at Inode 0
    pub fn new() -> Self {
        let mut inodes = BTreeMap::new();
        // Create root directory at InodeId 0
        let root_meta = InodeMetadata::new(
            wasip1::FILETYPE_DIRECTORY,
            wasip1::RIGHTS_FD_READ
                | wasip1::RIGHTS_FD_WRITE
                | wasip1::RIGHTS_PATH_OPEN
                | wasip1::RIGHTS_PATH_CREATE_DIRECTORY
                | wasip1::RIGHTS_PATH_CREATE_FILE
                | wasip1::RIGHTS_PATH_FILESTAT_GET
                | wasip1::RIGHTS_PATH_FILESTAT_SET_SIZE
                | wasip1::RIGHTS_PATH_FILESTAT_SET_TIMES
                | wasip1::RIGHTS_FD_FILESTAT_GET
                | wasip1::RIGHTS_FD_FILESTAT_SET_SIZE
                | wasip1::RIGHTS_FD_FILESTAT_SET_TIMES
                | wasip1::RIGHTS_PATH_UNLINK_FILE
                | wasip1::RIGHTS_PATH_REMOVE_DIRECTORY,
        );
        let mut root_dir = DirMap::new();
        // Add "." and ".." to root pointing to itself
        root_dir.insert(SmallString::from_str("."), 0);
        root_dir.insert(SmallString::from_str(".."), 0);

        inodes.insert(
            0,
            Inode {
                meta: root_meta,
                data: InodeData::Dir(root_dir),
            },
        );

        let preopens = BTreeMap::new();

        Self {
            inodes,
            next_inode_id: 1,
            preopens,
            __marker: core::marker::PhantomData,
        }
    }

    fn allocate_inode(&mut self, inode: Inode) -> InodeId {
        let id = self.next_inode_id;
        self.next_inode_id += 1;
        self.inodes.insert(id, inode);
        id
    }

    /// Resolves a path to an inode ID. This is a simple implementation that doesn't fully handle symlink loops.
    pub fn get_inode_for_path<Wasm: WasmAccess>(
        &self,
        start_inode: InodeId,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Option<InodeId> {
        let path = WasmPathAccess::<Wasm>::new(path_ptr, path_len);
        let mut current_inode = start_inode;

        for part in path.components() {
            match part {
                WasmPathComponent::RootDir => current_inode = 0,
                WasmPathComponent::CurDir => {}
                WasmPathComponent::ParentDir => {
                    if let Some(Inode {
                        data: InodeData::Dir(dir_map),
                        ..
                    }) = self.inodes.get(&current_inode)
                    {
                        if let Some(&parent_id) = dir_map.get("..") {
                            current_inode = parent_id;
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
                WasmPathComponent::Normal(wasm_array) => {
                    if let Some(Inode {
                        data: InodeData::Dir(dir_map),
                        ..
                    }) = self.inodes.get(&current_inode)
                    {
                        // Compare the slice directly
                        let mut found = false;
                        for (name, &child_id) in dir_map.iter() {
                            if wasm_array.len() == name.len() && (0..wasm_array.len()).all(|i| wasm_array.get(i) == name.as_bytes()[i]) {
                                current_inode = child_id;
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
            }
        }
        Some(current_inode)
    }

    /// Recursively resolve a symlink. For brevity, simply returning the inode if it's not a symlink.
    fn resolve_inode(&self, inode_id: InodeId, _depth: usize) -> Option<InodeId> {
        // Simplified symlink resolution
        let inode = self.inodes.get(&inode_id)?;
        match &inode.data {
            InodeData::Symlink(_) => {
                // To properly resolve, we'd need to parse the target path here.
                // For now, we return None to indicate unhandled symlink loop or resolution.
                None
            }
            _ => Some(inode_id),
        }
    }

    /// Creates a new directory at the root or under another directory.
    pub fn add_dir(&mut self, parent_inode: InodeId, name: &str) -> Result<InodeId, ()> {
        let new_inode = Inode {
            meta: InodeMetadata::new(wasip1::FILETYPE_DIRECTORY, !0),
            data: InodeData::Dir({
                let mut m = DirMap::new();
                m.insert(SmallString::from_str("."), 0); // Self (updated below)
                m.insert(SmallString::from_str(".."), parent_inode);
                m
            }),
        };
        
        let new_id = self.allocate_inode(new_inode);

        if let Some(Inode { data: InodeData::Dir(map), .. }) = self.inodes.get_mut(&new_id) {
            map.insert(SmallString::from_str("."), new_id);
        }
        
        if let Some(Inode { data: InodeData::Dir(map), .. }) = self.inodes.get_mut(&parent_inode) {
            map.insert(SmallString::from_str(name), new_id);
        } else {
            return Err(());
        }
        
        Ok(new_id)
    }

    /// Adds a file with the given name and content to the given parent directory.
    pub fn add_file(&mut self, parent_inode: InodeId, name: &str, content: Vec<u8>) -> Result<InodeId, ()> {
        let new_inode = Inode {
            meta: InodeMetadata::new(wasip1::FILETYPE_REGULAR_FILE, !0),
            data: InodeData::File(content),
        };
        
        let new_id = self.allocate_inode(new_inode);
        
        if let Some(Inode { data: InodeData::Dir(map), .. }) = self.inodes.get_mut(&parent_inode) {
            map.insert(SmallString::from_str(name), new_id);
        } else {
            return Err(());
        }
        
        Ok(new_id)
    }

    /// Adds a preopen entry to the LFS so it can be queried by `fd_prestat_dir_name`.
    pub fn add_preopen(&mut self, inode: InodeId, path: &str) {
        self.preopens.insert(inode, String::from(path));
    }

    /// Removes a preopen entry from the LFS.
    pub fn remove_preopen(&mut self, inode: InodeId) {
        self.preopens.remove(&inode);
    }
}

impl<StdIo: StdIO + 'static> Wasip1LFS for ChangeableLFS<StdIo> {
    type Inode = InodeId;
    const PRE_OPEN: &'static [Self::Inode] = &[];

    fn fd_write_raw<Wasm: WasmAccess>(
        &mut self,
        inode: Self::Inode,
        data: *const u8,
        data_len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        let inode_entry = self.inodes.get_mut(&inode).ok_or(wasip1::ERRNO_BADF)?;
        if let InodeData::File(ref mut vec) = inode_entry.data {
            let mut buf = alloc::vec![0u8; data_len];
            Wasm::memcpy_to(&mut buf, data);
            vec.extend_from_slice(&buf);
            inode_entry.meta.mtim += 1; // Simplistic time update
            Ok(data_len)
        } else {
            Err(wasip1::ERRNO_ISDIR)
        }
    }

    fn fd_write_stdout_raw<Wasm: WasmAccess>(
        &mut self,
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
        &mut self,
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
        &mut self,
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
            let entry_slice = unsafe {
                core::slice::from_raw_parts(&entry as *const _ as *const u8, entry_size)
            };
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
        &mut self,
        inode: Self::Inode,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno> {
        let mut target_inode = self
            .get_inode_for_path::<Wasm>(inode, path_ptr, path_len)
            .ok_or(wasip1::ERRNO_NOENT)?;

        if flags & wasip1::LOOKUPFLAGS_SYMLINK_FOLLOW == wasip1::LOOKUPFLAGS_SYMLINK_FOLLOW {
            target_inode = self.resolve_inode(target_inode, 0).ok_or(wasip1::ERRNO_LOOP)?;
        }

        self.fd_filestat_get_raw::<Wasm>(target_inode)
    }

    fn fd_prestat_get_raw<Wasm: WasmAccess>(
        &mut self,
        inode: Self::Inode,
    ) -> Result<wasip1::Prestat, wasip1::Errno> {
        let name = self.preopens.get(&inode).ok_or(wasip1::ERRNO_BADF)?;
        Ok(wasip1::Prestat {
            tag: 0,
            u: wasip1::PrestatU {
                dir: wasip1::PrestatDir { pr_name_len: name.len() as usize },
            },
        })
    }

    fn fd_prestat_dir_name_raw<Wasm: WasmAccess>(
        &mut self,
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
        &mut self,
        inode: Self::Inode,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno> {
        let node = self.inodes.get(&inode).ok_or(wasip1::ERRNO_BADF)?;
        Ok(FilestatWithoutDevice {
            ino: inode as u64,
            filetype: node.meta.filetype,
            nlink: node.meta.nlink,
            size: node.meta.size(&node.data),
            atim: node.meta.atim,
            mtim: node.meta.mtim,
            ctim: node.meta.ctim,
        })
    }

    fn fd_pread_raw<Wasm: WasmAccess>(
        &mut self,
        inode: Self::Inode,
        buf: *mut u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        let node = self.inodes.get_mut(&inode).ok_or(wasip1::ERRNO_BADF)?;
        if let InodeData::File(ref file_data) = node.data {
            if offset >= file_data.len() {
                return Ok(0);
            }
            let available = file_data.len() - offset;
            let read_len = core::cmp::min(buf_len, available);
            Wasm::memcpy(buf, &file_data[offset..offset + read_len]);
            node.meta.atim += 1;
            Ok(read_len)
        } else {
            Err(wasip1::ERRNO_ISDIR)
        }
    }

    fn fd_read_stdin_raw<Wasm: WasmAccess>(
        &mut self,
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
        &mut self,
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
            if o_flags & wasip1::OFLAGS_DIRECTORY == wasip1::OFLAGS_DIRECTORY && !self.is_dir(inode) {
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
                            if let Some(Inode { data: InodeData::Dir(map), .. }) = self.inodes.get_mut(&new_id) {
                                map.insert(SmallString::from_str("."), new_id);
                            }
                        }

                        // Link in parent
                        if let Some(Inode { data: InodeData::Dir(dir_map), .. }) = self.inodes.get_mut(&dir_ino) {
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
