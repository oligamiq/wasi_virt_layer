#[unsafe(no_mangle)]
pub unsafe extern "C" fn environ_sizes_get(environ_count: *mut u32, environ_size: *mut u32) -> u32 {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn environ_get(environ: *mut *mut u8, environ_buf: *mut u8) -> u32 {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn proc_exit(exit_code: u32) -> ! {
    std::process::exit(exit_code as i32);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fd_write(
    fd: u32,
    iovs: *const *const u8,
    iovs_len: u32,
    written: *mut u32,
) -> u32 {
    0
}

pub mod errno {
    pub type Errno = u16;
    pub const SUCCESS: Errno = 0;
}
