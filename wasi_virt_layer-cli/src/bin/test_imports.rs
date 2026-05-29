use wasmparser::{Parser, Payload};

fn main() {
    let wasm = b"\0asm\x01\0\0\0";
    for payload in Parser::new(0).parse_all(wasm) {
        let payload = payload.unwrap();
        match payload {
            Payload::ImportSection(s) => {
                for import_group in s {
                    for import_res in import_group.unwrap() {
                        let (_, import) = import_res.unwrap();
                        println!("{} {}", import.module, import.name);
                    }
                }
            }
            _ => {}
        }
    }
}
