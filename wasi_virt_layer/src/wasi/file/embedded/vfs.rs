#![cfg(feature = "embedded-fs")]

use crate::__private::wasip1;
use crate::__private::wasip1::{Ciovec, Dircookie, Fd, Size};
pub use crate::wasi::file::OpenFdInfo;
use crate::wasi::file::{ConstDefault, EmbeddedLFS, InodeIdCommon};
#[cfg(feature = "threads")]
use parking_lot::RwLock;

use crate::memory::{WasmAccess, WasmAccessName};

#[cfg(not(feature = "threads"))]
use core::cell::UnsafeCell;

#[derive(Debug, Clone, Copy)]
pub struct SimpleOpenFd {
    pub cursor: usize,
}

impl Default for SimpleOpenFd {
    fn default() -> Self {
        Self { cursor: 0 }
    }
}

impl OpenFdInfo for SimpleOpenFd {
    #[inline]
    fn cursor(&self) -> usize {
        self.cursor
    }

    #[inline]
    fn set_cursor(&mut self, cursor: usize) {
        self.cursor = cursor;
    }
}

impl ConstDefault for SimpleOpenFd {
    const DEFAULT: Self = Self { cursor: 0 };
}

/// A embedded virtual file system implementation.
#[derive(Debug)]
pub struct StandardEmbeddedFileSystem<
    LFS: EmbeddedLFS + Sync + core::fmt::Debug,
    const FLAT_LEN: usize,
    OpenFd: OpenFdInfo + 'static = SimpleOpenFd,
> where
    LFS::Inode: InodeIdCommon,
{
    lfs: LFS,
    // (inode, cursor)
    #[cfg(feature = "threads")]
    map: [RwLock<Option<(LFS::Inode, OpenFd)>>; FLAT_LEN],
    #[cfg(not(feature = "threads"))]
    map: [UnsafeCell<Option<(LFS::Inode, OpenFd)>>; FLAT_LEN],
}

unsafe impl<LFS: EmbeddedLFS + Sync + core::fmt::Debug, const FLAT_LEN: usize, OpenFd: OpenFdInfo>
    Sync for StandardEmbeddedFileSystem<LFS, FLAT_LEN, OpenFd>
where
    LFS::Inode: InodeIdCommon,
{
}

unsafe impl<LFS: EmbeddedLFS + Sync + core::fmt::Debug, const FLAT_LEN: usize, OpenFd: OpenFdInfo>
    Send for StandardEmbeddedFileSystem<LFS, FLAT_LEN, OpenFd>
where
    LFS::Inode: InodeIdCommon,
{
}

impl<
    LFS: EmbeddedLFS + Sync + core::fmt::Debug,
    const FLAT_LEN: usize,
    OpenFd: OpenFdInfo + Copy + ConstDefault + 'static,
> StandardEmbeddedFileSystem<LFS, FLAT_LEN, OpenFd>
where
    LFS::Inode: InodeIdCommon + Copy,
{
    /// Creates a new `StandardEmbeddedFileSystem` with thread support.
    #[cfg(feature = "threads")]
    pub const fn new_const(lfs: LFS) -> Self {
        let mut map: [RwLock<Option<(LFS::Inode, OpenFd)>>; FLAT_LEN] =
            [const { RwLock::new(None) }; FLAT_LEN];

        use const_for::const_for;

        const_for!(i in 0..LFS::PRE_OPEN.len() => {
            map[i] = RwLock::new(Some((LFS::PRE_OPEN[i], OpenFd::DEFAULT)));
        });

        Self { lfs, map }
    }

    /// Creates a new `StandardEmbeddedFileSystem` without thread support.
    #[cfg(not(feature = "threads"))]
    pub const fn new_const(lfs: LFS) -> Self {
        let mut map: [UnsafeCell<Option<(LFS::Inode, OpenFd)>>; FLAT_LEN] =
            [const { UnsafeCell::new(None) }; FLAT_LEN];

        use const_for::const_for;

        const_for!(i in 0..LFS::PRE_OPEN.len() => {
            map[i] = UnsafeCell::new(Some((LFS::PRE_OPEN[i], OpenFd::DEFAULT)));
        });

        Self { lfs, map }
    }
}

impl<
    LFS: EmbeddedLFS + Sync + core::fmt::Debug,
    const FLAT_LEN: usize,
    OpenFd: OpenFdInfo + Default + 'static,
> StandardEmbeddedFileSystem<LFS, FLAT_LEN, OpenFd>
where
    LFS::Inode: InodeIdCommon + Clone,
{
    pub fn new(lfs: LFS) -> Self {
        let mut map: [Option<(LFS::Inode, OpenFd)>; FLAT_LEN] = [const { None }; FLAT_LEN];

        for (i, inode) in LFS::PRE_OPEN.iter().enumerate() {
            map[i] = Some((inode.clone(), OpenFd::default()));
        }

        Self {
            lfs,
            #[cfg(feature = "threads")]
            map: map.map(RwLock::new),
            #[cfg(not(feature = "threads"))]
            map: map.map(UnsafeCell::new),
        }
    }
}

impl<
    LFS: EmbeddedLFS + Sync + core::fmt::Debug,
    const FLAT_LEN: usize,
    OpenFd: OpenFdInfo + Default + 'static,
> StandardEmbeddedFileSystem<LFS, FLAT_LEN, OpenFd>
where
    LFS::Inode: InodeIdCommon,
{
    pub fn with_inode<R, F: FnOnce(&LFS::Inode) -> R>(
        &self,
        fd: Fd,
        f: F,
    ) -> Result<R, wasip1::Errno> {
        #[cfg(feature = "threads")]
        {
            self.map
                .get(fd as usize - 3)
                .ok_or(wasip1::ERRNO_BADF)?
                .read()
                .as_ref()
                .map(|(inode, _)| f(inode))
                .ok_or(wasip1::ERRNO_BADF)
        }

        #[cfg(not(feature = "threads"))]
        {
            unsafe {
                &*self
                    .map
                    .get(fd as usize - 3)
                    .ok_or(wasip1::ERRNO_BADF)?
                    .get()
            }
            .as_ref()
            .map(|(inode, _)| f(inode))
            .ok_or(wasip1::ERRNO_BADF)
        }
    }

    /// Removes and returns the inode associated with the given file descriptor.
    #[inline]
    pub fn remove_inode(&self, fd: Fd) -> Option<LFS::Inode> {
        #[cfg(feature = "threads")]
        {
            self.map
                .get(fd as usize - 3)?
                .write()
                .take()
                .map(|(inode, _)| inode)
        }

        #[cfg(not(feature = "threads"))]
        {
            unsafe { &mut *self.map.get(fd as usize - 3)?.get() }
                .take()
                .map(|(inode, _)| inode)
        }
    }

    /// Pushes a new inode into the file descriptor map and returns the new file descriptor.
    #[inline]
    pub fn push_inode(&self, inode: LFS::Inode) -> Fd {
        #[cfg(feature = "threads")]
        {
            for (i, slot) in self.map.iter().enumerate() {
                let mut slot = slot.write();
                if slot.is_none() {
                    *slot = Some((inode, OpenFd::default()));
                    return (i + 3) as Fd;
                }
            }
        }

        #[cfg(not(feature = "threads"))]
        {
            for (i, slot) in self.map.iter().enumerate() {
                let slot_opt = unsafe { &mut *slot.get() };
                if slot_opt.is_none() {
                    *slot_opt = Some((inode, OpenFd::default()));
                    return (i + 3) as Fd;
                }
            }
        }

        unreachable!();
    }

    /// Returns a mutable reference to the underlying local file system.
    #[inline]
    pub fn get_lfs(&mut self) -> &mut LFS {
        &mut self.lfs
    }

    /// Returns both the inode and a mutable reference to the local file system.
    #[inline]
    pub fn with_inode_and_lfs<R, F: FnOnce(&LFS::Inode, &LFS) -> R>(
        &self,
        fd: Fd,
        f: F,
    ) -> Result<R, wasip1::Errno> {
        #[cfg(feature = "threads")]
        {
            self.map
                .get(fd as usize - 3)
                .ok_or(wasip1::ERRNO_BADF)?
                .read()
                .as_ref()
                .map(|(inode, _)| f(inode, &self.lfs))
                .ok_or(wasip1::ERRNO_BADF)
        }

        #[cfg(not(feature = "threads"))]
        {
            unsafe {
                &mut *self
                    .map
                    .get(fd as usize - 3)
                    .ok_or(wasip1::ERRNO_BADF)?
                    .get()
            }
            .as_ref()
            .map(|(inode, _)| f(inode, &self.lfs))
            .ok_or(wasip1::ERRNO_BADF)
        }
    }

    pub(crate) fn fd_readdir_raw_inner<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        mut buf: *mut u8,
        mut buf_len: usize,
        mut cookie: Dircookie,
    ) -> Result<Size, wasip1::Errno> {
        self.with_inode_and_lfs(fd, |inode, lfs| {
            // check is this a directory
            if !lfs.is_dir(inode) {
                return Err(wasip1::ERRNO_NOTDIR);
            }

            let mut read = 0;

            loop {
                let (n, next_cookie) = lfs.fd_readdir_raw::<Wasm>(inode, buf, buf_len, cookie)?;
                if n == 0 {
                    return Ok(read);
                }
                read += n;
                buf = unsafe { buf.add(n) };
                buf_len -= n;
                cookie = next_cookie;
            }
        })?
    }

    pub(crate) fn fd_write_raw_inner<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        iovs_ptr: *const Ciovec,
        iovs_len: usize,
    ) -> Result<Size, wasip1::Errno> {
        match fd {
            0 => return Err(wasip1::ERRNO_BADF),
            1 | 2 => {
                // stdout
                let lfs = &self.lfs;

                let iovs_vec = Wasm::as_array(iovs_ptr, iovs_len);

                let mut written = 0;

                for iovs in iovs_vec {
                    let buf_len = iovs.buf_len;
                    let buf_ptr = iovs.buf;

                    match fd {
                        1 => written += lfs.fd_write_stdout_raw::<Wasm>(buf_ptr, buf_len)?,
                        2 => written += lfs.fd_write_stderr_raw::<Wasm>(buf_ptr, buf_len)?,
                        _ => unreachable!(),
                    }
                }

                Ok(written)
            }
            fd => self.with_inode_and_lfs(fd, |inode, lfs| {
                let iovs_vec = Wasm::as_array(iovs_ptr, iovs_len);

                let mut written = 0;

                for iovs in iovs_vec {
                    let buf_len = iovs.buf_len;
                    let buf_ptr = iovs.buf;

                    written += lfs.fd_write_raw::<Wasm>(inode, buf_ptr, buf_len)?;
                }

                Ok(written)
            })?,
        }
    }

    pub(crate) fn path_filestat_get_raw_inner<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<wasip1::Filestat, wasip1::Errno> {
        self.with_inode_and_lfs(fd, |inode, lfs| {
            let status = lfs.path_filestat_get_raw::<Wasm>(inode, flags, path_ptr, path_len)?;

            Ok(wasip1::Filestat {
                dev: 0, // no device
                ino: status.ino,
                filetype: status.filetype,
                nlink: status.nlink,
                size: status.size,
                atim: status.atim,
                mtim: status.mtim,
                ctim: status.ctim,
            })
        })?
    }

    pub(crate) fn fd_prestat_get_raw_inner<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
    ) -> Result<wasip1::Prestat, wasip1::Errno> {
        self.with_inode_and_lfs(fd, |inode, lfs| {
            let prestat = lfs.fd_prestat_get_raw::<Wasm>(inode)?;

            Ok(prestat)
        })?
    }

    pub(crate) fn fd_prestat_dir_name_raw_inner<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        self.with_inode_and_lfs(fd, |inode, lfs| {
            lfs.fd_prestat_dir_name_raw::<Wasm>(inode, dir_path_ptr, dir_path_len)
        })?
    }

    pub(crate) fn fd_filestat_get_raw_inner<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
    ) -> Result<wasip1::Filestat, wasip1::Errno> {
        self.with_inode_and_lfs(fd, |inode, lfs| {
            let filestat = lfs.fd_filestat_get_raw::<Wasm>(inode)?;

            Ok(wasip1::Filestat {
                dev: 0, // no device
                ino: filestat.ino,
                filetype: filestat.filetype,
                nlink: filestat.nlink,
                size: filestat.size,
                atim: filestat.atim,
                mtim: filestat.mtim,
                ctim: filestat.ctim,
            })
        })?
    }

    pub(crate) fn fd_fdstat_get_raw_inner<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
    ) -> Result<wasip1::Fdstat, wasip1::Errno> {
        self.with_inode_and_lfs(fd, |inode, lfs| {
            let filestat = lfs.fd_filestat_get_raw::<Wasm>(inode)?;

            Ok(wasip1::Fdstat {
                fs_filetype: filestat.filetype,
                fs_flags: 0,
                fs_rights_base: !0,
                fs_rights_inheriting: !0,
            })
        })?
    }

    pub(crate) fn fd_close_raw_inner<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
    ) -> Result<(), wasip1::Errno> {
        if self.remove_inode(fd).is_none() {
            return Err(wasip1::ERRNO_BADF);
        }
        Ok(())
    }

    pub(crate) fn get_cursor(&self, fd: Fd) -> Result<usize, wasip1::Errno> {
        #[cfg(feature = "threads")]
        {
            self.map
                .get(fd as usize - 3)
                .ok_or(wasip1::ERRNO_BADF)?
                .read()
                .as_ref()
                .map(|(_, cursor)| cursor.cursor())
                .ok_or(wasip1::ERRNO_BADF)
        }

        #[cfg(not(feature = "threads"))]
        {
            unsafe {
                &*self
                    .map
                    .get(fd as usize - 3)
                    .ok_or(wasip1::ERRNO_BADF)?
                    .get()
            }
            .as_ref()
            .map(|(_, cursor)| cursor.cursor())
            .ok_or(wasip1::ERRNO_BADF)
        }
    }

    pub(crate) fn set_cursor(&self, fd: Fd, cursor: usize) -> Result<(), wasip1::Errno> {
        #[cfg(feature = "threads")]
        {
            self.map
                .get(fd as usize - 3)
                .ok_or(wasip1::ERRNO_BADF)?
                .write()
                .as_mut()
                .map(|(_, cur)| cur.set_cursor(cursor))
                .ok_or(wasip1::ERRNO_BADF)
        }

        #[cfg(not(feature = "threads"))]
        {
            unsafe {
                &mut *self
                    .map
                    .get(fd as usize - 3)
                    .ok_or(wasip1::ERRNO_BADF)?
                    .get()
            }
            .as_mut()
            .map(|(_, cur)| cur.set_cursor(cursor))
            .ok_or(wasip1::ERRNO_BADF)
        }
    }

    pub(crate) fn fd_read_raw_inner<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        iovs_ptr: *const Ciovec,
        iovs_len: usize,
    ) -> Result<Size, wasip1::Errno> {
        match fd {
            0 => {
                let lfs = &self.lfs;

                let iovs_vec = Wasm::as_array(iovs_ptr, iovs_len);

                let mut read = 0;

                for iovs in iovs_vec {
                    read += lfs.fd_read_stdin_raw::<Wasm>(iovs.buf as *mut _, iovs.buf_len)?;
                }

                Ok(read)
            }
            1 | 2 => return Err(wasip1::ERRNO_BADF),
            fd => {
                let cursor = self.get_cursor(fd)?;

                let (read, new_cursor) = self.with_inode_and_lfs(fd, |inode, lfs| {
                    if lfs.is_dir(inode) {
                        return Err(wasip1::ERRNO_ISDIR);
                    }

                    let iovs_vec = Wasm::as_array(iovs_ptr, iovs_len);

                    let mut read = 0;
                    let mut inner_cursor = cursor;

                    for iovs in iovs_vec {
                        let nread = lfs.fd_pread_raw::<Wasm>(
                            inode,
                            iovs.buf as *mut _,
                            iovs.buf_len,
                            inner_cursor,
                        )?;
                        read += nread;
                        inner_cursor += nread;
                    }

                    Ok((read, inner_cursor))
                })??;

                self.set_cursor(fd, new_cursor)?;

                Ok(read)
            }
        }
    }

    pub(crate) fn fd_seek_raw_inner<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        offset: i64,
        whence: wasip1::Whence,
        new_offset_ptr: *mut i64,
    ) -> Result<(), wasip1::Errno> {
        let cursor = self.get_cursor(fd)?;

        let new_offset_i64 = match whence {
            wasip1::WHENCE_SET => offset,
            wasip1::WHENCE_CUR => (cursor as i64)
                .checked_add(offset)
                .ok_or(wasip1::ERRNO_OVERFLOW)?,
            wasip1::WHENCE_END => {
                let size = self.with_inode_and_lfs(fd, |inode, lfs| {
                    if lfs.is_dir(inode) {
                        return Err(wasip1::ERRNO_ISDIR);
                    }
                    Ok(lfs.fd_filestat_get_raw::<Wasm>(inode)?.size)
                })??;
                (size as i64)
                    .checked_add(offset)
                    .ok_or(wasip1::ERRNO_OVERFLOW)?
            }
            _ => return Err(wasip1::ERRNO_INVAL),
        };

        if new_offset_i64 < 0 {
            return Err(wasip1::ERRNO_INVAL);
        }

        Wasm::store_le(new_offset_ptr, new_offset_i64);
        let new_offset = new_offset_i64 as usize;
        self.set_cursor(fd, new_offset)?;

        Ok(())
    }

    pub(crate) fn path_open_raw_inner<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        dir_fd: Fd,
        dir_flags: wasip1::Fdflags,
        path_ptr: *const u8,
        path_len: usize,
        o_flags: wasip1::Oflags,
        fs_rights_base: wasip1::Rights,
        fs_rights_inheriting: wasip1::Rights,
        fd_flags: wasip1::Fdflags,
    ) -> Result<Fd, wasip1::Errno> {
        let new_inode = self.with_inode_and_lfs(dir_fd, |inode, lfs| {
            lfs.path_open_raw::<Wasm>(
                inode,
                dir_flags,
                path_ptr,
                path_len,
                o_flags,
                fs_rights_base,
                fs_rights_inheriting,
                fd_flags,
            )
        })??;

        Ok(self.push_inode(new_inode))
    }

    pub(crate) fn path_create_directory_raw_inner<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        self.with_inode_and_lfs(fd, |inode, lfs| {
            lfs.path_create_directory_raw::<Wasm>(inode, path_ptr, path_len)
        })?
    }

    pub(crate) fn path_link_raw_inner<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        old_fd: Fd,
        old_flags: wasip1::Lookupflags,
        old_path_ptr: *const u8,
        old_path_len: usize,
        new_fd: Fd,
        new_path_ptr: *const u8,
        new_path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        self.with_inode_and_lfs(old_fd, |old_inode, lfs| {
            self.with_inode_and_lfs(new_fd, |new_inode, _| {
                lfs.path_link_raw::<Wasm>(
                    old_inode,
                    old_flags,
                    old_path_ptr,
                    old_path_len,
                    new_inode,
                    new_path_ptr,
                    new_path_len,
                )
            })?
        })?
    }

    pub(crate) fn path_remove_directory_raw_inner<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        self.with_inode_and_lfs(fd, |inode, lfs| {
            lfs.path_remove_directory_raw::<Wasm>(inode, path_ptr, path_len)
        })?
    }

    pub(crate) fn path_rename_raw_inner<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        old_fd: Fd,
        old_path_ptr: *const u8,
        old_path_len: usize,
        new_fd: Fd,
        new_path_ptr: *const u8,
        new_path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        self.with_inode_and_lfs(old_fd, |old_inode, lfs| {
            self.with_inode_and_lfs(new_fd, |new_inode, _| {
                lfs.path_rename_raw::<Wasm>(
                    old_inode,
                    old_path_ptr,
                    old_path_len,
                    new_inode,
                    new_path_ptr,
                    new_path_len,
                )
            })?
        })?
    }

    pub(crate) fn path_unlink_file_raw_inner<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        self.with_inode_and_lfs(fd, |inode, lfs| {
            lfs.path_unlink_file_raw::<Wasm>(inode, path_ptr, path_len)
        })?
    }

    pub(crate) fn path_readlink_raw_inner<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        path_ptr: *const u8,
        path_len: usize,
        buf: *mut u8,
        buf_len: usize,
    ) -> Result<Size, wasip1::Errno> {
        self.with_inode_and_lfs(fd, |inode, lfs| {
            lfs.path_readlink_raw::<Wasm>(inode, path_ptr, path_len, buf, buf_len)
        })?
    }
}
