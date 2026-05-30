use crate::wasm_stream::pipeline::StreamPass;
use std::collections::HashMap;
use wasm_encoder::{EntityType, ExportSection, ImportSection, MemorySection, Module, RawSection};

pub struct TemporaryRefugeMemoryStreamPass {
    pub memory_count: usize,
    pub had_shared: bool,
    pub rename_memory_export: Option<String>,
}

impl TemporaryRefugeMemoryStreamPass {
    pub fn new(rename_memory_export: Option<String>) -> Self {
        Self {
            memory_count: 0,
            had_shared: false,
            rename_memory_export,
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
                                        let n = (1..)
                                            .find(|n| {
                                                !mem_names
                                                    .values()
                                                    .any(|s| *s == format!("{name}_{n}"))
                                            })
                                            .unwrap();
                                        mem_names.insert(mem_idx, format!("{name}_{n}"));
                                    } else {
                                        mem_names.insert(mem_idx, name);
                                    }

                                    // Turn off shared
                                    if mem_ty.shared {
                                        if !is_first {
                                            is_first = true;
                                            log::warn!(
                                                "Transpiling with threads is not supported yet. so this wasm off memory shared flag and can't be used as it is."
                                            );
                                        }
                                        self.had_shared = true;
                                        mem_ty.shared = false;
                                    }

                                    // Instead of importing, we will define it as a local memory.
                                    memory_types.push(mem_ty);
                                    mem_idx += 1;
                                }
                                _ => {
                                    let entity = match import.ty {
                                        wasmparser::TypeRef::Func(i) => EntityType::Function(i),
                                        wasmparser::TypeRef::Table(t) => EntityType::Table(
                                            crate::wasm_stream::translator::translate_table_type(
                                                t,
                                                &crate::wasm_stream::translator::DefaultRebinder,
                                            ),
                                        ),
                                        wasmparser::TypeRef::Global(g) => EntityType::Global(
                                            crate::wasm_stream::translator::translate_global_type(
                                                g,
                                                &crate::wasm_stream::translator::DefaultRebinder,
                                            ),
                                        ),
                                        wasmparser::TypeRef::Tag(t) => EntityType::Tag(
                                            crate::wasm_stream::translator::translate_tag_type(t),
                                        ),
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
                                log::warn!(
                                    "Transpiling with threads is not supported yet. so this wasm off memory shared flag and can't be used as it is."
                                );
                            }
                            self.had_shared = true;
                            mem_ty.shared = false;
                        }
                        memory_types.push(mem_ty);
                        mem_idx += 1;
                    }
                }
                _ => {
                    // Emit memories right before GlobalSection or any section after MemorySection (ID 5)
                    // If we encounter ID >= 6 (Global, Export, Start, Element, Code, Data, DataCount)
                    if let Some((id, _)) = payload.as_section() {
                        if id >= 6 && !memory_types.is_empty() {
                            let mut memories = MemorySection::new();
                            for mem_ty in &memory_types {
                                memories.memory(
                                    crate::wasm_stream::translator::translate_memory_type(*mem_ty),
                                );
                            }
                            module.section(&memories);
                            memory_types.clear();
                        }
                    }

                    match payload {
                        wasmparser::Payload::ExportSection(s) => {
                            let mut exports = ExportSection::new();
                            for export in s {
                                let export = export?;
                                let mut name = export.name;
                                if export.kind == wasmparser::ExternalKind::Memory
                                    && name == "memory"
                                {
                                    log::info!(
                                        "Found memory export. rename_memory_export is {:?}",
                                        self.rename_memory_export
                                    );
                                    if let Some(ref new_name) = self.rename_memory_export {
                                        name = new_name;
                                    }
                                }
                                exports.export(
                                    name,
                                    match export.kind {
                                        wasmparser::ExternalKind::Func
                                        | wasmparser::ExternalKind::FuncExact => {
                                            wasm_encoder::ExportKind::Func
                                        }
                                        wasmparser::ExternalKind::Table => {
                                            wasm_encoder::ExportKind::Table
                                        }
                                        wasmparser::ExternalKind::Memory => {
                                            wasm_encoder::ExportKind::Memory
                                        }
                                        wasmparser::ExternalKind::Global => {
                                            wasm_encoder::ExportKind::Global
                                        }
                                        wasmparser::ExternalKind::Tag => {
                                            wasm_encoder::ExportKind::Tag
                                        }
                                    },
                                    export.index,
                                );
                            }
                            module.section(&exports);
                        }
                        wasmparser::Payload::CustomSection(c) => {
                            module.section(&wasm_encoder::CustomSection {
                                name: c.name().into(),
                                data: std::borrow::Cow::Borrowed(c.data()),
                            });
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
            }
        }

        if !memory_types.is_empty() {
            let mut memories = MemorySection::new();
            for mem_ty in &memory_types {
                memories.memory(crate::wasm_stream::translator::translate_memory_type(
                    *mem_ty,
                ));
            }
            module.section(&memories);
            memory_types.clear();
        }

        self.memory_count = mem_idx as usize;

        Ok(module.finish())
    }
}
