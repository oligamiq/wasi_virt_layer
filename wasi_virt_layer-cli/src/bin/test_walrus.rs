use std::fs;

fn main() {
    let wasm = fs::read("debug_pass_1779615197461317_2.wasm").unwrap();
    match walrus::Module::from_buffer(&wasm) {
        Ok(_) => println!("Walrus successfully loaded the module!"),
        Err(e) => {
            println!("Walrus failed:");
            println!("{:?}", e);
        }
    }
}
