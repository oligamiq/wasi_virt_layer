// This is an example of a target Wasm module acting as a library.
// It exposes functions instead of having a standard executable main loop.

#[unsafe(no_mangle)]
pub extern "C" fn print_hello() {
    let env_var = std::env::var("HELLO").unwrap_or_else(|_| "Not found".to_string());
    println!("Hello from test_lib! HELLO={}", env_var);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    // dummy start to satisfy wasi_virt_layer-cli IsRustWasm generator
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
