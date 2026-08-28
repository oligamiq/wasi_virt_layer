use crate::wasm_stream::{
    pipeline::StreamPass,
    translator::{
        DefaultRebinder, Rebind, translate, translate_global_type, translate_memory_type,
        translate_ref_type, translate_sub_type, translate_table_type, translate_tag_type,
    },
};
use eyre::Result;
use std::collections::HashMap;
use wasm_encoder::{
    CodeSection, CustomSection, EntityType, ExportKind, ExportSection, Function, FunctionSection,
    GlobalSection, ImportSection, Instruction, Module, TypeSection, ValType,
};
use wasmparser::{Parser, Payload, TypeRef};

pub const THREAD_ID_GLOBAL_SECTION: &str = "wvl.deadlock_thread_id.v1";
const VFS_IMPORT_MODULE: &str = "wasi_snapshot_preview1";
const THREAD_ENTER_IMPORT: &str = "__vfs_deadlock_thread_enter";
const THREAD_EXIT_IMPORT: &str = "__vfs_deadlock_thread_exit";
const THREAD_LIFECYCLE_IMPORTS: u32 = 2;

/// Injects a target-local wasm thread id global for deadlock detection.
pub struct DeadlockThreadIdPreTargetStreamPass {
    enabled: bool,
    target_name: Option<String>,
}

impl DeadlockThreadIdPreTargetStreamPass {
    /// Creates the pass.
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            target_name: None,
        }
    }

    /// Creates the pass for a named target after special entry exports were renamed.
    pub fn for_target(enabled: bool, target_name: impl Into<String>) -> Self {
        Self {
            enabled,
            target_name: Some(target_name.into()),
        }
    }
}

#[derive(Default)]
struct EntryExports {
    wasi_thread_start: Option<(String, u32)>,
    start: Option<(String, u32)>,
    main_void: Option<(String, u32)>,
}

fn is_wasi_thread_start_export(name: &str, target_name: Option<&str>) -> bool {
    if let Some(target_name) = target_name {
        name == format!("__wasip1_vfs_{target_name}_wasi_thread_start")
    } else {
        name == "wasi_thread_start"
    }
}

fn is_start_export(name: &str, target_name: Option<&str>) -> bool {
    if let Some(target_name) = target_name {
        name == format!("__wasip1_vfs_{target_name}__start")
    } else {
        name == "_start"
    }
}

fn is_main_void_export(name: &str, target_name: Option<&str>) -> bool {
    if let Some(target_name) = target_name {
        name == format!("__wasip1_vfs_{target_name}___main_void")
    } else {
        name == "__main_void"
    }
}

fn entity_type(ty: TypeRef) -> EntityType {
    match ty {
        TypeRef::Func(f) => EntityType::Function(f),
        TypeRef::Table(t) => EntityType::Table(translate_table_type(t, &DefaultRebinder)),
        TypeRef::Memory(m) => EntityType::Memory(translate_memory_type(m)),
        TypeRef::Global(g) => EntityType::Global(translate_global_type(g, &DefaultRebinder)),
        TypeRef::Tag(t) => EntityType::Tag(translate_tag_type(t)),
        _ => unreachable!(),
    }
}

struct LifecycleImportRebinder {
    original_func_imports: u32,
}

impl Rebind for LifecycleImportRebinder {
    fn function(&self, index: u32) -> u32 {
        if index < self.original_func_imports {
            index
        } else {
            index + THREAD_LIFECYCLE_IMPORTS
        }
    }
}

impl StreamPass for DeadlockThreadIdPreTargetStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> Result<Vec<u8>> {
        if !self.enabled {
            return Ok(input_wasm.to_vec());
        }

        let mut types = Vec::new();
        let mut local_func_types = Vec::new();
        let mut import_func_count = 0;
        let mut global_count = 0;
        let mut entries = EntryExports::default();
        let target_name = self.target_name.as_deref();

        for payload in Parser::new(0).parse_all(input_wasm) {
            match payload? {
                Payload::TypeSection(section) => {
                    for ty in section {
                        for sub_ty in ty?.into_types() {
                            types.push(sub_ty);
                        }
                    }
                }
                Payload::ImportSection(section) => {
                    for group in section {
                        for import in group? {
                            let (_, import) = import?;
                            match import.ty {
                                TypeRef::Func(_) => import_func_count += 1,
                                TypeRef::Global(_) => global_count += 1,
                                _ => {}
                            }
                        }
                    }
                }
                Payload::FunctionSection(section) => {
                    for ty in section {
                        local_func_types.push(ty?);
                    }
                }
                Payload::GlobalSection(section) => {
                    global_count += section.count();
                }
                Payload::ExportSection(section) => {
                    for export in section {
                        let export = export?;
                        if export.kind == wasmparser::ExternalKind::Func {
                            if export.index < import_func_count {
                                continue;
                            }

                            if is_wasi_thread_start_export(export.name, target_name) {
                                entries.wasi_thread_start =
                                    Some((export.name.to_string(), export.index));
                            } else if is_start_export(export.name, target_name) {
                                entries.start = Some((export.name.to_string(), export.index));
                            } else if is_main_void_export(export.name, target_name) {
                                entries.main_void = Some((export.name.to_string(), export.index));
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        let thread_id_global = global_count;
        let original_defined_count = local_func_types.len() as u32;
        let mut next_wrapper_idx =
            import_func_count + THREAD_LIFECYCLE_IMPORTS + original_defined_count;
        let mut wrapper_indices = HashMap::new();
        if let Some((name, _)) = &entries.wasi_thread_start {
            wrapper_indices.insert(name.clone(), next_wrapper_idx);
            next_wrapper_idx += 1;
        }
        if let Some((name, _)) = &entries.start {
            wrapper_indices.insert(name.clone(), next_wrapper_idx);
            next_wrapper_idx += 1;
        }
        if let Some((name, _)) = &entries.main_void {
            wrapper_indices.insert(name.clone(), next_wrapper_idx);
        }

        let mut module = Module::new();
        let mut type_section = TypeSection::new();
        for ty in &types {
            type_section
                .ty()
                .subtype(&translate_sub_type(ty, &DefaultRebinder));
        }
        let mut type_count = types.len() as u32;
        let mut get_or_add_type = |params: &[ValType], results: &[ValType]| -> u32 {
            for (idx, ty) in types.iter().enumerate() {
                if let wasmparser::CompositeInnerType::Func(func) = &ty.composite_type.inner {
                    if func.params().len() == params.len()
                        && func.results().len() == results.len()
                        && func.params().iter().zip(params).all(|(a, b)| {
                            crate::wasm_stream::translator::translate_val_type(*a, &DefaultRebinder)
                                == *b
                        })
                        && func.results().iter().zip(results).all(|(a, b)| {
                            crate::wasm_stream::translator::translate_val_type(*a, &DefaultRebinder)
                                == *b
                        })
                    {
                        return idx as u32;
                    }
                }
            }
            type_section
                .ty()
                .function(params.iter().copied(), results.iter().copied());
            let idx = type_count;
            type_count += 1;
            idx
        };

        let thread_start_ty = get_or_add_type(&[ValType::I32, ValType::I32], &[]);
        let start_ty = get_or_add_type(&[], &[]);
        let main_void_ty = get_or_add_type(&[], &[ValType::I32]);
        let lifecycle_ty = get_or_add_type(&[ValType::I32], &[]);
        module.section(&type_section);

        let rebinder = LifecycleImportRebinder {
            original_func_imports: import_func_count,
        };
        let enter_idx = import_func_count;
        let exit_idx = import_func_count + 1;

        let mut global_emitted = false;
        let mut custom_emitted = false;
        let mut imports_emitted = false;
        let emit_imports = |module: &mut Module,
                            imports_emitted: &mut bool,
                            original: Option<wasmparser::ImportSectionReader<'_>>|
         -> Result<()> {
            if !*imports_emitted {
                let mut imports = ImportSection::new();
                if let Some(section) = original {
                    for group in section {
                        for import in group? {
                            let (_, import) = import?;
                            imports.import(import.module, import.name, entity_type(import.ty));
                        }
                    }
                }
                imports.import(
                    VFS_IMPORT_MODULE,
                    THREAD_ENTER_IMPORT,
                    EntityType::Function(lifecycle_ty),
                );
                imports.import(
                    VFS_IMPORT_MODULE,
                    THREAD_EXIT_IMPORT,
                    EntityType::Function(lifecycle_ty),
                );
                module.section(&imports);
                *imports_emitted = true;
            }
            Ok(())
        };
        let emit_global = |module: &mut Module, global_emitted: &mut bool| {
            if !*global_emitted {
                let mut globals = GlobalSection::new();
                globals.global(
                    wasm_encoder::GlobalType {
                        val_type: ValType::I32,
                        mutable: true,
                        shared: false,
                    },
                    &wasm_encoder::ConstExpr::i32_const(0),
                );
                module.section(&globals);
                *global_emitted = true;
            }
        };
        let emit_custom = |module: &mut Module, custom_emitted: &mut bool| {
            if !*custom_emitted {
                module.section(&CustomSection {
                    name: THREAD_ID_GLOBAL_SECTION.into(),
                    data: std::borrow::Cow::Owned(thread_id_global.to_le_bytes().to_vec()),
                });
                *custom_emitted = true;
            }
        };

        for payload in Parser::new(0).parse_all(input_wasm) {
            let payload = payload?;
            match payload {
                Payload::TypeSection(_) => {}
                Payload::ImportSection(section) => {
                    emit_imports(&mut module, &mut imports_emitted, Some(section))?;
                }
                Payload::FunctionSection(section) => {
                    emit_imports(&mut module, &mut imports_emitted, None)?;
                    let mut functions = FunctionSection::new();
                    for ty in section {
                        functions.function(ty?);
                    }
                    if entries.wasi_thread_start.is_some() {
                        functions.function(thread_start_ty);
                    }
                    if entries.start.is_some() {
                        functions.function(start_ty);
                    }
                    if entries.main_void.is_some() {
                        functions.function(main_void_ty);
                    }
                    module.section(&functions);
                }
                Payload::TableSection(section) => {
                    let mut tables = wasm_encoder::TableSection::new();
                    for table in section {
                        tables.table(translate_table_type(table?.ty, &DefaultRebinder));
                    }
                    module.section(&tables);
                }
                Payload::MemorySection(section) => {
                    let mut memories = wasm_encoder::MemorySection::new();
                    for memory in section {
                        memories.memory(translate_memory_type(memory?));
                    }
                    module.section(&memories);
                }
                Payload::TagSection(section) => {
                    let mut tags = wasm_encoder::TagSection::new();
                    for tag in section {
                        tags.tag(translate_tag_type(tag?));
                    }
                    module.section(&tags);
                }
                Payload::GlobalSection(section) => {
                    emit_imports(&mut module, &mut imports_emitted, None)?;
                    let mut globals = GlobalSection::new();
                    for global in section {
                        let global = global?;
                        let mut instrs = Vec::new();
                        for op in global.init_expr.get_operators_reader() {
                            let op = op?;
                            if !matches!(op, wasmparser::Operator::End) {
                                instrs.push(translate(&op, &rebinder));
                            }
                        }
                        let init = wasm_encoder::ConstExpr::extended(instrs);
                        globals.global(translate_global_type(global.ty, &DefaultRebinder), &init);
                    }
                    globals.global(
                        wasm_encoder::GlobalType {
                            val_type: ValType::I32,
                            mutable: true,
                            shared: false,
                        },
                        &wasm_encoder::ConstExpr::i32_const(0),
                    );
                    module.section(&globals);
                    global_emitted = true;
                    emit_custom(&mut module, &mut custom_emitted);
                }
                Payload::ExportSection(section) => {
                    emit_imports(&mut module, &mut imports_emitted, None)?;
                    emit_global(&mut module, &mut global_emitted);
                    emit_custom(&mut module, &mut custom_emitted);
                    let mut exports = ExportSection::new();
                    for export in section {
                        let export = export?;
                        let kind = match export.kind {
                            wasmparser::ExternalKind::Func
                            | wasmparser::ExternalKind::FuncExact => ExportKind::Func,
                            wasmparser::ExternalKind::Table => ExportKind::Table,
                            wasmparser::ExternalKind::Memory => ExportKind::Memory,
                            wasmparser::ExternalKind::Global => ExportKind::Global,
                            wasmparser::ExternalKind::Tag => ExportKind::Tag,
                        };
                        let idx = if export.kind == wasmparser::ExternalKind::Func {
                            wrapper_indices
                                .get(export.name)
                                .copied()
                                .unwrap_or_else(|| rebinder.function(export.index))
                        } else {
                            export.index
                        };
                        exports.export(export.name, kind, idx);
                    }
                    module.section(&exports);
                }
                Payload::StartSection { func, .. } => {
                    emit_imports(&mut module, &mut imports_emitted, None)?;
                    emit_global(&mut module, &mut global_emitted);
                    emit_custom(&mut module, &mut custom_emitted);
                    module.section(&wasm_encoder::StartSection {
                        function_index: rebinder.function(func),
                    });
                }
                Payload::ElementSection(section) => {
                    emit_imports(&mut module, &mut imports_emitted, None)?;
                    emit_global(&mut module, &mut global_emitted);
                    emit_custom(&mut module, &mut custom_emitted);
                    let mut elements = wasm_encoder::ElementSection::new();
                    for element in section {
                        let element = element?;
                        let funcs;
                        let exprs;
                        let items = match element.items {
                            wasmparser::ElementItems::Functions(reader) => {
                                funcs = reader
                                    .into_iter()
                                    .collect::<Result<Vec<u32>, _>>()?
                                    .iter()
                                    .copied()
                                    .map(|idx| rebinder.function(idx))
                                    .collect::<Vec<_>>();
                                wasm_encoder::Elements::Functions(std::borrow::Cow::Borrowed(
                                    &funcs,
                                ))
                            }
                            wasmparser::ElementItems::Expressions(ref_ty, reader) => {
                                exprs = reader
                                    .into_iter()
                                    .map(|expr| {
                                        let mut instrs = Vec::new();
                                        for op in expr?.get_operators_reader() {
                                            let op = op?;
                                            if !matches!(op, wasmparser::Operator::End) {
                                                instrs.push(translate(&op, &rebinder));
                                            }
                                        }
                                        Ok(wasm_encoder::ConstExpr::extended(instrs))
                                    })
                                    .collect::<Result<Vec<_>, eyre::Error>>()?;
                                wasm_encoder::Elements::Expressions(
                                    translate_ref_type(ref_ty, &DefaultRebinder),
                                    std::borrow::Cow::Borrowed(&exprs),
                                )
                            }
                        };
                        match element.kind {
                            wasmparser::ElementKind::Passive => {
                                elements.passive(items);
                            }
                            wasmparser::ElementKind::Declared => {
                                elements.declared(items);
                            }
                            wasmparser::ElementKind::Active {
                                table_index,
                                offset_expr,
                            } => {
                                let mut instrs = Vec::new();
                                for op in offset_expr.get_operators_reader() {
                                    let op = op?;
                                    if !matches!(op, wasmparser::Operator::End) {
                                        instrs.push(translate(&op, &rebinder));
                                    }
                                }
                                let offset = wasm_encoder::ConstExpr::extended(instrs);
                                elements.active(table_index, &offset, items);
                            }
                        }
                    }
                    module.section(&elements);
                }
                Payload::DataCountSection { count, .. } => {
                    emit_imports(&mut module, &mut imports_emitted, None)?;
                    emit_global(&mut module, &mut global_emitted);
                    emit_custom(&mut module, &mut custom_emitted);
                    module.section(&wasm_encoder::DataCountSection { count });
                }
                Payload::CodeSectionStart { range, .. } => {
                    emit_imports(&mut module, &mut imports_emitted, None)?;
                    emit_global(&mut module, &mut global_emitted);
                    emit_custom(&mut module, &mut custom_emitted);
                    let reader = wasmparser::BinaryReader::new(
                        &input_wasm[range.start..range.end],
                        range.start,
                    );
                    let section = wasmparser::CodeSectionReader::new(reader)?;
                    let mut code = CodeSection::new();
                    for body in section {
                        let body = body?;
                        let mut locals = Vec::new();
                        for local in body.get_locals_reader()? {
                            let (count, ty) = local?;
                            locals.push((
                                count,
                                crate::wasm_stream::translator::translate_val_type(
                                    ty,
                                    &DefaultRebinder,
                                ),
                            ));
                        }
                        let mut func = Function::new(locals);
                        for op in body.get_operators_reader()? {
                            let op = op?;
                            func.instruction(&translate(&op, &rebinder));
                        }
                        code.function(&func);
                    }
                    if let Some((_, orig)) = &entries.wasi_thread_start {
                        let mut func = Function::new([]);
                        func.instruction(&Instruction::LocalGet(0));
                        func.instruction(&Instruction::GlobalSet(thread_id_global));
                        func.instruction(&Instruction::LocalGet(0));
                        func.instruction(&Instruction::Call(enter_idx));
                        func.instruction(&Instruction::LocalGet(0));
                        func.instruction(&Instruction::LocalGet(1));
                        func.instruction(&Instruction::Call(rebinder.function(*orig)));
                        func.instruction(&Instruction::LocalGet(0));
                        func.instruction(&Instruction::Call(exit_idx));
                        func.instruction(&Instruction::End);
                        code.function(&func);
                    }
                    if let Some((_, orig)) = &entries.start {
                        let mut func = Function::new([]);
                        func.instruction(&Instruction::I32Const(0));
                        func.instruction(&Instruction::GlobalSet(thread_id_global));
                        func.instruction(&Instruction::I32Const(0));
                        func.instruction(&Instruction::Call(enter_idx));
                        func.instruction(&Instruction::Call(rebinder.function(*orig)));
                        func.instruction(&Instruction::I32Const(0));
                        func.instruction(&Instruction::Call(exit_idx));
                        func.instruction(&Instruction::End);
                        code.function(&func);
                    }
                    if let Some((_, orig)) = &entries.main_void {
                        let mut func = Function::new([(1, ValType::I32)]);
                        func.instruction(&Instruction::I32Const(0));
                        func.instruction(&Instruction::GlobalSet(thread_id_global));
                        func.instruction(&Instruction::I32Const(0));
                        func.instruction(&Instruction::Call(enter_idx));
                        func.instruction(&Instruction::Call(rebinder.function(*orig)));
                        func.instruction(&Instruction::LocalSet(0));
                        func.instruction(&Instruction::I32Const(0));
                        func.instruction(&Instruction::Call(exit_idx));
                        func.instruction(&Instruction::LocalGet(0));
                        func.instruction(&Instruction::End);
                        code.function(&func);
                    }
                    module.section(&code);
                }
                Payload::DataSection(section) => {
                    emit_imports(&mut module, &mut imports_emitted, None)?;
                    emit_global(&mut module, &mut global_emitted);
                    emit_custom(&mut module, &mut custom_emitted);
                    let mut data = wasm_encoder::DataSection::new();
                    for segment in section {
                        let segment = segment?;
                        match segment.kind {
                            wasmparser::DataKind::Passive => {
                                data.passive(segment.data.iter().copied());
                            }
                            wasmparser::DataKind::Active {
                                memory_index,
                                offset_expr,
                            } => {
                                let mut instrs = Vec::new();
                                for op in offset_expr.get_operators_reader() {
                                    let op = op?;
                                    if !matches!(op, wasmparser::Operator::End) {
                                        instrs.push(translate(&op, &rebinder));
                                    }
                                }
                                let offset = wasm_encoder::ConstExpr::extended(instrs);
                                data.active(memory_index, &offset, segment.data.iter().copied());
                            }
                        }
                    }
                    module.section(&data);
                }
                Payload::CustomSection(section) => {
                    if section.name() != THREAD_ID_GLOBAL_SECTION {
                        module.section(&CustomSection {
                            name: section.name().into(),
                            data: std::borrow::Cow::Borrowed(section.data()),
                        });
                    }
                }
                Payload::CodeSectionEntry(_) => {}
                _ => {}
            }
        }

        if !global_emitted {
            emit_imports(&mut module, &mut imports_emitted, None)?;
            emit_global(&mut module, &mut global_emitted);
        }
        if !custom_emitted {
            emit_custom(&mut module, &mut custom_emitted);
        }

        Ok(module.finish())
    }
}
