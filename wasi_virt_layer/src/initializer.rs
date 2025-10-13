#[cfg(feature = "threads")]
#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    // Initialize thread-local storage for WASI threads
    // In the current version of Rust,
    // thread initialisation is not performed.
    // Therefore, we must force linking to perform initialisation.
    // https://github.com/rust-lang/rust/pull/108097
    // https://github.com/rust-lang/rust/issues/146843
    fn __wasi_init_tp();
    fn __wasm_call_ctors();
}

#[cfg(feature = "threads")]
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn __wasip1_vfs_thread_initializer() {
    unsafe { __wasi_init_tp() };
    unsafe { __wasm_call_ctors() };
}
