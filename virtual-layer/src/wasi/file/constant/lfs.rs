use crate::__private::wasip1;
use crate::{
    memory::{WasmAccess, WasmPathAccess, WasmPathComponent},
    wasi::file::{
        FilestatWithoutDevice, Wasip1FileTrait,
        constant::{
            lfs_impl::VFSConstNormalAddInfo,
            lfs_raw::{VFSConstNormalFilesTy, VFSConstNormalInode},
        },
        stdio::StdIO,
    },
};

pub struct VFSConstNormalLFS<
    ConstRoot: VFSConstNormalFilesTy<File, FLAT_LEN>,
    File: Wasip1FileTrait + 'static + Copy,
    const FLAT_LEN: usize,
    StdIo: StdIO + 'static,
> {
    add_info: [VFSConstNormalAddInfo; FLAT_LEN],
    __marker: core::marker::PhantomData<(ConstRoot, File, StdIo)>,
}

impl<
    ConstRoot: VFSConstNormalFilesTy<File, FLAT_LEN>,
    File: Wasip1FileTrait + 'static + Copy,
    const FLAT_LEN: usize,
    StdIo: StdIO + 'static,
> VFSConstNormalLFS<ConstRoot, File, FLAT_LEN, StdIo>
{
    pub const fn new() -> Self {
        Self {
            add_info: [VFSConstNormalAddInfo::new(); FLAT_LEN],
            __marker: core::marker::PhantomData,
        }
    }

    #[inline]
    pub const fn update_access_time(&mut self, inode: usize, atime: usize) {
        let add_info = &mut self.add_info[inode];
        add_info.set_access_time(atime);
    }

    #[inline]
    pub const fn is_dir(&self, inode: usize) -> bool {
        let (_, file_or_dir) = ConstRoot::FILES[inode];
        match file_or_dir {
            VFSConstNormalInode::Dir(..) => true,
            VFSConstNormalInode::File(..) => false,
        }
    }

    #[inline]
    pub const fn parent_inode(&self, inode: usize) -> Option<usize> {
        let (_, file_or_dir) = ConstRoot::FILES[inode];
        match file_or_dir {
            VFSConstNormalInode::Dir(_, parent, ..) => parent,
            VFSConstNormalInode::File(_, parent, ..) => Some(parent),
        }
    }

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

    pub fn access_time(&self, inode: usize) -> wasip1::Timestamp {
        self.add_info[inode].access_time() as wasip1::Timestamp
    }

    pub fn filestat_from_inode(&self, inode: usize) -> FilestatWithoutDevice {
        FilestatWithoutDevice {
            ino: inode as _,
            filetype: ConstRoot::FILES[inode].1.filetype(),
            nlink: 1,
            size: ConstRoot::FILES[inode].1.size() as _,
            atim: self.access_time(inode),
            mtim: 0,
            ctim: 0,
        }
    }
}
