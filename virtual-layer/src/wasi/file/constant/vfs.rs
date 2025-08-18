#[cfg(feature = "threads")]
use parking_lot::RwLock;
use wasip1::{Ciovec, Dircookie, Fd, Size};

use crate::{memory::WasmAccess, wasi::file::Wasip1LFS};

/// small posix like virtual file system
/// but inode has some metadata
pub struct Wasip1ConstVFS<LFS: Wasip1LFS + Sync, const FLAT_LEN: usize>
where
    LFS::Inode: Copy,
{
    lfs: LFS,
    // (inode, cursor)
    #[cfg(feature = "threads")]
    map: [RwLock<Option<(LFS::Inode, usize)>>; FLAT_LEN],
    #[cfg(not(feature = "threads"))]
    map: [Option<(LFS::Inode, usize)>; FLAT_LEN],
}

impl<LFS: Wasip1LFS + Sync, const FLAT_LEN: usize> Wasip1ConstVFS<LFS, FLAT_LEN>
where
    LFS::Inode: Copy,
{
    #[cfg(feature = "threads")]
    pub const fn new(lfs: LFS) -> Self {
        let mut map: [RwLock<Option<(LFS::Inode, usize)>>; FLAT_LEN] =
            [const { RwLock::new(None) }; FLAT_LEN];

        use const_for::const_for;

        const_for!(i in 0..LFS::PRE_OPEN.len() => {
            map[i] = RwLock::new(Some((LFS::PRE_OPEN[i], i)));
        });

        Self { lfs, map }
    }

    #[cfg(not(feature = "threads"))]
    pub const fn new(lfs: LFS) -> Self {
        let map: [Option<(LFS::Inode, usize)>; FLAT_LEN] = [const { None }; FLAT_LEN];

        Self { lfs, map }
    }

    #[inline]
    pub fn get_inode(&self, fd: Fd) -> Option<LFS::Inode> {
        #[cfg(feature = "threads")]
        {
            self.map
                .get(fd as usize - 3)?
                .read()
                .map(|(inode, _)| inode)
        }

        #[cfg(not(feature = "threads"))]
        {
            self.map.get(fd as usize - 3)?.map(|(inode, _)| inode)
        }
    }

    #[inline]
    pub fn remove_inode(&mut self, fd: Fd) -> Option<LFS::Inode> {
        #[cfg(feature = "threads")]
        {
            self.map
                .get_mut(fd as usize - 3)?
                .write()
                .take()
                .map(|(inode, _)| inode)
        }

        #[cfg(not(feature = "threads"))]
        {
            self.map
                .get_mut(fd as usize - 3)?
                .take()
                .map(|(inode, _)| inode)
        }
    }

    #[inline]
    pub fn push_inode(&mut self, inode: LFS::Inode) -> Fd {
        #[cfg(feature = "threads")]
        {
            for (i, slot) in self.map.iter_mut().enumerate() {
                let mut slot = slot.write();
                if slot.is_none() {
                    *slot = Some((inode, 0));
                    return (i + 3) as Fd;
                }
            }
        }

        #[cfg(not(feature = "threads"))]
        {
            for (i, slot) in self.map.iter_mut().enumerate() {
                if slot.is_none() {
                    *slot = Some((inode, 0));
                    return (i + 3) as Fd;
                }
            }
        }

        unreachable!();
    }

    #[inline]
    pub fn get_inode_and_lfs(&mut self, fd: Fd) -> Option<(LFS::Inode, &mut LFS)> {
        self.get_inode(fd).map(|inode| (inode, &mut self.lfs))
    }

    pub(crate) fn fd_readdir_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
        mut buf: *mut u8,
        mut buf_len: usize,
        mut cookie: Dircookie,
    ) -> Result<Size, wasip1::Errno> {
        let (inode, lfs) = self.get_inode_and_lfs(fd).ok_or(wasip1::ERRNO_BADF)?;

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
    }

    pub(crate) fn fd_write_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
        iovs_ptr: *const Ciovec,
        iovs_len: usize,
    ) -> Result<Size, wasip1::Errno> {
        match fd {
            0 => return Err(wasip1::ERRNO_BADF),
            1 | 2 => {
                // stdout
                let lfs = &mut self.lfs;

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
            fd => {
                let (inode, lfs) = self.get_inode_and_lfs(fd).ok_or(wasip1::ERRNO_BADF)?;

                let iovs_vec = Wasm::as_array(iovs_ptr, iovs_len);

                let mut written = 0;

                for iovs in iovs_vec {
                    let buf_len = iovs.buf_len;
                    let buf_ptr = iovs.buf;

                    written += lfs.fd_write_raw::<Wasm>(inode, buf_ptr, buf_len)?;
                }

                Ok(written)
            }
        }
    }

    pub(crate) fn path_filestat_get_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<wasip1::Filestat, wasip1::Errno> {
        let (inode, lfs) = self.get_inode_and_lfs(fd).ok_or(wasip1::ERRNO_BADF)?;

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
    }

    pub(crate) fn fd_prestat_get_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
    ) -> Result<wasip1::Prestat, wasip1::Errno> {
        let (inode, lfs) = self.get_inode_and_lfs(fd).ok_or(wasip1::ERRNO_BADF)?;

        let prestat = lfs.fd_prestat_get_raw::<Wasm>(inode)?;

        Ok(prestat)
    }

    pub(crate) fn fd_prestat_dir_name_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        let (inode, lfs) = self.get_inode_and_lfs(fd).ok_or(wasip1::ERRNO_BADF)?;

        lfs.fd_prestat_dir_name_raw::<Wasm>(inode, dir_path_ptr, dir_path_len)
    }

    pub(crate) fn fd_filestat_get_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
    ) -> Result<wasip1::Filestat, wasip1::Errno> {
        let (inode, lfs) = self.get_inode_and_lfs(fd).ok_or(wasip1::ERRNO_BADF)?;

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
    }

    pub(crate) fn fd_close_raw<Wasm: WasmAccess>(&mut self, fd: Fd) -> Result<(), wasip1::Errno> {
        if self.remove_inode(fd).is_none() {
            return Err(wasip1::ERRNO_BADF);
        }
        Ok(())
    }

    pub(crate) fn get_cursor(&mut self, fd: Fd) -> Result<usize, wasip1::Errno> {
        #[cfg(feature = "threads")]
        {
            self.map
                .get(fd as usize - 3)
                .ok_or(wasip1::ERRNO_BADF)?
                .read()
                .map(|(_, cursor)| cursor)
                .ok_or(wasip1::ERRNO_BADF)
        }

        #[cfg(not(feature = "threads"))]
        {
            self.map
                .get(fd as usize - 3)
                .ok_or(wasip1::ERRNO_BADF)?
                .map(|(_, cursor)| cursor)
                .ok_or(wasip1::ERRNO_BADF)
        }
    }

    pub(crate) fn set_cursor(&mut self, fd: Fd, cursor: usize) -> Result<(), wasip1::Errno> {
        #[cfg(feature = "threads")]
        {
            self.map
                .get(fd as usize - 3)
                .ok_or(wasip1::ERRNO_BADF)?
                .write()
                .as_mut()
                .map(|(_, cur)| *cur = cursor)
                .ok_or(wasip1::ERRNO_BADF)
        }

        #[cfg(not(feature = "threads"))]
        {
            self.map
                .get(fd as usize - 3)
                .ok_or(wasip1::ERRNO_BADF)?
                .as_mut()
                .map(|(_, cur)| *cur = cursor)
                .ok_or(wasip1::ERRNO_BADF)
        }
    }

    pub(crate) fn fd_read_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
        iovs_ptr: *const Ciovec,
        iovs_len: usize,
    ) -> Result<Size, wasip1::Errno> {
        match fd {
            0 => {
                let lfs = &mut self.lfs;

                let iovs_vec = Wasm::as_array(iovs_ptr, iovs_len);

                let mut read = 0;

                for iovs in iovs_vec {
                    read += lfs.fd_read_stdin_raw::<Wasm>(iovs.buf as *mut _, iovs.buf_len)?;
                }

                Ok(read)
            }
            1 | 2 => return Err(wasip1::ERRNO_BADF),
            fd => {
                let mut cursor = self.get_cursor(fd)?;

                let (inode, lfs) = self.get_inode_and_lfs(fd).ok_or(wasip1::ERRNO_BADF)?;

                if lfs.is_dir(inode) {
                    return Err(wasip1::ERRNO_ISDIR);
                }

                let iovs_vec = Wasm::as_array(iovs_ptr, iovs_len);

                let mut read = 0;

                for iovs in iovs_vec {
                    let nread =
                        lfs.fd_pread_raw::<Wasm>(inode, iovs.buf as *mut _, iovs.buf_len, cursor)?;
                    read += nread;
                    cursor += nread;
                }

                self.set_cursor(fd, cursor)?;

                Ok(read)
            }
        }
    }

    pub(crate) fn path_open_raw<Wasm: WasmAccess>(
        &mut self,
        dir_fd: Fd,
        dir_flags: wasip1::Fdflags,
        path_ptr: *const u8,
        path_len: usize,
        o_flags: wasip1::Oflags,
        fs_rights_base: wasip1::Rights,
        fs_rights_inheriting: wasip1::Rights,
        fd_flags: wasip1::Fdflags,
    ) -> Result<Fd, wasip1::Errno> {
        let (inode, lfs) = self.get_inode_and_lfs(dir_fd).ok_or(wasip1::ERRNO_BADF)?;

        let new_inode = lfs.path_open_raw::<Wasm>(
            inode,
            dir_flags,
            path_ptr,
            path_len,
            o_flags,
            fs_rights_base,
            fs_rights_inheriting,
            fd_flags,
        )?;

        Ok(self.push_inode(new_inode))
    }
}
