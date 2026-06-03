use compact_str::CompactString;
use std::collections::HashMap;

pub fn extract_memory_sizes(wasm_bytes: &[u8]) -> eyre::Result<HashMap<CompactString, (u64, u64)>> {
    let mut mem_sizes = HashMap::new();
    let mut imports = Vec::new();
    let mut memories = Vec::new();

    let parser = wasmparser::Parser::new(0);
    for payload in parser.parse_all(wasm_bytes) {
        match payload? {
            wasmparser::Payload::ImportSection(s) => {
                for import_group in s {
                    for import_res in import_group? {
                        let (_, import) = import_res?;
                        imports.push((import.module.to_string(), import.name.to_string(), import.ty));
                    }
                }
            }
            wasmparser::Payload::MemorySection(s) => {
                for mem in s {
                    memories.push(mem?);
                }
            }
            _ => {}
        }
    }

    // A module's memories space starts with imported memories, followed by defined memories.
    for (_module, name, ty) in imports {
        if let wasmparser::TypeRef::Memory(mem_ty) = ty {
            let initial = mem_ty.initial;
            let maximum = mem_ty.maximum.unwrap_or(mem_ty.initial);
            mem_sizes.insert(CompactString::from(name.as_str()), (initial, maximum));
        }
    }

    Ok(mem_sizes)
}
