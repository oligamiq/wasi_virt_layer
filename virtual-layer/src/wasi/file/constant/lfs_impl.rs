use crate::__private::wasip1;
use crate::__private::wasip1::Dircookie;

use crate::{
    memory::WasmAccess,
    wasi::file::{
        FilestatWithoutDevice, Wasip1FileTrait, Wasip1LFS,
        constant::{
            lfs::VFSConstNormalLFS,
            lfs_raw::{VFSConstNormalFilesTy, VFSConstNormalInode},
        },
        stdio::StdIO,
    },
};

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
    ) -> Result<wasip1::Size, wasip1::Errno> {
        Err(wasip1::ERRNO_PERM)
    }

    fn fd_write_stdout_raw<Wasm: WasmAccess>(
        &mut self,
        data: *const u8,
        data_len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
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
    ) -> Result<wasip1::Size, wasip1::Errno> {
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
    ) -> Result<(wasip1::Size, Dircookie), wasip1::Errno> {
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

        Ok(self.filestat_from_inode(inode))
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

    fn fd_filestat_get_raw<Wasm: WasmAccess>(
        &mut self,
        inode: Self::Inode,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno> {
        Ok(self.filestat_from_inode(inode))
    }

    fn fd_pread_raw<Wasm: WasmAccess>(
        &mut self,
        inode: Self::Inode,
        buf: *mut u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        let (_, file_or_dir) = ROOT::FILES[inode];

        if let VFSConstNormalInode::File(file, _) = file_or_dir {
            if offset >= file.size() {
                return Ok(0); // No data to read
            }

            let buf_len = core::cmp::min(buf_len, file.size() - offset);
            let nread = file.pread_raw::<Wasm>(buf, buf_len, offset)?;

            Ok(nread)
        } else {
            unreachable!();
        }
    }

    fn fd_read_stdin_raw<Wasm: WasmAccess>(
        &mut self,
        buf: *mut u8,
        buf_len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        #[cfg(not(feature = "multi_memory"))]
        {
            StdIo::read_direct::<Wasm>(buf, buf_len)
        }

        #[cfg(feature = "multi_memory")]
        {
            let mut buf_vec = alloc::vec::Vec::with_capacity(buf_len);
            unsafe { buf_vec.set_len(buf_len) };
            let read = StdIo::read(&mut buf_vec)?;
            Wasm::memcpy(buf, &buf_vec);
            Ok(read)
        }
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

    pub const fn access_time(&self) -> usize {
        self.atime
    }

    pub const fn set_access_time(&mut self, atime: usize) {
        self.atime = atime;
    }
}
