// https://docs.rs/wasmtime-wasi/17.0.3/wasmtime_wasi/struct.WasiCtx.html
// https://docs.rs/wasi-common/17.0.3/wasi_common/table/struct.Table.html

use crate::memory::WasmAccess;
pub mod constant;
pub mod stdio;

// no implementing dcache

use wasip1::*;

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

    fn fd_filestat_get_raw<Wasm: WasmAccess>(
        &mut self,
        inode: Self::Inode,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno>;

    fn fd_pread_raw<Wasm: WasmAccess>(
        &mut self,
        inode: Self::Inode,
        buf: *mut u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<Size, wasip1::Errno>;

    fn fd_read_stdin_raw<Wasm: WasmAccess>(
        &mut self,
        buf: *mut u8,
        buf_len: usize,
    ) -> Result<Size, wasip1::Errno>;

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

pub trait Wasip1FileTrait {
    fn size(&self) -> usize;

    /// Reads data from the file into the provided buffer.
    /// Returns the number of bytes read.
    #[allow(unused_variables)]
    fn pread(&self, buf: &mut [u8], offset: usize) -> Result<usize, wasip1::Errno> {
        return Err(wasip1::ERRNO_NOSYS);
    }

    /// This function is called,
    /// but if the read function is implemented
    /// and the alloc feature is ON,
    /// this function is automatically implemented.
    #[allow(unused_variables)]
    fn pread_raw<Wasm: WasmAccess>(
        &self,
        buf_ptr: *mut u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<usize, wasip1::Errno> {
        #[cfg(feature = "alloc")]
        {
            let mut array = alloc::vec::Vec::with_capacity(buf_len);
            unsafe { array.set_len(buf_len) };
            let nread = self.pread(&mut array, offset)?;
            Wasm::memcpy(buf_ptr, &array[..nread]);

            Ok(nread)
        }

        #[cfg(not(feature = "alloc"))]
        {
            // Stub implementation for non-std environments
            Err(wasip1::ERRNO_NOSYS)
        }
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

    fn fd_filestat_get_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
        filestat: *mut wasip1::Filestat,
    ) -> wasip1::Errno;

    fn fd_read_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
        iovs_ptr: *const Ciovec,
        iovs_len: usize,
        nread: *mut Size,
    ) -> wasip1::Errno;

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

            #[unsafe(no_mangle)]
            #[cfg(target_arch = "wasm32")]
            pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_read>](
                fd: $crate::wasip1::Fd,
                iovs_ptr: *const $crate::wasip1::Ciovec,
                iovs_len: usize,
                nread_ret: *mut $crate::wasip1::Size,
            ) -> $crate::wasip1::Errno {
                let state = $state;
                $crate::wasi::file::Wasip1FileSystem::fd_read_raw::<$wasm>(state, fd, iovs_ptr, iovs_len, nread_ret)
            }

            #[unsafe(no_mangle)]
            #[cfg(target_arch = "wasm32")]
            pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_filestat_get>](
                fd: $crate::wasip1::Fd,
                filestat: *mut $crate::wasip1::Filestat,
            ) -> $crate::wasip1::Errno {
                let state = $state;
                $crate::wasi::file::Wasip1FileSystem::fd_filestat_get_raw::<$wasm>(state, fd, filestat)
            }
        }
    };
}
