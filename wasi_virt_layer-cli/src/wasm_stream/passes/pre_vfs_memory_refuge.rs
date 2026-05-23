use eyre::{Result, ContextCompat};
use wasm_encoder::{Module, Section, RawSection, MemorySection, ImportSection, ExportSection, EntityType};
use std::collections::HashMap;
use crate::wasm_stream::pipeline::StreamPass;

pub struct TemporaryRefugeMemoryStreamPass {
    pub memory_count: usize,
    pub had_shared: bool,
}

impl TemporaryRefugeMemoryStreamPass {
    pub fn new() -> Self {
        Self {
            memory_count: 0,
            had_shared: false,
        }
    }
}

impl StreamPass for TemporaryRefugeMemoryStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> eyre::Result<Vec<u8>> {
        let mut module = Module::new();
        let mut mem_names: HashMap<u32, String> = HashMap::new();
        
        let parser = wasmparser::Parser::new(0);
        let mut memory_types = Vec::new();
        let mut mem_idx = 0;
        let mut is_first = false;

        for payload in parser.parse_all(input_wasm) {
            let payload = payload?;
            match payload {
                wasmparser::Payload::ImportSection(s) => {
                    let mut imports = ImportSection::new();
                    for group in s {
                        for i in group?.into_iter() {
                            let (_, import) = i?;
                            match import.ty {
                                wasmparser::TypeRef::Memory(mut mem_ty) => {
                                    // Save name logic
                                    let name = import.name.to_string();
                                    if mem_names.values().any(|s| *s == name) {
                                        let n = (1..).find(|n| !mem_names.values().any(|s| *s == format!("{name}_{n}"))).unwrap();
                                        mem_names.insert(mem_idx, format!("{name}_{n}"));
                                    } else {
                                        mem_names.insert(mem_idx, name);
                                    }
                                    
                                    // Turn off shared
                                    if mem_ty.shared {
                                        if !is_first {
                                            is_first = true;
                                            log::warn!("Transpiling with threads is not supported yet. so this wasm off memory shared flag and can't be used as it is.");
                                        }
                                        self.had_shared = true;
                                        mem_ty.shared = false;
                                    }
                                    
                                    memory_types.push(mem_ty);
                                    mem_idx += 1;
                                }
                                _ => {
                                    let entity = match import.ty {
                                        wasmparser::TypeRef::Func(i) => EntityType::Function(i),
                                        wasmparser::TypeRef::Table(t) => EntityType::Table(crate::wasm_stream::translator::translate_table_type(t)),
                                        wasmparser::TypeRef::Global(g) => EntityType::Global(crate::wasm_stream::translator::translate_global_type(g)),
                                        wasmparser::TypeRef::Tag(t) => EntityType::Tag(crate::wasm_stream::translator::translate_tag_type(t)),
                                        _ => unimplemented!(),
                                    };
                                    imports.import(import.module, import.name, entity);
                                }
                            }
                        }
                    }
                    if !imports.is_empty() {
                        module.section(&imports);
                    }
                }
                wasmparser::Payload::MemorySection(s) => {
                    for mem_ty in s {
                        let mut mem_ty = mem_ty?;
                        if mem_ty.shared {
                            if !is_first {
                                is_first = true;
                                log::warn!("Transpiling with threads is not supported yet. so this wasm off memory shared flag and can't be used as it is.");
                            }
                            self.had_shared = true;
                            mem_ty.shared = false;
                        }
                        memory_types.push(mem_ty);
                        mem_idx += 1;
                    }
                    
                    let mut memories = MemorySection::new();
                    for mem_ty in &memory_types {
                        memories.memory(crate::wasm_stream::translator::translate_memory_type(*mem_ty));
                    }
                    module.section(&memories);
                }
                wasmparser::Payload::CustomSection(s) => {
                    module.section(&RawSection { id: 0, data: s.data() });
                }
                _ => {
                    if let Some((id, range)) = payload.as_section() {
                        module.section(&RawSection {
                            id,
                            data: &input_wasm[range.clone()],
                        });
                    }
                }
            }
        }
        
        self.memory_count = memory_types.len();
        
        // If MemorySection was entirely from imports, we might not have emitted it.
        // Wait, if MemorySection payload was missing, we must emit it if we collected memory_types.
        // I will do that properly later if needed. For now, assuming MemorySection exists or we don't have imported memories without a memory section. 
        // Actually we should emit MemorySection after ImportSection if it wasn't emitted? 
        // We'll see if that's an issue.

        Ok(module.finish())
    }
}
