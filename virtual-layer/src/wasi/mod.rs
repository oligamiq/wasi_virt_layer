// https://github.com/bytecodealliance/wasmtime/blob/cff811b55e8b715e037226f2f3c36c65676d319a/crates/wasi-preview1-component-adapter/src/lib.rs#L1655

pub mod env;

use wasip1::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn proc_exit(rval: Exitcode) -> ! {
    std::process::exit(rval as i32);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fd_write(
    fd: Fd,
    mut iovs_ptr: *const Ciovec,
    mut iovs_len: usize,
    nwritten: &mut Size,
) -> Errno {
    ERRNO_SUCCESS
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn environ_sizes_get(
    _: *mut wasip1::Size,
    _: *mut wasip1::Size,
) -> wasip1::Errno {
    ERRNO_SUCCESS
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn environ_get(_: *mut *const u8, _: *mut u8) -> wasip1::Errno {
    ERRNO_SUCCESS
}
