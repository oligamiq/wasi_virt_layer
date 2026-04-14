use crate::__private::wasip1;
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
}
