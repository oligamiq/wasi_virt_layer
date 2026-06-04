use crate::abi::{Wasip1ThreadsABIExportFunc, Wasip1ThreadsABIFunc};
use crate::unique_name::UniqueName;
use crate::wasm_stream::pipeline::StreamPass;
use strum::VariantNames;
use wasm_encoder::{EntityType, ExportKind, ExportSection, ImportSection, Module, RawSection};

pub struct ThreadsSpawnPreTargetStreamPass {
    threads: bool,
    wasm_name_str: String,
}

impl ThreadsSpawnPreTargetStreamPass {
    pub fn new(threads: bool, wasm_name_str: String) -> Self {
        Self {
            threads,
            wasm_name_str,
        }
    }
}

impl StreamPass for ThreadsSpawnPreTargetStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> eyre::Result<Vec<u8>> {
        if !self.threads {
            return Ok(input_wasm.to_vec());
        }

        let start_name = <Wasip1ThreadsABIFunc as VariantNames>::VARIANTS
            .first()
            .copied()
            .unwrap(); // "thread-spawn"

        let export_names = <Wasip1ThreadsABIExportFunc as VariantNames>::VARIANTS;

        let new_import_name = format!("__wasip1_vfs_wasi_thread_spawn_{}", self.wasm_name_str);

        let mut module = Module::new();
        let parser = wasmparser::Parser::new(0);

        for payload in parser.parse_all(input_wasm) {
            let payload = payload?;
            match payload {
                wasmparser::Payload::ImportSection(s) => {
                    let mut import_sec = ImportSection::new();
                    for group in s.clone() {
                        for import in group? {
                            let (_, import) = import?;
                            let ty = match import.ty {
                                wasmparser::TypeRef::Func(f) => EntityType::Function(f),
                                wasmparser::TypeRef::Table(t) => EntityType::Table(
                                    crate::wasm_stream::translator::translate_table_type(
                                        t,
                                        &crate::wasm_stream::translator::DefaultRebinder,
                                    ),
                                ),
                                wasmparser::TypeRef::Memory(m) => EntityType::Memory(
                                    crate::wasm_stream::translator::translate_memory_type(m),
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
                                _ => unreachable!(),
                            };

                            let mut name = import.name;
                            if import.module == UniqueName::WASIP1_THREADS_ABI_MODULE
                                && name == start_name
                            {
                                name = &new_import_name;
                                import_sec.import("env", name, ty);
                            } else {
                                import_sec.import(import.module, name, ty);
                            }
                        }
                    }
                    module.section(&import_sec);
                }
                wasmparser::Payload::ExportSection(s) => {
                    let mut export_sec = ExportSection::new();
                    for export in s {
                        let export = export?;
                        let mut name = export.name.to_string();
                        if export_names.contains(&export.name) {
                            name = format!("__wasip1_vfs_{}_{}", self.wasm_name_str, name);
                        }

                        let kind = match export.kind {
                            wasmparser::ExternalKind::Func
                            | wasmparser::ExternalKind::FuncExact => ExportKind::Func,
                            wasmparser::ExternalKind::Table => ExportKind::Table,
                            wasmparser::ExternalKind::Memory => ExportKind::Memory,
                            wasmparser::ExternalKind::Global => ExportKind::Global,
                            wasmparser::ExternalKind::Tag => ExportKind::Tag,
                            _ => unimplemented!(),
                        };
                        export_sec.export(&name, kind, export.index);
                    }
                    module.section(&export_sec);
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
        Ok(module.finish())
    }
}

pub struct ThreadsSpawnPreVfsStreamPass {
    threads: bool,
}

impl ThreadsSpawnPreVfsStreamPass {
    pub fn new(threads: bool) -> Self {
        Self { threads }
    }
}

impl StreamPass for ThreadsSpawnPreVfsStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> eyre::Result<Vec<u8>> {
        if !self.threads {
            return Ok(input_wasm.to_vec());
        }

        let start_name = <Wasip1ThreadsABIFunc as VariantNames>::VARIANTS
            .first()
            .copied()
            .unwrap(); // "thread-spawn"

        let export_name = <Wasip1ThreadsABIExportFunc as VariantNames>::VARIANTS
            .first()
            .copied()
            .unwrap(); // "wasi_thread_start"

        crate::wasm_stream::passes::abi_connect::rewrite_imports(input_wasm, |module, name, _ty| {
            if module == UniqueName::WASIP1_THREADS_ABI_MODULE && name == start_name {
                return (
                    "env".to_string(),
                    "__wasip1_vfs_wasi_thread_spawn_wrapper".to_string(),
                );
            }
            if module == "wasi:thread/spawn/real" && name == start_name {
                let component_name = crate::util::gen_component_name(
                    UniqueName::WASIP1_THREADS_ABI_MODULE_ALT,
                    start_name,
                );
                return (UniqueName::THREADS_MODULE_ROOT.to_string(), component_name);
            }
            if module == UniqueName::NAMESPACE && name == "__wasip1_vfs___self_wasi_thread_start" {
                return ("__wasip1_vfs-host".to_string(), export_name.to_string());
            }
            (module.to_string(), name.to_string())
        })
    }
}
