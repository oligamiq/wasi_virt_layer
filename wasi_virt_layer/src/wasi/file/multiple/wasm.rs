use core::ops::Deref as _;

use alloc::collections::BTreeMap;

use smallvec::SmallVec;

use crate::{
    __private::wasip1::{self, *},
    file::FilestatWithoutDevice,
    memory::{
        WasmAccess, WasmAccessDynCompatible, WasmAccessDynCompatibleRaw,
        WasmAccessNameDynCompatible,
    },
    wasi::file::Wasip1DynamicLFS,
};

pub trait WasmAccessDynCompatibleTuple:
    WasmAccessNameDynCompatible + WasmAccessDynCompatibleRaw
{
}

impl<T: WasmAccessNameDynCompatible + WasmAccessDynCompatibleRaw> WasmAccessDynCompatibleTuple
    for T
{
}

#[derive(Debug)]
pub struct WasmAccessDynCompatibleWrapper(pub alloc::boxed::Box<dyn WasmAccessDynCompatibleTuple>);

impl AsRef<dyn WasmAccessNameDynCompatible> for WasmAccessDynCompatibleWrapper {
    fn as_ref(&self) -> &(dyn WasmAccessNameDynCompatible + 'static) {
        self.0.as_ref()
    }
}

impl AsRef<dyn WasmAccessDynCompatibleRaw> for WasmAccessDynCompatibleWrapper {
    fn as_ref(&self) -> &(dyn WasmAccessDynCompatibleRaw + 'static) {
        self.0.as_ref()
    }
}

impl WasmAccessDynCompatibleWrapper {
    pub fn new<T: WasmAccessDynCompatibleTuple + 'static>(access: T) -> Self {
        Self(alloc::boxed::Box::new(access))
    }
}

// impl WasmAccessDynCompatibleRaw for WasmAccessDynCompatibleWrapper {
//     fn memcpy_raw(&self, offset: *mut u8, src: *const u8, len: usize) {
//         self.0.memcpy_raw(offset, src, len);
//     }

//     fn memcpy_to_raw(&self, offset: *mut u8, src: *const u8, len: usize) {
//         self.0.memcpy_to_raw(offset, src, len);
//     }

//     #[cfg(not(feature = "multi_memory"))]
//     fn memory_director_raw(&self, ptr: isize) -> isize {
//         self.0.memory_director_raw(ptr)
//     }

//     fn _main_raw(&self) -> wasip1::Errno {
//         self.0._main_raw()
//     }

//     fn _reset_raw(&self) {
//         self.0._reset_raw()
//     }

//     fn _start_raw(&self) {
//         self.0._start_raw()
//     }
// }

#[derive(Debug)]
pub struct WasmMultipleIntegrator {
    pub wasm_accesses: BTreeMap<u64, WasmAccessDynCompatibleWrapper>,
}

fn hasher(name: &str) -> u64 {
    // Simple hash function for demonstration purposes
    fxhash::hash64(name.as_bytes())
}

impl WasmMultipleIntegrator {
    pub fn new() -> Self {
        Self {
            wasm_accesses: BTreeMap::new(),
        }
    }

    pub fn add_wasm_access<T: WasmAccessDynCompatibleTuple + 'static>(&mut self, access: T) {
        self.wasm_accesses.insert(
            hasher(access.name()),
            WasmAccessDynCompatibleWrapper::new(access),
        );
    }

    fn fd_write_integrate<LFS: Wasip1DynamicLFS>(
        &self,
        lfs: &LFS,
        inode: &LFS::Inode,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno> {
        for access in self.wasm_accesses.values() {}
        Err(wasip1::ERRNO_BADF)
    }
}

impl WasmAccessMiddleIntegrator for WasmMultipleIntegrator {
    type Key = u64;

    fn generate_key(&self, name: &str) -> Self::Key {
        hasher(name)
    }

    fn fd_write_integrate<LFS: Wasip1DynamicLFS>(
        &self,
        key: &Self::Key,
        lfs: &LFS,
        inode: &LFS::Inode,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno> {
        let access = self.wasm_accesses.get(key).unwrap();

        todo!();
    }

    fn fd_write_stdout_integrate<LFS: Wasip1DynamicLFS>(
        &self,
        key: &Self::Key,
        lfs: &LFS,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno> {
        todo!();
    }

    fn fd_write_stderr_integrate<LFS: Wasip1DynamicLFS>(
        &self,
        key: &Self::Key,
        lfs: &LFS,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno> {
        todo!();
    }

    fn fd_readdir_integrate<LFS: Wasip1DynamicLFS>(
        &self,
        key: &Self::Key,
        lfs: &LFS,
        inode: &LFS::Inode,
        buf: *mut u8,
        buf_len: usize,
        cookie: Dircookie,
    ) -> Result<(Size, Dircookie), wasip1::Errno> {
        todo!()
    }

    fn path_filestat_get_integrate<LFS: Wasip1DynamicLFS>(
        &self,
        key: &Self::Key,
        lfs: &LFS,
        inode: &LFS::Inode,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno> {
        todo!()
    }

    fn fd_prestat_get_integrate<LFS: Wasip1DynamicLFS>(
        &self,
        key: &Self::Key,
        lfs: &LFS,
        inode: &LFS::Inode,
    ) -> Result<wasip1::Prestat, wasip1::Errno> {
        todo!()
    }

    fn fd_prestat_dir_name_integrate<LFS: Wasip1DynamicLFS>(
        &self,
        key: &Self::Key,
        lfs: &LFS,
        inode: &LFS::Inode,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        todo!()
    }

    fn fd_filestat_get_integrate<LFS: Wasip1DynamicLFS>(
        &self,
        key: &Self::Key,
        lfs: &LFS,
        inode: &LFS::Inode,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno> {
        todo!()
    }

    fn fd_pread_integrate<LFS: Wasip1DynamicLFS>(
        &self,
        key: &Self::Key,
        lfs: &LFS,
        inode: &LFS::Inode,
        buf: *mut u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<Size, wasip1::Errno> {
        todo!()
    }

    fn fd_read_stdin_integrate<LFS: Wasip1DynamicLFS>(
        &self,
        key: &Self::Key,
        lfs: &LFS,
        buf: *mut u8,
        buf_len: usize,
    ) -> Result<Size, wasip1::Errno> {
        todo!()
    }

    fn path_open_integrate<LFS: Wasip1DynamicLFS>(
        &self,
        key: &Self::Key,
        lfs: &LFS,
        dir_ino: &LFS::Inode,
        dir_flags: wasip1::Fdflags,
        path_ptr: *const u8,
        path_len: usize,
        oflags: wasip1::Oflags,
        fs_rights_base: wasip1::Rights,
        fs_rights_inheriting: wasip1::Rights,
        fd_flags: wasip1::Fdflags,
    ) -> Result<(LFS::Inode, wasip1::Fd), wasip1::Errno> {
        todo!()
    }
}

pub trait WasmAccessMiddleIntegrator: core::fmt::Debug {
    type Key;

    fn generate_key(&self, name: &str) -> Self::Key;

    fn fd_write_integrate<LFS: Wasip1DynamicLFS>(
        &self,
        key: &Self::Key,
        lfs: &LFS,
        inode: &LFS::Inode,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno>;

    fn fd_write_stdout_integrate<LFS: Wasip1DynamicLFS>(
        &self,
        key: &Self::Key,
        lfs: &LFS,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno>;

    fn fd_write_stderr_integrate<LFS: Wasip1DynamicLFS>(
        &self,
        key: &Self::Key,
        lfs: &LFS,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno>;

    fn fd_readdir_integrate<LFS: Wasip1DynamicLFS>(
        &self,
        key: &Self::Key,
        lfs: &LFS,
        inode: &LFS::Inode,
        buf: *mut u8,
        buf_len: usize,
        cookie: Dircookie,
    ) -> Result<(Size, Dircookie), wasip1::Errno>;

    fn path_filestat_get_integrate<LFS: Wasip1DynamicLFS>(
        &self,
        key: &Self::Key,
        lfs: &LFS,
        inode: &LFS::Inode,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno>;

    fn fd_prestat_get_integrate<LFS: Wasip1DynamicLFS>(
        &self,
        key: &Self::Key,
        lfs: &LFS,
        inode: &LFS::Inode,
    ) -> Result<wasip1::Prestat, wasip1::Errno>;

    fn fd_prestat_dir_name_integrate<LFS: Wasip1DynamicLFS>(
        &self,
        key: &Self::Key,
        lfs: &LFS,
        inode: &LFS::Inode,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> Result<(), wasip1::Errno>;

    fn fd_filestat_get_integrate<LFS: Wasip1DynamicLFS>(
        &self,
        key: &Self::Key,
        lfs: &LFS,
        inode: &LFS::Inode,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno>;

    fn fd_pread_integrate<LFS: Wasip1DynamicLFS>(
        &self,
        key: &Self::Key,
        lfs: &LFS,
        inode: &LFS::Inode,
        buf: *mut u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<Size, wasip1::Errno>;

    fn fd_read_stdin_integrate<LFS: Wasip1DynamicLFS>(
        &self,
        key: &Self::Key,
        lfs: &LFS,
        buf: *mut u8,
        buf_len: usize,
    ) -> Result<Size, wasip1::Errno>;

    fn path_open_integrate<LFS: Wasip1DynamicLFS>(
        &self,
        key: &Self::Key,
        lfs: &LFS,
        dir_ino: &LFS::Inode,
        dir_flags: wasip1::Fdflags,
        path_ptr: *const u8,
        path_len: usize,
        oflags: wasip1::Oflags,
        fs_rights_base: wasip1::Rights,
        fs_rights_inheriting: wasip1::Rights,
        fd_flags: wasip1::Fdflags,
    ) -> Result<(LFS::Inode, wasip1::Fd), wasip1::Errno>;
}
