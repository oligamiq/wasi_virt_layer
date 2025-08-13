// https://github.com/bytecodealliance/wasmtime/blob/cff811b55e8b715e037226f2f3c36c65676d319a/crates/wasi-preview1-component-adapter/src/lib.rs#L1655

pub mod env;
pub mod file;

use core::arch::wasm32::unreachable;

use wasip1::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wasip1_vfs_test_wasm_opt_proc_exit(rval: Exitcode) -> ! {
    // std::process::exit(rval);
    unsafe { wasip1::proc_exit(rval) };

    unreachable();
}

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn clock_time_get(
//     clock_id: Clockid,
// _precision: Timestamp,
//     time: &mut Timestamp,
// ) -> Errno {
//     match clock_id {
//         CLOCKID_MONOTONIC => {
//             *time = 0;
//             ERRNO_SUCCESS
//         }
//         CLOCKID_REALTIME => {
//             *time = 0;
//             ERRNO_SUCCESS
//         }
//         _ => ERRNO_BADF,
//     }
// }

//  Temporarily yield execution of the calling thread.
// Note: This is similar to `sched_yield` in POSIX.
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn sched_yield() -> Errno {
//     ERRNO_SUCCESS
// }

//  Write high-quality random data into a buffer.
//  This function blocks when the implementation is unable to immediately
//  provide sufficient high-quality random data.
//  This function may execute slowly, so when large mounts of random data are
//  required, it's advisable to use this function to seed a pseudo-random
//  number generator, rather than to provide the random data directly.
//  #[unsafe(no_mangle)]
//  pub unsafe extern "C" fn random_get(buf: *mut u8, buf_len: Size) -> Errno {
//      ERRNO_SUCCESS
//  }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn fd_write(
//     fd: Fd,
//     mut iovs_ptr: *const Ciovec,
//     mut iovs_len: usize,
//     nwritten: &mut Size,
// ) -> Errno {
//     ERRNO_SUCCESS
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn environ_sizes_get(
//     _: *mut wasip1::Size,
//     _: *mut wasip1::Size,
// ) -> wasip1::Errno {
//     ERRNO_SUCCESS
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn environ_get(_: *mut *const u8, _: *mut u8) -> wasip1::Errno {
//     ERRNO_SUCCESS
// }
