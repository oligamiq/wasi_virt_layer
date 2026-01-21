// cargo +nightly b -r --target wasm32-wasip1-threads -p test_threads
// https://github.com/rust-lang/rust/issues/146721
// wasm-opt target/wasm32-wasip1-threads/release/test_threads.wasm -o examples/test_wasm/test_threads/test_threads.wasm -Oz
// cargo r -r -- -p threads_vfs examples/test_wasm/test_threads/test_threads.wasm -t single --threads true
// wasmtime run -Sthreads=y --env RUST_MIN_STACK=16777216 --env RUST_BACKTRACE=full target/wasm32-wasip1-threads/release/test_threads.wasm

fn main() {
    println!("Hello, world!");

    std::thread::spawn(|| {
        println!("Hello from a thread!");
    })
    .join()
    .unwrap();
}
