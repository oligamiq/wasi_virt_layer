use crate::wasm_stream::pipeline::StreamPass;
use wasm_encoder::{ImportSection, Module, RawSection};

pub struct PostComponentsMemoryFixStreamPass {
    pub threads: bool,
}

impl PostComponentsMemoryFixStreamPass {
    pub fn new(threads: bool) -> Self {
        Self { threads }
    }
}

impl StreamPass for PostComponentsMemoryFixStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> eyre::Result<Vec<u8>> {
        if !self.threads {
            return Ok(input_wasm.to_vec());
        }

        let mut defined_memories = Vec::new();
        let parser = wasmparser::Parser::new(0);

        // Pass 1: find defined memories
        for payload in parser.parse_all(input_wasm) {
            let payload = payload?;
            if let wasmparser::Payload::MemorySection(s) = payload {
                for mem in s {
                    defined_memories.push(mem?);
                }
            }
        }

        // Pass 2: rewrite
        let mut module = Module::new();
        let parser = wasmparser::Parser::new(0);
        let mut appended_memories = false;

        for payload in parser.parse_all(input_wasm) {
            let payload = payload?;
            match payload {
                wasmparser::Payload::ImportSection(s) => {
                    let mut import_sec = ImportSection::new();
                    for group in s {
                        for import in group? {
                            let (_, import) = import?;
                            let ty = match import.ty {
                                wasmparser::TypeRef::Func(f) => wasm_encoder::EntityType::Function(f),
                                wasmparser::TypeRef::Table(t) => wasm_encoder::EntityType::Table(crate::wasm_stream::translator::translate_table_type(t, &crate::wasm_stream::translator::DefaultRebinder)),
                                wasmparser::TypeRef::Memory(m) => wasm_encoder::EntityType::Memory(crate::wasm_stream::translator::translate_memory_type(m)),
                                wasmparser::TypeRef::Global(g) => wasm_encoder::EntityType::Global(crate::wasm_stream::translator::translate_global_type(g, &crate::wasm_stream::translator::DefaultRebinder)),
                                wasmparser::TypeRef::Tag(t) => wasm_encoder::EntityType::Tag(crate::wasm_stream::translator::translate_tag_type(t)),
                                _ => unreachable!(),
                            };
                            import_sec.import(import.module, import.name, ty);
                        }
                    }
                    // Add defined memories as imports to env.memory
                    for mem in &defined_memories {
                        let mut max = mem.maximum;
                        if max.is_none() {
                            max = Some(mem.initial.max(65536));
                        }
                        import_sec.import(
                            "env",
                            "memory",
                            wasm_encoder::EntityType::Memory(wasm_encoder::MemoryType {
                                minimum: mem.initial,
                                maximum: max,
                                memory64: mem.memory64,
                                shared: true,
                                page_size_log2: mem.page_size_log2,
                            }),
                        );
                    }
                    module.section(&import_sec);
                    appended_memories = true;
                }
                wasmparser::Payload::MemorySection(_) => {
                    // We moved all memories to imports, so we omit this section!
                    // Wait, what if there are other defined memories we didn't want to move?
                    // The original walrus pass modified `module.memories`, moving ALL of them to imports.
                    // So we can completely drop the MemorySection!
                }
                _ => {
                    if let Some((id, range)) = payload.as_section() {
                        if id > 2 && !appended_memories && !defined_memories.is_empty() {
                            // If there was no ImportSection, we must create one before FunctionSection (id 3)
                            let mut import_sec = ImportSection::new();
                            for mem in &defined_memories {
                                let mut max = mem.maximum;
                                if max.is_none() {
                                    max = Some(mem.initial.max(65536));
                                }
                                import_sec.import(
                                    "env",
                                    "memory",
                                    wasm_encoder::EntityType::Memory(wasm_encoder::MemoryType {
                                        minimum: mem.initial,
                                        maximum: max,
                                        memory64: mem.memory64,
                                        shared: true,
                                        page_size_log2: mem.page_size_log2,
                                    }),
                                );
                            }
                            module.section(&import_sec);
                            appended_memories = true;
                        }

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
