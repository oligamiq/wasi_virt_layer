// adapter check https://github.com/bytecodealliance/wasmtime/tree/main/crates/wasi-preview1-component-adapter/verify
// cargo install verify-component-adapter --git https://github.com/bytecodealliance/wasmtime

// Use a procedural macro to generate bindings for the world we specified in
// `host.wit`
wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "wasip1-zero-layer",
});

// Define a custom type and implement the generated `Guest` trait for it which
// represents implementing all the necessary exported interfaces for this
// component.

#[unsafe(export_name = "environ_sizes_get")]
pub unsafe extern "C" fn environ_sizes_get(environ_count: *mut u32, environ_size: *mut u32) -> u32 {
    ZeroLayer::environ_sizes_get_import(environ_count as i32, environ_size as i32) as u32
}

#[unsafe(export_name = "environ_get")]
pub unsafe extern "C" fn environ_get(environ: *mut *mut u8, environ_buf: *mut u8) -> u32 {
    ZeroLayer::environ_get_import(environ as i32, environ_buf as i32) as u32
}

#[unsafe(export_name = "fd_write")]
pub unsafe extern "C" fn fd_write(
    fd: u32,
    iovs: *const *const u8,
    iovs_len: u32,
    written: *mut u32,
) -> u32 {
    ZeroLayer::fd_write_import(fd as i32, iovs as i32, iovs_len as i32, written as i32) as u32
}

#[unsafe(export_name = "proc_exit")]
pub unsafe extern "C" fn proc_exit(exit_code: u32) -> ! {
    ZeroLayer::proc_exit_import(exit_code as i32);
    unreachable!()
}

// wasm-opt target/wasm32-unknown-unknown/release/wasip1_zero_layer.wasm -o wasip1_zero_layer_opt.wasm -Oz
