use std::str::FromStr;
use wasi_virt_layer_cli::generator::WasmPath;

#[test]
fn test_wasm_path_parsing() {
    let s = "crates/vfs/rustc_opt.wasm";
    let p = WasmPath::from_str(s).unwrap();
    match p {
        WasmPath::Original { .. } => {}
        _ => panic!("Expected Original, got {:?}", p),
    }
}
