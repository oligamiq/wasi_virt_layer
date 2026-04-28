#![cfg(feature = "embedded-fs")]

use crate::__private::wasip1;
use crate::__private::wasip1::Dircookie;

use crate::memory::{WasmAccessDynCompatible, WasmAccessDynCompatibleRaw};
use crate::wasi::file::Wasip1LFSBaseWrapper as _;
use crate::wasi::file::{
    BoxedInode, DerefToStrCustom, DynamicLFS, EmbeddedLFS, InodeIdCommon, Wasip1DynCompatibleLFS,
    Wasip1DynCompatibleLFSSlice, Wasip1LFSBase,
};
use crate::{
    memory::WasmAccess,
    wasi::file::{
        FilestatWithoutDevice, WasiAddInfo, Wasip1FileTrait,
        embedded::{
            lfs::StandardEmbeddedNormalLFS,
            lfs_raw::{StandardEmbeddedFilesTy, StandardEmbeddedInode},
        },
        stdio::StdIO,
    },
};

impl<
    ROOT: StandardEmbeddedFilesTy<File, FLAT_LEN> + core::fmt::Debug,
    File: Wasip1FileTrait + 'static + Copy,
    const FLAT_LEN: usize,
    StdIo: StdIO + 'static,
    AddInfo: WasiAddInfo + 'static,
> Wasip1LFSBase for StandardEmbeddedNormalLFS<ROOT, File, FLAT_LEN, StdIo, AddInfo>
{
    type Inode = usize;

    fn fd_write_raw<Wasm: WasmAccess>(
        &self,
        _: &Self::Inode,
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
    fn is_dir(&self, inode: &Self::Inode) -> bool {
        self.is_dir(*inode)
    }

    fn fd_readdir_raw<Wasm: WasmAccess>(
        &self,
        inode: &Self::Inode,
        buf: *mut u8,
        buf_len: usize,
        cookie: Dircookie,
    ) -> Result<(wasip1::Size, Dircookie), wasip1::Errno> {
        self.fd_readdir_raw_inner(*inode, buf, buf_len, cookie, |dst, src, len| {
            Wasm::memcpy_raw(dst, src, len);
        })
    }

    fn path_filestat_get_raw<Wasm: WasmAccess>(
        &self,
        inode: &Self::Inode,
        _: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno> {
        let inode = self
            .get_inode_for_path::<Wasm>(*inode, path_ptr, path_len)
            .ok_or(wasip1::ERRNO_NOENT)?;

        Ok(self.filestat_from_inode(inode))
    }

    fn fd_prestat_get_raw<Wasm: WasmAccess>(
        &self,
        inode: &Self::Inode,
    ) -> Result<wasip1::Prestat, wasip1::Errno> {
        if !Self::PRE_OPEN.contains(inode) {
            return Err(wasip1::ERRNO_BADF);
        }

        let (name, _) = ROOT::FILES[*inode];

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
        inode: &Self::Inode,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        if !Self::PRE_OPEN.contains(inode) {
            return Err(wasip1::ERRNO_BADF);
        }

        let (name, _) = ROOT::FILES[*inode];

        Wasm::memcpy(
            dir_path_ptr,
            &name.as_bytes()[..core::cmp::min(name.len(), dir_path_len)],
        );

        Ok(())
    }

    fn fd_filestat_get_raw<Wasm: WasmAccess>(
        &self,
        inode: &Self::Inode,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno> {
        Ok(self.filestat_from_inode(*inode))
    }

    fn fd_pread_raw<Wasm: WasmAccess>(
        &self,
        inode: &Self::Inode,
        buf: *mut u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        let (_, file_or_dir) = ROOT::FILES[*inode];

        if let StandardEmbeddedInode::File(file, _) = file_or_dir {
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
        dir_inode: &Self::Inode,
        _: wasip1::Fdflags,
        path_ptr: *const u8,
        path_len: usize,
        o_flags: wasip1::Oflags,
        fs_rights_base: wasip1::Rights,
        _: wasip1::Rights,
        _: wasip1::Fdflags,
    ) -> Result<Self::Inode, wasip1::Errno> {
        if let Some(inode) = self.get_inode_for_path::<Wasm>(*dir_inode, path_ptr, path_len) {
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

    fn path_readlink_raw<Wasm: WasmAccess>(
        &self,
        inode: &Self::Inode,
        path_ptr: *const u8,
        path_len: usize,
        _: *mut u8,
        _: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        let inode = self
            .get_inode_for_path::<Wasm>(*inode, path_ptr, path_len)
            .ok_or(wasip1::ERRNO_NOENT)?;

        let (_, file_or_dir) = ROOT::FILES[inode];
        match file_or_dir {
            StandardEmbeddedInode::File(..) | StandardEmbeddedInode::Dir(..) => {
                Err(wasip1::ERRNO_INVAL)
            }
        }
    }
}

impl<
    ROOT: StandardEmbeddedFilesTy<File, FLAT_LEN> + core::fmt::Debug,
    File: Wasip1FileTrait + 'static + Copy,
    const FLAT_LEN: usize,
    StdIo: StdIO + 'static,
    AddInfo: WasiAddInfo + 'static,
> EmbeddedLFS for StandardEmbeddedNormalLFS<ROOT, File, FLAT_LEN, StdIo, AddInfo>
{
    const PRE_OPEN: &'static [Self::Inode] = ROOT::PRE_OPEN;
}

impl<
    ROOT: StandardEmbeddedFilesTy<File, FLAT_LEN> + core::fmt::Debug,
    File: Wasip1FileTrait + 'static + Copy,
    const FLAT_LEN: usize,
    StdIo: StdIO + 'static,
    AddInfo: WasiAddInfo + 'static,
> DynamicLFS for StandardEmbeddedNormalLFS<ROOT, File, FLAT_LEN, StdIo, AddInfo>
{
    fn pre_open_inodes(&self) -> impl IntoIterator<Item = (Self::Inode, impl DerefToStrCustom)> {
        ROOT::PRE_OPEN.iter().map(|&inode| {
            let (name, _) = ROOT::FILES[inode];
            (inode, name)
        })
    }
}

impl<
    B: BoxedInode,
    ROOT: StandardEmbeddedFilesTy<File, FLAT_LEN> + core::fmt::Debug + 'static,
    File: Wasip1FileTrait + 'static + Copy,
    const FLAT_LEN: usize,
    StdIo: StdIO + 'static,
    AddInfo: WasiAddInfo + 'static,
> Wasip1DynCompatibleLFS<B> for StandardEmbeddedNormalLFS<ROOT, File, FLAT_LEN, StdIo, AddInfo>
{
    fn pre_open_inodes<'a>(
        &'a self,
        f: &mut dyn for<'b> FnMut(&'b dyn Wasip1DynCompatibleLFSSlice),
    ) {
        #[derive(Debug)]
        struct PreOpenInodesIndex<
            EmbeddedRoot: StandardEmbeddedFilesTy<File, FLAT_LEN> + core::fmt::Debug + 'static,
            File: Wasip1FileTrait + 'static + Copy,
            const FLAT_LEN: usize,
        >(core::marker::PhantomData<(EmbeddedRoot, File, usize)>);

        impl<
            EmbeddedRoot: StandardEmbeddedFilesTy<File, FLAT_LEN> + core::fmt::Debug + 'static,
            File: Wasip1FileTrait + 'static + Copy,
            const FLAT_LEN: usize,
        > Wasip1DynCompatibleLFSSlice for PreOpenInodesIndex<EmbeddedRoot, File, FLAT_LEN>
        {
            fn index(
                &self,
                idx: usize,
                f: &mut dyn for<'b, 'c> FnMut(
                    Option<(&'b dyn InodeIdCommon, &'c dyn DerefToStrCustom)>,
                ),
            ) {
                if idx >= EmbeddedRoot::PRE_OPEN.len() {
                    f(None);
                } else {
                    let inode = EmbeddedRoot::PRE_OPEN[idx];
                    let name_and_inode = EmbeddedRoot::FILES[inode];
                    f(Some((
                        &inode as &dyn InodeIdCommon,
                        &name_and_inode as &dyn DerefToStrCustom,
                    )));
                }
            }
        }

        let index = PreOpenInodesIndex::<ROOT, File, FLAT_LEN>(core::marker::PhantomData);

        f(&index);
    }

    fn fd_write_raw_dyn_compatible(
        &self,
        _: &dyn WasmAccessDynCompatibleRaw,
        _: &dyn InodeIdCommon,
        _: *const u8,
        _: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        Err(wasip1::ERRNO_PERM)
    }

    fn fd_write_stdout_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        data: *const u8,
        data_len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        #[cfg(not(feature = "multi_memory"))]
        {
            StdIo::write_direct_dyn_compatible(access, data, data_len)
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

    fn fd_write_stderr_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        data: *const u8,
        data_len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        #[cfg(not(feature = "multi_memory"))]
        {
            StdIo::ewrite_direct_dyn_compatible(access, data, data_len)
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

    fn fd_readdir_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        buf: *mut u8,
        buf_len: usize,
        cookie: Dircookie,
    ) -> Result<(wasip1::Size, Dircookie), wasip1::Errno> {
        let inode = Self::downcast_inode(inode);

        self.fd_readdir_raw_inner(*inode, buf, buf_len, cookie, |dst, src, len| {
            access.memcpy_to_raw(dst, src, len);
        })
    }

    fn path_filestat_get_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        _: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno> {
        let inode = Self::downcast_inode(inode);

        let inode = self
            .get_inode_for_path_dyn_compatible(access, *inode, path_ptr, path_len)
            .ok_or(wasip1::ERRNO_NOENT)?;

        Ok(self.filestat_from_inode(inode))
    }

    fn fd_prestat_get_raw_dyn_compatible(
        &self,
        _: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
    ) -> Result<wasip1::Prestat, wasip1::Errno> {
        let inode = Self::downcast_inode(inode);

        if !Self::PRE_OPEN.contains(&inode) {
            return Err(wasip1::ERRNO_BADF);
        }

        let (name, _) = ROOT::FILES[*inode];

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

    fn fd_prestat_dir_name_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        let inode = Self::downcast_inode(inode);

        if !Self::PRE_OPEN.contains(&inode) {
            return Err(wasip1::ERRNO_BADF);
        }

        let (name, _) = ROOT::FILES[*inode];

        access.memcpy_with(
            dir_path_ptr,
            &name.as_bytes()[..core::cmp::min(name.len(), dir_path_len)],
        );

        Ok(())
    }

    fn fd_filestat_get_raw_dyn_compatible(
        &self,
        _: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno> {
        let inode = Self::downcast_inode(inode);

        Ok(self.filestat_from_inode(*inode))
    }

    fn fd_pread_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        buf: *mut u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        let inode = Self::downcast_inode(inode);
        let (_, file_or_dir) = ROOT::FILES[*inode];

        if let StandardEmbeddedInode::File(file, _) = file_or_dir {
            if offset >= file.size() {
                return Ok(0); // No data to read
            }

            let buf_len = core::cmp::min(buf_len, file.size() - offset);
            let nread = file.pread_raw_dyn_compatible(access, buf, buf_len, offset)?;

            Ok(nread)
        } else {
            unreachable!();
        }
    }
    fn fd_read_stdin_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        buf: *mut u8,
        buf_len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        #[cfg(not(feature = "multi_memory"))]
        {
            StdIo::read_direct_dyn_compatible(access, buf, buf_len)
        }

        #[cfg(feature = "multi_memory")]
        {
            use crate::__private::utils;

            let (buf_vec, read) = unsafe { utils::alloc_buff(buf_len, |buf| StdIo::read(buf)) };
            access.memcpy_with(buf, &buf_vec);
            Ok(read?)
        }
    }

    fn path_open_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        dir_inode: &dyn InodeIdCommon,
        _: wasip1::Fdflags,
        path_ptr: *const u8,
        path_len: usize,
        o_flags: wasip1::Oflags,
        fs_rights_base: wasip1::Rights,
        _: wasip1::Rights,
        _: wasip1::Fdflags,
    ) -> Result<B, wasip1::Errno> {
        let dir_ino = Self::downcast_inode(dir_inode);

        if let Some(inode) =
            self.get_inode_for_path_dyn_compatible(access, *dir_ino, path_ptr, path_len)
        {
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

            Ok(B::from_inode_with_ty::<Self>(inode))
        } else {
            if o_flags & wasip1::OFLAGS_CREAT == wasip1::OFLAGS_CREAT {
                return Err(wasip1::ERRNO_PERM);
            }

            Err(wasip1::ERRNO_NOENT)
        }
    }

    fn path_readlink_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        path_ptr: *const u8,
        path_len: usize,
        _: *mut u8,
        _: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        let inode = Self::downcast_inode(inode);

        let inode = self
            .get_inode_for_path_dyn_compatible(access, *inode, path_ptr, path_len)
            .ok_or(wasip1::ERRNO_NOENT)?;

        let (_, file_or_dir) = ROOT::FILES[inode];
        match file_or_dir {
            StandardEmbeddedInode::File(..) | StandardEmbeddedInode::Dir(..) => {
                Err(wasip1::ERRNO_INVAL)
            }
        }
    }
}
