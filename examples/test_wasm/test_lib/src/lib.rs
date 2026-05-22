// This is an example of a target Wasm module acting as a library.
// It exposes functions instead of having a standard executable main loop.

#[unsafe(no_mangle)]
pub extern "C" fn print_hello() {
    let env_var = std::env::var("HELLO").unwrap_or_else(|_| "Not found".to_string());
    println!("Hello from test_lib! HELLO={}", env_var);
}

/// Entry point required by the WASI ABI.
/// For library-style targets this just calls the library's main logic.
#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    print_hello();
}
