use crate::__private::wasip1;
use crate::__private::wasip1::{Ciovec, Dircookie, Fd, Size};

use crate::{
    memory::WasmAccess,
    wasi::file::{Wasip1FileSystem, Wasip1LFS, constant::vfs::Wasip1ConstVFS},
};

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

    fn fd_filestat_get_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
        filestat_ptr: *mut wasip1::Filestat,
    ) -> wasip1::Errno {
        match self.fd_filestat_get_raw::<Wasm>(fd) {
            Ok(filestat) => {
                Wasm::store_le(filestat_ptr, filestat);
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
        nread: *mut Size,
    ) -> wasip1::Errno {
        match self.fd_read_raw::<Wasm>(fd, iovs_ptr, iovs_len) {
            Ok(n) => {
                Wasm::store_le(nread, n);
                wasip1::ERRNO_SUCCESS
            }
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
