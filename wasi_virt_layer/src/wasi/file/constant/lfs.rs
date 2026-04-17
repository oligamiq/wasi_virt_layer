use crate::__private::wasip1::{self, Dircookie};
use crate::memory::WasmAccessMemoryUtilUpper as _;
use crate::{
    memory::{WasmAccess, WasmPathAccess, WasmPathComponent},
    wasi::file::{
        DefaultAddInfo, FilestatWithoutDevice, WasiAddInfo, Wasip1FileTrait,
        constant::lfs_raw::{VFSConstNormalFilesTy, VFSConstNormalInode},
        stdio::StdIO,
    },
};

/// A constant, normal local file system implementation.
#[derive(Debug)]
pub struct VFSConstNormalLFS<
    ConstRoot: VFSConstNormalFilesTy<File, FLAT_LEN> + core::fmt::Debug,
    File: Wasip1FileTrait + 'static + Copy,
    const FLAT_LEN: usize,
    StdIo: StdIO + 'static,
    AddInfo: WasiAddInfo + 'static = DefaultAddInfo,
> {
    add_info: [AddInfo; FLAT_LEN],
    __marker: core::marker::PhantomData<(ConstRoot, File, StdIo)>,
}

type LocalInode = usize;

impl<
    ConstRoot: VFSConstNormalFilesTy<File, FLAT_LEN> + core::fmt::Debug,
    File: Wasip1FileTrait + 'static + Copy,
    const FLAT_LEN: usize,
    StdIo: StdIO + 'static,
    AddInfo: WasiAddInfo + 'static,
> VFSConstNormalLFS<ConstRoot, File, FLAT_LEN, StdIo, AddInfo>
{
    /// Creates a new `VFSConstNormalLFS`.
    pub const fn new() -> Self {
        Self {
            add_info: [AddInfo::DEFAULT; FLAT_LEN],
            __marker: core::marker::PhantomData,
        }
    }

    /// Updates the access time for a given inode.
    #[inline]
    pub fn update_access_time(&mut self, inode: usize, atime: wasip1::Timestamp) {
        let add_info = &mut self.add_info[inode];
        add_info.set_access_time(atime);
    }

    /// Returns whether the given inode is a directory.
    #[inline]
    pub const fn is_dir(&self, inode: usize) -> bool {
        let (_, file_or_dir) = ConstRoot::FILES[inode];
        match file_or_dir {
            VFSConstNormalInode::Dir(..) => true,
            VFSConstNormalInode::File(..) => false,
        }
    }

    /// Returns the parent inode of the given inode.
    #[inline]
    pub const fn parent_inode(&self, inode: usize) -> Option<usize> {
        let (_, file_or_dir) = ConstRoot::FILES[inode];
        match file_or_dir {
            VFSConstNormalInode::Dir(_, parent, ..) => parent,
            VFSConstNormalInode::File(_, parent, ..) => Some(parent),
        }
    }

    /// Resolves a path starting from a given inode to find its inode.
    pub fn get_inode_for_path<Wasm: WasmAccess>(
        &self,
        inode: usize,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Option<usize> {
        let path = WasmPathAccess::<Wasm>::new(path_ptr, path_len);

        let path_parts = path.components();

        let mut current_inode = inode;

        for part in path_parts {
            // Resolve each part of the path
            match part {
                WasmPathComponent::RootDir => unreachable!(),
                WasmPathComponent::CurDir => {
                    // Stay in the current directory
                }
                WasmPathComponent::ParentDir => {
                    current_inode = self.parent_inode(current_inode)?;
                }
                WasmPathComponent::Normal(wasm_array_access) => {
                    let (start, end) = match ConstRoot::FILES[current_inode] {
                        (_, VFSConstNormalInode::Dir(range, ..)) => range,
                        _ => return None, // Not a directory
                    };

                    if let Some(i) = ConstRoot::FILES[start..end].iter().position(|(name, _)| {
                        name.len() == wasm_array_access.len()
                            && name
                                .as_bytes()
                                .iter()
                                .zip(wasm_array_access.iter())
                                .all(|(a, b)| *a == b)
                    }) {
                        current_inode = start + i;
                    } else {
                        return None; // Not found
                    }
                }
            }
        }

        Some(current_inode)
    }

    pub fn get_inode_for_path_dyn_compatible(
        &self,
        access: &impl WasmAccess,
        inode: usize,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Option<usize> {
        self.get_inode_for_path::<impl WasmAccess>(inode, path_ptr, path_len)
    }

    /// Returns the access time for a given inode.
    pub fn access_time(&self, inode: usize) -> wasip1::Timestamp {
        self.add_info[inode].access_time()
    }

    /// Creates file statistics for a given inode.
    pub fn filestat_from_inode(&self, inode: usize) -> FilestatWithoutDevice {
        FilestatWithoutDevice {
            ino: inode as _,
            filetype: ConstRoot::FILES[inode].1.filetype(),
            nlink: 1,
            size: ConstRoot::FILES[inode].1.size() as _,
            atim: self.add_info[inode].access_time(),
            mtim: self.add_info[inode].modification_time(),
            ctim: self.add_info[inode].creation_time(),
        }
    }

    pub fn fd_readdir_raw_inner(
        &self,
        inode: LocalInode,
        buf: *mut u8,
        buf_len: usize,
        cookie: Dircookie,
        memcpy_raw: impl Fn(*mut u8, *const u8, usize) + Copy,
    ) -> Result<(wasip1::Size, Dircookie), wasip1::Errno> {
        let (_, dir) = ConstRoot::FILES[inode];

        // . (current directory)
        if cookie == 0 {
            let next_cookie = if dir.parent().is_some() { 1 } else { 2 };
            let entry = wasip1::Dirent {
                d_next: next_cookie,
                d_ino: inode as _,
                d_namlen: 1,
                d_type: dir.filetype(),
            };
            let entry_buf = unsafe {
                core::slice::from_raw_parts(
                    &entry as *const _ as *const u8,
                    core::cmp::min(core::mem::size_of::<wasip1::Dirent>(), buf_len),
                )
            };
            memcpy_raw.memcpy_upper(buf, entry_buf);

            if buf_len < core::mem::size_of::<wasip1::Dirent>() {
                return Ok((buf_len, cookie));
            }

            memcpy_raw.memcpy_upper(
                unsafe { buf.add(core::mem::size_of::<wasip1::Dirent>()) },
                b".",
            );

            return Ok((core::mem::size_of::<wasip1::Dirent>() + 1, next_cookie));
        }

        // .. (parent directory)
        if cookie == 1 {
            let parent = dir.parent().unwrap();
            let entry = wasip1::Dirent {
                d_next: 2,
                d_ino: parent as _,
                d_namlen: 2,
                d_type: ConstRoot::FILES[parent].1.filetype(),
            };
            let entry_buf = unsafe {
                core::slice::from_raw_parts(
                    &entry as *const _ as *const u8,
                    core::cmp::min(core::mem::size_of::<wasip1::Dirent>(), buf_len),
                )
            };
            memcpy_raw.memcpy_upper(buf, entry_buf);

            if buf_len < core::mem::size_of::<wasip1::Dirent>() {
                return Ok((buf_len, cookie));
            }

            memcpy_raw.memcpy_upper(
                unsafe { buf.add(core::mem::size_of::<wasip1::Dirent>()) },
                b"..",
            );

            return Ok((core::mem::size_of::<wasip1::Dirent>() + 2, 2));
        }

        let (start, end) = match dir {
            VFSConstNormalInode::Dir(range, ..) => range,
            _ => unreachable!(),
        };

        let index = start + cookie as usize - 2;
        if index >= end {
            return Ok((0, cookie)); // No more entries
        }

        let (name, file_or_dir) = ConstRoot::FILES[index];

        let next_cookie = cookie + 1;

        let name_len = name.len();

        let entry = wasip1::Dirent {
            d_next: if (next_cookie as usize) < end {
                next_cookie
            } else {
                0
            },
            d_ino: index as _,
            d_namlen: name_len as _,
            d_type: file_or_dir.filetype(),
        };

        let entry_buf = unsafe {
            core::slice::from_raw_parts(
                &entry as *const _ as *const u8,
                core::cmp::min(core::mem::size_of::<wasip1::Dirent>(), buf_len),
            )
        };

        memcpy_raw.memcpy_upper(buf, entry_buf);

        if buf_len < core::mem::size_of::<wasip1::Dirent>() {
            return Ok((buf_len, cookie));
        }

        let name_bytes = unsafe {
            core::slice::from_raw_parts(
                name.as_ptr(),
                core::cmp::min(name_len, buf_len - core::mem::size_of::<wasip1::Dirent>()),
            )
        };

        memcpy_raw.memcpy_upper(
            unsafe { buf.add(core::mem::size_of::<wasip1::Dirent>()) },
            name_bytes,
        );

        Ok((
            core::mem::size_of::<wasip1::Dirent>() + name_bytes.len(),
            next_cookie,
        ))
    }
}
