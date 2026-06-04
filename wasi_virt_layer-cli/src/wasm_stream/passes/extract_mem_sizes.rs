use compact_str::CompactString;
use std::collections::HashMap;

pub fn extract_memory_sizes(wasm_bytes: &[u8]) -> eyre::Result<HashMap<CompactString, (u64, u64)>> {
    let mut mem_sizes = HashMap::new();
    let mut imports = Vec::new();
    let mut memories = Vec::new();
    let mut exports = Vec::new();

    let parser = wasmparser::Parser::new(0);
    for payload in parser.parse_all(wasm_bytes) {
        match payload? {
            wasmparser::Payload::ImportSection(s) => {
                for import_group in s {
                    for import_res in import_group? {
                        let (_, import) = import_res?;
                        imports.push((
                            import.module.to_string(),
                            import.name.to_string(),
                            import.ty,
                        ));
                    }
                }
            }
            wasmparser::Payload::MemorySection(s) => {
                for mem in s {
                    memories.push(mem?);
                }
            }
            wasmparser::Payload::ExportSection(s) => {
                for export in s {
                    let export = export?;
                    if export.kind == wasmparser::ExternalKind::Memory {
                        exports.push((export.name.to_string(), export.index));
                    }
                }
            }
            _ => {}
        }
    }

    let mut mem_idx = 0;
    // A module's memories space starts with imported memories, followed by defined memories.
    for (_module, name, ty) in imports {
        if let wasmparser::TypeRef::Memory(mem_ty) = ty {
            let initial = mem_ty.initial;
            let maximum = mem_ty.maximum.unwrap_or(mem_ty.initial);
            mem_sizes.insert(CompactString::from(name.as_str()), (initial, maximum));
            mem_idx += 1;
        }
    }

    let mut i = 0;
    for mem_ty in memories {
        let mut name = String::new();
        for (exp_name, exp_idx) in &exports {
            if *exp_idx == mem_idx {
                name = exp_name.clone();
                break;
            }
        }
        if name.is_empty() {
            name = if i == 0 {
                "memory".to_string()
            } else {
                format!("memory_{i}")
            };
        }
        i += 1;

        let initial = mem_ty.initial;
        let maximum = mem_ty.maximum.unwrap_or(mem_ty.initial);
        mem_sizes.insert(CompactString::from(name.as_str()), (initial, maximum));
        mem_idx += 1;
    }

    Ok(mem_sizes)
}
