use crate::__private::wasip1;
use crate::__private::wasip1::Dircookie;

use crate::wasi::file::{Wasip1ConstLFS, Wasip1DynamicLFS};
use crate::{
    memory::WasmAccess,
    wasi::file::{
        FilestatWithoutDevice, WasiAddInfo, Wasip1FileTrait, Wasip1LFS,
        constant::{
            lfs::VFSConstNormalLFS,
            lfs_raw::{VFSConstNormalFilesTy, VFSConstNormalInode},
        },
        stdio::StdIO,
    },
};

impl<
    ROOT: VFSConstNormalFilesTy<File, FLAT_LEN> + core::fmt::Debug,
    File: Wasip1FileTrait + 'static + Copy,
    const FLAT_LEN: usize,
    StdIo: StdIO + 'static,
    AddInfo: WasiAddInfo + 'static,
> Wasip1LFS for VFSConstNormalLFS<ROOT, File, FLAT_LEN, StdIo, AddInfo>
{
    type Inode = usize;

    fn fd_write_raw<Wasm: WasmAccess>(
        &self,
        _: Self::Inode,
        _: *const u8,
        _: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        Err(wasip1::ERRNO_PERM)
    }

    fn fd_write_raw_dyn_compatible(
        &self,
        _: &impl crate::memory::WasmAccessDynCompatible,
        _: Self::Inode,
        _: *const u8,
        _: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        Err(wasip1::ERRNO_PERM)
    }

    fn fd_write_stdout_raw<Wasm: WasmAccess>(
        &self,
        data: *const u8,
        data_len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        #[cfg(not(feature = "multi_memory"))]
        {
            StdIo::write_direct::<Wasm>(data, data_len)
        }
        #[cfg(feature = "multi_memory")]
        {
            let (buf, _) = unsafe {
                use crate::utils::alloc_buff;
                alloc_buff(data_len, |buf| Wasm::memcpy_to(buf, data))
            };
            StdIo::write(&buf)
        }
    }

    fn fd_write_stdout_raw_dyn_compatible(
        &self,
        access: &impl crate::memory::WasmAccessDynCompatible,
        data: *const u8,
        data_len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        #[cfg(not(feature = "multi_memory"))]
        {
            StdIo::write_direct_dyn_compatible::<_, Self>(access, data, data_len)
        }
        #[cfg(feature = "multi_memory")]
        {
            let (buf, _) = unsafe {
                use crate::utils::alloc_buff;
                alloc_buff(data_len, |buf| access.memcpy_to_with(buf, data))
            };
            StdIo::write(&buf)
        }
    }

    fn fd_write_stderr_raw<Wasm: WasmAccess>(
        &self,
        data: *const u8,
        data_len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        #[cfg(not(feature = "multi_memory"))]
        {
            StdIo::ewrite_direct::<Wasm>(data, data_len)
        }
        #[cfg(feature = "multi_memory")]
        {
            let (buf, _) = unsafe {
                use crate::utils::alloc_buff;
                alloc_buff(data_len, |buf| Wasm::memcpy_to(buf, data))
            };
            StdIo::ewrite(&buf)
        }
    }

    fn fd_write_stderr_raw_dyn_compatible(
        &self,
        access: &impl crate::memory::WasmAccessDynCompatible,
        data: *const u8,
        data_len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        #[cfg(not(feature = "multi_memory"))]
        {
            StdIo::ewrite_direct_dyn_compatible::<_, Self>(access, data, data_len)
        }
        #[cfg(feature = "multi_memory")]
        {
            let (buf, _) = unsafe {
                use crate::utils::alloc_buff;
                alloc_buff(data_len, |buf| access.memcpy_to_with(buf, data))
            };
            StdIo::ewrite(&buf)
        }
    }

    fn is_dir(&self, inode: Self::Inode) -> bool {
        self.is_dir(inode)
    }

    fn fd_readdir_raw<Wasm: WasmAccess>(
        &self,
        inode: Self::Inode,
        buf: *mut u8,
        buf_len: usize,
        cookie: Dircookie,
    ) -> Result<(wasip1::Size, Dircookie), wasip1::Errno> {
        self.fd_readdir_raw_inner(inode, buf, buf_len, cookie, |dst, src, len| {
            Wasm::memcpy_raw(dst, src, len);
        })
    }

    fn fd_readdir_raw_dyn_compatible(
        &self,
        access: &impl crate::memory::WasmAccessDynCompatible,
        inode: Self::Inode,
        buf: *mut u8,
        buf_len: usize,
        cookie: Dircookie,
    ) -> Result<(wasip1::Size, Dircookie), wasip1::Errno> {
        self.fd_readdir_raw_inner(inode, buf, buf_len, cookie, |dst, src, len| {
            access.memcpy_to_raw(dst, src, len);
        })
    }

    fn path_filestat_get_raw<Wasm: WasmAccess>(
        &self,
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

    fn path_filestat_get_raw_dyn_compatible(
        &self,
        access: &impl crate::memory::WasmAccessDynCompatible,
        inode: Self::Inode,
        _: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno> {
        let inode = self.get_inode_for_path_dyn_compatible(access, inode, path_ptr, path_len)
            .ok_or(wasip1::ERRNO_NOENT)?;

        Ok(self.filestat_from_inode(inode))
    }

    fn fd_prestat_get_raw<Wasm: WasmAccess>(
        &self,
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
        &self,
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
        &self,
        inode: Self::Inode,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno> {
        Ok(self.filestat_from_inode(inode))
    }

    fn fd_pread_raw<Wasm: WasmAccess>(
        &self,
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
        &self,
        buf: *mut u8,
        buf_len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        #[cfg(not(feature = "multi_memory"))]
        {
            StdIo::read_direct::<Wasm>(buf, buf_len)
        }

        #[cfg(feature = "multi_memory")]
        {
            use crate::__private::utils;

            let (buf_vec, read) = unsafe { utils::alloc_buff(buf_len, |buf| StdIo::read(buf)) };
            Wasm::memcpy(buf, &buf_vec);
            Ok(read?)
        }
    }

    fn path_open_raw<Wasm: WasmAccess>(
        &self,
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

impl<
    ROOT: VFSConstNormalFilesTy<File, FLAT_LEN> + core::fmt::Debug,
    File: Wasip1FileTrait + 'static + Copy,
    const FLAT_LEN: usize,
    StdIo: StdIO + 'static,
    AddInfo: WasiAddInfo + 'static,
> Wasip1ConstLFS for VFSConstNormalLFS<ROOT, File, FLAT_LEN, StdIo, AddInfo>
{
    const PRE_OPEN: &'static [Self::Inode] = ROOT::PRE_OPEN;
}

impl<
    ROOT: VFSConstNormalFilesTy<File, FLAT_LEN> + core::fmt::Debug,
    File: Wasip1FileTrait + 'static + Copy,
    const FLAT_LEN: usize,
    StdIo: StdIO + 'static,
    AddInfo: WasiAddInfo + 'static,
> Wasip1DynamicLFS for VFSConstNormalLFS<ROOT, File, FLAT_LEN, StdIo, AddInfo>
{
    fn pre_open_inodes<'a>(
        &'a self,
    ) -> impl IntoIterator<Item = (Self::Inode, impl crate::wasi::file::DerefToStr)> {
        ROOT::PRE_OPEN.iter().map(|&inode| {
            let (name, _) = ROOT::FILES[inode];
            (inode, name)
        })
    }
}
