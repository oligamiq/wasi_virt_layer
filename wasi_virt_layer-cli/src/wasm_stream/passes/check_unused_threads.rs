use crate::generator::GeneratorCtx;
use crate::wasm_stream::pipeline::StreamPass;
use eyre::Result;
use wasm_encoder::{ExportSection, ImportSection, MemorySection, Module, RawSection};
use wasmparser::Parser;

/// Detects unused thread imports and efficiently cleans thread-related exports statically.
#[derive(Debug)]
pub struct CheckUnusedThreadsStreamPass {
    ctx: GeneratorCtx,
}

impl CheckUnusedThreadsStreamPass {
    pub fn new(ctx: GeneratorCtx) -> Self {
        Self { ctx }
    }
}

impl StreamPass for CheckUnusedThreadsStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> Result<Vec<u8>> {
        if !self.ctx.threads {
            return Ok(input_wasm.to_vec());
        }

        let mut module = Module::new();
        let parser = Parser::new(0);

        // First pass: check if wasi.thread-spawn is imported
        let mut has_thread_spawn = false;
        let mut target_memory_index = None;

        for payload in Parser::new(0).parse_all(input_wasm) {
            let payload = payload?;
            match payload {
                wasmparser::Payload::ImportSection(s) => {
                    for import_group in s {
                        for import_res in import_group.unwrap() {
                            let (_, import) = import_res.unwrap();
                            if import.module == "wasi" && import.name == "thread-spawn" {
                                has_thread_spawn = true;
                            }
                            if let wasmparser::TypeRef::Memory(_) = import.ty {
                                if target_memory_index.is_none() {
                                    target_memory_index = Some(1); // placeholder logic just to mark existence
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        if has_thread_spawn {
            // No modifications needed
            return Ok(input_wasm.to_vec());
        }

        log::warn!(
            "wasi.thread-spawn is not imported, but --threads is true. Attempting to strip threads."
        );

        let mut removed_memory: Option<wasmparser::MemoryType> = None;

        for payload in parser.parse_all(input_wasm) {
            let payload = payload?;
            let section_info = payload.as_section().map(|(id, r)| (id, r.clone()));

            if let Some((id, _)) = section_info {
                if id >= 6 && removed_memory.is_some() {
                    let mut mem_section = MemorySection::new();
                    mem_section.memory(crate::wasm_stream::translator::translate_memory_type(
                        removed_memory.unwrap(),
                    ));
                    module.section(&mem_section);
                    removed_memory = None;
                }
            }

            match payload {
                wasmparser::Payload::ImportSection(s) => {
                    let mut new_import_section = ImportSection::new();
                    let mut memory_removed = false;

                    for import_group in s {
                        for import_res in import_group.unwrap() {
                            let (_, import) = import_res.unwrap();
                            if let wasmparser::TypeRef::Memory(m) = import.ty {
                                if !memory_removed {
                                    memory_removed = true;
                                    removed_memory = Some(m);
                                    continue;
                                }
                            }
                            let entity_type = match import.ty {
                                wasmparser::TypeRef::Func(f) => {
                                    wasm_encoder::EntityType::Function(f)
                                }
                                wasmparser::TypeRef::Table(t) => wasm_encoder::EntityType::Table(
                                    crate::wasm_stream::translator::translate_table_type(
                                        t,
                                        &crate::wasm_stream::translator::DefaultRebinder,
                                    ),
                                ),
                                wasmparser::TypeRef::Memory(m) => wasm_encoder::EntityType::Memory(
                                    crate::wasm_stream::translator::translate_memory_type(m),
                                ),
                                wasmparser::TypeRef::Global(g) => wasm_encoder::EntityType::Global(
                                    crate::wasm_stream::translator::translate_global_type(
                                        g,
                                        &crate::wasm_stream::translator::DefaultRebinder,
                                    ),
                                ),
                                wasmparser::TypeRef::Tag(t) => wasm_encoder::EntityType::Tag(
                                    crate::wasm_stream::translator::translate_tag_type(t),
                                ),
                                _ => unimplemented!("TypeRef variant not supported"),
                            };
                            new_import_section.import(import.module, import.name, entity_type);
                        }
                    }
                    module.section(&new_import_section);
                }
                wasmparser::Payload::MemorySection(s) => {
                    let mut mem_section = MemorySection::new();
                    if let Some(mem) = removed_memory {
                        mem_section
                            .memory(crate::wasm_stream::translator::translate_memory_type(mem));
                        removed_memory = None;
                    }
                    for mem in s {
                        mem_section.memory(crate::wasm_stream::translator::translate_memory_type(
                            mem.unwrap(),
                        ));
                    }
                    module.section(&mem_section);
                }
                wasmparser::Payload::ExportSection(s) => {
                    let mut new_export_section = ExportSection::new();

                    for export in s {
                        let export = export.unwrap();
                        if export.name == "wasi_thread_start" {
                            continue; // Remove this export
                        }
                        new_export_section.export(export.name, export.kind.into(), export.index);
                    }
                    module.section(&new_export_section);
                }
                _ => {
                    if let Some((id, range)) = section_info {
                        module.section(&RawSection {
                            id,
                            data: &input_wasm[range.clone()],
                        });
                    }
                }
            }
        }

        Ok(module.finish())
    }
}
