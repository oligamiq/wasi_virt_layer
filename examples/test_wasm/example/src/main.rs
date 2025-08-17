use walkdir::DirEntry;

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
    // .flat_map(|e| {
    //     if e.file_type().is_file() {
    //         Box::new(core::iter::once(e)) as Box<dyn Iterator<Item = DirEntry>>
    //     } else if e.file_type().is_dir() {
    //         Box::new(
    //             walkdir::WalkDir::new(e.path().join(e.file_name()))
    //                 .into_iter()
    //                 .filter_map(Result::ok)
    //                 .filter(|e| e.file_type().is_file()),
    //         ) as Box<dyn Iterator<Item = DirEntry>>
    //     } else {
    //         Box::new(core::iter::empty()) as Box<dyn Iterator<Item = DirEntry>>
    //     }
    // })
    {
        println!("Current File: {}", file.path().display());
    }
}
