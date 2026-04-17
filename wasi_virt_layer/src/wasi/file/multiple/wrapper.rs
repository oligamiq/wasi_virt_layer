use core::any::Any;
use core::ffi::c_void;

use smallvec::SmallVec;

use crate::{
    __private::wasip1::{self, *},
    file::FilestatWithoutDevice,
    memory::{WasmAccess, WasmAccessRaw},
    wasi::file::Wasip1DynamicLFS,
};

pub(crate) struct WasmAccessWrapperBase<'a, WasmWrapper: WasmAccessWrapper> {
    pub inner: &'a WasmWrapper,
}

pub(crate) enum WasmAccessArgsWrapperInner<'a> {
    Memcpy {
        t_size: u8,
        offset: *mut c_void,
        data: &'a c_void,
        data_len: usize,
    },
    MemcpyTo {
        t_size: u8,
        offset: &'a mut c_void,
        src: *const c_void,
        data_len: usize,
    },
    StoreLe {
        t_size: u8,
        offset: *mut c_void,
        value: &'a c_void,
    },
    LoadLe {
        t_size: u8,
        offset: *const c_void,
        ret: &'a mut c_void,
    },
    #[cfg(not(feature = "multi_memory"))]
    MemoryDirector { ptr: *const c_void },
    #[cfg(not(feature = "multi_memory"))]
    MemoryDirectorMut { ptr: *mut c_void },
}

impl<'a, WasmWrapper: WasmAccessWrapper> WasmAccessWrapperBase<'a, WasmWrapper> {
    pub fn memcpy<T: 'static>(&self, offset: *mut T, data: &'a [T]) {
        let cmd = WasmAccessArgsWrapperInner::Memcpy {
            t_size: core::mem::size_of::<T>() as u8,
            offset: offset as *mut c_void,
            data: unsafe { &*(data.as_ptr() as *const c_void) } as &c_void,
            data_len: data.len(),
        };
        self.inner.apply(cmd);
    }

    pub fn memcpy_to<T: 'static>(&self, offset: &mut [T], src: *const T) {
        let cmd = WasmAccessArgsWrapperInner::MemcpyTo {
            t_size: core::mem::size_of::<T>() as u8,
            offset: unsafe { &mut *(offset.as_mut_ptr() as *mut c_void) } as &mut c_void,
            src: src as *const c_void,
            data_len: offset.len(),
        };
        self.inner.apply(cmd);
    }

    pub fn store_le<T: 'static>(&self, offset: *mut T, value: &'a impl core::borrow::Borrow<T>) {
        let cmd = WasmAccessArgsWrapperInner::StoreLe {
            t_size: core::mem::size_of::<T>() as u8,
            offset: offset as *mut c_void,
            value: unsafe { &*(value.borrow() as *const T as *const c_void) } as &c_void,
        };
        self.inner.apply(cmd);
    }

    pub fn load_le<T: 'static + core::fmt::Debug + Copy>(&self, offset: *const T) -> T {
        let holder = core::mem::MaybeUninit::<T>::uninit();
        let cmd = WasmAccessArgsWrapperInner::LoadLe {
            t_size: core::mem::size_of::<T>() as u8,
            offset: offset as *const c_void,
            ret: unsafe { &mut *(holder.as_ptr() as *mut c_void) } as &mut c_void,
        };
        self.inner.apply(cmd);
        unsafe { core::ptr::read(holder.as_ptr() as *const T) }
    }

    #[cfg(not(feature = "multi_memory"))]
    pub fn memory_director<T>(&self, ptr: *const T) -> *const T {
        let cmd = WasmAccessArgsWrapperInner::MemoryDirector {
            ptr: ptr as *const c_void,
        };
        let res = self.inner.memory_director(cmd);
        res as *const T
    }

    #[cfg(not(feature = "multi_memory"))]
    pub fn memory_director_mut<T>(&self, ptr: *mut T) -> *mut T {
        let cmd = WasmAccessArgsWrapperInner::MemoryDirectorMut {
            ptr: ptr as *mut c_void,
        };
        let res = self.inner.memory_director_mut(cmd);
        res as *mut T
    }
}

impl<'a> WasmAccessArgsWrapperInner<'a> {
    pub fn apply<Wasm: WasmAccessRaw>(self) {
        match self {
            WasmAccessArgsWrapperInner::Memcpy {
                t_size,
                offset,
                data,
                data_len,
            } => {
                // second, we need to call the memcpy function with the correct type
                Wasm::memcpy_raw(
                    offset as *mut u8,
                    data as *const c_void as *const u8,
                    data_len * t_size as usize,
                );
            }
            WasmAccessArgsWrapperInner::MemcpyTo {
                t_size,
                offset,
                src,
                data_len,
            } => {
                Wasm::memcpy_to_raw(
                    offset as *mut c_void as *mut u8,
                    src as *const c_void as *const u8,
                    data_len * t_size as usize,
                );
            }
            WasmAccessArgsWrapperInner::StoreLe {
                t_size,
                offset,
                value,
            } => {
                Wasm::memcpy_raw(
                    offset as *mut u8,
                    value as *const c_void as *const u8,
                    t_size as usize,
                );
            }
            WasmAccessArgsWrapperInner::LoadLe {
                t_size,
                offset,
                ret,
            } => {
                Wasm::memcpy_to_raw(
                    ret as *mut c_void as *mut u8,
                    offset as *const c_void as *const u8,
                    t_size as usize,
                );
            }
            _ => unreachable!(),
        }
    }

    pub fn memory_director<Wasm: WasmAccessRaw>(self) -> *const c_void {
        match self {
            #[cfg(not(feature = "multi_memory"))]
            WasmAccessArgsWrapperInner::MemoryDirector { ptr } => {
                Wasm::memory_director_raw(ptr as isize) as *const c_void
            }
            #[cfg(not(feature = "multi_memory"))]
            WasmAccessArgsWrapperInner::MemoryDirectorMut { ptr } => {
                Wasm::memory_director_raw(ptr as isize) as *mut c_void
            }
            _ => unreachable!(),
        }
    }
}

pub trait WasmAccessWrapper: core::fmt::Debug {
    /// Copies a slice of data into WASM memory starting at the given offset.
    fn apply(&self, arg: WasmAccessArgsWrapperInner);

    #[cfg(not(feature = "multi_memory"))]
    fn memory_director(&self, arg: WasmAccessArgsWrapperInner) -> *const c_void;

    #[cfg(not(feature = "multi_memory"))]
    fn memory_director_mut(&self, arg: WasmAccessArgsWrapperInner) -> *mut c_void;

    fn _main(&self) -> wasip1::Errno;

    fn _reset(&self);

    fn _start(&self);
}

#[derive(Debug)]
pub struct DynamicLFSWrapper<Wasm: WasmAccessWrapper, LFS: Wasip1DynamicLFS> {
    pub lfs: LFS,
    pub wasm_wrapper: Wasm,
}

impl<Wasm: WasmAccessWrapper, LFS: Wasip1DynamicLFS> Wasip1DynCompatibleLFS
    for DynamicLFSWrapper<Wasm, LFS>
{
    type Inode = LFS::Inode;
    type Wasm = Wasm;

    fn fd_write_raw(
        &self,
        wasm: Self::Wasm,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno> {
        wasm.fd_write_integrate(self.lfs, data, data_len)
    }

    fn is_dir(&self, inode: Self::Inode) -> bool {
        wasm.is_dir(inode)
    }

    fn fd_readdir_raw(
        &self,
        wasm: Self::Wasm,
        inode: Self::Inode,
        buf: *mut u8,
        buf_len: usize,
        cookie: Dircookie,
    ) -> Result<(Size, Dircookie), wasip1::Errno> {
        wasm.fd_readdir_integrate(self.lfs, inode, buf, buf_len, cookie)
    }

    fn path_filestat_get_raw(
        &self,
        wasm: Self::Wasm,
        inode: Self::Inode,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno> {
        wasm.path_filestat_get_integrate(self.lfs, inode, flags, path_ptr, path_len)
    }

    fn fd_prestat_get_raw(
        &self,
        wasm: Self::Wasm,
        inode: Self::Inode,
    ) -> Result<wasip1::Prestat, wasip1::Errno> {
        wasm.fd_prestat_get_integrate(self.lfs, inode)
    }

    fn fd_prestat_dir_name_raw(
        &self,
        wasm: Self::Wasm,
        inode: Self::Inode,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        wasm.fd_prestat_dir_name_integrate(self.lfs, inode, dir_path_ptr, dir_path_len)
    }

    fn fd_filestat_get_raw(
        &self,
        wasm: Self::Wasm,
        inode: Self::Inode,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno> {
        wasm.fd_filestat_get_integrate(self.lfs, inode)
    }

    fn fd_pread_raw(
        &self,
        wasm: Self::Wasm,
        inode: Self::Inode,
        buf: *mut u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<Size, wasip1::Errno> {
        wasm.fd_pread_integrate(self.lfs, inode, buf, buf_len, offset)
    }
}

pub trait Wasip1DynCompatibleLFS: core::fmt::Debug {
    type Inode: 'static + core::fmt::Debug;
    type Wasm: WasmAccessWrapper;

    fn fd_write_raw(
        &self,
        wasm: Self::Wasm,
        // inode: Self::Inode,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno>;

    fn is_dir(&self, inode: Self::Inode) -> bool;

    fn fd_readdir_raw(
        &self,
        wasm: Self::Wasm,
        inode: Self::Inode,
        buf: *mut u8,
        buf_len: usize,
        cookie: Dircookie,
    ) -> Result<(Size, Dircookie), wasip1::Errno>;

    fn path_filestat_get_raw(
        &self,
        wasm: Self::Wasm,
        inode: Self::Inode,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno>;

    fn fd_prestat_get_raw(
        &self,
        wasm: Self::Wasm,
        inode: Self::Inode,
    ) -> Result<wasip1::Prestat, wasip1::Errno>;

    fn fd_prestat_dir_name_raw(
        &self,
        wasm: Self::Wasm,
        inode: Self::Inode,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> Result<(), wasip1::Errno>;

    fn fd_filestat_get_raw(
        &self,
        wasm: Self::Wasm,
        inode: Self::Inode,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno>;

    fn fd_pread_raw(
        &self,
        wasm: Self::Wasm,
        inode: Self::Inode,
        buf: *mut u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<Size, wasip1::Errno>;
}
