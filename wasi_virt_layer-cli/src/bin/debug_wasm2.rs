use std::fs;
fn main() {
    let bytes =
        fs::read("F:/wasi_virt_layer/wasi_virt_layer-cli/tests/debug_walrus_fail.wasm").unwrap();
    let mut func_count = 0;
    let mut import_func_count = 0;
    for payload in wasmparser::Parser::new(0).parse_all(&bytes) {
        match payload.unwrap() {
            wasmparser::Payload::ImportSection(s) => {
                for g in s {
                    for i in g.unwrap() {
                        if let wasmparser::TypeRef::Func(_) = i.unwrap().1.ty {
                            import_func_count += 1;
                        }
                    }
                }
            }
            wasmparser::Payload::FunctionSection(s) => {
                func_count = s.into_iter().count();
            }
            wasmparser::Payload::ExportSection(s) => {
                for e in s {
                    let e = e.unwrap();
                    println!("Export: {} {:?} {}", e.name, e.kind, e.index);
                }
            }
            _ => {}
        }
    }
    println!(
        "Total funcs: {} (import: {}, local: {})",
        import_func_count + func_count,
        import_func_count,
        func_count
    );
}
