#![cfg(feature = "dynamic-fs")]

use crate::__private::wasip1;
use crate::__private::wasip1::{Ciovec, Dircookie, Fd, Size};

use crate::wasi::file::dynamic::inode::DetailedOpenFd;
use crate::wasi::file::trace::trace_fs;
use crate::wasi::file::{DynamicLFS, InodeIdCommon, OpenFdInfoWithInode, Wasip1LFSBase};
use crate::{memory::WasmAccess, wasi::file::Wasip1FileSystem};

#[cfg(feature = "threads")]
use dashmap::DashMap;

#[cfg(feature = "threads")]
use std::sync::atomic::{AtomicU32, Ordering};

#[cfg(not(feature = "threads"))]
use core::cell::UnsafeCell;

#[cfg(not(feature = "threads"))]
use alloc::collections::BTreeMap;

/// A virtual file system implementation that maps file descriptors to inodes in a StandardDynamicLFS.
pub struct StandardDynamicFileSystem<
    LFS: DynamicLFS + core::fmt::Debug,
    OpenFd: OpenFdInfoWithInode + 'static = DetailedOpenFd<<LFS as Wasip1LFSBase>::Inode>,
> where
    LFS::Inode: InodeIdCommon,
{
    pub lfs: LFS,
    #[cfg(feature = "threads")]
    pub fd_map: DashMap<Fd, OpenFd>,
    #[cfg(feature = "threads")]
    pub next_fd: AtomicU32,
    #[cfg(not(feature = "threads"))]
    pub fd_map: UnsafeCell<BTreeMap<Fd, OpenFd>>,
    #[cfg(not(feature = "threads"))]
    pub next_fd: UnsafeCell<Fd>,
}

impl<LFS: DynamicLFS + core::fmt::Debug, OpenFd: OpenFdInfoWithInode + 'static> core::fmt::Debug
    for StandardDynamicFileSystem<LFS, OpenFd>
where
    LFS::Inode: InodeIdCommon,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("StandardDynamicFileSystem")
            .field("lfs", &self.lfs)
            .field("fd_map", &{
                #[cfg(feature = "threads")]
                {
                    &self.fd_map
                }
                #[cfg(not(feature = "threads"))]
                {
                    use alloc::collections::BTreeMap;
                    unsafe { &*self.fd_map.get() }
                        .iter()
                        .map(|(fd, open_fd)| (*fd, open_fd.inode_id()))
                        .collect::<BTreeMap<_, _>>()
                }
            })
            .field("next_fd", &{
                #[cfg(feature = "threads")]
                {
                    self.next_fd.load(Ordering::SeqCst)
                }
                #[cfg(not(feature = "threads"))]
                {
                    unsafe { *self.next_fd.get() }
                }
            })
            .finish()
    }
}

unsafe impl<LFS: DynamicLFS + core::fmt::Debug, OpenFd: OpenFdInfoWithInode + 'static> Send
    for StandardDynamicFileSystem<LFS, OpenFd>
where
    LFS::Inode: InodeIdCommon,
{
}
unsafe impl<LFS: DynamicLFS + core::fmt::Debug, OpenFd: OpenFdInfoWithInode + 'static> Sync
    for StandardDynamicFileSystem<LFS, OpenFd>
where
    LFS::Inode: InodeIdCommon,
{
}

macro_rules! get_open_fd {
    ($name:ident = $self:ident, $fd:ident) => {
        trace_fs!($self, Wasm; "get_open_fd: fd={}", $fd);

        #[cfg(feature = "threads")]
        let __bind = $self.fd_map.get(&$fd);
        #[cfg(feature = "threads")]
        let $name = match __bind.as_ref() {
            Some(entry) => entry.value(),
            None => return wasip1::ERRNO_BADF,
        };

        #[cfg(not(feature = "threads"))]
        let $name = match unsafe { &*$self.fd_map.get() }.get(&$fd) {
            Some(entry) => entry,
            None => return wasip1::ERRNO_BADF,
        };
    };
}

impl<
    LFS: DynamicLFS + core::fmt::Debug,
    OpenFd: OpenFdInfoWithInode<InodeId = LFS::Inode> + 'static,
> StandardDynamicFileSystem<LFS, OpenFd>
where
    LFS::Inode: InodeIdCommon,
{
    pub fn new(lfs: LFS) -> Self {
        Self {
            lfs,
            #[cfg(feature = "threads")]
            fd_map: DashMap::new(),
            #[cfg(feature = "threads")]
            next_fd: AtomicU32::new(3),
            #[cfg(not(feature = "threads"))]
            fd_map: UnsafeCell::new(BTreeMap::new()),
            #[cfg(not(feature = "threads"))]
            next_fd: UnsafeCell::new(3),
        }
    }

    #[inline]
    fn set_cursor(&self, fd: Fd, cursor: usize) -> Option<()> {
        #[cfg(feature = "threads")]
        {
            self.fd_map.get_mut(&fd).map(|mut entry| {
                entry.value_mut().set_cursor(cursor);
            })
        }
        #[cfg(not(feature = "threads"))]
        {
            unsafe { &mut *self.fd_map.get() }
                .get_mut(&fd)
                .map(|entry: &mut OpenFd| {
                    entry.set_cursor(cursor);
                })
        }
    }

    #[inline]
    fn remove_open_fd(&self, fd: Fd) -> Option<OpenFd> {
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
    fn allocate_fd(&self, open_fd: OpenFd) -> Fd {
        #[cfg(feature = "threads")]
        {
            let fd = self.next_fd.fetch_add(1, Ordering::SeqCst);
            self.fd_map.insert(fd, open_fd);
            fd
        }
        #[cfg(not(feature = "threads"))]
        {
            let fd = unsafe { *self.next_fd.get() };
            unsafe { *self.next_fd.get() += 1 };
            unsafe { &mut *self.fd_map.get() }.insert(fd, open_fd);
            fd
        }
    }

    fn allocate_fd_integrated(&self, f: impl FnOnce(Fd) -> OpenFd) -> Fd {
        #[cfg(feature = "threads")]
        {
            let fd = self.next_fd.fetch_add(1, Ordering::SeqCst);
            let open_fd = f(fd);
            self.fd_map.insert(fd, open_fd);
            fd
        }
        #[cfg(not(feature = "threads"))]
        {
            let fd = unsafe { *self.next_fd.get() };
            unsafe { *self.next_fd.get() += 1 };
            let open_fd = f(fd);
            unsafe { &mut *self.fd_map.get() }.insert(fd, open_fd);
            fd
        }
    }

    /// Adds a dynamically created preopen to the VFS and returns its new file descriptor.
    pub fn add_fd(
        &self,
        inode: LFS::Inode,
        base_rights: wasip1::Rights,
        inheriting_rights: wasip1::Rights,
    ) -> Fd {
        self.allocate_fd_integrated(|_fd| {
            let mut open_fd = OpenFd::from_inode_id(inode);
            open_fd.set_base_rights(base_rights);
            open_fd.set_inheriting_rights(inheriting_rights);
            open_fd
        })
    }

    /// Removes a dynamically created preopen or fd from the VFS.
    pub fn remove_fd(&self, fd: Fd) {
        self.remove_open_fd(fd);
    }
}

impl<
    LFS: DynamicLFS + core::fmt::Debug,
    OpenFd: OpenFdInfoWithInode<InodeId = LFS::Inode> + 'static,
> Wasip1FileSystem for StandardDynamicFileSystem<LFS, OpenFd>
where
    LFS::Inode: InodeIdCommon,
{
    fn fd_readdir_raw<Wasm: WasmAccess>(
        &self,
        fd: Fd,
        mut buf: *mut u8,
        mut buf_len: usize,
        mut cookie: Dircookie,
        nread_ret: *mut Size,
    ) -> wasip1::Errno {
        get_open_fd!(open_fd = self, fd);

        trace_fs!(self, Wasm; "fd_readdir: fd={fd}, buf_len={buf_len}, cookie={cookie}");

        if !self.lfs.is_dir(open_fd.inode_id()) {
            return wasip1::ERRNO_NOTDIR;
        }

        let mut total_read = 0;
        loop {
            match self
                .lfs
                .fd_readdir_raw::<Wasm>(open_fd.inode_id(), buf, buf_len, cookie)
            {
                Ok((0, _)) => {
                    Wasm::store_le(nread_ret, total_read as Size);
                    return wasip1::ERRNO_SUCCESS;
                }
                Ok((n, next_cookie)) => {
                    total_read += n;
                    buf = unsafe { buf.add(n) };
                    buf_len -= n;
                    cookie = next_cookie;
                }
                Err(e) => return e,
            }
        }
    }

    fn fd_write_raw<Wasm: WasmAccess>(
        &self,
        fd: Fd,
        iovs_ptr: *const Ciovec,
        iovs_len: usize,
        nwritten_ret: *mut Size,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "fd_write: fd={fd}, iovs_len={iovs_len}");

        match fd {
            0 => wasip1::ERRNO_BADF,
            1 | 2 => {
                trace_fs!(self, Wasm; "fd_write to stdio: fd={fd}, iovs_len={iovs_len}");

                let iovs_vec = Wasm::as_array(iovs_ptr, iovs_len);
                let mut written = 0;
                for iovs in iovs_vec {
                    match fd {
                        1 => match self
                            .lfs
                            .fd_write_stdout_raw::<Wasm>(iovs.buf as *const u8, iovs.buf_len)
                        {
                            Ok(w) => written += w,
                            Err(e) => return e,
                        },
                        2 => match self
                            .lfs
                            .fd_write_stderr_raw::<Wasm>(iovs.buf as *const u8, iovs.buf_len)
                        {
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
                get_open_fd!(open_fd = self, fd);

                let iovs_vec = Wasm::as_array(iovs_ptr, iovs_len);
                let mut written = 0;
                for iovs in iovs_vec {
                    match self.lfs.fd_write_raw::<Wasm>(
                        open_fd.inode_id(),
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

    fn path_filestat_get_raw<Wasm: WasmAccess>(
        &self,
        fd: Fd,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
        filestat: *mut wasip1::Filestat,
    ) -> wasip1::Errno {
        get_open_fd!(open_fd = self, fd);

        trace_fs!(self, Wasm; "path_filestat_get: fd={fd}, flags={flags}, path_len={path_len}");

        match self
            .lfs
            .path_filestat_get_raw::<Wasm>(open_fd.inode_id(), flags, path_ptr, path_len)
        {
            Ok(stat) => {
                let s = wasip1::Filestat {
                    dev: 0,
                    ino: stat.ino,
                    filetype: stat.filetype,
                    nlink: stat.nlink,
                    size: stat.size,
                    atim: stat.atim,
                    mtim: stat.mtim,
                    ctim: stat.ctim,
                };
                Wasm::store_le(filestat, s);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }

    fn fd_prestat_get_raw<Wasm: WasmAccess>(
        &self,
        fd: Fd,
        prestat_ret: *mut wasip1::Prestat,
    ) -> wasip1::Errno {
        get_open_fd!(open_fd = self, fd);

        trace_fs!(self, Wasm; "fd_prestat_get: fd={fd}");

        match self.lfs.fd_prestat_get_raw::<Wasm>(open_fd.inode_id()) {
            Ok(prestat) => {
                Wasm::store_le(prestat_ret, prestat);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }

    fn fd_prestat_dir_name_raw<Wasm: WasmAccess>(
        &self,
        fd: Fd,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> wasip1::Errno {
        get_open_fd!(open_fd = self, fd);

        trace_fs!(self, Wasm; "fd_prestat_dir_name: fd={fd}, dir_path_len={dir_path_len}");

        match self.lfs.fd_prestat_dir_name_raw::<Wasm>(
            open_fd.inode_id(),
            dir_path_ptr,
            dir_path_len,
        ) {
            Ok(()) => wasip1::ERRNO_SUCCESS,
            Err(e) => e,
        }
    }

    fn fd_close_raw<Wasm: WasmAccess>(&self, fd: Fd) -> wasip1::Errno {
        trace_fs!(self, Wasm; "fd_close: fd={fd}");

        match self.remove_open_fd(fd) {
            Some(_) => wasip1::ERRNO_SUCCESS,
            None => wasip1::ERRNO_BADF,
        }
    }

    fn fd_filestat_get_raw<Wasm: WasmAccess>(
        &self,
        fd: Fd,
        filestat_ret: *mut wasip1::Filestat,
    ) -> wasip1::Errno {
        get_open_fd!(open_fd = self, fd);

        trace_fs!(self, Wasm; "fd_filestat_get: fd={fd}");

        match self.lfs.fd_filestat_get_raw::<Wasm>(open_fd.inode_id()) {
            Ok(stat) => {
                let s = wasip1::Filestat {
                    dev: 0,
                    ino: stat.ino,
                    filetype: stat.filetype,
                    nlink: stat.nlink,
                    size: stat.size,
                    atim: stat.atim,
                    mtim: stat.mtim,
                    ctim: stat.ctim,
                };
                Wasm::store_le(filestat_ret, s);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }

    fn fd_fdstat_get_raw<Wasm: WasmAccess>(
        &self,
        fd: Fd,
        fdstat_ret: *mut wasip1::Fdstat,
    ) -> wasip1::Errno {
        get_open_fd!(open_fd = self, fd);

        trace_fs!(self, Wasm; "fd_fdstat_get: fd={fd}");

        match self.lfs.fd_filestat_get_raw::<Wasm>(open_fd.inode_id()) {
            Ok(stat) => {
                let s = wasip1::Fdstat {
                    fs_filetype: stat.filetype,
                    fs_flags: open_fd.fd_flags(),
                    fs_rights_base: open_fd.base_rights(),
                    fs_rights_inheriting: open_fd.inheriting_rights(),
                };
                Wasm::store_le(fdstat_ret, s);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }

    fn fd_read_raw<Wasm: WasmAccess>(
        &self,
        fd: Fd,
        iovs_ptr: *const Ciovec,
        iovs_len: usize,
        nread_ret: *mut Size,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "fd_read: fd={fd}, iovs_len={iovs_len}");

        match fd {
            0 => {
                trace_fs!(self, Wasm; "fd_read from stdin: fd={fd}, iovs_len={iovs_len}");

                let iovs_vec = Wasm::as_array(iovs_ptr, iovs_len);
                let mut total_read = 0;
                for iovs in iovs_vec {
                    match self
                        .lfs
                        .fd_read_stdin_raw::<Wasm>(iovs.buf as *mut u8, iovs.buf_len)
                    {
                        Ok(n) => total_read += n,
                        Err(e) => return e,
                    }
                }
                Wasm::store_le(nread_ret, total_read as Size);
                wasip1::ERRNO_SUCCESS
            }
            1 | 2 => wasip1::ERRNO_BADF,
            fd => {
                get_open_fd!(open_fd = self, fd);

                trace_fs!(self, Wasm; "fd_read from file: fd={fd}, iovs_len={iovs_len}");

                let mut cursor = open_fd.cursor();
                let iovs_vec = Wasm::as_array(iovs_ptr, iovs_len);
                let mut total_read = 0;

                for iovs in iovs_vec {
                    match self.lfs.fd_pread_raw::<Wasm>(
                        open_fd.inode_id(),
                        iovs.buf as *mut u8,
                        iovs.buf_len,
                        cursor,
                    ) {
                        Ok(nread) => {
                            if nread == 0 {
                                break;
                            }

                            total_read += nread;
                            cursor += nread;

                            if nread < iovs.buf_len as usize {
                                break;
                            }
                        }
                        Err(e) => return e,
                    }
                }

                let _ = self.set_cursor(fd, cursor);
                Wasm::store_le(nread_ret, total_read as Size);
                wasip1::ERRNO_SUCCESS
            }
        }
    }

    fn path_open_raw<Wasm: WasmAccess>(
        &self,
        dir_fd: Fd,
        dir_flags: wasip1::Fdflags,
        path_ptr: *const u8,
        path_len: usize,
        o_flags: wasip1::Oflags,
        fs_rights_base: wasip1::Rights,
        fs_rights_inheriting: wasip1::Rights,
        fd_flags: wasip1::Fdflags,
        fd_ret: *mut Fd,
    ) -> wasip1::Errno {
        get_open_fd!(open_fd = self, dir_fd);

        trace_fs!(self, Wasm; "path_open: dir_fd={dir_fd}, dir_flags={dir_flags}, path_len={path_len}, o_flags={o_flags}, fs_rights_base={fs_rights_base}, fs_rights_inheriting={fs_rights_inheriting}, fd_flags={fd_flags}");

        match self.lfs.path_open_raw::<Wasm>(
            open_fd.inode_id(),
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
                new_open_fd.set_base_rights(fs_rights_base);
                new_open_fd.set_inheriting_rights(fs_rights_inheriting);
                let new_fd = self.allocate_fd(new_open_fd);
                Wasm::store_le(fd_ret, new_fd);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }
}
