// cargo b -r --target wasm32-wasip1 -p test_wasm
// wasm-opt target/wasm32-wasip1/release/test_wasm.wasm -o examples/test_wasm/example/test_wasm_opt.wasm -Oz
fn main() {
    println!("Hello, world!");

    let envs = std::env::vars()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>();
    println!("Environ: {:?}", envs);

    for file in walkdir::WalkDir::new("/root")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        println!("Root File: {}", file.path().display());
    }

    for file in walkdir::WalkDir::new("~")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        println!("User File: {}", file.path().display());
    }

    for file in walkdir::WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        println!("Current File: {}", file.path().display());
    }
}
