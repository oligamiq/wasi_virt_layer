#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    0
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Dummy main for non-wasm targets
}
