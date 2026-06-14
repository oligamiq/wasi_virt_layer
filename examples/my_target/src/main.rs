#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    0
}
