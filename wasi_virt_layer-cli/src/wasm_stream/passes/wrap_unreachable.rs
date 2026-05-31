use crate::wasm_stream::{
    pipeline::StreamPass,
    tracker::IndexTracker,
    translator::{
        translate, translate_global_type, translate_memory_type, translate_sub_type,
        translate_table_type, translate_tag_type, translate_val_type, DefaultRebinder, Rebind,
    },
};
use eyre::Result;
use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, Function, FunctionSection, GlobalSection,
    ImportSection, Instruction, Module, StartSection, TypeSection, ValType,
};
use wasmparser::{Payload, TypeRef};

/// Streaming pass that replaces `unreachable` instructions and hooks call sites
/// with a global flag check in a pre-target Wasm module.
///
/// This is the `wasm-encoder` equivalent of `WrapUnreachableGenerator::pre_target`.
pub struct WrapUnreachablePreTargetStreamPass {
    target_name: String,
    is_opted_in: bool,
}

impl WrapUnreachablePreTargetStreamPass {
    /// Creates a new pass for the given target.
    pub fn new(target_name: String, is_opted_in: bool) -> Self {
        Self {
            target_name,
            is_opted_in,
        }
    }
}

/// Returns a zero-valued instruction for the given Wasm value type.
fn dummy_value(ty: &wasmparser::ValType) -> Instruction<'static> {
    match ty {
        wasmparser::ValType::I32 => Instruction::I32Const(0),
        wasmparser::ValType::I64 => Instruction::I64Const(0),
        wasmparser::ValType::F32 => Instruction::F32Const(0.0_f32.into()),
        wasmparser::ValType::F64 => Instruction::F64Const(0.0_f64.into()),
        wasmparser::ValType::V128 => Instruction::V128Const(0),
        wasmparser::ValType::Ref(_) => unimplemented!("Ref types not supported for dummy returns"),
    }
}

/// Pushes dummy constants for each return type followed by a `return`.
fn push_dummy_return(func: &mut Function, return_types: &[wasmparser::ValType]) {
    for ty in return_types {
        func.instruction(&dummy_value(ty));
    }
    func.instruction(&Instruction::Return);
}

/// Rebinder that shifts function indices by the number of injected imports.
struct FuncRebinder {
    import_func_count: u32,
    shift_offset: u32,
}

impl Rebind for FuncRebinder {
    fn function(&self, index: u32) -> u32 {
        if index < self.import_func_count {
            index
        } else {
            index + self.shift_offset
        }
    }
}

impl StreamPass for WrapUnreachablePreTargetStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> Result<Vec<u8>> {
        if !self.is_opted_in {
            return Ok(input_wasm.to_vec());
        }

        println!(
            "Applying WrapUnreachable streaming pass for target: {}",
            self.target_name
        );

        let mut encoder = Module::new();

        // ── First pass: collect metadata ──────────────────────────────────
        let mut types: Vec<wasmparser::SubType> = Vec::new();
        let mut func_type_indices: Vec<u32> = Vec::new();
        let mut import_func_count: u32 = 0;
        let mut import_global_count: u32 = 0;
        let mut local_global_count: u32 = 0;

        let mut orig_main_void_idx: Option<u32> = None;
        let mut orig_thread_start_idx: Option<u32> = None;

        for payload in wasmparser::Parser::new(0).parse_all(input_wasm) {
            match payload? {
                Payload::TypeSection(s) => {
                    for rec_group in s {
                        for sub_ty in rec_group?.into_types() {
                            types.push(sub_ty);
                        }
                    }
                }
                Payload::ImportSection(s) => {
                    for group in s {
                        for item in group?.into_iter() {
                            let (_, import) = item?;
                            match import.ty {
                                TypeRef::Func(_) | TypeRef::FuncExact(_) => {
                                    import_func_count += 1
                                }
                                TypeRef::Global(_) => import_global_count += 1,
                                _ => {}
                            }
                        }
                    }
                }
                Payload::FunctionSection(s) => {
                    for f in s {
                        func_type_indices.push(f?);
                    }
                }
                Payload::GlobalSection(s) => {
                    for g in s {
                        let _ = g?;
                        local_global_count += 1;
                    }
                }
                Payload::ExportSection(s) => {
                    for e in s {
                        let e = e?;
                        if let wasmparser::ExternalKind::Func
                        | wasmparser::ExternalKind::FuncExact = e.kind
                        {
                            if e.name == "__main_void" {
                                orig_main_void_idx = Some(e.index);
                            }
                            if e.name == "wasi_thread_start" {
                                orig_thread_start_idx = Some(e.index);
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        let total_global_count = import_global_count + local_global_count;

        // ── New types ─────────────────────────────────────────────────────
        let mut type_sec = TypeSection::new();
        // Keep track of whether we found existing matching types
        let mut type_count = 0u32;
        for payload in wasmparser::Parser::new(0).parse_all(input_wasm) {
            if let Ok(Payload::TypeSection(s)) = payload {
                for rec_group in s {
                    let rec_group = rec_group?;
                    if rec_group.is_explicit_rec_group() {
                        let rec_types = rec_group
                            .into_types()
                            .map(|sub_ty| translate_sub_type(&sub_ty, &DefaultRebinder))
                            .collect::<Vec<_>>();
                        type_sec.ty().rec(rec_types);
                    } else {
                        for sub_ty in rec_group.into_types() {
                            type_sec
                                .ty()
                                .subtype(&translate_sub_type(&sub_ty, &DefaultRebinder));
                        }
                    }
                    type_count += 1;
                }
            }
        }

        // Append 4 new types
        let get_flag_ty_idx = type_count;
        type_sec.ty().function(vec![], vec![ValType::I32]); // () -> i32

        let set_flag_ty_idx = type_count + 1;
        type_sec
            .ty()
            .function(vec![ValType::I32], vec![]); // (i32) -> ()

        let fix_exit_ty_idx = type_count + 2;
        type_sec
            .ty()
            .function(vec![ValType::I32], vec![ValType::I32]); // (i32) -> i32

        let handle_exit_ty_idx = type_count + 3;
        type_sec
            .ty()
            .function(vec![ValType::I32], vec![]); // (i32) -> ()

        encoder.section(&type_sec);

        // ── Index calculations ────────────────────────────────────────────
        // We inject 2 new imports (fix_exit_code, handle_thread_exit),
        // which shifts all original function indices by 2.
        let fix_exit_import_idx = import_func_count;
        let handle_exit_import_idx = import_func_count + 1;

        let mut func_tracker = IndexTracker::new();
        func_tracker.original_count = import_func_count + func_type_indices.len() as u32;
        func_tracker.injected_count = 2;
        func_tracker.shift_offset = 2;

        let rebinder = FuncRebinder {
            import_func_count,
            shift_offset: 2,
        };

        let flag_global_idx = total_global_count; // appended at end of globals

        // New function indices (appended after all existing + shifted functions)
        let total_orig_funcs = func_tracker.original_count + func_tracker.shift_offset;
        let get_flag_idx = total_orig_funcs;
        let set_flag_idx = get_flag_idx + 1;
        let mut next_new_idx = set_flag_idx + 1;
        let main_void_wrapper_idx = orig_main_void_idx.map(|_| {
            let idx = next_new_idx;
            next_new_idx += 1;
            idx
        });
        let thread_start_wrapper_idx = orig_thread_start_idx.map(|_| {
            let idx = next_new_idx;
            next_new_idx += 1;
            idx
        });

        // ── Second pass: rewrite sections ─────────────────────────────────
        let mut code_bodies: Vec<wasmparser::FunctionBody> = Vec::new();
        let mut code_flushed = false;

        for payload in wasmparser::Parser::new(0).parse_all(input_wasm) {
            match payload? {
                Payload::TypeSection(_) => {} // Already emitted above

                Payload::ImportSection(s) => {
                    let mut import_sec = ImportSection::new();
                    for group in s {
                        for item in group?.into_iter() {
                            let (_, import) = item?;
                            let entity = match import.ty {
                                TypeRef::Func(idx) | TypeRef::FuncExact(idx) => {
                                    wasm_encoder::EntityType::Function(idx)
                                }
                                TypeRef::Table(t) => wasm_encoder::EntityType::Table(
                                    translate_table_type(t, &DefaultRebinder),
                                ),
                                TypeRef::Memory(t) => {
                                    wasm_encoder::EntityType::Memory(translate_memory_type(t))
                                }
                                TypeRef::Global(t) => wasm_encoder::EntityType::Global(
                                    translate_global_type(t, &DefaultRebinder),
                                ),
                                TypeRef::Tag(t) => {
                                    wasm_encoder::EntityType::Tag(translate_tag_type(t))
                                }
                            };
                            import_sec.import(import.module, import.name, entity);
                        }
                    }
                    // Inject the 2 new imports
                    import_sec.import(
                        "__wasip1_virt_layer",
                        &format!(
                            "__wasip1_virt_layer_{}_fix_main_raw_exit_code",
                            self.target_name
                        ),
                        wasm_encoder::EntityType::Function(fix_exit_ty_idx),
                    );
                    import_sec.import(
                        "__wasip1_virt_layer",
                        &format!(
                            "__wasip1_virt_layer_{}_handle_thread_exit",
                            self.target_name
                        ),
                        wasm_encoder::EntityType::Function(handle_exit_ty_idx),
                    );
                    encoder.section(&import_sec);
                }

                Payload::FunctionSection(s) => {
                    let mut func_sec = FunctionSection::new();
                    for f in s {
                        func_sec.function(f?);
                    }
                    // Declare new functions
                    func_sec.function(get_flag_ty_idx);
                    func_sec.function(set_flag_ty_idx);
                    if let Some(idx) = orig_main_void_idx {
                        // Wrapper has the same type as the original __main_void
                        let orig_ty_idx =
                            func_type_indices[(idx - import_func_count) as usize];
                        func_sec.function(orig_ty_idx);
                    }
                    if let Some(idx) = orig_thread_start_idx {
                        let orig_ty_idx =
                            func_type_indices[(idx - import_func_count) as usize];
                        func_sec.function(orig_ty_idx);
                    }
                    encoder.section(&func_sec);
                }

                Payload::TableSection(s) => {
                    let mut sec = wasm_encoder::TableSection::new();
                    for t in s {
                        sec.table(translate_table_type(t?.ty, &DefaultRebinder));
                    }
                    encoder.section(&sec);
                }

                Payload::MemorySection(s) => {
                    let mut sec = wasm_encoder::MemorySection::new();
                    for m in s {
                        sec.memory(translate_memory_type(m?));
                    }
                    encoder.section(&sec);
                }

                Payload::TagSection(s) => {
                    let mut sec = wasm_encoder::TagSection::new();
                    for t in s {
                        let t = t?;
                        sec.tag(wasm_encoder::TagType {
                            kind: wasm_encoder::TagKind::Exception,
                            func_type_idx: t.func_type_idx,
                        });
                    }
                    encoder.section(&sec);
                }

                Payload::GlobalSection(s) => {
                    let mut sec = GlobalSection::new();
                    for g in s {
                        let g = g?;
                        let mut instrs = Vec::new();
                        for op in g.init_expr.get_operators_reader() {
                            let op = op?;
                            if matches!(op, wasmparser::Operator::End) {
                                continue;
                            }
                            instrs.push(translate(&op, &rebinder));
                        }
                        let init_expr = wasm_encoder::ConstExpr::extended(instrs);
                        sec.global(
                            translate_global_type(g.ty, &DefaultRebinder),
                            &init_expr,
                        );
                    }
                    // Append the unreachable_flag global
                    sec.global(
                        wasm_encoder::GlobalType {
                            val_type: ValType::I32,
                            mutable: true,
                            shared: false,
                        },
                        &wasm_encoder::ConstExpr::i32_const(0),
                    );
                    encoder.section(&sec);
                }

                Payload::ExportSection(s) => {
                    let mut sec = ExportSection::new();
                    let marker_name = format!(
                        "__wasip1_virt_layer_{}_wrap_unreachable",
                        self.target_name
                    );
                    for e in s {
                        let e = e?;
                        if e.name == marker_name {
                            continue;
                        }
                        
                        let kind = match e.kind {
                            wasmparser::ExternalKind::Func
                            | wasmparser::ExternalKind::FuncExact => ExportKind::Func,
                            wasmparser::ExternalKind::Table => ExportKind::Table,
                            wasmparser::ExternalKind::Memory => ExportKind::Memory,
                            wasmparser::ExternalKind::Global => ExportKind::Global,
                            wasmparser::ExternalKind::Tag => ExportKind::Tag,
                            _ => continue,
                        };

                        let idx = match e.kind {
                            wasmparser::ExternalKind::Func
                            | wasmparser::ExternalKind::FuncExact => rebinder.function(e.index),
                            _ => e.index,
                        };

                        sec.export(e.name, kind, idx);
                    }
                    // Export getter / setter for the flag
                    sec.export(
                        &format!(
                            "__wasip1_virt_layer_{}_get_unreachable_flag",
                            self.target_name
                        ),
                        ExportKind::Func,
                        get_flag_idx,
                    );
                    sec.export(
                        &format!(
                            "__wasip1_virt_layer_{}_set_unreachable_flag",
                            self.target_name
                        ),
                        ExportKind::Func,
                        set_flag_idx,
                    );
                    encoder.section(&sec);
                }

                Payload::StartSection { func, .. } => {
                    encoder.section(&StartSection {
                        function_index: rebinder.function(func),
                    });
                }

                Payload::ElementSection(s) => {
                    let mut sec = wasm_encoder::ElementSection::new();
                    for elem in s {
                        let elem = elem?;
                        let mut funcs_vec = Vec::new();
                        let items = match elem.items {
                            wasmparser::ElementItems::Functions(f) => {
                                funcs_vec = f
                                    .into_iter()
                                    .map(|idx| Ok(rebinder.function(idx?)))
                                    .collect::<Result<Vec<u32>, eyre::Error>>()?;
                                wasm_encoder::Elements::Functions(
                                    std::borrow::Cow::Borrowed(&funcs_vec),
                                )
                            }
                            wasmparser::ElementItems::Expressions(ref_ty, exprs) => {
                                let mut const_exprs = Vec::new();
                                for expr in exprs {
                                    let mut instrs = Vec::new();
                                    for op in expr?.get_operators_reader() {
                                        let op = op?;
                                        if matches!(op, wasmparser::Operator::End) {
                                            continue;
                                        }
                                        instrs.push(translate(&op, &rebinder));
                                    }
                                    const_exprs.push(wasm_encoder::ConstExpr::extended(instrs));
                                }
                                let enc_ref_ty = crate::wasm_stream::translator::translate_ref_type(
                                    ref_ty, &DefaultRebinder,
                                );
                                wasm_encoder::Elements::Expressions(
                                    enc_ref_ty,
                                    std::borrow::Cow::Owned(const_exprs),
                                )
                            }
                        };
                        match elem.kind {
                            wasmparser::ElementKind::Passive => {
                                sec.passive(items);
                            }
                            wasmparser::ElementKind::Active {
                                table_index,
                                offset_expr,
                            } => {
                                let mut instrs = Vec::new();
                                for op in offset_expr.get_operators_reader() {
                                    let op = op?;
                                    if matches!(op, wasmparser::Operator::End) {
                                        continue;
                                    }
                                    instrs.push(translate(&op, &rebinder));
                                }
                                let offset = wasm_encoder::ConstExpr::extended(instrs);
                                sec.active(table_index, &offset, items);
                            }
                            wasmparser::ElementKind::Declared => {
                                sec.declared(items);
                            }
                        }
                    }
                    encoder.section(&sec);
                }

                Payload::DataCountSection { count, .. } => {
                    encoder.section(&wasm_encoder::DataCountSection { count });
                }

                Payload::CodeSectionStart { .. } => {}

                Payload::CodeSectionEntry(body) => {
                    code_bodies.push(body);
                }

                Payload::DataSection(s) => {
                    // Code section must be emitted before data section.
                    // Flush code bodies now.
                    if !code_flushed {
                        self.emit_code_section(
                            &mut encoder,
                            &code_bodies,
                            &types,
                            &func_type_indices,
                            import_func_count,
                            flag_global_idx,
                            &rebinder,
                            orig_main_void_idx,
                            orig_thread_start_idx,
                            fix_exit_import_idx,
                            handle_exit_import_idx,
                            main_void_wrapper_idx,
                            thread_start_wrapper_idx,
                        )?;
                        code_flushed = true;
                    }

                    let mut sec = wasm_encoder::DataSection::new();
                    for d in s {
                        let d = d?;
                        match d.kind {
                            wasmparser::DataKind::Passive => {
                                sec.passive(d.data.iter().copied());
                            }
                            wasmparser::DataKind::Active {
                                memory_index,
                                offset_expr,
                            } => {
                                let mut instrs = Vec::new();
                                for op in offset_expr.get_operators_reader() {
                                    let op = op?;
                                    if matches!(op, wasmparser::Operator::End) {
                                        continue;
                                    }
                                    instrs.push(translate(&op, &rebinder));
                                }
                                let offset = wasm_encoder::ConstExpr::extended(instrs);
                                sec.active(memory_index, &offset, d.data.iter().copied());
                            }
                        }
                    }
                    encoder.section(&sec);
                }

                Payload::CustomSection(c) => {
                    // Custom sections after code need code flushed first
                    if !code_flushed && !code_bodies.is_empty() {
                        self.emit_code_section(
                            &mut encoder,
                            &code_bodies,
                            &types,
                            &func_type_indices,
                            import_func_count,
                            flag_global_idx,
                            &rebinder,
                            orig_main_void_idx,
                            orig_thread_start_idx,
                            fix_exit_import_idx,
                            handle_exit_import_idx,
                            main_void_wrapper_idx,
                            thread_start_wrapper_idx,
                        )?;
                        code_flushed = true;
                    }
                    encoder.section(&wasm_encoder::CustomSection {
                        name: c.name().into(),
                        data: std::borrow::Cow::Borrowed(c.data()),
                    });
                }

                _ => {}
            }
        }

        // Flush code section if not already done
        if !code_flushed {
            self.emit_code_section(
                &mut encoder,
                &code_bodies,
                &types,
                &func_type_indices,
                import_func_count,
                flag_global_idx,
                &rebinder,
                orig_main_void_idx,
                orig_thread_start_idx,
                fix_exit_import_idx,
                handle_exit_import_idx,
                main_void_wrapper_idx,
                thread_start_wrapper_idx,
            )?;
        }

        Ok(encoder.finish())
    }
}

impl WrapUnreachablePreTargetStreamPass {
    /// Emits the code section with all original function bodies rewritten
    /// (unreachable replaced, call sites hooked) plus injected helper functions.
    #[allow(clippy::too_many_arguments)]
    fn emit_code_section(
        &self,
        encoder: &mut Module,
        code_bodies: &[wasmparser::FunctionBody<'_>],
        types: &[wasmparser::SubType],
        func_type_indices: &[u32],
        import_func_count: u32,
        flag_global_idx: u32,
        rebinder: &FuncRebinder,
        orig_main_void_idx: Option<u32>,
        orig_thread_start_idx: Option<u32>,
        fix_exit_import_idx: u32,
        handle_exit_import_idx: u32,
        main_void_wrapper_idx: Option<u32>,
        thread_start_wrapper_idx: Option<u32>,
    ) -> Result<()> {
        let mut code_sec = CodeSection::new();
        let mut original_main_void_func = None;
        let mut original_thread_start_func = None;

        for (body_idx, body) in code_bodies.iter().enumerate() {
            let orig_idx = body_idx as u32 + import_func_count;

            let ty_idx = func_type_indices[body_idx];
            let ty = &types[ty_idx as usize];
            let return_types = match &ty.composite_type.inner {
                wasmparser::CompositeInnerType::Func(f) => f.results().to_vec(),
                _ => vec![],
            };

            let mut locals = Vec::new();
            for local in body.get_locals_reader()? {
                let local = local?;
                locals.push((local.0, translate_val_type(local.1, rebinder)));
            }
            let mut func = Function::new(locals);

            for op in body.get_operators_reader()? {
                let op = op?;
                match op {
                    wasmparser::Operator::Unreachable => {
                        func.instruction(&Instruction::I32Const(1));
                        func.instruction(&Instruction::GlobalSet(flag_global_idx));
                        push_dummy_return(&mut func, &return_types);
                    }
                    wasmparser::Operator::Call { function_index } => {
                        func.instruction(&Instruction::Call(
                            rebinder.function(function_index),
                        ));
                        func.instruction(&Instruction::GlobalGet(flag_global_idx));
                        func.instruction(&Instruction::If(
                            wasm_encoder::BlockType::Empty,
                        ));
                        push_dummy_return(&mut func, &return_types);
                        func.instruction(&Instruction::End);
                    }
                    wasmparser::Operator::CallIndirect {
                        type_index,
                        table_index,
                        ..
                    } => {
                        func.instruction(&Instruction::CallIndirect {
                            type_index,
                            table_index,
                        });
                        func.instruction(&Instruction::GlobalGet(flag_global_idx));
                        func.instruction(&Instruction::If(
                            wasm_encoder::BlockType::Empty,
                        ));
                        push_dummy_return(&mut func, &return_types);
                        func.instruction(&Instruction::End);
                    }
                    _ => {
                        func.instruction(&translate(&op, rebinder));
                    }
                }
            }

            if Some(orig_idx) == orig_main_void_idx {
                original_main_void_func = Some(func);
                let mut wrapper = Function::new([(1, ValType::I32)]);
                wrapper.instruction(&Instruction::Call(main_void_wrapper_idx.unwrap()));
                wrapper.instruction(&Instruction::LocalSet(0));
                wrapper.instruction(&Instruction::GlobalGet(flag_global_idx));
                wrapper.instruction(&Instruction::If(wasm_encoder::BlockType::Result(
                    ValType::I32,
                )));
                wrapper.instruction(&Instruction::GlobalGet(flag_global_idx));
                wrapper.instruction(&Instruction::Call(fix_exit_import_idx));
                wrapper.instruction(&Instruction::Else);
                wrapper.instruction(&Instruction::LocalGet(0));
                wrapper.instruction(&Instruction::End);
                wrapper.instruction(&Instruction::End);
                code_sec.function(&wrapper);
            } else if Some(orig_idx) == orig_thread_start_idx {
                original_thread_start_func = Some(func);
                let mut wrapper = Function::new([]);
                wrapper.instruction(&Instruction::LocalGet(0));
                wrapper.instruction(&Instruction::LocalGet(1));
                wrapper.instruction(&Instruction::Call(thread_start_wrapper_idx.unwrap()));
                wrapper.instruction(&Instruction::GlobalGet(flag_global_idx));
                wrapper.instruction(&Instruction::If(wasm_encoder::BlockType::Empty));
                wrapper.instruction(&Instruction::GlobalGet(flag_global_idx));
                wrapper.instruction(&Instruction::Call(handle_exit_import_idx));
                wrapper.instruction(&Instruction::End);
                wrapper.instruction(&Instruction::End);
                code_sec.function(&wrapper);
            } else {
                code_sec.function(&func);
            }
        }

        // ── Injected function bodies ──────────────────────────────────────

        // get_unreachable_flag: () -> i32
        let mut get_flag_func = Function::new([]);
        get_flag_func.instruction(&Instruction::GlobalGet(flag_global_idx));
        get_flag_func.instruction(&Instruction::End);
        code_sec.function(&get_flag_func);

        // set_unreachable_flag: (i32) -> ()
        let mut set_flag_func = Function::new([]);
        set_flag_func.instruction(&Instruction::LocalGet(0));
        set_flag_func.instruction(&Instruction::GlobalSet(flag_global_idx));
        set_flag_func.instruction(&Instruction::End);
        code_sec.function(&set_flag_func);

        if main_void_wrapper_idx.is_some() {
            code_sec.function(&original_main_void_func.unwrap());
        }
        if thread_start_wrapper_idx.is_some() {
            code_sec.function(&original_thread_start_func.unwrap());
        }

        encoder.section(&code_sec);
        Ok(())
    }
}
