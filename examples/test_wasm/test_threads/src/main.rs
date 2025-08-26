// cargo b -r --target wasm32-wasip1-threads -p test_threads
// wasm-opt target/wasm32-wasip1-threads/release/test_threads.wasm -o examples/test_wasm/example/test_threads.wasm -Oz
// cargo r -r -- -p threads_vfs examples/test_wasm/example/test_threads.wasm -t single --no-tracing --threads true

fn main() {
    println!("Hello, world!");

    std::thread::spawn(|| {
        println!("Hello from a thread!");
    });
}
