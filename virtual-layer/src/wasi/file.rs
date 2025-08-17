// https://docs.rs/wasmtime-wasi/17.0.3/wasmtime_wasi/struct.WasiCtx.html
// https://docs.rs/wasi-common/17.0.3/wasi_common/table/struct.Table.html

use crate::memory::WasmAccess;

// no implementing dcache

#[cfg(feature = "threads")]
use parking_lot::RwLock;
use wasip1::*;

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

    pub(crate) fn fd_close_raw<Wasm: WasmAccess>(&mut self, fd: Fd) -> Result<(), wasip1::Errno> {
        if self.remove_inode(fd).is_none() {
            return Err(wasip1::ERRNO_BADF);
        }
        Ok(())
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

pub struct FilestatWithoutDevice {
    /// File serial number.
    pub ino: Inode,
    /// File type.
    pub filetype: Filetype,
    /// Number of hard links to the file.
    pub nlink: Linkcount,
    /// For regular files, the file size in bytes. For symbolic links, the length in bytes of the pathname contained in the symbolic link.
    pub size: Filesize,
    /// Last data access timestamp.
    pub atim: Timestamp,
    /// Last data modification timestamp.
    pub mtim: Timestamp,
    /// Last file status change timestamp.
    pub ctim: Timestamp,
}

impl<LFS: Wasip1LFS + Sync, const FLAT_LEN: usize> Wasip1FileSystem
    for Wasip1ConstVFS<LFS, FLAT_LEN>
where
    LFS::Inode: Copy,
{
    fn fd_write_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
        iovs_ptr: *const Ciovec,
        iovs_len: usize,
        nwritten: *mut Size,
    ) -> wasip1::Errno {
        match self.fd_write_raw::<Wasm>(fd, iovs_ptr, iovs_len) {
            Ok(n) => {
                Wasm::store_le(nwritten, n);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }

    fn fd_readdir_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
        buf: *mut u8,
        buf_len: usize,
        cookie: Dircookie,
        nread: *mut Size,
    ) -> wasip1::Errno {
        match self.fd_readdir_raw::<Wasm>(fd, buf, buf_len, cookie) {
            Ok(n) => {
                Wasm::store_le(nread, n);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }

    fn path_filestat_get_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
        filestat_ptr: *mut wasip1::Filestat,
    ) -> wasip1::Errno {
        match self.path_filestat_get_raw::<Wasm>(fd, flags, path_ptr, path_len) {
            Ok(filestat) => {
                Wasm::store_le(filestat_ptr, filestat);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }

    fn fd_prestat_get_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
        prestat_ptr: *mut wasip1::Prestat,
    ) -> wasip1::Errno {
        match self.fd_prestat_get_raw::<Wasm>(fd) {
            Ok(prestat) => {
                Wasm::store_le(prestat_ptr, prestat);
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
        match self.fd_prestat_dir_name_raw::<Wasm>(fd, dir_path_ptr, dir_path_len) {
            Ok(()) => wasip1::ERRNO_SUCCESS,
            Err(e) => e,
        }
    }

    fn fd_close_raw<Wasm: WasmAccess>(&mut self, fd: Fd) -> wasip1::Errno {
        match self.fd_close_raw::<Wasm>(fd) {
            Ok(()) => wasip1::ERRNO_SUCCESS,
            Err(e) => e,
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
        fd_ret: *mut wasip1::Fd,
    ) -> wasip1::Errno {
        match self.path_open_raw::<Wasm>(
            dir_fd,
            dir_flags,
            path_ptr,
            path_len,
            o_flags,
            fs_rights_base,
            fs_rights_inheriting,
            fd_flags,
        ) {
            Ok(fd) => {
                Wasm::store_le(fd_ret, fd);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }
}

/// small posix like local file system
pub trait Wasip1LFS {
    type Inode: 'static;
    const PRE_OPEN: &'static [Self::Inode];

    fn fd_write_raw<Wasm: WasmAccess>(
        &mut self,
        inode: Self::Inode,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno>;

    fn fd_write_stdout_raw<Wasm: WasmAccess>(
        &mut self,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno>;

    fn fd_write_stderr_raw<Wasm: WasmAccess>(
        &mut self,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno>;

    fn is_dir(&self, inode: Self::Inode) -> bool;

    fn fd_readdir_raw<Wasm: WasmAccess>(
        &mut self,
        inode: Self::Inode,
        buf: *mut u8,
        buf_len: usize,
        cookie: Dircookie,
    ) -> Result<(Size, Dircookie), wasip1::Errno>;

    fn path_filestat_get_raw<Wasm: WasmAccess>(
        &mut self,
        inode: Self::Inode,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno>;

    fn fd_prestat_get_raw<Wasm: WasmAccess>(
        &mut self,
        inode: Self::Inode,
    ) -> Result<wasip1::Prestat, wasip1::Errno>;

    fn fd_prestat_dir_name_raw<Wasm: WasmAccess>(
        &mut self,
        inode: Self::Inode,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> Result<(), wasip1::Errno>;

    fn path_open_raw<Wasm: WasmAccess>(
        &mut self,
        dir_ino: Self::Inode,
        dir_flags: wasip1::Fdflags,
        path_ptr: *const u8,
        path_len: usize,
        o_flags: wasip1::Oflags,
        fs_rights_base: wasip1::Rights,
        fs_rights_inheriting: wasip1::Rights,
        fd_flags: wasip1::Fdflags,
    ) -> Result<Self::Inode, wasip1::Errno>;
}

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
        add_info.atime = atime;
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

    fn access_time(&self, inode: usize) -> wasip1::Timestamp {
        self.add_info[inode].atime as wasip1::Timestamp
    }
}

impl<
    ROOT: VFSConstNormalFilesTy<File, FLAT_LEN>,
    File: Wasip1FileTrait + 'static + Copy,
    const FLAT_LEN: usize,
    StdIo: StdIO + 'static,
> Wasip1LFS for VFSConstNormalLFS<ROOT, File, FLAT_LEN, StdIo>
{
    type Inode = usize;
    const PRE_OPEN: &'static [Self::Inode] = ROOT::PRE_OPEN;

    fn fd_write_raw<Wasm: WasmAccess>(
        &mut self,
        _: Self::Inode,
        _: *const u8,
        _: usize,
    ) -> Result<Size, wasip1::Errno> {
        Err(wasip1::ERRNO_PERM)
    }

    fn fd_write_stdout_raw<Wasm: WasmAccess>(
        &mut self,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno> {
        #[cfg(not(feature = "multi_memory"))]
        {
            StdIo::write_direct::<Wasm>(data, data_len)
        }
        #[cfg(feature = "multi_memory")]
        {
            let mut buf = alloc::vec::Vec::with_capacity(data_len);
            unsafe { buf.set_len(data_len) };
            Wasm::memcpy_to(&mut buf, data);
            StdIo::write(&buf)
        }
    }

    fn fd_write_stderr_raw<Wasm: WasmAccess>(
        &mut self,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno> {
        #[cfg(not(feature = "multi_memory"))]
        {
            StdIo::write_direct::<Wasm>(data, data_len)
        }
        #[cfg(feature = "multi_memory")]
        {
            let mut buf = alloc::vec::Vec::with_capacity(data_len);
            unsafe { buf.set_len(data_len) };
            Wasm::memcpy_to(&mut buf, data);
            StdIo::write(&buf)
        }
    }

    fn is_dir(&self, inode: Self::Inode) -> bool {
        self.is_dir(inode)
    }

    fn fd_readdir_raw<Wasm: WasmAccess>(
        &mut self,
        inode: Self::Inode,
        buf: *mut u8,
        buf_len: usize,
        cookie: Dircookie,
    ) -> Result<(Size, Dircookie), wasip1::Errno> {
        let (_, dir) = ROOT::FILES[inode];

        // . (current directory)
        if cookie == 0 {
            let next_cookie = if dir.parent().is_some() { 1 } else { 2 };
            let entry = wasip1::Dirent {
                d_next: next_cookie,
                d_ino: inode as _,
                d_namlen: 1,
                d_type: dir.filetype(),
            };
            let entry_buf = unsafe {
                core::slice::from_raw_parts(
                    &entry as *const _ as *const u8,
                    core::cmp::min(core::mem::size_of::<wasip1::Dirent>(), buf_len),
                )
            };
            Wasm::memcpy(buf, entry_buf);

            if buf_len < core::mem::size_of::<wasip1::Dirent>() {
                return Ok((buf_len, cookie));
            }

            Wasm::memcpy(
                unsafe { buf.add(core::mem::size_of::<wasip1::Dirent>()) },
                b".",
            );

            return Ok((core::mem::size_of::<wasip1::Dirent>() + 1, next_cookie));
        }

        // .. (parent directory)
        if cookie == 1 {
            let parent = dir.parent().unwrap();
            let entry = wasip1::Dirent {
                d_next: 2,
                d_ino: parent as _,
                d_namlen: 2,
                d_type: ROOT::FILES[parent].1.filetype(),
            };
            let entry_buf = unsafe {
                core::slice::from_raw_parts(
                    &entry as *const _ as *const u8,
                    core::cmp::min(core::mem::size_of::<wasip1::Dirent>(), buf_len),
                )
            };
            Wasm::memcpy(buf, entry_buf);

            if buf_len < core::mem::size_of::<wasip1::Dirent>() {
                return Ok((buf_len, cookie));
            }

            Wasm::memcpy(
                unsafe { buf.add(core::mem::size_of::<wasip1::Dirent>()) },
                b"..",
            );

            return Ok((core::mem::size_of::<wasip1::Dirent>() + 2, 2));
        }

        let (start, end) = match dir {
            VFSConstNormalInode::Dir(range, ..) => range,
            _ => unreachable!(),
        };

        let index = start + cookie as usize - 2;
        if index >= end {
            return Ok((0, cookie)); // No more entries
        }

        let (name, file_or_dir) = ROOT::FILES[index];

        let next_cookie = cookie + 1;

        let name_len = name.len();

        let entry = wasip1::Dirent {
            d_next: if (next_cookie as usize) < end {
                next_cookie
            } else {
                0
            },
            d_ino: index as _,
            d_namlen: name_len as _,
            d_type: file_or_dir.filetype(),
        };

        let entry_buf = unsafe {
            core::slice::from_raw_parts(
                &entry as *const _ as *const u8,
                core::cmp::min(core::mem::size_of::<wasip1::Dirent>(), buf_len),
            )
        };

        Wasm::memcpy(buf, entry_buf);

        if buf_len < core::mem::size_of::<wasip1::Dirent>() {
            return Ok((buf_len, cookie));
        }

        let name_bytes = unsafe {
            core::slice::from_raw_parts(
                name.as_ptr(),
                core::cmp::min(name_len, buf_len - core::mem::size_of::<wasip1::Dirent>()),
            )
        };

        Wasm::memcpy(
            unsafe { buf.add(core::mem::size_of::<wasip1::Dirent>()) },
            name_bytes,
        );

        Ok((
            core::mem::size_of::<wasip1::Dirent>() + name_bytes.len(),
            next_cookie,
        ))
    }

    fn path_filestat_get_raw<Wasm: WasmAccess>(
        &mut self,
        inode: Self::Inode,
        _: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno> {
        let inode = self
            .get_inode_for_path::<Wasm>(inode, path_ptr, path_len)
            .ok_or(wasip1::ERRNO_NOENT)?;

        Ok(FilestatWithoutDevice {
            ino: inode as _,
            filetype: ROOT::FILES[inode].1.filetype(),
            nlink: 1,
            size: ROOT::FILES[inode].1.size() as _,
            atim: self.access_time(inode),
            mtim: 0,
            ctim: 0,
        })
    }

    fn fd_prestat_get_raw<Wasm: WasmAccess>(
        &mut self,
        inode: Self::Inode,
    ) -> Result<wasip1::Prestat, wasip1::Errno> {
        if !Self::PRE_OPEN.contains(&inode) {
            return Err(wasip1::ERRNO_BADF);
        }

        let (name, _) = ROOT::FILES[inode];

        Ok(wasip1::Prestat {
            tag: 0, // prestat is enum but variant is only 0
            // union type but we only have one variant
            u: wasip1::PrestatU {
                dir: wasip1::PrestatDir {
                    pr_name_len: name.len() as _,
                },
            },
        })
    }

    fn fd_prestat_dir_name_raw<Wasm: WasmAccess>(
        &mut self,
        inode: Self::Inode,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        if !Self::PRE_OPEN.contains(&inode) {
            return Err(wasip1::ERRNO_BADF);
        }

        let (name, _) = ROOT::FILES[inode];

        Wasm::memcpy(
            dir_path_ptr,
            &name.as_bytes()[..core::cmp::min(name.len(), dir_path_len)],
        );

        Ok(())
    }

    fn path_open_raw<Wasm: WasmAccess>(
        &mut self,
        dir_inode: Self::Inode,
        _: wasip1::Fdflags,
        path_ptr: *const u8,
        path_len: usize,
        o_flags: wasip1::Oflags,
        fs_rights_base: wasip1::Rights,
        _: wasip1::Rights,
        _: wasip1::Fdflags,
    ) -> Result<Self::Inode, wasip1::Errno> {
        if let Some(inode) = self.get_inode_for_path::<Wasm>(dir_inode, path_ptr, path_len) {
            if o_flags & wasip1::OFLAGS_EXCL == wasip1::OFLAGS_EXCL {
                return Err(wasip1::ERRNO_EXIST);
            }

            if o_flags & wasip1::OFLAGS_DIRECTORY == wasip1::OFLAGS_DIRECTORY && !self.is_dir(inode)
            {
                return Err(wasip1::ERRNO_NOTDIR);
            }

            if fs_rights_base & wasip1::RIGHTS_FD_WRITE == wasip1::RIGHTS_FD_WRITE {
                return Err(wasip1::ERRNO_PERM);
            }

            if o_flags & wasip1::OFLAGS_TRUNC == wasip1::OFLAGS_TRUNC {
                return Err(wasip1::ERRNO_PERM);
            }

            Ok(inode)
        } else {
            if o_flags & wasip1::OFLAGS_CREAT == wasip1::OFLAGS_CREAT {
                return Err(wasip1::ERRNO_PERM);
            }

            Err(wasip1::ERRNO_NOENT)
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct VFSConstNormalAddInfo {
    atime: usize,
}

impl VFSConstNormalAddInfo {
    pub const fn new() -> Self {
        Self { atime: 0 }
    }
}

use const_struct::ConstStruct;

use crate::{
    memory::{WasmPathAccess, WasmPathComponent},
    transporter::Wasip1Transporter,
};

/// A constant file system root that can be used in a WASI component.
#[derive(ConstStruct, Debug)]
pub struct VFSConstNormalFiles<File: Wasip1FileTrait + 'static + Copy, const FLAT_LEN: usize> {
    pub files: [(&'static str, VFSConstNormalInode<File>); FLAT_LEN],
    pub pre_open: &'static [usize],
}

impl<File: Wasip1FileTrait + 'static + Copy, const FLAT_LEN: usize>
    VFSConstNormalFiles<File, FLAT_LEN>
{
    pub const fn new(
        files: (
            [(&'static str, VFSConstNormalInode<File>); FLAT_LEN],
            &'static [usize],
        ),
    ) -> Self {
        Self {
            files: files.0,
            pre_open: files.1,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum VFSConstNormalInode<File: Wasip1FileTrait + 'static + Copy> {
    /// file, parent
    File(File, usize),
    /// (first index..last index), parent
    Dir((usize, usize), Option<usize>),
}

impl<File: Wasip1FileTrait + 'static + Copy> VFSConstNormalInode<File> {
    pub const fn filetype(&self) -> wasip1::Filetype {
        match self {
            Self::File(..) => wasip1::FILETYPE_REGULAR_FILE,
            Self::Dir(..) => wasip1::FILETYPE_DIRECTORY,
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Self::File(file, _) => file.size(),
            Self::Dir(..) => core::mem::size_of::<((usize, usize), Option<usize>)>(), // directory size is just the size of the inode
        }
    }

    pub fn parent(&self) -> Option<usize> {
        match self {
            Self::File(_, parent) => Some(*parent),
            Self::Dir(_, parent) => *parent,
        }
    }
}

#[macro_export]
macro_rules! ConstFiles {
        (
            [
                $(($dir_name:expr, $file_or_dir:tt)),* $(,)?
            ] $(,)?
        ) => {
            $crate::wasi::file::VFSConstNormalFiles::new({
                const COUNT: usize = {
                    let mut count = 0;

                    $(
                        $crate::ConstFiles!(@counter, count, $file_or_dir);
                    )*

                    count
                };

                let mut static_array = $crate::binary_map::StaticArrayBuilder::new();

                struct CheckEqNumberOfFilesAndDirs<const L: usize, const R: usize>;

                #[allow(dead_code)]
                impl<const L: usize, const R: usize> CheckEqNumberOfFilesAndDirs<L, R> {
                    #[allow(non_upper_case_globals)]
                    const number_of_files_and_dirs_equals_FLAT_LEN_so_you_must_set_VFSConstNormalFiles_num: usize = (R - L) + (L - R);
                }

                const fn asserter<S: 'static + Copy, const N: usize>(
                    _: &$crate::binary_map::StaticArrayBuilder<S, N>,
                ) {
                    #[allow(path_statements)]
                    CheckEqNumberOfFilesAndDirs::<COUNT, N>::number_of_files_and_dirs_equals_FLAT_LEN_so_you_must_set_VFSConstNormalFiles_num;
                }

                asserter(&static_array);

                use $crate::__private::const_for;

                const fn eq_str(a: &str, b: &str) -> bool {
                    let a_bytes = a.as_bytes();
                    let b_bytes = b.as_bytes();

                    if a_bytes.len() != b_bytes.len() {
                        return false;
                    }

                    const_for!(i in 0..a_bytes.len() => {
                        if a_bytes[i] != b_bytes[i] {
                            return false;
                        }
                    });

                    true
                }

                /// if b is a parent of a
                const fn is_parent(a: &str, b: &str) -> bool {
                    let a_bytes = a.as_bytes();
                    let b_bytes = b.as_bytes();

                    if a_bytes.len() < b_bytes.len() {
                        return false;
                    }

                    const_for!(i in 0..b_bytes.len() => {
                        if a_bytes[i] != b_bytes[i] {
                            return false;
                        }
                    });

                    let mut i = b_bytes.len();
                    while i < a_bytes.len() && a_bytes[i] == b"/"[0] {
                        i += 1;
                    }
                    if i == a_bytes.len() {
                        return false;
                    }
                    if i == b_bytes.len() {
                        return false;
                    }

                    const_for!(n in i..a_bytes.len() => {
                        if a_bytes[n] == b"/"[0] {
                            return false;
                        }
                    });

                    true
                }

                const fn get_child_range<S: 'static + Copy, const N: usize>(
                    fake_files: [&'static str; N],
                    name: &'static str,
                    _: &$crate::binary_map::StaticArrayBuilder<S, N>,
                ) -> (usize, usize) {
                    get_child_range_inner(fake_files, name)
                }

                const fn get_child_range_inner<const N: usize>(
                    fake_files: [&'static str; N],
                    name: &'static str,
                ) -> (usize, usize) {
                    let mut first_index = None;
                    let mut last_index = None;
                    const_for!(i in 0..N => {
                        if is_parent(fake_files[i], name) {
                            if first_index.is_none() {
                                first_index = Some(i);
                            }
                            last_index = Some(i);
                        }
                    });

                    (first_index.unwrap(), last_index.unwrap() + 1)
                }

                const fn get_parent<S: 'static + Copy, const N: usize>(
                    fake_files: [&'static str; N],
                    name: &'static str,
                    _: &$crate::binary_map::StaticArrayBuilder<S, N>,
                ) -> Option<usize> {
                    const_for!(i in 0..N => {
                        if is_parent(name, fake_files[i]) {
                            return Some(i);
                        }
                    });
                    None
                }

                const fn get_self<const N: usize>(
                    fake_files: [&'static str; N],
                    name: &'static str,
                ) -> usize {
                    const_for!(i in 0..N => {
                        if eq_str(name, fake_files[i]) {
                            return i;
                        }
                    });
                    unreachable!()
                }

                const fn depth(
                    name: &'static str,
                ) -> usize {
                    let mut depth = 0;
                    let mut i = 0;
                    while (i < name.len()) {
                        if name.as_bytes()[i] == b'/' {
                            depth += 1;
                        }
                        i += 1;
                        while (i < name.len() && name.as_bytes()[i] == b'/') {
                            i += 1;
                        }
                    }
                    depth
                }

                const fn custom_sort<T: Copy, const N: usize>(
                    mut files: $crate::binary_map::StaticArrayBuilder<(usize, T), N>,
                ) -> [T; N] {
                    let mut sorted = $crate::binary_map::StaticArrayBuilder::<_, N>::new();

                    while (files.len() > 0) {
                        let mut depth = None;
                        let mut index = None;
                        const_for!(i in 0..files.len() => {
                            let file = files.get(i).unwrap();

                            if let Some(d) = depth {
                                if file.0 < d {
                                    depth = Some(file.0);
                                    index = Some((i, file.1));
                                }
                            } else {
                                depth = Some(file.0);
                                index = Some((i, file.1));
                            }
                        });
                        if let Some(index) = index {
                            sorted.push(index.1);
                            files.remove(index.0);
                        }
                    }

                    sorted.build()
                }

                const EMPTY_ARR: [&'static str; COUNT] = {
                    let mut empty_arr = $crate::binary_map::StaticArrayBuilder::new();

                    $(
                        $crate::ConstFiles!(@empty, 0, empty_arr, [$dir_name], $file_or_dir);
                    )*

                    let _ = empty_arr.build();

                    custom_sort(empty_arr)
                };

                $(
                    $crate::ConstFiles!(
                        @next,
                        0,
                        static_array,
                        [EMPTY_ARR],
                        [$dir_name],
                        [$dir_name],
                        $file_or_dir
                    );
                )*

                const PRE_OPEN_COUNT: usize = {
                    let mut count = 0;

                    $(
                        $crate::ConstFiles!(@pre_open_counter, count, $file_or_dir);
                    )*
                    count
                };

                const PRE_OPEN: [usize; PRE_OPEN_COUNT] = {
                    let mut static_array = $crate::binary_map::StaticArrayBuilder::new();

                    $(
                        static_array.push(get_self(EMPTY_ARR, $dir_name));
                    )*

                    static_array.build()
                };

                let static_array = custom_sort(static_array);

                let mut file_array = $crate::binary_map::StaticArrayBuilder::new();
                const_for!(i in 0..static_array.len() => {
                    let (_, name, file_or_dir) = static_array[i];
                    file_array.push((
                        name,
                        file_or_dir
                    ));
                });

                (file_array.build(), &PRE_OPEN)
            })
        };

        (@counter, $count:ident, [
            $(($file_or_dir_name:expr, $file_or_dir:tt)),* $(,)?
        ]) => {
            $(
                $crate::ConstFiles!(@counter, $count, $file_or_dir);
            )*
            $count += 1;
        };

        (@counter, $count:ident, $file:tt) => {
            $count += 1;
        };

        (@pre_open_counter, $count:ident, $file:tt) => {
            $count += 1;
        };

        (@empty, $depth:expr, $empty_arr:ident, [$parent_name:expr], [
            $(($file_or_dir_name:expr, $file_or_dir:tt)),* $(,)?
        ]) => {
            $(
                $crate::ConstFiles!(@empty, $depth + 1, $empty_arr, [concat!($parent_name, "/", $file_or_dir_name)], $file_or_dir);
            )*
            $empty_arr.push(($depth, $parent_name));
        };

        (@empty, $depth:expr, $empty_arr:ident, [$name:expr], $file:tt) => {
            $empty_arr.push(($depth, $name));
        };

        (@next, $depth:expr, $static_array:ident, [$empty:expr], [$parent_path:expr], [$name:expr], [
            $(($file_or_dir_name:expr, $file_or_dir:tt)),* $(,)?
        ]) => {
            $(
                $crate::ConstFiles!(@next, $depth + 1, $static_array, [$empty], [concat!($parent_path, "/", $file_or_dir_name)], [$file_or_dir_name], $file_or_dir);
            )*
            $static_array.push(($depth, (
                $parent_path,
                $name,
                $crate::wasi::file::VFSConstNormalInode::Dir(
                    get_child_range(
                        $empty,
                        $parent_path,
                        &$static_array
                    ),
                    get_parent($empty, $parent_path, &$static_array)
                )
            )));
        };

        (@next, $depth:expr, $static_array:ident, [$empty:expr], [$path:expr], [$name:expr], $file:tt) => {
            $static_array.push((
                $depth,
                (
                    $path,
                    $name,
                $crate::wasi::file::VFSConstNormalInode::File($file, get_parent($empty, $path, &$static_array).unwrap())
            )));
        };
    }

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WasiConstFile<File: WasiConstPrimitiveFile> {
    file: File,
}

impl<File: WasiConstPrimitiveFile> WasiConstFile<File> {
    pub const fn new(file: File) -> Self {
        Self { file }
    }
}

pub trait WasiConstPrimitiveFile {
    fn len(&self) -> usize;
}

impl<'a> WasiConstPrimitiveFile for &'a str {
    #[inline(always)]
    fn len(&self) -> usize {
        <Self as core::ops::Deref>::deref(self).len()
    }
}

impl<File: WasiConstPrimitiveFile> Wasip1FileTrait for WasiConstFile<File> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.file.len()
    }

    fn size(&self) -> usize {
        self.file.len()
    }
}

pub struct DefaultStdIO;

impl StdIO for DefaultStdIO {
    fn write(buf: &[u8]) -> Result<Size, wasip1::Errno> {
        Wasip1Transporter::write_to_stdout(buf)
    }

    #[cfg(not(feature = "multi_memory"))]
    fn write_direct<Wasm: WasmAccess>(buf: *const u8, len: usize) -> Result<Size, wasip1::Errno> {
        Wasip1Transporter::write_to_stdout_direct::<Wasm>(buf, len)
    }

    fn ewrite(buf: &[u8]) -> Result<Size, wasip1::Errno> {
        Wasip1Transporter::write_to_stderr(buf)
    }

    #[cfg(not(feature = "multi_memory"))]
    fn ewrite_direct<Wasm: WasmAccess>(buf: *const u8, len: usize) -> Result<Size, wasip1::Errno> {
        Wasip1Transporter::write_to_stderr_direct::<Wasm>(buf, len)
    }
}

pub trait StdIO {
    /// This function is called when the alloc feature is ON
    /// and write_direct is not implemented.
    /// If you are not familiar with Wasm memory, etc.,
    /// it is better to use this.
    #[allow(unused_variables)]
    fn write(buf: &[u8]) -> Result<Size, wasip1::Errno> {
        Err(wasip1::ERRNO_NOSYS)
    }

    /// This function is called,
    /// but if the write function is implemented
    /// and the alloc feature is ON,
    /// this function is automatically implemented.
    #[cfg(not(feature = "multi_memory"))]
    #[allow(unused_variables)]
    fn write_direct<Wasm: WasmAccess>(buf: *const u8, len: usize) -> Result<Size, wasip1::Errno> {
        #[cfg(feature = "alloc")]
        {
            Self::write(&Wasm::get_array(buf, len))
        }

        #[cfg(not(feature = "alloc"))]
        {
            // Stub implementation for non-std environments
            Err(wasip1::ERRNO_NOSYS)
        }
    }

    /// This function is called when the alloc feature is ON
    /// and ewrite_direct is not implemented.
    /// If you are not familiar with Wasm memory, etc.,
    /// it is better to use this.
    #[allow(unused_variables)]
    fn ewrite(buf: &[u8]) -> Result<Size, wasip1::Errno> {
        Err(wasip1::ERRNO_NOSYS)
    }

    /// This function is called,
    /// but if the ewrite function is implemented
    /// and the alloc feature is ON,
    /// this function is automatically implemented.
    #[cfg(not(feature = "multi_memory"))]
    #[allow(unused_variables)]
    fn ewrite_direct<Wasm: WasmAccess>(buf: *const u8, len: usize) -> Result<Size, wasip1::Errno> {
        #[cfg(feature = "alloc")]
        {
            Self::ewrite(&Wasm::get_array(buf, len))
        }

        #[cfg(not(feature = "alloc"))]
        {
            // Stub implementation for non-std environments
            Err(wasip1::ERRNO_NOSYS)
        }
    }
}

pub trait Wasip1FileTrait {
    fn size(&self) -> usize;

    fn len(&self) -> usize;

    /// Reads data from the file into the provided buffer.
    /// Returns the number of bytes read.
    /// This function is called by the `fd_read` function.
    /// Implementing this function directly is more efficient,
    /// but it is recommended to implement `fn read`!
    #[cfg_attr(not(feature = "alloc"), allow(unused_variables))]
    fn read_iovs<Wasm: WasmAccess>(
        &self,
        iovs: *const wasip1::Ciovec,
        iovs_len: usize,
    ) -> Result<usize, wasip1::Errno> {
        #[cfg(feature = "alloc")]
        {
            let mut total_read = 0;

            for i in 0..iovs_len {
                let iov = unsafe { iovs.add(i).as_ref() }.ok_or(wasip1::ERRNO_FAULT)?;
                let mut buf = alloc::vec![0u8; iov.buf_len];
                let read = self.read(&mut buf)?;
                if read == 0 {
                    break; // EOF
                }
                total_read += read;
                Wasm::memcpy(iov.buf as *mut _, &buf[..read]);
            }

            Ok(total_read)
        }

        #[cfg(not(feature = "alloc"))]
        {
            // Stub implementation for non-std environments
            Err(wasip1::ERRNO_NOSYS)
        }
    }

    /// Reads data from the file into the provided buffer.
    /// Returns the number of bytes read.
    fn read(&self, _buf: &mut [u8]) -> Result<usize, wasip1::Errno> {
        return Err(wasip1::ERRNO_NOSYS);
    }
}

pub trait Wasip1FileSystem {
    fn fd_write_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
        iovs_ptr: *const Ciovec,
        iovs_len: usize,
        nwritten: *mut Size,
    ) -> wasip1::Errno;

    fn fd_readdir_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
        buf: *mut u8,
        buf_len: usize,
        cookie: Dircookie,
        nread: *mut Size,
    ) -> wasip1::Errno;

    fn path_filestat_get_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
        filestat: *mut wasip1::Filestat,
    ) -> wasip1::Errno;

    fn fd_prestat_get_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
        prestat: *mut wasip1::Prestat,
    ) -> wasip1::Errno;

    fn fd_prestat_dir_name_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> wasip1::Errno;

    fn fd_close_raw<Wasm: WasmAccess>(&mut self, fd: Fd) -> wasip1::Errno;

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
        fd_ret: *mut wasip1::Fd,
    ) -> wasip1::Errno;
}

#[macro_export]
macro_rules! export_fs {
    (@const, $state:expr, $wasm:ty) => {
        $crate::__private::paste::paste! {
            #[cfg(target_arch = "wasm32")]
            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_write>](
                fd: $crate::wasip1::Fd,
                iovs_ptr: *const $crate::wasip1::Ciovec,
                iovs_len: usize,
                nwritten: *mut usize,
            ) -> $crate::wasip1::Errno {
                let state = $state;
                $crate::wasi::file::Wasip1FileSystem::fd_write_raw::<$wasm>(state, fd, iovs_ptr, iovs_len, nwritten)
            }

            #[cfg(target_arch = "wasm32")]
            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_readdir>](
                fd: $crate::wasip1::Fd,
                buf: *mut u8,
                buf_len: usize,
                cookie: $crate::wasip1::Dircookie,
                nread: *mut $crate::wasip1::Size,
            ) -> $crate::wasip1::Errno {
                let state = $state;
                $crate::wasi::file::Wasip1FileSystem::fd_readdir_raw::<$wasm>(state, fd, buf, buf_len, cookie, nread)
            }

            #[cfg(target_arch = "wasm32")]
            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _path_filestat_get>](
                fd: $crate::wasip1::Fd,
                flags: $crate::wasip1::Lookupflags,
                path_ptr: *const u8,
                path_len: usize,
                filestat: *mut $crate::wasip1::Filestat,
            ) -> $crate::wasip1::Errno {
                let state = $state;
                $crate::wasi::file::Wasip1FileSystem::path_filestat_get_raw::<$wasm>(state, fd, flags, path_ptr, path_len, filestat)
            }

            #[cfg(target_arch = "wasm32")]
            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_prestat_get>](
                fd: $crate::wasip1::Fd,
                prestat: *mut $crate::wasip1::Prestat,
            ) -> $crate::wasip1::Errno {
                let state = $state;
                $crate::wasi::file::Wasip1FileSystem::fd_prestat_get_raw::<$wasm>(state, fd, prestat)
            }

            #[cfg(target_arch = "wasm32")]
            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_prestat_dir_name>](
                fd: $crate::wasip1::Fd,
                dir_path_ptr: *mut u8,
                dir_path_len: usize,
            ) -> $crate::wasip1::Errno {
                let state = $state;
                $crate::wasi::file::Wasip1FileSystem::fd_prestat_dir_name_raw::<$wasm>(state, fd, dir_path_ptr, dir_path_len)
            }

            #[cfg(target_arch = "wasm32")]
            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_close>](
                fd: $crate::wasip1::Fd,
            ) -> $crate::wasip1::Errno {
                let state = $state;
                $crate::wasi::file::Wasip1FileSystem::fd_close_raw::<$wasm>(state, fd)
            }

            #[unsafe(no_mangle)]
            #[cfg(target_arch = "wasm32")]
            pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _path_open>](
                fd: $crate::wasip1::Fd,
                dir_flags: $crate::wasip1::Fdflags,
                path_ptr: *const u8,
                path_len: usize,
                o_flags: $crate::wasip1::Oflags,
                fs_rights_base: $crate::wasip1::Rights,
                fs_rights_inheriting: $crate::wasip1::Rights,
                fd_flags: $crate::wasip1::Fdflags,
                fd_ret: *mut $crate::wasip1::Fd,
            ) -> $crate::wasip1::Errno {
                let state = $state;
                $crate::wasi::file::Wasip1FileSystem::path_open_raw::<$wasm>(state, fd, dir_flags, path_ptr, path_len, o_flags, fs_rights_base, fs_rights_inheriting, fd_flags, fd_ret)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{
        ConstFiles,
        wasi::file::{VFSConstNormalFiles, WasiConstFile},
    };

    use const_for::const_for;

    const fn is_parent(a: &str, b: &str) -> bool {
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();

        if a_bytes.len() < b_bytes.len() {
            return false;
        }

        const_for!(i in 0..b_bytes.len() => {
            if a_bytes[i] != b_bytes[i] {
                return false;
            }
        });

        let mut i = b_bytes.len();
        while i < a_bytes.len() && a_bytes[i] == b"/"[0] {
            i += 1;
        }
        if i == a_bytes.len() {
            return false;
        }
        if i == b_bytes.len() {
            return false;
        }

        const_for!(n in i..a_bytes.len() => {
            if a_bytes[n] == b"/"[0] {
                return false;
            }
        });

        true
    }

    /// If not using `--release`, compilation will fail with: link error
    /// cargo test -r --package wasip1-virtual-layer --lib -- wasi::file::tests::test_file_flat_iterate --exact --show-output
    #[test]
    fn test_file_flat_iterate() {
        #[allow(dead_code)]
        const FILES: VFSConstNormalFiles<WasiConstFile<&'static str>, 10> = ConstFiles!([
            (
                "/root",
                [("root.txt", { WasiConstFile::new("This is root") })]
            ),
            (
                ".",
                [
                    ("hey", { WasiConstFile::new("Hey!") }),
                    (
                        "hello",
                        [
                            ("world", { WasiConstFile::new("Hello, world!") }),
                            ("everyone", { WasiConstFile::new("Hello, everyone!") }),
                        ]
                    )
                ]
            ),
            (
                "~",
                [
                    ("home", { WasiConstFile::new("This is home") }),
                    ("user", { WasiConstFile::new("This is user") }),
                ]
            )
        ]);

        #[cfg(feature = "std")]
        println!("Files: {:#?}", FILES);

        assert!(is_parent("/root/root.txt", "/root"));
        assert!(is_parent("./hey", "."));
        assert!(is_parent("./hello", "."));
        assert!(is_parent("~/home", "~"));
        assert!(is_parent("~/user", "~"));
        assert!(is_parent("./hello/world", "./hello"));
        assert!(is_parent("./hello/everyone", "./hello"));
        assert!(!is_parent("./hello/world", "."));
    }
}
