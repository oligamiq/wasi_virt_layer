fn main() {
    let bytes = std::fs::read("debug_pass_2.wasm").unwrap();
    println!("Parsing...");
    let _module = walrus::Module::from_buffer(&bytes).unwrap();
    println!("Done!");
}
