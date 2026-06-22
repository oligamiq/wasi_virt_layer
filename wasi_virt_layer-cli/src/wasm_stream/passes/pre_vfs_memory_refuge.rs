use crate::wasm_stream::pipeline::StreamPass;
use std::collections::HashMap;
use wasm_encoder::{EntityType, ExportSection, ImportSection, MemorySection, Module, RawSection};

pub struct TemporaryRefugeMemoryStreamPass {
    pub memory_count: usize,
    pub had_shared: bool,
    pub rename_memory_export: Option<String>,
    type_section_data: Vec<u8>,
}

impl TemporaryRefugeMemoryStreamPass {
    pub fn new(rename_memory_export: Option<String>) -> Self {
        Self {
            memory_count: 0,
            had_shared: false,
            rename_memory_export,
            type_section_data: Vec::new(),
        }
    }
}

impl StreamPass for TemporaryRefugeMemoryStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> eyre::Result<Vec<u8>> {
        // Pre-scan type section bytes from the binary header.
        // wasmparser::Payload::TypeSection does not reliably expose these
        // through as_section(), so we read the section manually.
        self.type_section_data = {
            let mut offset = 8; // skip magic + version
            let mut result = Vec::new();
            while offset + 1 < input_wasm.len() {
                let id = input_wasm[offset];
                offset += 1;
                let mut size: u64 = 0;
                let mut shift = 0;
                while offset < input_wasm.len() {
                    let byte = input_wasm[offset];
                    offset += 1;
                    size |= ((byte & 0x7f) as u64) << shift;
                    shift += 7;
                    if byte & 0x80 == 0 {
                        break;
                    }
                }
                let content_end = offset + size as usize;
                if id == 1 && content_end <= input_wasm.len() {
                    result = input_wasm[offset..content_end].to_vec();
                    break;
                }
                offset = content_end;
            }
            result
        };

        let mut module = Module::new();
        let mut mem_names: HashMap<u32, String> = HashMap::new();
        let mut memory_types = Vec::new();
        let mut mem_idx = 0;
        let mut is_first = false;

        for payload in wasmparser::Parser::new(0).parse_all(input_wasm) {
            let payload = payload?;
            match payload {
                // ── Memory imports → local memories ──────────────────
                wasmparser::Payload::ImportSection(s) => {
                    let mut imports = ImportSection::new();
                    for group in s {
                        for i in group?.into_iter() {
                            let (_, import) = i?;
                            match import.ty {
                                wasmparser::TypeRef::Memory(mut mem_ty) => {
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
                                    if mem_ty.shared {
                                        if !is_first {
                                            is_first = true;
                                            log::warn!(
                                                "Transpiling with threads is not supported yet."
                                            );
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

                // ── Memory section ──────────────────────────────────
                wasmparser::Payload::MemorySection(s) => {
                    for mem_ty in s {
                        let mut mem_ty = mem_ty?;
                        if mem_ty.shared {
                            if !is_first {
                                is_first = true;
                                log::warn!("Transpiling with threads is not supported yet.");
                            }
                            self.had_shared = true;
                            mem_ty.shared = false;
                        }
                        memory_types.push(mem_ty);
                        mem_idx += 1;
                    }
                }

                // ── Type section (pre-scanned bytes) ────────────────
                wasmparser::Payload::TypeSection(_) => {
                    if !self.type_section_data.is_empty() {
                        module.section(&RawSection {
                            id: 1,
                            data: &self.type_section_data,
                        });
                        self.type_section_data.clear();
                    }
                }

                // ── Code section (copy raw) ─────────────────────────
                wasmparser::Payload::CodeSectionStart { range, .. } => {
                    if !memory_types.is_empty() {
                        let mut memories = MemorySection::new();
                        for mt in &memory_types {
                            memories
                                .memory(crate::wasm_stream::translator::translate_memory_type(*mt));
                        }
                        module.section(&memories);
                        memory_types.clear();
                    }
                    module.section(&RawSection {
                        id: 10,
                        data: &input_wasm[range],
                    });
                }
                wasmparser::Payload::CodeSectionEntry(_) => {}

                // ── Everything else ─────────────────────────────────
                _ => {
                    // Emit pending memories
                    if let Some((id, _)) = payload.as_section() {
                        if id >= 6 && !memory_types.is_empty() {
                            let mut memories = MemorySection::new();
                            for mt in &memory_types {
                                memories.memory(
                                    crate::wasm_stream::translator::translate_memory_type(*mt),
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
                                    data: &input_wasm[range],
                                });
                            }
                        }
                    }
                }
            }
        }

        if !memory_types.is_empty() {
            let mut memories = MemorySection::new();
            for mt in &memory_types {
                memories.memory(crate::wasm_stream::translator::translate_memory_type(*mt));
            }
            module.section(&memories);
        }

        self.memory_count = mem_idx as usize;

        Ok(module.finish())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_encoder::{
        EntityType, ExportKind, ExportSection, ImportSection, MemoryType, Module, TypeSection,
    };

    fn fixture(shared: bool) -> Vec<u8> {
        let mut module = Module::new();

        let mut types = TypeSection::new();
        types.ty().function([], []);
        module.section(&types);

        let mut imports = ImportSection::new();
        imports.import(
            "env",
            "memory",
            EntityType::Memory(MemoryType {
                minimum: 1,
                maximum: Some(2),
                memory64: false,
                shared,
                page_size_log2: None,
            }),
        );
        imports.import("env", "dummy_func", EntityType::Function(0));
        module.section(&imports);

        let mut exports = ExportSection::new();
        exports.export("memory", ExportKind::Memory, 0);
        module.section(&exports);

        module.finish()
    }

    #[test]
    fn test_refuge_memory_non_shared() -> eyre::Result<()> {
        let input = fixture(false);
        let mut pass = TemporaryRefugeMemoryStreamPass::new(None);
        let output = pass.run(&input)?;

        wasmparser::Validator::new_with_features(wasmparser::WasmFeatures::all())
            .validate_all(&output)?;

        assert_eq!(pass.memory_count, 1);
        assert!(!pass.had_shared);

        // Verify imports and exports
        let mut has_mem_import = false;
        let mut has_local_mem = false;
        let mut memory_shared = false;
        let mut memory_export_name = String::new();

        for payload in wasmparser::Parser::new(0).parse_all(&output) {
            match payload? {
                wasmparser::Payload::ImportSection(s) => {
                    for group in s {
                        for i in group? {
                            let (_, import) = i?;
                            if let wasmparser::TypeRef::Memory(_) = import.ty {
                                has_mem_import = true;
                            }
                        }
                    }
                }
                wasmparser::Payload::MemorySection(s) => {
                    for mem in s {
                        has_local_mem = true;
                        memory_shared = mem?.shared;
                    }
                }
                wasmparser::Payload::ExportSection(s) => {
                    for export in s {
                        let export = export?;
                        if export.kind == wasmparser::ExternalKind::Memory {
                            memory_export_name = export.name.to_string();
                        }
                    }
                }
                _ => {}
            }
        }

        assert!(!has_mem_import, "Imported memory should be removed");
        assert!(has_local_mem, "Memory should become local");
        assert!(!memory_shared, "Memory should not be shared");
        assert_eq!(memory_export_name, "memory");

        Ok(())
    }

    #[test]
    fn test_refuge_memory_shared_and_renamed() -> eyre::Result<()> {
        let input = fixture(true);
        let mut pass = TemporaryRefugeMemoryStreamPass::new(Some("vfs_memory".to_string()));
        let output = pass.run(&input)?;

        wasmparser::Validator::new_with_features(wasmparser::WasmFeatures::all())
            .validate_all(&output)?;

        assert_eq!(pass.memory_count, 1);
        assert!(pass.had_shared);

        let mut has_mem_import = false;
        let mut has_local_mem = false;
        let mut memory_shared = true;
        let mut memory_export_name = String::new();

        for payload in wasmparser::Parser::new(0).parse_all(&output) {
            match payload? {
                wasmparser::Payload::ImportSection(s) => {
                    for group in s {
                        for i in group? {
                            let (_, import) = i?;
                            if let wasmparser::TypeRef::Memory(_) = import.ty {
                                has_mem_import = true;
                            }
                        }
                    }
                }
                wasmparser::Payload::MemorySection(s) => {
                    for mem in s {
                        has_local_mem = true;
                        memory_shared = mem?.shared;
                    }
                }
                wasmparser::Payload::ExportSection(s) => {
                    for export in s {
                        let export = export?;
                        if export.kind == wasmparser::ExternalKind::Memory {
                            memory_export_name = export.name.to_string();
                        }
                    }
                }
                _ => {}
            }
        }

        assert!(!has_mem_import, "Imported memory should be removed");
        assert!(has_local_mem, "Memory should become local");
        assert!(!memory_shared, "Memory should not be shared in output");
        assert_eq!(memory_export_name, "vfs_memory");

        Ok(())
    }
}
