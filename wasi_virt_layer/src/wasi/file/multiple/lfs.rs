use core::any::Any;
use core::ffi::c_void;
use core::ops::Deref as _;

use smallvec::SmallVec;

use crate::{
    __private::wasip1::{self, *},
    file::{FilestatWithoutDevice, Wasip1FileSystem, multiple::wasm::WasmAccessDynCompatibleTuple},
    memory::{WasmAccess, WasmAccessDynCompatible, WasmAccessDynCompatibleRaw, WasmAccessName},
    wasi::file::{
        ConstDefault, Wasip1DynCompatibleLFS, Wasip1DynamicLFS,
        changeable::inode::{self, BoxedInode, DetailedOpenFd},
        constant::vfs::OpenFdInfoWithInode,
        multiple::{
            inode::{BoxedInodeNormal, DetailedDynamicOpenFd},
            wasm::WasmAccessDynCompatibleWrapper,
        },
    },
};

#[derive(Debug)]
pub struct Wasip1DynCompatibleLFSWrapper<B: BoxedInode> {
    pub lfs: alloc::boxed::Box<dyn Wasip1DynCompatibleLFS<B>>,
}

unsafe impl<B: BoxedInode> Send for Wasip1DynCompatibleLFSWrapper<B> {}
unsafe impl<B: BoxedInode> Sync for Wasip1DynCompatibleLFSWrapper<B> {}

impl<B: BoxedInode> Wasip1DynCompatibleLFSWrapper<B> {
    pub fn new(lfs: alloc::boxed::Box<dyn Wasip1DynCompatibleLFS<B>>) -> Self {
        Self { lfs }
    }
}

impl<B: BoxedInode> AsRef<dyn Wasip1DynCompatibleLFS<B>> for Wasip1DynCompatibleLFSWrapper<B> {
    fn as_ref(&self) -> &(dyn Wasip1DynCompatibleLFS<B> + 'static) {
        self.lfs.as_ref()
    }
}

impl<B: BoxedInode> core::ops::Deref for Wasip1DynCompatibleLFSWrapper<B> {
    type Target = dyn Wasip1DynCompatibleLFS<B>;

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

pub struct Wasip1MultipleVFS<B: BoxedInode = BoxedInodeNormal, OpenFd: OpenFdInfoWithInode + 'static = DetailedDynamicOpenFd> {
    pub lfss: SmallVec<[Wasip1DynCompatibleLFSWrapper<B>; 4]>,
    #[cfg(feature = "threads")]
    pub wasms: DashMap<smallstr::SmallString<[u8; 32]>, WasmAccessDynCompatibleWrapper>,
    #[cfg(not(feature = "threads"))]
    pub wasms:
        UnsafeCell<BTreeMap<smallstr::SmallString<[u8; 32]>, WasmAccessDynCompatibleWrapper>>,
    #[cfg(feature = "threads")]
    pub fd_map: DashMap<Fd, (usize, OpenFd)>,
    #[cfg(feature = "threads")]
    pub next_fd: std::sync::atomic::AtomicU32,
    #[cfg(not(feature = "threads"))]
    pub fd_map: UnsafeCell<BTreeMap<Fd, (usize, OpenFd)>>,
    #[cfg(not(feature = "threads"))]
    pub next_fd: UnsafeCell<Fd>,
}

unsafe impl<B: BoxedInode, OpenFd: OpenFdInfoWithInode + 'static> Send for Wasip1MultipleVFS<B, OpenFd> {}
unsafe impl<B: BoxedInode, OpenFd: OpenFdInfoWithInode + 'static> Sync for Wasip1MultipleVFS<B, OpenFd> {}

impl<B: BoxedInode, OpenFd: OpenFdInfoWithInode + 'static> core::fmt::Debug for Wasip1MultipleVFS<B, OpenFd> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut debug_struct = f.debug_struct("Wasip1MultipleVFS");
        debug_struct.field("lfss", &self.lfss);

        #[cfg(feature = "threads")]
        {
            debug_struct.field("wasms", &self.wasms);
            debug_struct.field("fd_map", &self.fd_map);
            debug_struct.field("next_fd", &self.next_fd);
        }
        #[cfg(not(feature = "threads"))]
        {
            debug_struct.field("wasms", unsafe { &*self.wasms.get() });
            debug_struct.field("fd_map", unsafe { &*self.fd_map.get() });
            debug_struct.field("next_fd", unsafe { &*self.next_fd.get() });
        }
        debug_struct.finish()
    }
}


impl<B: BoxedInode, OpenFd: OpenFdInfoWithInode + 'static> Wasip1MultipleVFS<B, OpenFd> {
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

    pub fn add_lfs(&mut self, lfs: alloc::boxed::Box<dyn Wasip1DynCompatibleLFS<B>>) {
        self.lfss.push(Wasip1DynCompatibleLFSWrapper::new(lfs));
    }

    pub fn add_wasm_access(
        &mut self,
        name: smallstr::SmallString<[u8; 32]>,
        access: WasmAccessDynCompatibleWrapper,
    ) {
        #[cfg(feature = "threads")]
        self.wasms.insert(name, access);
        #[cfg(not(feature = "threads"))]
        unsafe { &mut *self.wasms.get() }.insert(name, access);
    }

    pub fn add_wasm<Wasm: WasmAccessDynCompatibleTuple + WasmAccessName + ConstDefault + 'static>(&mut self) {
        self.add_wasm_access(<Wasm as WasmAccessName>::NAME.into(), WasmAccessDynCompatibleWrapper::new(Wasm::DEFAULT));
    }

    #[cfg(feature = "threads")]
    pub fn get_wasm_access(
        &self,
        name: &str,
    ) -> Option<impl core::ops::Deref<Target = WasmAccessDynCompatibleWrapper>> {
        self.wasms.get(name)
    }

    #[inline]
    fn set_cursor(&self, fd: Fd, cursor: usize) -> Option<()> {
        #[cfg(feature = "threads")]
        {
            self.fd_map.get_mut(&fd).map(|mut entry| {
                entry.value_mut().1.set_cursor(cursor);
            })
        }
        #[cfg(not(feature = "threads"))]
        {
            unsafe { &mut *self.fd_map.get() }
                .get_mut(&fd)
                .map(|entry: &mut (usize, OpenFd)| {
                    entry.1.set_cursor(cursor);
                })
        }
    }

    #[inline]
    fn remove_open_fd(&self, fd: Fd) -> Option<(usize, OpenFd)> {
        #[cfg(feature = "threads")]
        {
            self.fd_map.remove(&fd).map(|(_, v)| v)
        }
        #[cfg(not(feature = "threads"))]
        {
            unsafe { &mut *self.fd_map.get() }.remove(&fd)
        }
    }

    #[inline]
    fn allocate_fd(&self, lfs_idx: usize, open_fd: OpenFd) -> Fd {
        #[cfg(feature = "threads")]
        {
            let fd = self.next_fd.fetch_add(1, Ordering::SeqCst);
            self.fd_map.insert(fd, (lfs_idx, open_fd));
            fd
        }
        #[cfg(not(feature = "threads"))]
        {
            let fd = unsafe { *self.next_fd.get() };
            unsafe { *self.next_fd.get() += 1 };
            unsafe { &mut *self.fd_map.get() }.insert(fd, (lfs_idx, open_fd));
            fd
        }
    }

    pub fn add_fd(
        &self,
        lfs_idx: usize,
        inode: OpenFd::InodeId,
        base_rights: wasip1::Rights,
        inheriting_rights: wasip1::Rights,
    ) -> Fd {
        let mut open_fd = OpenFd::from_inode_id(inode);
        open_fd.set_base_rights(base_rights);
        open_fd.set_inheriting_rights(inheriting_rights);
        self.allocate_fd(lfs_idx, open_fd)
    }

    pub fn add_preopen_fd(
        &self,
        lfs_idx: usize,
        inode: OpenFd::InodeId,
    ) -> Fd {
        self.add_fd(
            lfs_idx,
            inode,
            wasip1::RIGHTS_FD_READ
                | wasip1::RIGHTS_FD_READDIR
                | wasip1::RIGHTS_FD_WRITE
                | wasip1::RIGHTS_PATH_CREATE_DIRECTORY
                | wasip1::RIGHTS_PATH_CREATE_FILE
                | wasip1::RIGHTS_PATH_FILESTAT_GET
                | wasip1::RIGHTS_PATH_FILESTAT_SET_SIZE
                | wasip1::RIGHTS_PATH_FILESTAT_SET_TIMES
                | wasip1::RIGHTS_PATH_LINK_SOURCE
                | wasip1::RIGHTS_PATH_LINK_TARGET
                | wasip1::RIGHTS_PATH_OPEN
                | wasip1::RIGHTS_PATH_READLINK
                | wasip1::RIGHTS_PATH_REMOVE_DIRECTORY
                | wasip1::RIGHTS_PATH_RENAME_SOURCE
                | wasip1::RIGHTS_PATH_RENAME_TARGET
                | wasip1::RIGHTS_PATH_SYMLINK
                | wasip1::RIGHTS_PATH_UNLINK_FILE,
            wasip1::RIGHTS_FD_READ
                | wasip1::RIGHTS_FD_READDIR
                | wasip1::RIGHTS_FD_WRITE
                | wasip1::RIGHTS_PATH_CREATE_DIRECTORY
                | wasip1::RIGHTS_PATH_CREATE_FILE
                | wasip1::RIGHTS_PATH_FILESTAT_GET
                | wasip1::RIGHTS_PATH_FILESTAT_SET_SIZE
                | wasip1::RIGHTS_PATH_FILESTAT_SET_TIMES
                | wasip1::RIGHTS_PATH_LINK_SOURCE
                | wasip1::RIGHTS_PATH_LINK_TARGET
                | wasip1::RIGHTS_PATH_OPEN
                | wasip1::RIGHTS_PATH_READLINK
                | wasip1::RIGHTS_PATH_REMOVE_DIRECTORY
                | wasip1::RIGHTS_PATH_RENAME_SOURCE
                | wasip1::RIGHTS_PATH_RENAME_TARGET
                | wasip1::RIGHTS_PATH_SYMLINK
                | wasip1::RIGHTS_PATH_UNLINK_FILE,
        )
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
        trace_fs!($self, $wasm; "get_access: wasm={}", <$wasm as WasmAccessName>::NAME);

        #[cfg(feature = "threads")]
        let $name = match $self.get_wasm_access(<$wasm as WasmAccessName>::NAME) {
            Some(a) => a,
            None => panic!("get_access: wasm={} not found", <$wasm as WasmAccessName>::NAME),
        };

        #[cfg(feature = "threads")]
        let $name = $name.deref();

        #[cfg(not(feature = "threads"))]
        let $name = match unsafe { &*$self.wasms.get() }.get(<$wasm as WasmAccessName>::NAME) {
            Some(a) => a,
            None => panic!("get_access: wasm={} not found", <$wasm as WasmAccessName>::NAME),
        };
    };
}

use crate::wasi::file::InodeIdCommon;

macro_rules! get_inode {
    ($inode:ident = $open_fd:ident) => {
        let $inode: &dyn InodeIdCommon = $open_fd.inode_id().as_inode();
    };
}

impl<B: BoxedInode, OpenFd: OpenFdInfoWithInode<InodeId = B> + 'static> Wasip1FileSystem
    for Wasip1MultipleVFS<B, OpenFd>
{
    fn fd_write_raw<Wasm: WasmAccess + WasmAccessName>(
        &self,
        fd: Fd,
        iovs_ptr: *const Ciovec,
        iovs_len: usize,
        nwritten_ret: *mut Size,
    ) -> wasip1::Errno {
        if fd != 2 {
            trace_fs!(self, Wasm; "fd_write: fd={fd}, iovs_len={iovs_len}");
        }
        get_access!(access = self, Wasm);

        match fd {
            0 => wasip1::ERRNO_BADF, // stdin is not writable
            1 | 2 => {
                if fd != 2 {
                    trace_fs!(self, Wasm; "fd_write to stdio: fd={fd}, iovs_len={iovs_len}");
                }
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

                get_inode!(inode = open_fd);

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
        nread_ret: *mut Size,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "fd_readdir: fd={fd}, buf_len={buf_len}, cookie={cookie}");
        get_access!(access = self, Wasm);
        get_open_fd!((open_fd, lfs) = self, fd);

        get_inode!(inode = open_fd);

        match lfs.fd_readdir_raw_dyn_compatible(
            access,
            inode,
            buf,
            buf_len,
            cookie,
        ) {
            Ok((read, _)) => {
                Wasm::store_le(nread_ret, read as Size);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }

    fn path_filestat_get_raw<Wasm: WasmAccess + WasmAccessName>(
        &self,
        fd: Fd,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
        filestat_ret: *mut wasip1::Filestat,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "path_filestat_get: fd={fd}, flags={flags}, path_len={path_len}");
        get_access!(access = self, Wasm);
        get_open_fd!((open_fd, lfs) = self, fd);

        get_inode!(inode = open_fd);

        match lfs.path_filestat_get_raw_dyn_compatible(
            access,
            inode,
            flags,
            path_ptr,
            path_len,
        ) {
            Ok(filestat) => {
                let filestat = wasip1::Filestat {
                    dev: 0,
                    ino: filestat.ino,
                    filetype: filestat.filetype,
                    nlink: filestat.nlink,
                    size: filestat.size,
                    atim: filestat.atim,
                    mtim: filestat.mtim,
                    ctim: filestat.ctim,
                };
                Wasm::store_le(filestat_ret, filestat);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }

    fn fd_prestat_get_raw<Wasm: WasmAccess + WasmAccessName>(
        &self,
        fd: Fd,
        prestat_ret: *mut wasip1::Prestat,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "fd_prestat_get: fd={fd}");
        get_access!(access = self, Wasm);
        get_open_fd!((open_fd, lfs) = self, fd);
        get_inode!(inode = open_fd);

        match lfs.fd_prestat_get_raw_dyn_compatible(access, inode) {
            Ok(prestat) => {
                trace_fs!(self, Wasm; "fd_prestat_get: storing prestat");
                Wasm::store_le(prestat_ret, prestat);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }

    fn fd_prestat_dir_name_raw<Wasm: WasmAccess + WasmAccessName>(
        &self,
        fd: Fd,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "fd_prestat_dir_name: fd={fd}, dir_path_len={dir_path_len}");
        get_access!(access = self, Wasm);
        get_open_fd!((open_fd, lfs) = self, fd);

        get_inode!(inode = open_fd);

        match lfs.fd_prestat_dir_name_raw_dyn_compatible(
            access,
            inode,
            dir_path_ptr,
            dir_path_len,
        ) {
            Ok(_) => wasip1::ERRNO_SUCCESS,
            Err(e) => e,
        }
    }

    fn fd_close_raw<Wasm: WasmAccess + WasmAccessName>(&self, fd: Fd) -> wasip1::Errno {
        trace_fs!(self, Wasm; "fd_close: fd={fd}");
        if fd < 3 {
            return wasip1::ERRNO_SUCCESS; // Ignore closing stdio
        }
        self.remove_open_fd(fd);
        wasip1::ERRNO_SUCCESS
    }

    fn fd_filestat_get_raw<Wasm: WasmAccess + WasmAccessName>(
        &self,
        fd: Fd,
        filestat_ret: *mut wasip1::Filestat,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "fd_filestat_get: fd={fd}");
        get_access!(access = self, Wasm);
        get_open_fd!((open_fd, lfs) = self, fd);

        get_inode!(inode = open_fd);

        match lfs.fd_filestat_get_raw_dyn_compatible(access, inode) {
            Ok(filestat) => {
                let filestat = wasip1::Filestat {
                    dev: 0,
                    ino: filestat.ino,
                    filetype: filestat.filetype,
                    nlink: filestat.nlink,
                    size: filestat.size,
                    atim: filestat.atim,
                    mtim: filestat.mtim,
                    ctim: filestat.ctim,
                };
                Wasm::store_le(filestat_ret, filestat);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }

    fn fd_fdstat_get_raw<Wasm: WasmAccess + WasmAccessName>(
        &self,
        fd: Fd,
        fdstat_ret: *mut wasip1::Fdstat,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "fd_fdstat_get: fd={fd}");
        get_access!(access = self, Wasm);
        match fd {
            0 | 1 | 2 => {
                let fdstat = wasip1::Fdstat {
                    fs_filetype: wasip1::FILETYPE_CHARACTER_DEVICE,
                    fs_flags: 0,
                    fs_rights_base: !0,
                    fs_rights_inheriting: !0,
                };
                Wasm::store_le(fdstat_ret, fdstat);
                wasip1::ERRNO_SUCCESS
            }
            fd => {
                get_open_fd!((open_fd, lfs) = self, fd);

                get_inode!(inode = open_fd);

                let filetype = match lfs.fd_filestat_get_raw_dyn_compatible(access, inode) {
                    Ok(f) => f.filetype,
                    Err(e) => return e,
                };

                let fdstat = wasip1::Fdstat {
                    fs_filetype: filetype,
                    fs_flags: open_fd.fd_flags(),
                    fs_rights_base: open_fd.base_rights(),
                    fs_rights_inheriting: open_fd.inheriting_rights(),
                };

                Wasm::store_le(fdstat_ret, fdstat);
                wasip1::ERRNO_SUCCESS
            }
        }
    }

    fn fd_read_raw<Wasm: WasmAccess + WasmAccessName>(
        &self,
        fd: Fd,
        iovs_ptr: *const Ciovec,
        iovs_len: usize,
        nread_ret: *mut Size,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "fd_read: fd={fd}, iovs_len={iovs_len}");
        get_access!(access = self, Wasm);
        get_open_fd!((open_fd, lfs) = self, fd);

        let mut read = 0;
        let mut cursor = open_fd.cursor();
        let iovs_vec = Wasm::as_array(iovs_ptr, iovs_len);

        get_inode!(inode = open_fd);
        for iovs in iovs_vec {
            match lfs.fd_pread_raw_dyn_compatible(access, inode, iovs.buf as *mut u8, iovs.buf_len, cursor)
            {
                Ok(r) => {
                    read += r;
                    cursor += r;
                }
                Err(e) => {
                    if read > 0 {
                        break;
                    }
                    return e;
                }
            }
        }

        self.set_cursor(fd, cursor);
        Wasm::store_le(nread_ret, read as Size);
        wasip1::ERRNO_SUCCESS
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
        trace_fs!(self, Wasm; "path_open: dir_fd={dir_fd}, dir_flags={dir_flags}, path_len={path_len}, o_flags={o_flags}, fs_rights_base={fs_rights_base}, fs_rights_inheriting={fs_rights_inheriting}, fd_flags={fd_flags}");
        get_access!(access = self, Wasm);

        #[cfg(feature = "threads")]
        let __bind = self.fd_map.get(&dir_fd);
        #[cfg(feature = "threads")]
        let (lfs_idx, open_fd) = match __bind.as_ref() {
            Some(entry) => entry.value(),
            None => return wasip1::ERRNO_BADF,
        };

        #[cfg(not(feature = "threads"))]
        let (lfs_idx, open_fd) = match unsafe { &*self.fd_map.get() }.get(&dir_fd) {
            Some(entry) => entry,
            None => return wasip1::ERRNO_BADF,
        };

        let lfs_idx = *lfs_idx;
        let lfs = &self.lfss[lfs_idx];

        get_inode!(inode = open_fd);

        match lfs.path_open_raw_dyn_compatible(
            access,
            inode,
            dir_flags,
            path_ptr,
            path_len,
            o_flags,
            fs_rights_base,
            fs_rights_inheriting,
            fd_flags,
        ) {
            Ok(new_inode) => {
                let mut new_open_fd = OpenFd::from_inode_id(new_inode);
                new_open_fd.set_cursor(0);
                new_open_fd.set_base_rights(fs_rights_base);
                new_open_fd.set_inheriting_rights(fs_rights_inheriting);
                new_open_fd.set_fd_flags(fd_flags);
                let new_fd = self.allocate_fd(lfs_idx, new_open_fd);
                Wasm::store_le(fd_ret, new_fd);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }
}
