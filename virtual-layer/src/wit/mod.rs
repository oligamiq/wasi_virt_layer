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
    iovs: i32,
    iovs_len: i32,
    written: i32,
) -> i32 {
    Wasip1::fd_write_import(fd, iovs, iovs_len, written)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn environ_sizes_get_import_wrap(
    environ_count: i32,
    environ_size: i32,
) -> i32 {
    Wasip1::environ_sizes_get_import(environ_count, environ_size)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn environ_get_import_wrap(environ: i32, environ_buf: i32) -> i32 {
    Wasip1::environ_get_import(environ, environ_buf)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn proc_exit_import_wrap(code: i32) {
    Wasip1::proc_exit_import(code)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn random_get_import_wrap(buf: i32, buf_len: i32) -> i32 {
    Wasip1::random_get_import(buf, buf_len)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sched_yield_import_wrap() -> i32 {
    Wasip1::sched_yield_import()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn clock_time_get_import_wrap(
    clock_id: i32,
    precision: i64,
    time: i32,
) -> i32 {
    Wasip1::clock_time_get_import(clock_id, precision, time)
}
