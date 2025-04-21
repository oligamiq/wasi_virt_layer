#![no_std]

use wasip1::*;

pub unsafe extern "C" fn environ_sizes_get(
    _: *mut wasip1::Size,
    _: *mut wasip1::Size,
) -> wasip1::Errno {
    ERRNO_SUCCESS
}

pub unsafe extern "C" fn environ_get(_: *mut *const u8, _: *mut u8) -> wasip1::Errno {
    ERRNO_SUCCESS
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unreachable!()
}
