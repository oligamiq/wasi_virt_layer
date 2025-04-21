#![no_std]

use wasip1::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn environ_sizes_get(
    environ_count: *mut wasip1::Size,
    environ_buf: *mut wasip1::Size,
) -> wasip1::Errno {
    unsafe { *environ_count = 0 };
    unsafe { *environ_buf = 0 };
    ERRNO_SUCCESS
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn environ_get(_: *mut *const u8, _: *mut u8) -> wasip1::Errno {
    ERRNO_SUCCESS
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unreachable!()
}
