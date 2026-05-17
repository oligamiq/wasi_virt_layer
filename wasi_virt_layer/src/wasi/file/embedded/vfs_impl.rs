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

    fn fd_pread_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        iovs_ptr: *const Ciovec,
        iovs_len: usize,
        offset: u64,
        nread_ret: *mut Size,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "fd_pread: fd={fd}, iovs_len={iovs_len}, offset={offset}");

        match fd {
            0 => wasip1::ERRNO_SPIPE,
            1 | 2 => wasip1::ERRNO_BADF,
            fd => {
                match self.with_inode_and_lfs(fd, |inode, lfs| {
                    let iovs_vec = Wasm::as_array(iovs_ptr, iovs_len);
                    let mut total_read = 0;
                    let mut current_offset = offset as usize;

                    for iovs in iovs_vec {
                        let nread = lfs.fd_pread_raw::<Wasm>(
                            inode,
                            iovs.buf as *mut u8,
                            iovs.buf_len,
                            current_offset,
                        )?;
                        if nread == 0 {
                            break;
                        }
                        total_read += nread;
                        current_offset += nread;
                        if nread < iovs.buf_len as usize {
                            break;
                        }
                    }
                    Ok(total_read)
                }) {
                    Ok(Ok(n)) => {
                        Wasm::store_le(nread_ret, n as Size);
                        wasip1::ERRNO_SUCCESS
                    }
                    Ok(Err(e)) => e,
                    Err(e) => e,
                }
            }
        }
    }

    fn fd_pwrite_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        _fd: Fd,
        _iovs_ptr: *const Ciovec,
        _iovs_len: usize,
        _offset: u64,
        _nwritten: *mut Size,
    ) -> wasip1::Errno {
        wasip1::ERRNO_ROFS
    }

    fn fd_advise_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        offset: u64,
        len: u64,
        advice: wasip1::Advice,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "fd_advise: fd={fd}, offset={offset}, len={len}, advice={advice:?}");

        match self.with_inode_and_lfs(fd, |inode, lfs| {
            lfs.fd_advise_raw::<Wasm>(inode, offset, len, advice)
        }) {
            Ok(Ok(())) => wasip1::ERRNO_SUCCESS,
            Ok(Err(e)) => e,
            Err(e) => e,
        }
    }

    fn fd_allocate_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        _fd: Fd,
        _offset: u64,
        _len: u64,
    ) -> wasip1::Errno {
        wasip1::ERRNO_ROFS
    }

    fn fd_datasync_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "fd_datasync: fd={fd}");

        match self.with_inode_and_lfs(fd, |inode, lfs| {
            lfs.fd_datasync_raw::<Wasm>(inode)
        }) {
            Ok(Ok(())) => wasip1::ERRNO_SUCCESS,
            Ok(Err(e)) => e,
            Err(e) => e,
        }
    }

    fn fd_sync_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "fd_sync: fd={fd}");

        match self.with_inode_and_lfs(fd, |inode, lfs| {
            lfs.fd_sync_raw::<Wasm>(inode)
        }) {
            Ok(Ok(())) => wasip1::ERRNO_SUCCESS,
            Ok(Err(e)) => e,
            Err(e) => e,
        }
    }

    fn fd_tell_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        offset_ret: *mut u64,
    ) -> wasip1::Errno {
        trace_fs!(self, Wasm; "fd_tell: fd={fd}");

        match self.get_cursor(fd) {
            Ok(cursor) => {
                Wasm::store_le(offset_ret, cursor as u64);
                wasip1::ERRNO_SUCCESS
            }
            Err(e) => e,
        }
    }

    fn fd_fdstat_set_flags_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        _fd: Fd,
        _flags: wasip1::Fdflags,
    ) -> wasip1::Errno {
        // Embedded FS doesn't store per-fd flags in a mutable way
        wasip1::ERRNO_SUCCESS
    }

    fn fd_fdstat_set_rights_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        _fd: Fd,
        _fs_rights_base: wasip1::Rights,
        _fs_rights_inheriting: wasip1::Rights,
    ) -> wasip1::Errno {
        // Embedded FS uses max rights, set_rights can only narrow which is fine
        wasip1::ERRNO_SUCCESS
    }

    fn fd_filestat_set_size_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        _fd: Fd,
        _size: u64,
    ) -> wasip1::Errno {
        wasip1::ERRNO_ROFS
    }

    fn fd_filestat_set_times_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        _fd: Fd,
        _atim: wasip1::Timestamp,
        _mtim: wasip1::Timestamp,
        _fst_flags: wasip1::Fstflags,
    ) -> wasip1::Errno {
        wasip1::ERRNO_ROFS
    }

    fn path_filestat_set_times_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        _fd: Fd,
        _flags: wasip1::Lookupflags,
        _path_ptr: *const u8,
        _path_len: usize,
        _atim: wasip1::Timestamp,
        _mtim: wasip1::Timestamp,
        _fst_flags: wasip1::Fstflags,
    ) -> wasip1::Errno {
        wasip1::ERRNO_ROFS
    }

    fn path_symlink_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        _old_path_ptr: *const u8,
        _old_path_len: usize,
        _fd: Fd,
        _new_path_ptr: *const u8,
        _new_path_len: usize,
    ) -> wasip1::Errno {
        wasip1::ERRNO_ROFS
    }

    fn fd_renumber_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        _fd: Fd,
        _to: Fd,
    ) -> wasip1::Errno {
        // Embedded FS doesn't support renumbering
        wasip1::ERRNO_NOSYS
    }
}
