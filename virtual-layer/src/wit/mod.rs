use virtual_file_system::*;

// Use a procedural macro to generate bindings for the world we specified in
// `host.wit`
// wit_bindgen::generate!({
//     // the name of the world in the `*.wit` input file
//     world: "virtual-file-system",
// });
// cargo binstall wit-bindgen-cli -y
// wit-bindgen rust wit
pub mod virtual_file_system;

// Define a custom type and implement the generated `Guest` trait for it which
// represents implementing all the necessary exported interfaces for this
// component.

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fd_write_import_wrap(
    fd: i32,
    iovs_ptr: i32,
    iovs_len: i32,
    written_ptr: i32,
) -> i32 {
    Wasip1::fd_write_import(fd, iovs_ptr, iovs_len, written_ptr)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn environ_sizes_get_import_wrap(
    environ_count_ptr: i32,
    environ_size_ptr: i32,
) -> i32 {
    Wasip1::environ_sizes_get_import(environ_count_ptr, environ_size_ptr)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn environ_get_import_wrap(
    environ_ptr_ptr: i32,
    environ_buf_ptr: i32,
) -> i32 {
    Wasip1::environ_get_import(environ_ptr_ptr, environ_buf_ptr)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn proc_exit_import_wrap(code: i32) {
    Wasip1::proc_exit_import(code)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn random_get_import_wrap(buf_ptr: i32, buf_len: i32) -> i32 {
    Wasip1::random_get_import(buf_ptr, buf_len)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sched_yield_import_wrap() -> i32 {
    Wasip1::sched_yield_import()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn clock_time_get_import_wrap(
    clock_id: i32,
    precision: i64,
    time_ptr: i32,
) -> i32 {
    Wasip1::clock_time_get_import(clock_id, precision, time_ptr)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fd_readdir_import_wrap(
    fd: i32,
    buf_ptr: i32,
    buf_len: i32,
    cookie: i64,
    buf_used_ptr: i32,
) -> i32 {
    Wasip1::fd_readdir_import(fd, buf_ptr, buf_len, cookie, buf_used_ptr)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_filestat_get_import_wrap(
    fd: i32,
    lookupflags: i32,
    path_ptr: i32,
    path_len: i32,
    filestat_ptr: i32,
) -> i32 {
    Wasip1::path_filestat_get_import(fd, lookupflags, path_ptr, path_len, filestat_ptr)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_open_import_wrap(
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
pub unsafe extern "C" fn fd_close_import_wrap(fd: i32) -> i32 {
    Wasip1::fd_close_import(fd)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fd_prestat_get_import_wrap(fd: i32, prestat_ptr: i32) -> i32 {
    Wasip1::fd_prestat_get_import(fd, prestat_ptr)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fd_prestat_dir_name_import_wrap(
    fd: i32,
    path_ptr: i32,
    path_len: i32,
) -> i32 {
    Wasip1::fd_prestat_dir_name_import(fd, path_ptr, path_len)
}
