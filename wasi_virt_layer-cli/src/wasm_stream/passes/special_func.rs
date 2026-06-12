use crate::wasm_stream::pipeline::StreamPass;
use crate::wasm_stream::translator::{DefaultRebinder, translate_sub_type};
use std::sync::{Arc, Mutex};
use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, Function, FunctionSection, Instruction, Module,
    RawSection, TypeSection, ValType,
};

pub struct SpecialFuncPreTargetStreamPass {
    pub target_name: String,
    pub is_synthesized: Arc<Mutex<bool>>,
}

impl SpecialFuncPreTargetStreamPass {
    pub fn new(target_name: String, is_synthesized: Arc<Mutex<bool>>) -> Self {
        Self {
            target_name,
            is_synthesized,
        }
    }
}

impl StreamPass for SpecialFuncPreTargetStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> eyre::Result<Vec<u8>> {
        let mut start_func_idx = None;
        let mut has_main_void = false;

        let mut func_count = 0;
        let mut import_func_count = 0;
        let mut import_global_count = 0;
        let mut mutable_globals: Vec<(u32, i64, bool)> = Vec::new(); // index, value, is_i32

        let mut type_count = 0;
        let mut void_to_i32_type_idx = None;
        let mut void_to_void_type_idx = None;

        // Pass 1: analyze
        for payload in wasmparser::Parser::new(0).parse_all(input_wasm) {
            match payload? {
                wasmparser::Payload::TypeSection(s) => {
                    for (i, ty) in s.into_iter().enumerate() {
                        let ty = ty?;
                        for sub_ty in ty.into_types() {
                            if let wasmparser::CompositeInnerType::Func(f) =
                                &sub_ty.composite_type.inner
                            {
                                if f.params().is_empty() && f.results().is_empty() {
                                    void_to_void_type_idx = Some(i as u32);
                                } else if f.params().is_empty()
                                    && f.results().len() == 1
                                    && f.results()[0] == wasmparser::ValType::I32
                                {
                                    void_to_i32_type_idx = Some(i as u32);
                                }
                            }
                        }
                        type_count += 1;
                    }
                }
                wasmparser::Payload::FunctionSection(s) => {
                    func_count = s.count();
                }
                wasmparser::Payload::ImportSection(s) => {
                    for group in s {
                        for import in group?.into_iter() {
                            let (_, import) = import?;
                            if let wasmparser::TypeRef::Func(_) = import.ty {
                                import_func_count += 1;
                            } else if let wasmparser::TypeRef::Global(_) = import.ty {
                                import_global_count += 1;
                            }
                        }
                    }
                }
                wasmparser::Payload::GlobalSection(s) => {
                    let mut global_idx = import_global_count;
                    for global in s {
                        let global = global?;
                        if global.ty.mutable {
                            let mut reader = global.init_expr.get_operators_reader();
                            let op = reader.read()?;
                            match op {
                                wasmparser::Operator::I32Const { value } => {
                                    mutable_globals.push((global_idx, value as i64, true));
                                }
                                wasmparser::Operator::I64Const { value } => {
                                    mutable_globals.push((global_idx, value, false));
                                }
                                _ => {}
                            }
                        }
                        global_idx += 1;
                    }
                }
                wasmparser::Payload::ExportSection(s) => {
                    for export in s {
                        let export = export?;
                        if export.name == "_start" {
                            if let wasmparser::ExternalKind::Func = export.kind {
                                start_func_idx = Some(export.index);
                            }
                        } else if export.name == "__main_void" {
                            if let wasmparser::ExternalKind::Func = export.kind {
                                has_main_void = true;
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        let synthesize_main_void = !has_main_void && start_func_idx.is_some();
        if synthesize_main_void {
            *self.is_synthesized.lock().unwrap() = true;
            log::warn!(
                "Target `{}` does not export `__main_void`; its generated `_main()` entrypoint \
                 will call `_start()`. Treat `_main()` as a command start, not as a reusable \
                 main function, and do not invoke it from a secondary WASI thread unless the \
                 target explicitly supports that execution context.",
                self.target_name
            );
        }

        let mut encoder = Module::new();
        let target_name = &self.target_name;

        let final_type_idx_main_void = void_to_i32_type_idx.unwrap_or(type_count);
        let final_type_idx_reset_globals = if void_to_void_type_idx.is_none() {
            if synthesize_main_void && void_to_i32_type_idx.is_none() {
                type_count + 1
            } else {
                type_count
            }
        } else {
            void_to_void_type_idx.unwrap()
        };

        let final_main_void_idx = import_func_count + func_count;
        let final_reset_globals_idx = if synthesize_main_void {
            final_main_void_idx + 1
        } else {
            final_main_void_idx
        };

        let mut codes = CodeSection::new();
        let mut in_code_section = false;
        let mut code_flushed = false;

        for payload in wasmparser::Parser::new(0).parse_all(input_wasm) {
            match payload? {
                wasmparser::Payload::TypeSection(s) => {
                    let mut types = TypeSection::new();
                    for ty in s {
                        let ty = ty?;
                        if ty.is_explicit_rec_group() {
                            let rec_types = ty
                                .into_types()
                                .map(|sub_ty| translate_sub_type(&sub_ty, &DefaultRebinder))
                                .collect::<Vec<_>>();
                            types.ty().rec(rec_types);
                        } else {
                            for sub_ty in ty.into_types() {
                                let st = translate_sub_type(&sub_ty, &DefaultRebinder);
                                types.ty().subtype(&st);
                            }
                        }
                    }
                    if synthesize_main_void && void_to_i32_type_idx.is_none() {
                        types.ty().function(vec![], vec![ValType::I32]);
                    }
                    if void_to_void_type_idx.is_none() {
                        types.ty().function(vec![], vec![]);
                    }
                    encoder.section(&types);
                }
                wasmparser::Payload::FunctionSection(s) => {
                    let mut funcs = FunctionSection::new();
                    for f in s {
                        funcs.function(f?);
                    }
                    if synthesize_main_void {
                        funcs.function(final_type_idx_main_void);
                    }
                    funcs.function(final_type_idx_reset_globals);
                    encoder.section(&funcs);
                }
                wasmparser::Payload::ExportSection(s) => {
                    let mut exports = ExportSection::new();
                    for export in s {
                        let export = export?;
                        let mut name = export.name.to_string();
                        if name == "_start" {
                            name = format!("__wasip1_vfs_{target_name}__start");
                        } else if name == "__main_void" {
                            name = format!("__wasip1_vfs_{target_name}___main_void");
                        } else if name == "wasi_thread_start" {
                            name = format!("__wasip1_vfs_{target_name}_wasi_thread_start");
                        }

                        let kind = match export.kind {
                            wasmparser::ExternalKind::Func
                            | wasmparser::ExternalKind::FuncExact => ExportKind::Func,
                            wasmparser::ExternalKind::Table => ExportKind::Table,
                            wasmparser::ExternalKind::Memory => ExportKind::Memory,
                            wasmparser::ExternalKind::Global => ExportKind::Global,
                            wasmparser::ExternalKind::Tag => ExportKind::Tag,
                        };

                        exports.export(&name, kind, export.index);
                    }
                    if synthesize_main_void {
                        exports.export(
                            &format!("__wasip1_vfs_{target_name}___main_void"),
                            ExportKind::Func,
                            final_main_void_idx,
                        );
                    }
                    exports.export(
                        &format!("__wasip1_vfs_{target_name}_reset_globals"),
                        ExportKind::Func,
                        final_reset_globals_idx,
                    );
                    encoder.section(&exports);
                }
                wasmparser::Payload::CodeSectionStart { .. } => {
                    in_code_section = true;
                }
                wasmparser::Payload::CodeSectionEntry(body) => {
                    codes.raw(&input_wasm[body.range().start..body.range().end]);
                }
                payload => {
                    if in_code_section && !code_flushed {
                        if payload.as_section().is_some() {
                            if synthesize_main_void {
                                let mut func = Function::new([]);
                                func.instruction(&Instruction::Call(start_func_idx.unwrap()));
                                func.instruction(&Instruction::I32Const(0));
                                func.instruction(&Instruction::End);
                                codes.function(&func);
                            }
                            let mut reset_func = Function::new([]);
                            for (idx, val, is_i32) in &mutable_globals {
                                if *is_i32 {
                                    reset_func.instruction(&Instruction::I32Const(*val as i32));
                                } else {
                                    reset_func.instruction(&Instruction::I64Const(*val));
                                }
                                reset_func.instruction(&Instruction::GlobalSet(*idx));
                            }
                            reset_func.instruction(&Instruction::End);
                            codes.function(&reset_func);

                            encoder.section(&codes);
                            code_flushed = true;
                        }
                    }
                    if let Some((id, range)) = payload.as_section() {
                        encoder.section(&RawSection {
                            id,
                            data: &input_wasm[range.start..range.end],
                        });
                    }
                }
            }
        }

        if in_code_section && !code_flushed {
            if synthesize_main_void {
                let mut func = Function::new([]);
                func.instruction(&Instruction::Call(start_func_idx.unwrap()));
                func.instruction(&Instruction::I32Const(0));
                func.instruction(&Instruction::End);
                codes.function(&func);
            }
            let mut reset_func = Function::new([]);
            for (idx, val, is_i32) in &mutable_globals {
                if *is_i32 {
                    reset_func.instruction(&Instruction::I32Const(*val as i32));
                } else {
                    reset_func.instruction(&Instruction::I64Const(*val));
                }
                reset_func.instruction(&Instruction::GlobalSet(*idx));
            }
            reset_func.instruction(&Instruction::End);
            codes.function(&reset_func);

            encoder.section(&codes);
        }

        Ok(encoder.finish())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_encoder::{CodeSection, ExportSection, FunctionSection};
    use wasmparser::{ExternalKind, Operator, Payload};

    #[test]
    fn synthesized_main_void_calls_start() -> eyre::Result<()> {
        let mut module = Module::new();

        let mut types = TypeSection::new();
        types.ty().function([], []);
        module.section(&types);

        let mut functions = FunctionSection::new();
        functions.function(0);
        module.section(&functions);

        let mut exports = ExportSection::new();
        exports.export("_start", ExportKind::Func, 0);
        module.section(&exports);

        let mut code = CodeSection::new();
        let mut start = Function::new([]);
        start.instruction(&Instruction::End);
        code.function(&start);
        module.section(&code);

        let synthesized = Arc::new(Mutex::new(false));
        let output = SpecialFuncPreTargetStreamPass::new("target".to_string(), synthesized.clone())
            .run(&module.finish())?;

        let mut main_void_index = None;
        let mut bodies = Vec::new();
        for payload in wasmparser::Parser::new(0).parse_all(&output) {
            match payload? {
                Payload::ExportSection(section) => {
                    for export in section {
                        let export = export?;
                        if export.name == "__wasip1_vfs_target___main_void" {
                            assert_eq!(export.kind, ExternalKind::Func);
                            main_void_index = Some(export.index);
                        }
                    }
                }
                Payload::CodeSectionEntry(body) => {
                    bodies.push(
                        body.get_operators_reader()?
                            .into_iter()
                            .collect::<Result<Vec<_>, _>>()?,
                    );
                }
                _ => {}
            }
        }

        assert!(*synthesized.lock().unwrap());
        assert_eq!(main_void_index, Some(1));
        assert!(matches!(bodies[1][0], Operator::Call { function_index: 0 }));
        assert!(matches!(bodies[1][1], Operator::I32Const { value: 0 }));
        assert!(matches!(bodies[1][2], Operator::End));

        Ok(())
    }
}
