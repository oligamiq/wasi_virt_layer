use core::any::Any;
use core::ffi::c_void;
use core::ops::Deref as _;

use smallvec::SmallVec;

use crate::{
    __private::wasip1::{self, *},
    file::{FilestatWithoutDevice, Wasip1FileSystem},
    memory::{WasmAccess, WasmAccessDynCompatible, WasmAccessDynCompatibleRaw, WasmAccessName},
    wasi::file::{
        ConstDefault, Wasip1DynCompatibleLFS, Wasip1DynamicLFS,
        changeable::inode::{self, DetailedOpenFd},
        constant::vfs::OpenFdInfoWithInode,
        multiple::{inode::DetailedDynamicOpenFd, wasm::WasmAccessDynCompatibleWrapper},
    },
};

#[derive(Debug)]
pub struct Wasip1DynCompatibleLFSWrapper {
    pub lfs: alloc::boxed::Box<dyn Wasip1DynCompatibleLFS>,
}

impl Wasip1DynCompatibleLFSWrapper {
    pub fn new(lfs: alloc::boxed::Box<dyn Wasip1DynCompatibleLFS>) -> Self {
        Self { lfs }
    }
}

impl AsRef<dyn Wasip1DynCompatibleLFS> for Wasip1DynCompatibleLFSWrapper {
    fn as_ref(&self) -> &(dyn Wasip1DynCompatibleLFS + 'static) {
        self.lfs.as_ref()
    }
}

impl core::ops::Deref for Wasip1DynCompatibleLFSWrapper {
    type Target = dyn Wasip1DynCompatibleLFS;

    fn deref(&self) -> &Self::Target {
        self.lfs.as_ref()
    }
}

#[cfg(feature = "threads")]
use dashmap::DashMap;

#[cfg(feature = "threads")]
use std::sync::atomic::{AtomicU32, Ordering};

#[cfg(not(feature = "threads"))]
use core::cell::UnsafeCell;

#[cfg(not(feature = "threads"))]
use alloc::collections::BTreeMap;

#[derive(Debug)]
pub struct Wasip1MultipleVFS<OpenFd: OpenFdInfoWithInode + 'static = DetailedDynamicOpenFd> {
    pub lfss: SmallVec<[Wasip1DynCompatibleLFSWrapper; 4]>,
    #[cfg(feature = "threads")]
    pub wasms: DashMap<smallstr::SmallString<[u8; 32]>, WasmAccessDynCompatibleWrapper>,
    #[cfg(not(feature = "threads"))]
    pub wasms:
        UnsafeCell<BTreeMap<smallstr::SmallString<[u8; 32]>, WasmAccessDynCompatibleWrapper>>,
    #[cfg(feature = "threads")]
    pub fd_map: DashMap<Fd, (usize, OpenFd)>,
    #[cfg(feature = "threads")]
    pub next_fd: AtomicU32,
    #[cfg(not(feature = "threads"))]
    pub fd_map: UnsafeCell<BTreeMap<Fd, (usize, OpenFd)>>,
    #[cfg(not(feature = "threads"))]
    pub next_fd: UnsafeCell<Fd>,
}

impl<OpenFd: OpenFdInfoWithInode + 'static> Wasip1MultipleVFS<OpenFd> {
    pub fn new() -> Self {
        Self {
            lfss: SmallVec::new(),
            #[cfg(feature = "threads")]
            wasms: DashMap::new(),
            #[cfg(not(feature = "threads"))]
            wasms: UnsafeCell::new(BTreeMap::new()),
            #[cfg(feature = "threads")]
            fd_map: DashMap::new(),
            #[cfg(feature = "threads")]
            next_fd: AtomicU32::new(3), // Start from 3, as 0, 1, 2 are reserved for stdio
            #[cfg(not(feature = "threads"))]
            fd_map: UnsafeCell::new(BTreeMap::new()),
            #[cfg(not(feature = "threads"))]
            next_fd: UnsafeCell::new(3), // Start from 3, as 0, 1, 2 are reserved for stdio
        }
    }

    pub fn add_lfs(&mut self, lfs: alloc::boxed::Box<dyn Wasip1DynCompatibleLFS>) {
        self.lfss.push(Wasip1DynCompatibleLFSWrapper::new(lfs));
    }

    pub fn add_wasm_access(
        &mut self,
        name: smallstr::SmallString<[u8; 32]>,
        access: WasmAccessDynCompatibleWrapper,
    ) {
        self.wasms.insert(name, access);
    }

    #[cfg(feature = "threads")]
    pub fn get_wasm_access(
        &self,
        name: &str,
    ) -> Option<impl core::ops::Deref<Target = WasmAccessDynCompatibleWrapper>> {
        self.wasms.get(name)
    }
}

use crate::wasi::file::constant::vfs_impl::trace_fs;

macro_rules! get_open_fd {
    (($name:ident, $lfs:ident) = $self:ident, $fd:ident) => {
        trace_fs!($self, Wasm; "get_open_fd: fd={}", $fd);

        #[cfg(feature = "threads")]
        let __bind = $self.fd_map.get(&$fd);
        #[cfg(feature = "threads")]
        let ($lfs, $name) = match __bind.as_ref() {
            Some(entry) => entry.value(),
            None => return wasip1::ERRNO_BADF,
        };

        #[cfg(not(feature = "threads"))]
        let ($lfs, $name) = match unsafe { &*$self.fd_map.get() }.get(&$fd) {
            Some(entry) => entry,
            None => return wasip1::ERRNO_BADF,
        };

        let $lfs = &$self.lfss[*$lfs];
    };
}

macro_rules! get_access {
    ($name:ident = $self:ident, $wasm:ty) => {
        #[cfg(feature = "threads")]
        let $name = match $self.get_wasm_access(<$wasm as WasmAccessName>::NAME) {
            Some(a) => a,
            None => return wasip1::ERRNO_BADF,
        };

        #[cfg(feature = "threads")]
        let $name = $name.deref();

        #[cfg(not(feature = "threads"))]
        let $name = match unsafe { &*$self.wasms }.get(<$wasm as WasmAccessName>::NAME) {
            Some(a) => a,
            None => return wasip1::ERRNO_BADF,
        };
    };
}

impl<OpenFd: OpenFdInfoWithInode + 'static> Wasip1FileSystem for Wasip1MultipleVFS<OpenFd> {
    fn fd_write_raw<Wasm: WasmAccess + WasmAccessName>(
        &self,
        fd: Fd,
        iovs_ptr: *const Ciovec,
        iovs_len: usize,
        nwritten_ret: *mut Size,
    ) -> wasip1::Errno {
        get_access!(access = self, Wasm);

        match fd {
            0 => wasip1::ERRNO_BADF, // stdin is not writable
            1 | 2 => {
                let iovs_vec = Wasm::as_array(iovs_ptr, iovs_len);
                let mut written = 0;
                for iovs in iovs_vec {
                    match fd {
                        1 => match self
                            .lfss
                            .first()
                            .unwrap()
                            .fd_write_stdout_raw_dyn_compatible(
                                access,
                                iovs.buf as *const u8,
                                iovs.buf_len,
                            ) {
                            Ok(w) => written += w,
                            Err(e) => return e,
                        },
                        2 => match self
                            .lfss
                            .first()
                            .unwrap()
                            .fd_write_stderr_raw_dyn_compatible(
                                access,
                                iovs.buf as *const u8,
                                iovs.buf_len,
                            ) {
                            Ok(w) => written += w,
                            Err(e) => return e,
                        },
                        _ => unreachable!(),
                    }
                }
                Wasm::store_le(nwritten_ret, written as Size);
                wasip1::ERRNO_SUCCESS
            }
            fd => {
                get_open_fd!((open_fd, lfs) = self, fd);

                let inode = open_fd.inode_id();

                let iovs_vec = Wasm::as_array(iovs_ptr, iovs_len);

                let mut written = 0;
                for iovs in iovs_vec {
                    match lfs.fd_write_raw_dyn_compatible(
                        access,
                        inode,
                        iovs.buf as *const u8,
                        iovs.buf_len,
                    ) {
                        Ok(w) => written += w,
                        Err(e) => return e,
                    }
                }

                Wasm::store_le(nwritten_ret, written as Size);
                wasip1::ERRNO_SUCCESS
            }
        }
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
