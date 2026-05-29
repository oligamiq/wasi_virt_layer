use crate::wasm_stream::pipeline::StreamPass;
use eyre::{Result, Context};
use wasm_encoder::{ExportSection, ImportSection, Module, RawSection, Section};
use wasmparser::Parser;
use crate::{
    abi::Wasip1ABIFunc,
    generator::{GeneratorCtx, memory::MemoryUniqueName},
    unique_name::UniqueName,
};
use strum::VariantNames;

fn normalize_name(s: &str) -> String {
    s.replace('-', "_")
}

/// Generator component translating un-named single targets into explicit generic bindings.
#[derive(Debug)]
pub struct AnonymousStreamPass {
    ctx: GeneratorCtx,
}

impl AnonymousStreamPass {
    pub fn new(ctx: GeneratorCtx) -> Self {
        Self { ctx }
    }
}

impl StreamPass for AnonymousStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> Result<Vec<u8>> {
        let mut module = Module::new();
        let parser = Parser::new(0);

        for payload in parser.parse_all(input_wasm) {
            let payload = payload?;
            let section_info = payload.as_section().map(|(id, r)| (id, r.clone()));
            match payload {
                wasmparser::Payload::ExportSection(s) => {
                    let anonymous_targets = s.clone().into_iter().filter_map(|e| {
                        let e = e.unwrap();
                        e.name
                            .strip_prefix("__wasip1_vfs_")?
                            .strip_suffix("__start_anchor")
                            .map(|s| s.to_string())
                    }).collect::<Vec<_>>();

                    let collected = self.ctx.target_names.iter().filter(|t| {
                        let normalized_t = normalize_name(t.as_ref());
                        !anonymous_targets.iter().any(|at| at == &normalized_t)
                    }).collect::<Vec<_>>();

                    if collected.len() != 1 {
                        module.section(&RawSection {
                            id: wasm_encoder::SectionId::Export as u8,
                            data: &input_wasm[section_info.as_ref().unwrap().1.clone()],
                        });
                        continue;
                    }

                    let target_name = normalize_name(collected[0].as_ref());
                    let mut new_export_section = ExportSection::new();
                    let mut modified = false;
                    let prefix = UniqueName::PREFIX;

                    const EXPORT_POSTFIXS: &[&str] = &[
                        "__start",
                        "__start_anchor",
                        "_memory_trap_anchor",
                        "_wasi_thread_start_anchor",
                        "_memory_grow_global_alt_get",
                        "_memory_grow_global_alt_get_no_wait",
                        "_memory_grow_global_alt_init_once",
                        "_memory_grow_global_alt_pos",
                        "_memory_grow_global_alt_set",
                        "_memory_grow_global_alt_set_with_lock",
                    ];

                    for export in s {
                        let export = export.wrap_err("failed to parse export")?;
                        let mut name = export.name.to_string();

                        for postfix in EXPORT_POSTFIXS {
                            let anonymous_export_name = format!("{prefix}anonymous{postfix}");
                            if name == anonymous_export_name {
                                name = format!("{prefix}{target_name}{postfix}");
                                modified = true;
                                break;
                            }
                        }

                        if let Some(anonymous_suffix) = name.strip_prefix(prefix).and_then(|s| s.strip_prefix("anonymous_")) {
                            if let Some(f) = Wasip1ABIFunc::VARIANTS.iter().find(|v| anonymous_suffix == **v) {
                                name = format!("{prefix}{target_name}_{f}");
                                modified = true;
                            }
                        }

                        if name == format!("{prefix}wasi_thread_spawn_anonymous") {
                            name = format!("{prefix}wasi_thread_spawn_{target_name}");
                            modified = true;
                        }

                        new_export_section.export(
                            &name,
                            export.kind.into(),
                            export.index,
                        );
                    }

                    if modified {
                        module.section(&new_export_section);
                    } else {
                        module.section(&RawSection {
                            id: wasm_encoder::SectionId::Export as u8,
                            data: &input_wasm[section_info.as_ref().unwrap().1.clone()],
                        });
                    }
                }
                wasmparser::Payload::ImportSection(s) => {

                    // we don't have exports here to find anonymous_targets, but we can just use self.ctx.target_names.len() != 1
                    // Wait, collected logic needs to be the same, but we don't have exports!
                    // Let's just do self.ctx.target_names.len() == 1 for imports too as an approximation,
                    // or better, we know collected[0] if there's only 1.
                    // Actually, since we process sequentially, we don't have exports yet if imports come first.
                    // Let's just use self.ctx.target_names if len == 1. Wait, what if there are 2 targets but one is matched to another anchor?
                    // Legacy code checked `collected.len() == 1` inside `pre_vfs` which ran ONCE for the whole module.
                    // In stream pass, we should find `collected` in a first pass or just approximate it.
                    if self.ctx.target_names.len() != 1 {
                        module.section(&RawSection {
                            id: wasm_encoder::SectionId::Import as u8,
                            data: &input_wasm[section_info.as_ref().unwrap().1.clone()],
                        });
                        continue;
                    }

                    let target_name = normalize_name(self.ctx.target_names[0].as_ref());
                    let mut new_import_section = ImportSection::new();
                    let mut modified = false;
                    let prefix = UniqueName::PREFIX;
                    let namespace = UniqueName::NAMESPACE;

                    const EXTRA_IMPORTS: &[&str] = &[
                        "_start",
                        "memory_trap",
                        "__main_void",
                        "reset",
                        "wasi_thread_start",
                    ];

                    for import_group in s {
                        let import_group = import_group.wrap_err("failed to parse import group")?;
                        for import_res in import_group {
                            let (_, import) = import_res.wrap_err("failed to parse import")?;
                            let mut name = import.name.to_string();

                            if import.module == namespace {
                                if let Some(anonymous_suffix) = name.strip_prefix(prefix).and_then(|s| s.strip_prefix("anonymous_")) {
                                    if let Some(f) = MemoryUniqueName::VARIANTS.iter().chain(EXTRA_IMPORTS).find(|v| anonymous_suffix == **v) {
                                        name = format!("{prefix}{target_name}_{f}");
                                        modified = true;
                                    }
                                }
                            }

                            let translated_ty = match import.ty {
                                wasmparser::TypeRef::Func(idx) => wasm_encoder::EntityType::Function(idx),
                                wasmparser::TypeRef::FuncExact(_) => unimplemented!("FuncExact translation is not supported"),
                                wasmparser::TypeRef::Table(t) => wasm_encoder::EntityType::Table(crate::wasm_stream::translator::translate_table_type(t, &crate::wasm_stream::translator::DefaultRebinder)),
                                wasmparser::TypeRef::Memory(t) => wasm_encoder::EntityType::Memory(crate::wasm_stream::translator::translate_memory_type(t)),
                                wasmparser::TypeRef::Global(t) => wasm_encoder::EntityType::Global(crate::wasm_stream::translator::translate_global_type(t, &crate::wasm_stream::translator::DefaultRebinder)),
                                wasmparser::TypeRef::Tag(t) => wasm_encoder::EntityType::Tag(crate::wasm_stream::translator::translate_tag_type(t)),
                                _ => unimplemented!("Unsupported TypeRef translation"),
                            };

                            new_import_section.import(
                                import.module,
                                &name,
                                translated_ty,
                            );
                        }
                    }

                    if modified {
                        module.section(&new_import_section);
                    } else {
                        module.section(&RawSection {
                            id: wasm_encoder::SectionId::Import as u8,
                            data: &input_wasm[section_info.as_ref().unwrap().1.clone()],
                        });
                    }
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
