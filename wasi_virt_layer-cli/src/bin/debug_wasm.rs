use std::fs;
fn main() {
    let bytes =
        fs::read("F:/wasi_virt_layer/wasi_virt_layer-cli/tests/debug_walrus_fail.wasm").unwrap();
    for payload in wasmparser::Parser::new(0).parse_all(&bytes) {
        match payload {
            Ok(p) => println!("Parsed: {:?}", std::mem::discriminant(&p)),
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
}
