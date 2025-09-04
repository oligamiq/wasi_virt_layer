#[allow(dead_code)]
#[cfg(feature = "threads")]
pub mod virtual_file_system_threads;
#[cfg(feature = "threads")]
pub use virtual_file_system_threads as virtual_file_system;

#[allow(dead_code)]
#[cfg(not(feature = "threads"))]
pub mod virtual_file_system;

// Define a custom type and implement the generated `Guest` trait for it which
// represents implementing all the necessary exported interfaces for this
// component.

#[cfg(target_os = "wasi")]
mod import {
    mod wasip1 {
        use super::super::virtual_file_system::wasip1_vfs::host::virtual_file_system_wasip1_core::*;

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn environ_sizes_get_import_anchor(
            environ_count_ptr: i32,
            environ_size_ptr: i32,
        ) -> i32 {
            Wasip1::environ_sizes_get_import(environ_count_ptr, environ_size_ptr)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn environ_get_import_anchor(
            environ_ptr_ptr: i32,
            environ_buf_ptr: i32,
        ) -> i32 {
            Wasip1::environ_get_import(environ_ptr_ptr, environ_buf_ptr)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn proc_exit_import_anchor(code: i32) {
            Wasip1::proc_exit_import(code)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn random_get_import_anchor(buf_ptr: i32, buf_len: i32) -> i32 {
            Wasip1::random_get_import(buf_ptr, buf_len)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn sched_yield_import_anchor() -> i32 {
            Wasip1::sched_yield_import()
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn clock_time_get_import_anchor(
            clock_id: i32,
            precision: i64,
            time_ptr: i32,
        ) -> i32 {
            Wasip1::clock_time_get_import(clock_id, precision, time_ptr)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn clock_res_get_import_anchor(
            clock_id: i32,
            resolution_ptr: i32,
        ) -> i32 {
            Wasip1::clock_res_get_import(clock_id, resolution_ptr)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fd_advise_import_anchor(
            fd: i32,
            offset: i64,
            len: i64,
            advice: i8,
        ) -> i32 {
            Wasip1::fd_advise_import(fd, offset, len, advice)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fd_allocate_import_anchor(fd: i32, offset: i64, len: i64) -> i32 {
            Wasip1::fd_allocate_import(fd, offset, len)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fd_datasync_import_anchor(fd: i32) -> i32 {
            Wasip1::fd_datasync_import(fd)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fd_fdstat_get_import_anchor(fd: i32, fdstat_ptr: i32) -> i32 {
            Wasip1::fd_fdstat_get_import(fd, fdstat_ptr)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fd_fdstat_set_flags_import_anchor(fd: i32, fdflags: i16) -> i32 {
            Wasip1::fd_fdstat_set_flags_import(fd, fdflags)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fd_fdstat_set_rights_import_anchor(
            fd: i32,
            fs_rights_base: i64,
            fs_rights_inheriting: i64,
        ) -> i32 {
            Wasip1::fd_fdstat_set_rights_import(fd, fs_rights_base, fs_rights_inheriting)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fd_write_import_anchor(
            fd: i32,
            iovs_ptr: i32,
            iovs_len: i32,
            written_ptr: i32,
        ) -> i32 {
            Wasip1::fd_write_import(fd, iovs_ptr, iovs_len, written_ptr)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fd_pwrite_import_anchor(
            fd: i32,
            iovs_ptr: i32,
            iovs_len: i32,
            offset: i64,
            written_ptr: i32,
        ) -> i32 {
            Wasip1::fd_pwrite_import(fd, iovs_ptr, iovs_len, offset, written_ptr)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fd_readdir_import_anchor(
            fd: i32,
            buf_ptr: i32,
            buf_len: i32,
            cookie: i64,
            buf_used_ptr: i32,
        ) -> i32 {
            Wasip1::fd_readdir_import(fd, buf_ptr, buf_len, cookie, buf_used_ptr)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fd_close_import_anchor(fd: i32) -> i32 {
            Wasip1::fd_close_import(fd)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fd_prestat_get_import_anchor(fd: i32, prestat_ptr: i32) -> i32 {
            Wasip1::fd_prestat_get_import(fd, prestat_ptr)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fd_prestat_dir_name_import_anchor(
            fd: i32,
            path_ptr: i32,
            path_len: i32,
        ) -> i32 {
            Wasip1::fd_prestat_dir_name_import(fd, path_ptr, path_len)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fd_filestat_get_import_anchor(fd: i32, filestat_ptr: i32) -> i32 {
            Wasip1::fd_filestat_get_import(fd, filestat_ptr)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fd_read_import_anchor(
            fd: i32,
            iovs_ptr: i32,
            iovs_len: i32,
            nread_ptr: i32,
        ) -> i32 {
            Wasip1::fd_read_import(fd, iovs_ptr, iovs_len, nread_ptr)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fd_pread_import_anchor(
            fd: i32,
            iovs_ptr: i32,
            iovs_len: i32,
            offset: i64,
            nread_ptr: i32,
        ) -> i32 {
            Wasip1::fd_pread_import(fd, iovs_ptr, iovs_len, offset, nread_ptr)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fd_filestat_set_size_import_anchor(fd: i32, size: i64) -> i32 {
            Wasip1::fd_filestat_set_size_import(fd, size)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fd_filestat_set_times_import_anchor(
            fd: i32,
            atim: i64,
            mtim: i64,
            fst_flags: i16,
        ) -> i32 {
            Wasip1::fd_filestat_set_times_import(fd, atim, mtim, fst_flags)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fd_renumber_import_anchor(fd: i32, new_fd: i32) -> i32 {
            Wasip1::fd_renumber_import(fd, new_fd)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fd_seek_import_anchor(
            fd: i32,
            offset: i64,
            whence: i8,
            new_offset_ptr: i32,
        ) -> i32 {
            Wasip1::fd_seek_import(fd, offset, whence, new_offset_ptr)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fd_sync_import_anchor(fd: i32) -> i32 {
            Wasip1::fd_sync_import(fd)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fd_tell_import_anchor(fd: i32, offset_ptr: i32) -> i32 {
            Wasip1::fd_tell_import(fd, offset_ptr)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn path_create_directory_import_anchor(
            fd: i32,
            path_ptr: i32,
            path_len: i32,
        ) -> i32 {
            Wasip1::path_create_directory_import(fd, path_ptr, path_len)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn path_filestat_get_import_anchor(
            fd: i32,
            lookupflags: i32,
            path_ptr: i32,
            path_len: i32,
            filestat_ptr: i32,
        ) -> i32 {
            Wasip1::path_filestat_get_import(fd, lookupflags, path_ptr, path_len, filestat_ptr)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn path_filestat_set_times_import_anchor(
            fd: i32,
            lookupflags: i32,
            path_ptr: i32,
            path_len: i32,
            atim: i64,
            mtim: i64,
            fst_flags: i16,
        ) -> i32 {
            Wasip1::path_filestat_set_times_import(
                fd,
                lookupflags,
                path_ptr,
                path_len,
                atim,
                mtim,
                fst_flags,
            )
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn path_link_import_anchor(
            old_fd: i32,
            old_flags: i32,
            old_path_ptr: i32,
            old_path_len: i32,
            new_fd: i32,
            new_path_ptr: i32,
            new_path_len: i32,
        ) -> i32 {
            Wasip1::path_link_import(
                old_fd,
                old_flags,
                old_path_ptr,
                old_path_len,
                new_fd,
                new_path_ptr,
                new_path_len,
            )
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn path_readlink_import_anchor(
            fd: i32,
            path_ptr: i32,
            path_len: i32,
            buf_ptr: i32,
            buf_len: i32,
            buf_used_ptr: i32,
        ) -> i32 {
            Wasip1::path_readlink_import(fd, path_ptr, path_len, buf_ptr, buf_len, buf_used_ptr)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn path_remove_directory_import_anchor(
            fd: i32,
            path_ptr: i32,
            path_len: i32,
        ) -> i32 {
            Wasip1::path_remove_directory_import(fd, path_ptr, path_len)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn path_rename_import_anchor(
            old_fd: i32,
            old_path_ptr: i32,
            old_path_len: i32,
            new_fd: i32,
            new_path_ptr: i32,
            new_path_len: i32,
        ) -> i32 {
            Wasip1::path_rename_import(
                old_fd,
                old_path_ptr,
                old_path_len,
                new_fd,
                new_path_ptr,
                new_path_len,
            )
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn path_open_import_anchor(
            fd: i32,
            dirflags: i32,
            path_ptr: i32,
            path_len: i32,
            oflags: i32,
            fs_rights_base: i64,
            fs_rights_inheriting: i64,
            fdflags: i32,
            fd_out_ptr: i32,
        ) -> i32 {
            Wasip1::path_open_import(
                fd,
                dirflags,
                path_ptr,
                path_len,
                oflags,
                fs_rights_base,
                fs_rights_inheriting,
                fdflags,
                fd_out_ptr,
            )
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn path_symlink_import_anchor(
            old_path_ptr: i32,
            old_path_len: i32,
            fd: i32,
            new_path_ptr: i32,
            new_path_len: i32,
        ) -> i32 {
            Wasip1::path_symlink_import(old_path_ptr, old_path_len, fd, new_path_ptr, new_path_len)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn path_unlink_file_import_anchor(
            fd: i32,
            path_ptr: i32,
            path_len: i32,
        ) -> i32 {
            Wasip1::path_unlink_file_import(fd, path_ptr, path_len)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn poll_oneoff_import_anchor(
            subscriptions_ptr: i32,
            results_ptr: i32,
            n_subscriptions: i32,
            stored_ptr: i32,
        ) -> i32 {
            Wasip1::poll_oneoff_import(subscriptions_ptr, results_ptr, n_subscriptions, stored_ptr)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn args_get_import_anchor(
            args_ptr_ptr: i32,
            args_buf_ptr: i32,
        ) -> i32 {
            Wasip1::args_get_import(args_ptr_ptr, args_buf_ptr)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn args_sizes_get_import_anchor(
            args_count_ptr: i32,
            args_size_ptr: i32,
        ) -> i32 {
            Wasip1::args_sizes_get_import(args_count_ptr, args_size_ptr)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn sock_accept_import_anchor(
            fd: i32,
            flags: i16,
            new_sock_fd_ptr: i32,
        ) -> i32 {
            Wasip1::sock_accept_import(fd, flags, new_sock_fd_ptr)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn sock_recv_import_anchor(
            fd: i32,
            receiver_ptr: i32,
            receiver_len: i32,
            ri_flags: i16,
            nread_ptr: i32,
            ro_flags: i16,
        ) -> i32 {
            Wasip1::sock_recv_import(
                fd,
                receiver_ptr,
                receiver_len,
                ri_flags,
                nread_ptr,
                ro_flags,
            )
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn sock_send_import_anchor(
            fd: i32,
            sender_ptr: i32,
            sender_len: i32,
            si_flags: i16,
            nwritten_ptr: i32,
        ) -> i32 {
            Wasip1::sock_send_import(fd, sender_ptr, sender_len, si_flags, nwritten_ptr)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn sock_shutdown_import_anchor(fd: i32, how: i8) -> i32 {
            Wasip1::sock_shutdown_import(fd, how)
        }
    }

    // This cfg can only nightly use
    // #[cfg(target_feature = "atomics")]
    #[cfg(feature = "threads")]
    mod wasip1_threads {
        use super::super::virtual_file_system_threads::wasip1_vfs::host::virtual_file_system_wasip1_threads_import::Wasip1Threads;

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn thread_spawn_import_anchor(data_ptr: i32) -> i32 {
            Wasip1Threads::thread_spawn_import(data_ptr)
        }
    }
}

#[cfg(target_os = "wasi")]
mod export {
    #[cfg(feature = "threads")]
    mod wasip1_threads {
        use super::super::virtual_file_system_threads::exports::wasip1_vfs::host::virtual_file_system_wasip1_threads_export::Guest;
        use super::super::virtual_file_system::*;

        struct Exporter;

        impl Guest for Exporter {
            fn wasi_thread_start(thread_id: i32, data_ptr: i32) -> i32 {
                #[link(wasm_import_module = "wasip1-vfs")]
                unsafe extern "C" {
                    pub fn __wasip1_vfs_wasi_thread_start_entry(thread_id: i32, ptr: i32) -> i32;
                }

                unsafe { __wasip1_vfs_wasi_thread_start_entry(thread_id, data_ptr) }
            }
        }

        export!(Exporter with_types_in super::super::virtual_file_system_threads);
    }
}
