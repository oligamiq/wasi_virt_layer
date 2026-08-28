use std::fs;
fn main() {
    let bytes =
        fs::read("F:/wasi_virt_layer/wasi_virt_layer-cli/tests/debug_walrus_fail.wasm").unwrap();
    for payload in wasmparser::Parser::new(0).parse_all(&bytes) {
        let p = payload.unwrap();
        if let Some((id, _)) = p.as_section() {
            println!("Section ID: {}", id);
        }
    }
}
