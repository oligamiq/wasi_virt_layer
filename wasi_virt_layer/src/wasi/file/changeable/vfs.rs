use crate::__private::wasip1;
use crate::__private::wasip1::{Ciovec, Dircookie, Fd, Size};

use crate::{
    memory::WasmAccess,
    wasi::file::{
        Wasip1FileSystem, Wasip1LFS,
        changeable::inode::{InodeId, OpenFd},
    },
};

use alloc::collections::BTreeMap;
#[cfg(feature = "threads")]
use parking_lot::RwLock;

/// A virtual file system implementation that maps file descriptors to inodes in a ChangeableLFS.
#[derive(Debug)]
pub struct ChangeableVFS<LFS: Wasip1LFS + core::fmt::Debug>
where
    LFS::Inode: core::fmt::Debug,
{
    pub lfs: LFS,
    // Maps File Descriptor (Fd) to OpenFd state
    #[cfg(feature = "threads")]
    pub fd_map: RwLock<BTreeMap<Fd, OpenFd>>,
    #[cfg(not(feature = "threads"))]
    pub fd_map: BTreeMap<Fd, OpenFd>,
    pub next_fd: Fd,
}

impl<LFS: Wasip1LFS + core::fmt::Debug> ChangeableVFS<LFS>
where
    LFS::Inode: core::fmt::Debug + Into<InodeId> + From<InodeId> + Copy,
{
    /// Creates a new ChangeableVFS wrapping the provided LFS
    pub fn new(lfs: LFS) -> Self {
        let mut map = BTreeMap::new();

        // Map pre-opened inodes
        for (i, &inode) in LFS::PRE_OPEN.iter().enumerate() {
            let fd = (i + 3) as Fd;
            map.insert(
                fd,
                OpenFd {
                    inode_id: inode.into(),
                    cursor: 0,
                    base_rights: !0,
                    inheriting_rights: !0,
                    fd_flags: 0,
                },
            );
        }

        Self {
            lfs,
            #[cfg(feature = "threads")]
            fd_map: RwLock::new(map),
            #[cfg(not(feature = "threads"))]
            fd_map: map,
            next_fd: LFS::PRE_OPEN.len() as Fd + 3,
        }
    }

    #[inline]
    fn get_open_fd(&self, fd: Fd) -> Option<OpenFd> {
        #[cfg(feature = "threads")]
        {
            self.fd_map.read().get(&fd).cloned()
        }
        #[cfg(not(feature = "threads"))]
        {
            self.fd_map.get(&fd).cloned()
        }
    }

    #[inline]
    fn get_cursor(&self, fd: Fd) -> Option<usize> {
        self.get_open_fd(fd).map(|open_fd| open_fd.cursor)
    }

    #[inline]
    fn set_cursor(&mut self, fd: Fd, cursor: usize) -> Option<()> {
        #[cfg(feature = "threads")]
        {
            if let Some(open_fd) = self.fd_map.write().get_mut(&fd) {
                open_fd.cursor = cursor;
                Some(())
            } else {
                None
            }
        }
        #[cfg(not(feature = "threads"))]
        {
            if let Some(open_fd) = self.fd_map.get_mut(&fd) {
                open_fd.cursor = cursor;
                Some(())
            } else {
                None
            }
        }
    }

    #[inline]
    fn remove_open_fd(&mut self, fd: Fd) -> Option<OpenFd> {
        #[cfg(feature = "threads")]
        {
            self.fd_map.write().remove(&fd)
        }
        #[cfg(not(feature = "threads"))]
        {
            self.fd_map.remove(&fd)
        }
    }

    #[inline]
    fn allocate_fd(&mut self, open_fd: OpenFd) -> Fd {
        let fd = self.next_fd;
        self.next_fd += 1;
        #[cfg(feature = "threads")]
        {
            self.fd_map.write().insert(fd, open_fd);
        }
        #[cfg(not(feature = "threads"))]
        {
            self.fd_map.insert(fd, open_fd);
        }
        fd
    }
}

impl<LFS: Wasip1LFS + core::fmt::Debug> Wasip1FileSystem for ChangeableVFS<LFS>
where
    LFS::Inode: core::fmt::Debug + Into<InodeId> + From<InodeId> + Copy,
{
    fn fd_readdir_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
        mut buf: *mut u8,
        mut buf_len: usize,
        mut cookie: Dircookie,
        nread_ret: *mut Size,
    ) -> wasip1::Errno {
        let open_fd = match self.get_open_fd(fd) {
            Some(f) => f,
            None => return wasip1::ERRNO_BADF,
        };

        if !self.lfs.is_dir(open_fd.inode_id.into()) {
            return wasip1::ERRNO_NOTDIR;
        }

        let mut total_read = 0;
        loop {
            match self.lfs.fd_readdir_raw::<Wasm>(
                open_fd.inode_id.into(),
                buf,
                buf_len,
                cookie,
            ) {
                Ok((0, _)) => {
                    unsafe { *nread_ret = total_read as Size };
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
        &mut self,
        fd: Fd,
        iovs_ptr: *const Ciovec,
        iovs_len: usize,
        nwritten_ret: *mut Size,
    ) -> wasip1::Errno {
        match fd {
            0 => wasip1::ERRNO_BADF,
            1 | 2 => {
                let iovs_vec = Wasm::as_array(iovs_ptr, iovs_len);
                let mut written = 0;
                for iovs in iovs_vec {
                    match fd {
                        1 => match self.lfs.fd_write_stdout_raw::<Wasm>(iovs.buf, iovs.buf_len) {
                            Ok(w) => written += w,
                            Err(e) => return e,
                        },
                        2 => match self.lfs.fd_write_stderr_raw::<Wasm>(iovs.buf, iovs.buf_len) {
                            Ok(w) => written += w,
                            Err(e) => return e,
                        },
                        _ => unreachable!(),
                    }
                }
                unsafe { *nwritten_ret = written as Size };
                wasip1::ERRNO_SUCCESS
            }
            fd => {
                let open_fd = match self.get_open_fd(fd) {
                    Some(f) => f,
                    None => return wasip1::ERRNO_BADF,
                };
                
                let iovs_vec = Wasm::as_array(iovs_ptr, iovs_len);
                let mut written = 0;
                for iovs in iovs_vec {
                    match self.lfs.fd_write_raw::<Wasm>(
                        open_fd.inode_id.into(),
                        iovs.buf,
                        iovs.buf_len,
                    ) {
                        Ok(w) => written += w,
                        Err(e) => return e,
                    }
                }
                unsafe { *nwritten_ret = written as Size };
                wasip1::ERRNO_SUCCESS
            }
        }
    }

    fn path_filestat_get_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
        filestat: *mut wasip1::Filestat,
    ) -> wasip1::Errno {
        let open_fd = match self.get_open_fd(fd) {
            Some(f) => f,
            None => return wasip1::ERRNO_BADF,
        };

        match self.lfs.path_filestat_get_raw::<Wasm>(
            open_fd.inode_id.into(),
            flags,
            path_ptr,
            path_len,
        ) {
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
                let slice = unsafe { core::slice::from_raw_parts_mut(filestat as *mut u8, core::mem::size_of::<wasip1::Filestat>()) };
                Wasm::memcpy_to(slice, &s as *const _ as *const u8);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }

    fn fd_prestat_get_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
        prestat_ret: *mut wasip1::Prestat,
    ) -> wasip1::Errno {
        let open_fd = match self.get_open_fd(fd) {
            Some(f) => f,
            None => return wasip1::ERRNO_BADF,
        };

        match self.lfs.fd_prestat_get_raw::<Wasm>(open_fd.inode_id.into()) {
            Ok(prestat) => {
                let slice = unsafe { core::slice::from_raw_parts_mut(prestat_ret as *mut u8, core::mem::size_of::<wasip1::Prestat>()) };
                Wasm::memcpy_to(slice, &prestat as *const _ as *const u8);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }

    fn fd_prestat_dir_name_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> wasip1::Errno {
        let open_fd = match self.get_open_fd(fd) {
            Some(f) => f,
            None => return wasip1::ERRNO_BADF,
        };

        match self.lfs.fd_prestat_dir_name_raw::<Wasm>(
            open_fd.inode_id.into(),
            dir_path_ptr,
            dir_path_len,
        ) {
            Ok(()) => wasip1::ERRNO_SUCCESS,
            Err(e) => e,
        }
    }

    fn fd_close_raw<Wasm: WasmAccess>(&mut self, fd: Fd) -> wasip1::Errno {
        match self.remove_open_fd(fd) {
            Some(_) => wasip1::ERRNO_SUCCESS,
            None => wasip1::ERRNO_BADF,
        }
    }

    fn fd_filestat_get_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
        filestat_ret: *mut wasip1::Filestat,
    ) -> wasip1::Errno {
        let open_fd = match self.get_open_fd(fd) {
            Some(f) => f,
            None => return wasip1::ERRNO_BADF,
        };

        match self.lfs.fd_filestat_get_raw::<Wasm>(open_fd.inode_id.into()) {
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
                let slice = unsafe { core::slice::from_raw_parts_mut(filestat_ret as *mut u8, core::mem::size_of::<wasip1::Filestat>()) };
                Wasm::memcpy_to(slice, &s as *const _ as *const u8);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }

    fn fd_fdstat_get_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
        fdstat_ret: *mut wasip1::Fdstat,
    ) -> wasip1::Errno {
        let open_fd = match self.get_open_fd(fd) {
            Some(f) => f,
            None => return wasip1::ERRNO_BADF,
        };

        match self.lfs.fd_filestat_get_raw::<Wasm>(open_fd.inode_id.into()) {
            Ok(stat) => {
                let s = wasip1::Fdstat {
                    fs_filetype: stat.filetype,
                    fs_flags: open_fd.fd_flags,
                    fs_rights_base: open_fd.base_rights,
                    fs_rights_inheriting: open_fd.inheriting_rights,
                };
                let slice = unsafe { core::slice::from_raw_parts_mut(fdstat_ret as *mut u8, core::mem::size_of::<wasip1::Fdstat>()) };
                Wasm::memcpy_to(slice, &s as *const _ as *const u8);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }

    fn fd_read_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
        iovs_ptr: *const Ciovec,
        iovs_len: usize,
        nread_ret: *mut Size,
    ) -> wasip1::Errno {
        match fd {
            0 => {
                let iovs_vec = Wasm::as_array(iovs_ptr, iovs_len);
                let mut total_read = 0;
                for iovs in iovs_vec {
                    match self.lfs.fd_read_stdin_raw::<Wasm>(iovs.buf as *mut _, iovs.buf_len) {
                        Ok(n) => total_read += n,
                        Err(e) => return e,
                    }
                }
                unsafe { *nread_ret = total_read as Size };
                wasip1::ERRNO_SUCCESS
            }
            1 | 2 => wasip1::ERRNO_BADF,
            fd => {
                let open_fd = match self.get_open_fd(fd) {
                    Some(f) => f,
                    None => return wasip1::ERRNO_BADF,
                };

                let mut cursor = open_fd.cursor;
                let iovs_vec = Wasm::as_array(iovs_ptr, iovs_len);
                let mut total_read = 0;

                for iovs in iovs_vec {
                    match self.lfs.fd_pread_raw::<Wasm>(
                        open_fd.inode_id.into(),
                        iovs.buf as *mut _,
                        iovs.buf_len,
                        cursor,
                    ) {
                        Ok(nread) => {
                            total_read += nread;
                            cursor += nread;
                        }
                        Err(e) => return e,
                    }
                }

                let _ = self.set_cursor(fd, cursor);
                unsafe { *nread_ret = total_read as Size };
                wasip1::ERRNO_SUCCESS
            }
        }
    }

    fn path_open_raw<Wasm: WasmAccess>(
        &mut self,
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
        let open_fd = match self.get_open_fd(dir_fd) {
            Some(f) => f,
            None => return wasip1::ERRNO_BADF,
        };

        match self.lfs.path_open_raw::<Wasm>(
            open_fd.inode_id.into(),
            dir_flags,
            path_ptr,
            path_len,
            o_flags,
            fs_rights_base,
            fs_rights_inheriting,
            fd_flags,
        ) {
            Ok(new_inode) => {
                let new_fd = self.allocate_fd(OpenFd {
                    inode_id: new_inode.into(),
                    cursor: 0,
                    base_rights: fs_rights_base,
                    inheriting_rights: fs_rights_inheriting,
                    fd_flags,
                });
                unsafe { *fd_ret = new_fd };
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }
}