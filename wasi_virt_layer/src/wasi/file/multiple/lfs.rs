use core::any::Any;
use core::ffi::c_void;

use smallvec::SmallVec;

use crate::{
    __private::wasip1::{self, *},
    file::{FilestatWithoutDevice, Wasip1FileSystem},
    memory::{WasmAccess, WasmAccessDynCompatible, WasmAccessDynCompatibleRaw, WasmAccessName},
    wasi::file::{Wasip1DynamicLFS, multiple::wasm::WasmAccessMiddleIntegrator},
};

// pub(crate) struct WasmAccessWrapperBase<'a, WasmWrapper: WasmAccessWrapper> {
//     pub inner: &'a WasmWrapper,
// }

// pub(crate) enum WasmAccessArgsWrapperInner<'a> {
//     Memcpy {
//         t_size: u8,
//         offset: *mut c_void,
//         data: &'a c_void,
//         data_len: usize,
//     },
//     MemcpyTo {
//         t_size: u8,
//         offset: &'a mut c_void,
//         src: *const c_void,
//         data_len: usize,
//     },
//     StoreLe {
//         t_size: u8,
//         offset: *mut c_void,
//         value: &'a c_void,
//     },
//     LoadLe {
//         t_size: u8,
//         offset: *const c_void,
//         ret: &'a mut c_void,
//     },
//     #[cfg(not(feature = "multi_memory"))]
//     MemoryDirector { ptr: *const c_void },
//     #[cfg(not(feature = "multi_memory"))]
//     MemoryDirectorMut { ptr: *mut c_void },
// }

// impl<'a, WasmWrapper: WasmAccessWrapper> WasmAccessWrapperBase<'a, WasmWrapper> {
//     pub fn memcpy<T: 'static>(&self, offset: *mut T, data: &'a [T]) {
//         let cmd = WasmAccessArgsWrapperInner::Memcpy {
//             t_size: core::mem::size_of::<T>() as u8,
//             offset: offset as *mut c_void,
//             data: unsafe { &*(data.as_ptr() as *const c_void) } as &c_void,
//             data_len: data.len(),
//         };
//         self.inner.apply(cmd);
//     }

//     pub fn memcpy_to<T: 'static>(&self, offset: &mut [T], src: *const T) {
//         let cmd = WasmAccessArgsWrapperInner::MemcpyTo {
//             t_size: core::mem::size_of::<T>() as u8,
//             offset: unsafe { &mut *(offset.as_mut_ptr() as *mut c_void) } as &mut c_void,
//             src: src as *const c_void,
//             data_len: offset.len(),
//         };
//         self.inner.apply(cmd);
//     }

//     pub fn store_le<T: 'static>(&self, offset: *mut T, value: &'a impl core::borrow::Borrow<T>) {
//         let cmd = WasmAccessArgsWrapperInner::StoreLe {
//             t_size: core::mem::size_of::<T>() as u8,
//             offset: offset as *mut c_void,
//             value: unsafe { &*(value.borrow() as *const T as *const c_void) } as &c_void,
//         };
//         self.inner.apply(cmd);
//     }

//     pub fn load_le<T: 'static + core::fmt::Debug + Copy>(&self, offset: *const T) -> T {
//         let holder = core::mem::MaybeUninit::<T>::uninit();
//         let cmd = WasmAccessArgsWrapperInner::LoadLe {
//             t_size: core::mem::size_of::<T>() as u8,
//             offset: offset as *const c_void,
//             ret: unsafe { &mut *(holder.as_ptr() as *mut c_void) } as &mut c_void,
//         };
//         self.inner.apply(cmd);
//         unsafe { core::ptr::read(holder.as_ptr() as *const T) }
//     }

//     #[cfg(not(feature = "multi_memory"))]
//     pub fn memory_director<T>(&self, ptr: *const T) -> *const T {
//         let cmd = WasmAccessArgsWrapperInner::MemoryDirector {
//             ptr: ptr as *const c_void,
//         };
//         let res = self.inner.memory_director(cmd);
//         res as *const T
//     }

//     #[cfg(not(feature = "multi_memory"))]
//     pub fn memory_director_mut<T>(&self, ptr: *mut T) -> *mut T {
//         let cmd = WasmAccessArgsWrapperInner::MemoryDirectorMut {
//             ptr: ptr as *mut c_void,
//         };
//         let res = self.inner.memory_director_mut(cmd);
//         res as *mut T
//     }
// }

// impl<'a> WasmAccessArgsWrapperInner<'a> {
//     pub fn apply<Wasm: WasmAccessDynCompatibleRaw>(self) {
//         match self {
//             WasmAccessArgsWrapperInner::Memcpy {
//                 t_size,
//                 offset,
//                 data,
//                 data_len,
//             } => {
//                 // second, we need to call the memcpy function with the correct type
//                 Wasm::memcpy_raw(
//                     offset as *mut u8,
//                     data as *const c_void as *const u8,
//                     data_len * t_size as usize,
//                 );
//             }
//             WasmAccessArgsWrapperInner::MemcpyTo {
//                 t_size,
//                 offset,
//                 src,
//                 data_len,
//             } => {
//                 Wasm::memcpy_to_raw(
//                     offset as *mut c_void as *mut u8,
//                     src as *const c_void as *const u8,
//                     data_len * t_size as usize,
//                 );
//             }
//             WasmAccessArgsWrapperInner::StoreLe {
//                 t_size,
//                 offset,
//                 value,
//             } => {
//                 Wasm::memcpy_raw(
//                     offset as *mut u8,
//                     value as *const c_void as *const u8,
//                     t_size as usize,
//                 );
//             }
//             WasmAccessArgsWrapperInner::LoadLe {
//                 t_size,
//                 offset,
//                 ret,
//             } => {
//                 Wasm::memcpy_to_raw(
//                     ret as *mut c_void as *mut u8,
//                     offset as *const c_void as *const u8,
//                     t_size as usize,
//                 );
//             }
//             #[cfg(not(feature = "multi_memory"))]
//             _ => unreachable!(),
//         }
//     }

//     pub fn memory_director<Wasm: WasmAccessDynCompatibleRaw>(self) -> *const c_void {
//         match self {
//             #[cfg(not(feature = "multi_memory"))]
//             WasmAccessArgsWrapperInner::MemoryDirector { ptr } => {
//                 Wasm::memory_director_raw(ptr as isize) as *const c_void
//             }
//             #[cfg(not(feature = "multi_memory"))]
//             WasmAccessArgsWrapperInner::MemoryDirectorMut { ptr } => {
//                 Wasm::memory_director_raw(ptr as isize) as *mut c_void
//             }
//             _ => unreachable!(),
//         }
//     }
// }

// pub trait WasmAccessWrapper: core::fmt::Debug {
//     /// Copies a slice of data into WASM memory starting at the given offset.
//     fn apply(&self, arg: WasmAccessArgsWrapperInner);

//     #[cfg(not(feature = "multi_memory"))]
//     fn memory_director(&self, arg: WasmAccessArgsWrapperInner) -> *const c_void;

//     #[cfg(not(feature = "multi_memory"))]
//     fn memory_director_mut(&self, arg: WasmAccessArgsWrapperInner) -> *mut c_void;
// }

pub struct Wasip1DynCompatibleLFSWrapper<WasmIntegrator: WasmAccessMiddleIntegrator> {
    pub lfs: alloc::boxed::Box<dyn Wasip1DynCompatibleLFS<WasmIntegrator = WasmIntegrator>>,
}

impl<WasmIntegrator: WasmAccessMiddleIntegrator> Wasip1DynCompatibleLFSWrapper<WasmIntegrator> {
    pub fn new(
        lfs: alloc::boxed::Box<dyn Wasip1DynCompatibleLFS<WasmIntegrator = WasmIntegrator>>,
    ) -> Self {
        Self { lfs }
    }

    pub fn fd_write_raw(
        &self,
        key: &<WasmIntegrator as WasmAccessMiddleIntegrator>::Key,
        inode: &dyn Any,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno> {
        self.lfs.fd_write_raw(key, inode, data, data_len)
    }

    pub fn is_dir(
        &self,
        key: &<WasmIntegrator as WasmAccessMiddleIntegrator>::Key,
        inode: &dyn Any,
    ) -> bool {
        self.lfs.is_dir(key, inode)
    }
}

#[derive(Debug)]
pub struct Wasip1MultipleVFS<WasmIntegrator: WasmAccessMiddleIntegrator> {
    pub lfss: SmallVec<
        [alloc::boxed::Box<dyn Wasip1DynCompatibleLFS<WasmIntegrator = WasmIntegrator>>; 4],
    >,
    pub integrator: WasmIntegrator,
}

impl<WasmIntegrator: WasmAccessMiddleIntegrator> Wasip1FileSystem
    for Wasip1MultipleVFS<WasmIntegrator>
{
    fn fd_write_raw<Wasm: WasmAccess + WasmAccessName>(
        &self,
        fd: Fd,
        iovs_ptr: *const Ciovec,
        iovs_len: usize,
        nwritten: *mut Size,
    ) -> wasip1::Errno {
        let key = WasmIntegrator::generate_key(&self.integrator, Wasm::NAME);

        todo!();
    }

    fn fd_readdir_raw<Wasm: WasmAccess + WasmAccessName>(
        &self,
        fd: Fd,
        buf: *mut u8,
        buf_len: usize,
        cookie: Dircookie,
        nread: *mut Size,
    ) -> wasip1::Errno {
        todo!()
    }

    fn path_filestat_get_raw<Wasm: WasmAccess + WasmAccessName>(
        &self,
        fd: Fd,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
        filestat: *mut wasip1::Filestat,
    ) -> wasip1::Errno {
        todo!()
    }

    fn fd_prestat_get_raw<Wasm: WasmAccess + WasmAccessName>(
        &self,
        fd: Fd,
        prestat: *mut wasip1::Prestat,
    ) -> wasip1::Errno {
        todo!()
    }

    fn fd_prestat_dir_name_raw<Wasm: WasmAccess + WasmAccessName>(
        &self,
        fd: Fd,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> wasip1::Errno {
        todo!()
    }

    fn fd_close_raw<Wasm: WasmAccess + WasmAccessName>(&self, fd: Fd) -> wasip1::Errno {
        todo!()
    }

    fn fd_filestat_get_raw<Wasm: WasmAccess + WasmAccessName>(
        &self,
        fd: Fd,
        filestat: *mut wasip1::Filestat,
    ) -> wasip1::Errno {
        todo!()
    }

    fn fd_fdstat_get_raw<Wasm: WasmAccess + WasmAccessName>(
        &self,
        fd: Fd,
        fdstat: *mut wasip1::Fdstat,
    ) -> wasip1::Errno {
        todo!()
    }

    fn fd_read_raw<Wasm: WasmAccess + WasmAccessName>(
        &self,
        fd: Fd,
        iovs_ptr: *const Ciovec,
        iovs_len: usize,
        nread: *mut Size,
    ) -> wasip1::Errno {
        todo!()
    }

    fn path_open_raw<Wasm: WasmAccess + WasmAccessName>(
        &self,
        dir_fd: Fd,
        dir_flags: wasip1::Fdflags,
        path_ptr: *const u8,
        path_len: usize,
        o_flags: wasip1::Oflags,
        fs_rights_base: wasip1::Rights,
        fs_rights_inheriting: wasip1::Rights,
        fd_flags: wasip1::Fdflags,
        fd_ret: *mut wasip1::Fd,
    ) -> wasip1::Errno {
        todo!()
    }
}

#[derive(Debug)]
pub struct DynamicLFSWrapper<Wasm: WasmAccessMiddleIntegrator, LFS: Wasip1DynamicLFS> {
    pub lfs: LFS,
    pub wasm_integrator: Wasm,
    __phantom: core::marker::PhantomData<Wasm>,
}

impl<WasmIntegrator: WasmAccessMiddleIntegrator, LFS: Wasip1DynamicLFS> Wasip1DynCompatibleLFS
    for DynamicLFSWrapper<WasmIntegrator, LFS>
{
    type WasmIntegrator = WasmIntegrator;

    fn fd_write_raw(
        &self,
        key: &<Self::WasmIntegrator as WasmAccessMiddleIntegrator>::Key,
        inode: &dyn Any,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno> {
        let inode = inode.downcast_ref::<LFS::Inode>().unwrap();

        self.wasm_integrator
            .fd_write_integrate(key, &self.lfs, inode, data, data_len)
    }

    fn fd_write_stdout_raw(
        &self,
        key: &<Self::WasmIntegrator as WasmAccessMiddleIntegrator>::Key,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno> {
        self.wasm_integrator
            .fd_write_stdout_integrate(key, &self.lfs, data, data_len)
    }

    fn fd_write_stderr_raw(
        &self,
        key: &<Self::WasmIntegrator as WasmAccessMiddleIntegrator>::Key,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno> {
        self.wasm_integrator
            .fd_write_stderr_integrate(key, &self.lfs, data, data_len)
    }

    fn is_dir(
        &self,
        key: &<Self::WasmIntegrator as WasmAccessMiddleIntegrator>::Key,
        inode: &dyn Any,
    ) -> bool {
        let inode = inode.downcast_ref::<LFS::Inode>().unwrap();

        self.wasm_integrator
            .fd_filestat_get_integrate(key, &self.lfs, inode)
            .map(|filestat| filestat.filetype == wasip1::FILETYPE_DIRECTORY)
            .unwrap_or(false)
    }

    fn fd_readdir_raw(
        &self,
        key: &<Self::WasmIntegrator as WasmAccessMiddleIntegrator>::Key,
        inode: &dyn Any,
        buf: *mut u8,
        buf_len: usize,
        cookie: Dircookie,
    ) -> Result<(Size, Dircookie), wasip1::Errno> {
        let inode = inode.downcast_ref::<LFS::Inode>().unwrap();

        self.wasm_integrator
            .fd_readdir_integrate(key, &self.lfs, inode, buf, buf_len, cookie)
    }

    fn path_filestat_get_raw(
        &self,
        key: &<Self::WasmIntegrator as WasmAccessMiddleIntegrator>::Key,
        inode: &dyn Any,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno> {
        let inode = inode.downcast_ref::<LFS::Inode>().unwrap();

        self.wasm_integrator
            .path_filestat_get_integrate(key, &self.lfs, inode, flags, path_ptr, path_len)
    }

    fn fd_prestat_get_raw(
        &self,
        key: &<Self::WasmIntegrator as WasmAccessMiddleIntegrator>::Key,
        inode: &dyn Any,
    ) -> Result<wasip1::Prestat, wasip1::Errno> {
        let inode = inode.downcast_ref::<LFS::Inode>().unwrap();

        self.wasm_integrator
            .fd_prestat_get_integrate(key, &self.lfs, inode)
    }

    fn fd_prestat_dir_name_raw(
        &self,
        key: &<Self::WasmIntegrator as WasmAccessMiddleIntegrator>::Key,
        inode: &dyn Any,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        let inode = inode.downcast_ref::<LFS::Inode>().unwrap();

        self.wasm_integrator.fd_prestat_dir_name_integrate(
            key,
            &self.lfs,
            inode,
            dir_path_ptr,
            dir_path_len,
        )
    }

    fn fd_filestat_get_raw(
        &self,
        key: &<Self::WasmIntegrator as WasmAccessMiddleIntegrator>::Key,
        inode: &dyn Any,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno> {
        let inode = inode.downcast_ref::<LFS::Inode>().unwrap();

        self.wasm_integrator
            .fd_filestat_get_integrate(key, &self.lfs, inode)
    }

    fn fd_pread_raw(
        &self,
        key: &<Self::WasmIntegrator as WasmAccessMiddleIntegrator>::Key,
        inode: &dyn Any,
        buf: *mut u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<Size, wasip1::Errno> {
        let inode = inode.downcast_ref::<LFS::Inode>().unwrap();

        self.wasm_integrator
            .fd_pread_integrate(key, &self.lfs, inode, buf, buf_len, offset)
    }

    fn fd_read_stdin_raw(
        &self,
        key: &<Self::WasmIntegrator as WasmAccessMiddleIntegrator>::Key,
        buf: *mut u8,
        buf_len: usize,
    ) -> Result<Size, wasip1::Errno> {
        self.wasm_integrator
            .fd_read_stdin_integrate(key, &self.lfs, buf, buf_len)
    }

    fn path_open_raw(
        &self,
        key: &<Self::WasmIntegrator as WasmAccessMiddleIntegrator>::Key,
        dir_inode: &dyn Any,
        dir_flags: wasip1::Fdflags,
        path_ptr: *const u8,
        path_len: usize,
        o_flags: wasip1::Oflags,
        fs_rights_base: wasip1::Rights,
        fs_rights_inheriting: wasip1::Rights,
        fd_flags: wasip1::Fdflags,
    ) -> Result<(alloc::boxed::Box<dyn Any>, wasip1::Fd), wasip1::Errno> {
        let dir_inode = dir_inode.downcast_ref::<LFS::Inode>().unwrap();

        let (inode, fd) = self.wasm_integrator.path_open_integrate(
            key,
            &self.lfs,
            dir_inode,
            dir_flags,
            path_ptr,
            path_len,
            o_flags,
            fs_rights_base,
            fs_rights_inheriting,
            fd_flags,
        )?;
        Ok((alloc::boxed::Box::new(inode), fd))
    }
}

pub trait Wasip1DynCompatibleLFS: core::fmt::Debug {
    type WasmIntegrator: WasmAccessMiddleIntegrator;

    fn fd_write_raw(
        &self,
        key: &<Self::WasmIntegrator as WasmAccessMiddleIntegrator>::Key,
        inode: &dyn Any,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno>;

    fn fd_write_stdout_raw(
        &self,
        key: &<Self::WasmIntegrator as WasmAccessMiddleIntegrator>::Key,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno> {
        self.fd_write_raw(key, &(), data, data_len)
    }

    fn fd_write_stderr_raw(
        &self,
        key: &<Self::WasmIntegrator as WasmAccessMiddleIntegrator>::Key,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno> {
        self.fd_write_raw(key, &(), data, data_len)
    }

    fn is_dir(
        &self,
        key: &<Self::WasmIntegrator as WasmAccessMiddleIntegrator>::Key,
        inode: &dyn Any,
    ) -> bool;

    fn fd_readdir_raw(
        &self,
        key: &<Self::WasmIntegrator as WasmAccessMiddleIntegrator>::Key,
        inode: &dyn Any,
        buf: *mut u8,
        buf_len: usize,
        cookie: Dircookie,
    ) -> Result<(Size, Dircookie), wasip1::Errno>;

    fn path_filestat_get_raw(
        &self,
        key: &<Self::WasmIntegrator as WasmAccessMiddleIntegrator>::Key,
        inode: &dyn Any,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno>;

    fn fd_prestat_get_raw(
        &self,
        key: &<Self::WasmIntegrator as WasmAccessMiddleIntegrator>::Key,
        inode: &dyn Any,
    ) -> Result<wasip1::Prestat, wasip1::Errno>;

    fn fd_prestat_dir_name_raw(
        &self,
        key: &<Self::WasmIntegrator as WasmAccessMiddleIntegrator>::Key,
        inode: &dyn Any,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> Result<(), wasip1::Errno>;

    fn fd_filestat_get_raw(
        &self,
        key: &<Self::WasmIntegrator as WasmAccessMiddleIntegrator>::Key,
        inode: &dyn Any,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno>;

    fn fd_pread_raw(
        &self,
        key: &<Self::WasmIntegrator as WasmAccessMiddleIntegrator>::Key,
        inode: &dyn Any,
        buf: *mut u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<Size, wasip1::Errno>;

    fn fd_read_stdin_raw(
        &self,
        key: &<Self::WasmIntegrator as WasmAccessMiddleIntegrator>::Key,
        buf: *mut u8,
        buf_len: usize,
    ) -> Result<Size, wasip1::Errno> {
        self.fd_pread_raw(key, &(), buf, buf_len, 0)
    }

    fn path_open_raw(
        &self,
        key: &<Self::WasmIntegrator as WasmAccessMiddleIntegrator>::Key,
        dir_inode: &dyn Any,
        dir_flags: wasip1::Fdflags,
        path_ptr: *const u8,
        path_len: usize,
        o_flags: wasip1::Oflags,
        fs_rights_base: wasip1::Rights,
        fs_rights_inheriting: wasip1::Rights,
        fd_flags: wasip1::Fdflags,
    ) -> Result<(alloc::boxed::Box<dyn Any>, wasip1::Fd), wasip1::Errno>;
}
