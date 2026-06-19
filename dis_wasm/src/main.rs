use std::fs;
use wasmparser::{Parser, Payload};

fn main() {
    let bytes = fs::read("dist/threads_vfs.core.wasm").unwrap();
    let mut func_idx = 0;

    let mut num_imports = 0;
    for payload in Parser::new(0).parse_all(&bytes) {
        if let Ok(Payload::ImportSection(s)) = payload {
            for import in s.into_imports() {
                let import = import.unwrap();
                if let wasmparser::TypeRef::Func(_) = import.ty {
                    num_imports += 1;
                }
            }
        }
    }

    func_idx = num_imports;

    for payload in Parser::new(0).parse_all(&bytes) {
        if let Ok(Payload::CodeSectionEntry(body)) = payload {
            if func_idx == 97 {
                println!("Function 97 offset: {:#x}", body.range().start);
                let mut reader = body.get_operators_reader().unwrap();
                while !reader.eof() {
                    let pos = reader.original_position();
                    let op = reader.read().unwrap();
                    println!("{:#x}: {:?}", pos, op);
                }
            }
            func_idx += 1;
        }
    }
}
