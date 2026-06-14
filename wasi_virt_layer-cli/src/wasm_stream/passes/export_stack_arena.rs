use eyre::Result;
use wasm_encoder::{
    CodeSection, ConstExpr, Function, FunctionSection, GlobalSection, Instruction, MemoryType,
    Module, RawSection, TypeSection, ValType,
};
use wasmparser::{ExternalKind, Payload, TypeRef};

use crate::wasm_stream::{
    pipeline::StreamPass,
    translator::{
        DefaultRebinder, Rebind, translate, translate_global_type, translate_sub_type,
        translate_val_type,
    },
};

struct OffsetRebinder {
    imported_funcs: u32,
    extra_funcs: u32,
}

impl Rebind for OffsetRebinder {
    fn function(&self, index: u32) -> u32 {
        if index < self.imported_funcs {
            index
        } else {
            index + self.extra_funcs
        }
    }
}

/// Inserts a fixed-size stack arena prefix into a multi-memory target module,
/// offsets all memory accesses, and virtualizes memory.size / memory.grow.
pub struct ExportStackArenaStreamPass {
    stack_size: u32,
    slots: u32,
}

impl ExportStackArenaStreamPass {
    pub const fn new(stack_size: u32, slots: u32) -> Self {
        Self { stack_size, slots }
    }

    fn arena_pages(&self) -> u32 {
        let bitmap_words = (self.slots + 31) / 32;
        let bitmap_bytes = bitmap_words * 4;
        let slots_bytes = self.stack_size * self.slots;
        (bitmap_bytes + slots_bytes).div_ceil(65536)
    }

    fn arena_offset(&self) -> u64 {
        (self.arena_pages() as u64) * 65536
    }
}

/// Temp locals used for saving value operands during address adjustment.
struct Temps {
    i32_idx: u32,
    i64_idx: u32,
    /// First additional local index (beyond orig locals + 2 temp locals)
    total_locals: u32,
}

fn tmp_local_for_type(ty: wasmparser::ValType, temps: &Temps) -> u32 {
    match ty {
        wasmparser::ValType::I32 | wasmparser::ValType::F32 => temps.i32_idx,
        wasmparser::ValType::I64 | wasmparser::ValType::F64 => temps.i64_idx,
        _ => temps.i32_idx,
    }
}

fn local_set_for(ty: ValType, temps: &Temps) -> Instruction<'static> {
    match ty {
        ValType::I32 | ValType::F32 | ValType::V128 | ValType::Ref(..) => {
            Instruction::LocalSet(temps.i32_idx)
        }
        ValType::I64 | ValType::F64 => Instruction::LocalSet(temps.i64_idx),
    }
}

fn local_get_for(ty: ValType, temps: &Temps) -> Instruction<'static> {
    match ty {
        ValType::I32 | ValType::F32 | ValType::V128 | ValType::Ref(..) => {
            Instruction::LocalGet(temps.i32_idx)
        }
        ValType::I64 | ValType::F64 => Instruction::LocalGet(temps.i64_idx),
    }
}

fn find_i32_result_type(types: &[wasmparser::SubType]) -> Option<u32> {
    types
        .iter()
        .position(|ty| {
            matches!(
                &ty.composite_type.inner,
                wasmparser::CompositeInnerType::Func(f)
                    if f.params().is_empty() && f.results() == [wasmparser::ValType::I32]
            )
        })
        .map(|i| i as u32)
}

fn find_i32_param_i32_result(types: &[wasmparser::SubType]) -> Option<u32> {
    types
        .iter()
        .position(|ty| {
            matches!(
                &ty.composite_type.inner,
                wasmparser::CompositeInnerType::Func(f)
                    if f.params() == [wasmparser::ValType::I32]
                        && f.results() == [wasmparser::ValType::I32]
            )
        })
        .map(|i| i as u32)
}

impl StreamPass for ExportStackArenaStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> Result<Vec<u8>> {
        if self.slots == 0 {
            return Ok(input_wasm.to_vec());
        }

        let arena_pages = self.arena_pages();
        let arena_offset = self.arena_offset();
        let arena_off_i32 = arena_offset as i32;

        let mut types = Vec::new();
        let mut function_types = Vec::new();
        let mut import_func_count = 0_u32;
        let mut defined_func_count = 0_u32;
        let mut mem_min = 0_u64;
        let mut mem_max = None;
        let mut mem_shared = false;
        let mut has_memory = false;

        for payload in wasmparser::Parser::new(0).parse_all(input_wasm) {
            match payload? {
                Payload::TypeSection(s) => {
                    for g in s {
                        types.extend(g?.into_types());
                    }
                }
                Payload::ImportSection(s) => {
                    for g in s {
                        for import in g?.into_iter() {
                            let (_, import) = import?;
                            match import.ty {
                                TypeRef::Func(i) | TypeRef::FuncExact(i) => {
                                    import_func_count += 1;
                                    function_types.push(i);
                                }
                                TypeRef::Memory(m) => {
                                    has_memory = true;
                                    mem_min = m.initial;
                                    mem_max = m.maximum;
                                    mem_shared = m.shared;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Payload::FunctionSection(s) => {
                    for idx in s {
                        function_types.push(idx?);
                        defined_func_count += 1;
                    }
                }
                Payload::MemorySection(s) => {
                    for m in s {
                        let m = m?;
                        has_memory = true;
                        mem_min = m.initial;
                        mem_max = m.maximum;
                        mem_shared = m.shared;
                    }
                }
                _ => {}
            }
        }

        if !has_memory {
            return Ok(input_wasm.to_vec());
        }

        let new_min = (mem_min as u64) + (arena_pages as u64);
        let new_max = mem_max.map(|m| m + (arena_pages as u64));

        let base_type_idx = types.len();
        let size_fn_type = find_i32_result_type(&types).unwrap_or_else(|| base_type_idx as u32);
        let append_size_type = size_fn_type >= base_type_idx as u32;
        let grow_fn_type = if append_size_type {
            (base_type_idx + 1) as u32
        } else {
            find_i32_param_i32_result(&types)
                .unwrap_or_else(|| (base_type_idx + usize::from(append_size_type)) as u32)
        };
        let append_grow_type = append_size_type
            || grow_fn_type >= (base_type_idx + usize::from(append_size_type)) as u32;

        let extra_funcs = 2_u32;
        let rebinder = OffsetRebinder {
            imported_funcs: import_func_count,
            extra_funcs,
        };

        let size_fn_idx = import_func_count + defined_func_count;
        let grow_fn_idx = size_fn_idx + 1;

        let mem_info_fn = crate::wasm_stream::mem_info::memory_op_info;

        let mut module = Module::new();
        let mut code_seen = false;

        for payload in wasmparser::Parser::new(0).parse_all(input_wasm) {
            let payload = payload?;
            match payload {
                Payload::TypeSection(s) => {
                    let mut out = TypeSection::new();
                    for g in s {
                        let g = g?;
                        for ty in g.into_types() {
                            out.ty().subtype(&translate_sub_type(&ty, &DefaultRebinder));
                        }
                    }
                    if append_size_type {
                        out.ty().function([], [ValType::I32]);
                    }
                    if append_grow_type {
                        out.ty().function([ValType::I32], [ValType::I32]);
                    }
                    module.section(&out);
                }
                Payload::ImportSection(s) => {
                    let mut out = wasm_encoder::ImportSection::new();
                    for g in s {
                        for import in g?.into_iter() {
                            let (_, import) = import?;
                            let entity = match import.ty {
                                TypeRef::Memory(m) => {
                                    wasm_encoder::EntityType::Memory(MemoryType {
                                        minimum: new_min,
                                        maximum: new_max,
                                        memory64: m.memory64,
                                        shared: m.shared,
                                        page_size_log2: m.page_size_log2,
                                    })
                                }
                                TypeRef::Func(i) | TypeRef::FuncExact(i) => {
                                    wasm_encoder::EntityType::Function(i)
                                }
                                TypeRef::Table(t) => wasm_encoder::EntityType::Table(
                                    crate::wasm_stream::translator::translate_table_type(
                                        t,
                                        &DefaultRebinder,
                                    ),
                                ),
                                TypeRef::Global(g) => wasm_encoder::EntityType::Global(
                                    translate_global_type(g, &DefaultRebinder),
                                ),
                                TypeRef::Tag(t) => wasm_encoder::EntityType::Tag(
                                    crate::wasm_stream::translator::translate_tag_type(t),
                                ),
                            };
                            out.import(import.module, import.name, entity);
                        }
                    }
                    module.section(&out);
                }
                Payload::FunctionSection(s) => {
                    let mut out = FunctionSection::new();
                    for f in s {
                        out.function(f?);
                    }
                    out.function(size_fn_type);
                    out.function(grow_fn_type);
                    module.section(&out);
                }
                Payload::MemorySection(s) => {
                    let mut out = wasm_encoder::MemorySection::new();
                    for m in s {
                        let m = m?;
                        out.memory(MemoryType {
                            minimum: new_min,
                            maximum: new_max,
                            memory64: m.memory64,
                            shared: m.shared,
                            page_size_log2: m.page_size_log2,
                        });
                    }
                    module.section(&out);
                }
                Payload::GlobalSection(s) => {
                    let mut out = GlobalSection::new();
                    for global in s {
                        let global = global?;
                        let mut insts = Vec::new();
                        for op in global.init_expr.get_operators_reader() {
                            let op = op?;
                            if !matches!(op, wasmparser::Operator::End) {
                                insts.push(translate(&op, &DefaultRebinder));
                            }
                        }
                        out.global(
                            translate_global_type(global.ty, &DefaultRebinder),
                            &ConstExpr::extended(insts),
                        );
                    }
                    module.section(&out);
                }
                Payload::ExportSection(s) => {
                    let mut out = wasm_encoder::ExportSection::new();
                    for export in s {
                        let export = export?;
                        let index = match export.kind {
                            ExternalKind::Func | ExternalKind::FuncExact => {
                                rebinder.function(export.index)
                            }
                            _ => export.index,
                        };
                        let kind = match export.kind {
                            ExternalKind::Func | ExternalKind::FuncExact => {
                                wasm_encoder::ExportKind::Func
                            }
                            ExternalKind::Table => wasm_encoder::ExportKind::Table,
                            ExternalKind::Memory => wasm_encoder::ExportKind::Memory,
                            ExternalKind::Global => wasm_encoder::ExportKind::Global,
                            ExternalKind::Tag => wasm_encoder::ExportKind::Tag,
                        };
                        out.export(export.name, kind, index);
                    }
                    module.section(&out);
                }
                Payload::StartSection { func, .. } => {
                    module.section(&wasm_encoder::StartSection {
                        function_index: rebinder.function(func),
                    });
                }
                Payload::ElementSection(s) => {
                    let mut out = wasm_encoder::ElementSection::new();
                    for elem in s {
                        let elem = elem?;
                        let funcs_buf;
                        let expr_storage;
                        let items = match elem.items {
                            wasmparser::ElementItems::Functions(fs) => {
                                funcs_buf = fs
                                    .into_iter()
                                    .map(|idx| Ok(rebinder.function(idx?)))
                                    .collect::<Result<Vec<_>>>()?;
                                wasm_encoder::Elements::Functions(std::borrow::Cow::Borrowed(
                                    &funcs_buf,
                                ))
                            }
                            wasmparser::ElementItems::Expressions(rt, exps) => {
                                expr_storage = exps
                                    .into_iter()
                                    .map(|e| {
                                        let e = e?;
                                        let mut insts = Vec::new();
                                        for op in e.get_operators_reader() {
                                            let op = op?;
                                            if !matches!(op, wasmparser::Operator::End) {
                                                insts.push(translate(&op, &rebinder));
                                            }
                                        }
                                        Ok(ConstExpr::extended(insts))
                                    })
                                    .collect::<Result<Vec<_>>>()?;
                                wasm_encoder::Elements::Expressions(
                                    crate::wasm_stream::translator::translate_ref_type(
                                        rt,
                                        &DefaultRebinder,
                                    ),
                                    std::borrow::Cow::Borrowed(&expr_storage),
                                )
                            }
                        };
                        match elem.kind {
                            wasmparser::ElementKind::Passive => {
                                out.passive(items);
                            }
                            wasmparser::ElementKind::Declared => {
                                out.declared(items);
                            }
                            wasmparser::ElementKind::Active {
                                table_index,
                                offset_expr,
                            } => {
                                let mut insts = Vec::new();
                                for op in offset_expr.get_operators_reader() {
                                    let op = op?;
                                    if !matches!(op, wasmparser::Operator::End) {
                                        insts.push(translate(&op, &rebinder));
                                    }
                                }
                                out.active(table_index, &ConstExpr::extended(insts), items);
                            }
                        }
                    }
                    module.section(&out);
                }
                Payload::CodeSectionStart { .. } => {
                    code_seen = true;
                    let mut out = CodeSection::new();

                    for nested in wasmparser::Parser::new(0).parse_all(input_wasm) {
                        if let Payload::CodeSectionEntry(body) = nested? {
                            let orig_locals = body
                                .get_locals_reader()?
                                .into_iter()
                                .map(|l| {
                                    let (c, t) = l?;
                                    Ok((c, translate_val_type(t, &DefaultRebinder)))
                                })
                                .collect::<Result<Vec<_>>>()?;

                            let orig_count: u32 = orig_locals.iter().map(|(c, _)| *c as u32).sum();
                            let mut locals = orig_locals.clone();
                            locals.push((1, ValType::I32)); // temp_i32
                            locals.push((1, ValType::I64)); // temp_i64
                            let temps = Temps {
                                i32_idx: orig_count,
                                i64_idx: orig_count + 1,
                                total_locals: orig_count + 2,
                            };

                            let mut func = Function::new(locals);
                            for operator in body.get_operators_reader()? {
                                let op = operator?;

                                if let Some(mem_info) = mem_info_fn(&op) {
                                    if mem_info.memory == 0 {
                                        let val_ops = &mem_info.value_operands;

                                        // Save value operands to temp locals (in reverse)
                                        for ty in val_ops.iter().rev() {
                                            func.instruction(&local_set_for(*ty, &temps));
                                        }

                                        // Add arena offset to address
                                        func.instruction(&Instruction::I32Const(arena_off_i32));
                                        func.instruction(&Instruction::I32Add);

                                        // Restore value operands from temp locals (in original order)
                                        for ty in val_ops.iter() {
                                            func.instruction(&local_get_for(*ty, &temps));
                                        }
                                    }
                                }

                                match op {
                                    wasmparser::Operator::MemorySize { mem } if mem == 0 => {
                                        func.instruction(&Instruction::Call(size_fn_idx));
                                    }
                                    wasmparser::Operator::MemoryGrow { mem } if mem == 0 => {
                                        func.instruction(&Instruction::Call(grow_fn_idx));
                                    }
                                    _ => {
                                        func.instruction(&translate(&op, &rebinder));
                                    }
                                }
                            }
                            out.function(&func);
                        }
                    }

                    let mut size_fn = Function::new([]);
                    size_fn.instruction(&Instruction::MemorySize(0));
                    size_fn.instruction(&Instruction::I32Const(arena_pages as i32));
                    size_fn.instruction(&Instruction::I32Sub);
                    size_fn.instruction(&Instruction::End);
                    out.function(&size_fn);

                    let mut grow_fn = Function::new([(1, ValType::I32)]);
                    grow_fn.instruction(&Instruction::LocalGet(0));
                    grow_fn.instruction(&Instruction::MemoryGrow(0));
                    grow_fn.instruction(&Instruction::End);
                    out.function(&grow_fn);

                    module.section(&out);
                }
                Payload::CodeSectionEntry(_) => {}
                Payload::DataSection(s) => {
                    let mut out = wasm_encoder::DataSection::new();
                    for data in s {
                        let data = data?;
                        match data.kind {
                            wasmparser::DataKind::Passive => {
                                out.passive(data.data.iter().copied());
                            }
                            wasmparser::DataKind::Active {
                                memory_index,
                                offset_expr,
                            } => {
                                let mut insts = Vec::new();
                                for op in offset_expr.get_operators_reader() {
                                    let op = op?;
                                    if let wasmparser::Operator::I32Const { value } = op {
                                        insts.push(Instruction::I32Const(
                                            (value as u64 + arena_offset) as i32,
                                        ));
                                    } else if !matches!(op, wasmparser::Operator::End) {
                                        insts.push(translate(&op, &rebinder));
                                    }
                                }
                                out.active(
                                    memory_index,
                                    &ConstExpr::extended(insts),
                                    data.data.iter().copied(),
                                );
                            }
                        }
                    }
                    module.section(&out);
                }
                Payload::CustomSection(s) => {
                    module.section(&wasm_encoder::CustomSection {
                        name: s.name().into(),
                        data: std::borrow::Cow::Borrowed(s.data()),
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

        if !code_seen {
            eyre::bail!("target has no code section");
        }

        Ok(module.finish())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_target() -> Vec<u8> {
        let mut module = Module::new();

        let mut types = TypeSection::new();
        types.ty().function([], [ValType::I32]);
        module.section(&types);

        let mut functions = FunctionSection::new();
        functions.function(0);
        functions.function(0);
        module.section(&functions);

        let mut memories = wasm_encoder::MemorySection::new();
        memories.memory(MemoryType {
            minimum: 1,
            maximum: Some(4),
            memory64: false,
            shared: false,
            page_size_log2: None,
        });
        module.section(&memories);

        let mut exports = wasm_encoder::ExportSection::new();
        exports.export("run", wasm_encoder::ExportKind::Func, 0);
        exports.export("memory", wasm_encoder::ExportKind::Memory, 0);
        module.section(&exports);

        let mut code = CodeSection::new();
        // Function 0: load from memory, return loaded value
        let mut f0 = Function::new([]);
        f0.instruction(&Instruction::I32Const(0));
        f0.instruction(&Instruction::I32Load(wasm_encoder::MemArg {
            offset: 0,
            align: 2,
            memory_index: 0,
        }));
        f0.instruction(&Instruction::End);
        code.function(&f0);
        // Function 1: store 42 at offset, return loaded value from same addr
        let mut f1 = Function::new([]);
        f1.instruction(&Instruction::I32Const(0));
        f1.instruction(&Instruction::I32Const(42));
        f1.instruction(&Instruction::I32Store(wasm_encoder::MemArg {
            offset: 0,
            align: 2,
            memory_index: 0,
        }));
        f1.instruction(&Instruction::I32Const(0));
        f1.instruction(&Instruction::I32Load(wasm_encoder::MemArg {
            offset: 0,
            align: 2,
            memory_index: 0,
        }));
        f1.instruction(&Instruction::End);
        code.function(&f1);
        module.section(&code);

        module.finish()
    }

    #[test]
    fn arena_pages_computation() {
        let pass = ExportStackArenaStreamPass::new(2 * 1024 * 1024, 32); // 2MB, 32 slots
        let arena_pages = pass.arena_pages();
        // 32 slots * 2MB + bitmap = 64MB + 4 bytes → ceil(67108868/65536) = 1025 pages
        assert_eq!(arena_pages, 1025);
    }

    #[test]
    fn arena_pass_memory_expanded() -> Result<()> {
        let mut pass = ExportStackArenaStreamPass::new(65536, 2);
        let output = pass.run(&simple_target())?;

        let mut mem_pages = 0u64;
        for payload in wasmparser::Parser::new(0).parse_all(&output) {
            if let Payload::MemorySection(s) = payload? {
                for m in s {
                    mem_pages = m?.initial;
                }
            }
        }
        // 1 original + 3 arena pages (2 slots * 64KB + 4B bitmap = 131076B → 3 pages)
        assert_eq!(mem_pages, 4);
        Ok(())
    }

    #[test]
    fn arena_pass_validates() -> Result<()> {
        let mut pass = ExportStackArenaStreamPass::new(65536, 2);
        let output = pass.run(&simple_target())?;
        wasmparser::Validator::new_with_features(wasmparser::WasmFeatures::all())
            .validate_all(&output)?;
        Ok(())
    }

    #[test]
    fn arena_pass_no_op_without_slots() -> Result<()> {
        let mut pass = ExportStackArenaStreamPass::new(65536, 0);
        let input = simple_target();
        let output = pass.run(&input)?;
        assert_eq!(output, input);
        Ok(())
    }
}
