#![cfg(feature = "embedded-fs")]

use crate::__private::wasip1;
use crate::__private::wasip1::{Ciovec, Dircookie, Fd, Size};

use crate::memory::WasmAccessName;
use crate::wasi::file::trace::trace_fs;
use crate::{
    memory::WasmAccess,
    wasi::file::{
        EmbeddedLFS, InodeIdCommon, OpenFdInfo, Wasip1FileSystem,
        embedded::vfs::StandardEmbeddedFileSystem,
    },
};

impl<LFS: EmbeddedLFS + Sync, const FLAT_LEN: usize, OpenFd: OpenFdInfo + Default> Wasip1FileSystem
    for StandardEmbeddedFileSystem<LFS, FLAT_LEN, OpenFd>
where
    LFS::Inode: InodeIdCommon,
{
    fn fd_write_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        iovs_ptr: *const Ciovec,
        iovs_len: usize,
        nwritten: *mut Size,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "fd_write: fd={fd}, iovs_len={iovs_len}");

        match self.fd_write_raw_inner::<Wasm>(fd, iovs_ptr, iovs_len) {
            Ok(n) => {
                Wasm::store_le(nwritten, n);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }

    fn fd_readdir_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        buf: *mut u8,
        buf_len: usize,
        cookie: Dircookie,
        nread: *mut Size,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "fd_readdir: fd={fd}, buf_len={buf_len}, cookie={cookie}");

        match self.fd_readdir_raw_inner::<Wasm>(fd, buf, buf_len, cookie) {
            Ok(n) => {
                Wasm::store_le(nread, n);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }

    fn path_filestat_get_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
        filestat_ptr: *mut wasip1::Filestat,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "path_filestat_get: fd={fd}, flags={flags}, path_len={path_len}");

        match self.path_filestat_get_raw_inner::<Wasm>(fd, flags, path_ptr, path_len) {
            Ok(filestat) => {
                Wasm::store_le(filestat_ptr, filestat);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }

    fn fd_prestat_get_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        prestat_ptr: *mut wasip1::Prestat,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "fd_prestat_get: fd={fd}");

        match self.fd_prestat_get_raw_inner::<Wasm>(fd) {
            Ok(prestat) => {
                trace_fs!(self, Wasm; "prestat_tag={}, prestat_u={}", prestat.tag, unsafe { prestat.u.dir }.pr_name_len );
                Wasm::store_le(prestat_ptr, prestat);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }

    fn fd_prestat_dir_name_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "fd_prestat_dir_name: fd={fd}, dir_path_len={dir_path_len}");

        match self.fd_prestat_dir_name_raw_inner::<Wasm>(fd, dir_path_ptr, dir_path_len) {
            Ok(()) => wasip1::ERRNO_SUCCESS,
            Err(e) => e,
        }
    }

    fn fd_close_raw<Wasm: WasmAccess + WasmAccessName + 'static>(&self, fd: Fd) -> wasip1::Errno {
        trace_fs!(self, Wasm; "fd_close: fd={fd}");

        match self.fd_close_raw_inner::<Wasm>(fd) {
            Ok(()) => wasip1::ERRNO_SUCCESS,
            Err(e) => e,
        }
    }

    fn fd_filestat_get_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        filestat_ptr: *mut wasip1::Filestat,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "fd_filestat_get: fd={fd}");

        match self.fd_filestat_get_raw_inner::<Wasm>(fd) {
            Ok(filestat) => {
                Wasm::store_le(filestat_ptr, filestat);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }

    fn fd_fdstat_get_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        fdstat_ptr: *mut wasip1::Fdstat,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "fd_fdstat_get: fd={fd}");

        match self.fd_fdstat_get_raw_inner::<Wasm>(fd) {
            Ok(fdstat) => {
                Wasm::store_le(fdstat_ptr, fdstat);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }

    fn fd_read_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        iovs_ptr: *const Ciovec,
        iovs_len: usize,
        nread: *mut Size,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "fd_read: fd={fd}, iovs_len={iovs_len}");

        match self.fd_read_raw_inner::<Wasm>(fd, iovs_ptr, iovs_len) {
            Ok(n) => {
                Wasm::store_le(nread, n);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }

    fn fd_seek_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        offset: i64,
        whence: wasip1::Whence,
        new_offset_ptr: *mut i64,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "fd_seek: fd={fd}, offset={offset}, whence={}", whence.raw());

        match self.fd_seek_raw_inner::<Wasm>(fd, offset, whence, new_offset_ptr) {
            Ok(()) => wasip1::ERRNO_SUCCESS,
            Err(e) => e,
        }
    }

    fn path_open_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
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
        trace_fs!(self, Wasm; "path_open: dir_fd={dir_fd}, dir_flags={dir_flags}, path_len={path_len}, o_flags={o_flags}, fs_rights_base={fs_rights_base}, fs_rights_inheriting={fs_rights_inheriting}, fd_flags={fd_flags}");

        match self.path_open_raw_inner::<Wasm>(
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

    fn path_create_directory_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        path_ptr: *const u8,
        path_len: usize,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "path_create_directory: fd={fd}, path_len={path_len}");

        match self.path_create_directory_raw_inner::<Wasm>(fd, path_ptr, path_len) {
            Ok(()) => wasip1::ERRNO_SUCCESS,
            Err(e) => e,
        }
    }

    fn path_link_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        old_fd: Fd,
        old_flags: wasip1::Lookupflags,
        old_path_ptr: *const u8,
        old_path_len: usize,
        new_fd: Fd,
        new_path_ptr: *const u8,
        new_path_len: usize,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "path_link: old_fd={old_fd}, new_fd={new_fd}");

        match self.path_link_raw_inner::<Wasm>(
            old_fd,
            old_flags,
            old_path_ptr,
            old_path_len,
            new_fd,
            new_path_ptr,
            new_path_len,
        ) {
            Ok(()) => wasip1::ERRNO_SUCCESS,
            Err(e) => e,
        }
    }

    fn path_remove_directory_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        path_ptr: *const u8,
        path_len: usize,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "path_remove_directory: fd={fd}, path_len={path_len}");

        match self.path_remove_directory_raw_inner::<Wasm>(fd, path_ptr, path_len) {
            Ok(()) => wasip1::ERRNO_SUCCESS,
            Err(e) => e,
        }
    }

    fn path_rename_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        old_fd: Fd,
        old_path_ptr: *const u8,
        old_path_len: usize,
        new_fd: Fd,
        new_path_ptr: *const u8,
        new_path_len: usize,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "path_rename: old_fd={old_fd}, new_fd={new_fd}");

        match self.path_rename_raw_inner::<Wasm>(
            old_fd,
            old_path_ptr,
            old_path_len,
            new_fd,
            new_path_ptr,
            new_path_len,
        ) {
            Ok(()) => wasip1::ERRNO_SUCCESS,
            Err(e) => e,
        }
    }

    fn path_unlink_file_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        path_ptr: *const u8,
        path_len: usize,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "path_unlink_file: fd={fd}, path_len={path_len}");

        match self.path_unlink_file_raw_inner::<Wasm>(fd, path_ptr, path_len) {
            Ok(()) => wasip1::ERRNO_SUCCESS,
            Err(e) => e,
        }
    }

    fn path_readlink_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        path_ptr: *const u8,
        path_len: usize,
        buf: *mut u8,
        buf_len: usize,
        buf_nread: *mut Size,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "path_readlink: fd={fd}, path_len={path_len}, buf_len={buf_len}");

        match self.path_readlink_raw_inner::<Wasm>(fd, path_ptr, path_len, buf, buf_len) {
            Ok(n) => {
                Wasm::store_le(buf_nread, n);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }
}
