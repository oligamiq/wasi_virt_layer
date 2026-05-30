use crate::wasm_stream::pipeline::StreamPass;
use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, FunctionSection, Instruction, Module, RawSection,
    TypeSection,
};

pub struct DummyInjectorStreamPass {
    pub dummy_names: Vec<String>,
}

impl DummyInjectorStreamPass {
    pub fn new(dummy_names: Vec<String>) -> Self {
        Self { dummy_names }
    }
}

impl StreamPass for DummyInjectorStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> eyre::Result<Vec<u8>> {
        if self.dummy_names.is_empty() {
            return Ok(input_wasm.to_vec());
        }

        let mut module = Module::new();
        let parser = wasmparser::Parser::new(0);

        let mut empty_type_idx = None;
        let mut type_count = 0;
        let mut orig_func_count = 0;
        let mut orig_import_func_count = 0;

        let mut existing_exports = std::collections::HashSet::new();

        // Pass 1: find empty type and counts
        for payload in wasmparser::Parser::new(0).parse_all(input_wasm) {
            let payload = payload?;
            match payload {
                wasmparser::Payload::TypeSection(s) => {
                    for (i, ty) in s.into_iter().enumerate() {
                        let ty = ty?;
                        for sub_ty in ty.into_types() {
                            if let wasmparser::CompositeInnerType::Func(f) =
                                &sub_ty.composite_type.inner
                            {
                                if f.params().is_empty() && f.results().is_empty() {
                                    empty_type_idx = Some(i as u32);
                                }
                            }
                        }
                        type_count += 1;
                    }
                }
                wasmparser::Payload::FunctionSection(s) => {
                    orig_func_count = s.count();
                }
                wasmparser::Payload::ImportSection(s) => {
                    for group in s {
                        for import in group?.into_iter() {
                            let (_, import) = import?;
                            if let wasmparser::TypeRef::Func(_) = import.ty {
                                orig_import_func_count += 1;
                            }
                        }
                    }
                }
                wasmparser::Payload::ExportSection(s) => {
                    for export in s {
                        let export = export?;
                        existing_exports.insert(export.name.to_string());
                    }
                }
                _ => {}
            }
        }

        self.dummy_names
            .retain(|name| !existing_exports.contains(name));

        if self.dummy_names.is_empty() {
            return Ok(input_wasm.to_vec());
        }

        for payload in parser.parse_all(input_wasm) {
            let payload = payload?;
            match payload {
                wasmparser::Payload::TypeSection(s) => {
                    let mut types = TypeSection::new();
                    for ty in s {
                        let ty = ty?;
                        if ty.is_explicit_rec_group() {
                            let rec_types = ty
                                .into_types()
                                .map(|sub_ty| {
                                    crate::wasm_stream::translator::translate_sub_type(
                                        &sub_ty,
                                        &crate::wasm_stream::translator::DefaultRebinder,
                                    )
                                })
                                .collect::<Vec<_>>();
                            types.ty().rec(rec_types);
                        } else {
                            for sub_ty in ty.into_types() {
                                types.ty().subtype(
                                    &crate::wasm_stream::translator::translate_sub_type(
                                        &sub_ty,
                                        &crate::wasm_stream::translator::DefaultRebinder,
                                    ),
                                );
                            }
                        }
                    }
                    if empty_type_idx.is_none() {
                        empty_type_idx = Some(type_count);
                        types.ty().function(vec![], vec![]);
                    }
                    module.section(&types);
                }
                wasmparser::Payload::FunctionSection(s) => {
                    let mut funcs = FunctionSection::new();
                    for f in s {
                        funcs.function(f?);
                    }
                    for _ in 0..self.dummy_names.len() {
                        funcs.function(empty_type_idx.unwrap());
                    }
                    module.section(&funcs);
                }
                wasmparser::Payload::ExportSection(s) => {
                    let mut exports = ExportSection::new();
                    for e in s {
                        let e = e?;
                        let kind = match e.kind {
                            wasmparser::ExternalKind::Func => ExportKind::Func,
                            wasmparser::ExternalKind::Table => ExportKind::Table,
                            wasmparser::ExternalKind::Memory => ExportKind::Memory,
                            wasmparser::ExternalKind::Global => ExportKind::Global,
                            wasmparser::ExternalKind::Tag => ExportKind::Tag,
                            _ => unimplemented!(),
                        };
                        exports.export(e.name, kind, e.index);
                    }
                    let mut cur_func_id = orig_import_func_count + orig_func_count;
                    for name in &self.dummy_names {
                        exports.export(name, ExportKind::Func, cur_func_id);
                        cur_func_id += 1;
                    }
                    module.section(&exports);
                }
                wasmparser::Payload::CodeSectionStart {
                    count: _,
                    range: _,
                    size: _,
                } => {
                    let mut code = CodeSection::new();
                    let body_parser = wasmparser::Parser::new(0);
                    for payload in body_parser.parse_all(input_wasm) {
                        if let wasmparser::Payload::CodeSectionEntry(body) = payload? {
                            code.raw(&input_wasm[body.range().start..body.range().end]);
                        }
                    }
                    for _ in 0..self.dummy_names.len() {
                        let mut func = wasm_encoder::Function::new(vec![]);
                        func.instruction(&Instruction::End);
                        code.function(&func);
                    }
                    module.section(&code);
                }
                wasmparser::Payload::CustomSection(c) => {
                    module.section(&wasm_encoder::CustomSection {
                        name: c.name().into(),
                        data: std::borrow::Cow::Borrowed(c.data()),
                    });
                }
                _ => {
                    if let Some((id, range)) = payload.as_section() {
                        if id != 1 && id != 3 && id != 7 && id != 10 {
                            module.section(&RawSection {
                                id,
                                data: &input_wasm[range.clone()],
                            });
                        }
                    }
                }
            }
        }

        Ok(module.finish())
    }
}
