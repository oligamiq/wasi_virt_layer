use crate::wasm_stream::pipeline::{par_process_code_section, StreamPass};
use eyre::Result;
use std::collections::{BTreeSet, HashMap};
use wasm_encoder::{
    CustomSection, DataCountSection, DataSection, ElementSection, EntityType,
    ExportSection, Function, FunctionSection, GlobalSection, ImportSection, Instruction, Module,
    StartSection, TypeSection, ValType,
};
use wasmparser::{Parser, Payload, TypeRef};

use crate::wasm_stream::translator::{
    translate, translate_global_type, translate_memory_type, translate_ref_type,
    translate_table_type, DefaultRebinder, Rebind,
};

pub struct AtomicPatchStreamPass {
    pub threads: bool,
    pub target_index: u32,
}

impl AtomicPatchStreamPass {
    pub fn new(threads: bool, target_index: u32) -> Self {
        Self {
            threads,
            target_index,
        }
    }
}

#[derive(Clone, Copy)]
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

impl StreamPass for AtomicPatchStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> Result<Vec<u8>> {
        if !self.threads {
            return Ok(input_wasm.to_vec());
        }

        let mut wait32_offsets = BTreeSet::new();
        let mut wait64_offsets = BTreeSet::new();
        let mut notify_offsets = BTreeSet::new();

        let mut func_types = Vec::new();
        let mut types = Vec::new();
        let mut import_func_count = 0;

        for payload in Parser::new(0).parse_all(input_wasm) {
            match payload? {
                Payload::TypeSection(s) => {
                    for t in s {
                        for sub_ty in t?.into_types() {
                            types.push(sub_ty);
                        }
                    }
                }
                Payload::FunctionSection(s) => {
                    for f in s {
                        func_types.push(f?);
                    }
                }
                Payload::ImportSection(s) => {
                    for group in s {
                        for import in group? {
                            let (_, import) = import?;
                            if let wasmparser::TypeRef::Func(f) = import.ty {
                                func_types.push(f);
                                import_func_count += 1;
                            }
                        }
                    }
                }
                Payload::CodeSectionStart { range, .. } => {
                    let reader = wasmparser::BinaryReader::new(
                        &input_wasm[range.start..range.end],
                        range.start,
                    );
                    let s = wasmparser::CodeSectionReader::new(reader)?;
                    for func_body in s {
                        let mut reader = func_body?.get_operators_reader()?;
                        while !reader.eof() {
                            match reader.read()? {
                                wasmparser::Operator::MemoryAtomicWait32 { memarg } => {
                                    wait32_offsets.insert(memarg.offset);
                                }
                                wasmparser::Operator::MemoryAtomicWait64 { memarg } => {
                                    wait64_offsets.insert(memarg.offset);
                                }
                                wasmparser::Operator::MemoryAtomicNotify { memarg } => {
                                    notify_offsets.insert(memarg.offset);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        if wait32_offsets.is_empty() && wait64_offsets.is_empty() && notify_offsets.is_empty() {
            return Ok(input_wasm.to_vec());
        }

        let mut module = Module::new();

        let mut wait32_import_ty = None;
        let mut wait64_import_ty = None;
        let mut notify_import_ty = None;

        let mut wait32_wrap_ty = None;
        let mut wait64_wrap_ty = None;
        let mut notify_wrap_ty = None;

        let mut type_sec = TypeSection::new();
        for t in &types {
            let enc_ty = crate::wasm_stream::translator::translate_sub_type(t, &DefaultRebinder);
            type_sec.ty().subtype(&enc_ty);
        }

        let mut type_count = types.len() as u32;

        let mut get_or_add_type = |params: &[ValType], results: &[ValType]| -> u32 {
            for (i, t) in types.iter().enumerate() {
                if let wasmparser::CompositeInnerType::Func(f) = &t.composite_type.inner {
                    if f.params().len() == params.len() && f.results().len() == results.len() {
                        let mut match_params = true;
                        for (a, b) in f.params().iter().zip(params.iter()) {
                            if crate::wasm_stream::translator::translate_val_type(
                                *a,
                                &DefaultRebinder,
                            ) != *b
                            {
                                match_params = false;
                                break;
                            }
                        }
                        let mut match_results = true;
                        for (a, b) in f.results().iter().zip(results.iter()) {
                            if crate::wasm_stream::translator::translate_val_type(
                                *a,
                                &DefaultRebinder,
                            ) != *b
                            {
                                match_results = false;
                                break;
                            }
                        }
                        if match_params && match_results {
                            return i as u32;
                        }
                    }
                }
            }
            type_sec
                .ty()
                .function(params.iter().cloned(), results.iter().cloned());
            let idx = type_count;
            type_count += 1;
            idx
        };

        let mut new_imports_count = 0;

        if !wait32_offsets.is_empty() {
            wait32_import_ty = Some(get_or_add_type(
                &[ValType::I32, ValType::I32, ValType::I32, ValType::I64],
                &[ValType::I32],
            ));
            wait32_wrap_ty = Some(get_or_add_type(
                &[ValType::I32, ValType::I32, ValType::I64],
                &[ValType::I32],
            ));
            new_imports_count += 1;
        }
        if !wait64_offsets.is_empty() {
            wait64_import_ty = Some(get_or_add_type(
                &[ValType::I32, ValType::I32, ValType::I64, ValType::I64],
                &[ValType::I32],
            ));
            wait64_wrap_ty = Some(get_or_add_type(
                &[ValType::I32, ValType::I64, ValType::I64],
                &[ValType::I32],
            ));
            new_imports_count += 1;
        }
        if !notify_offsets.is_empty() {
            notify_import_ty = Some(get_or_add_type(
                &[ValType::I32, ValType::I32, ValType::I32],
                &[ValType::I32],
            ));
            notify_wrap_ty = Some(get_or_add_type(
                &[ValType::I32, ValType::I32],
                &[ValType::I32],
            ));
            new_imports_count += 1;
        }

        module.section(&type_sec);

        let mut func_count = import_func_count;
        let mut wait32_import_idx = None;
        let mut wait64_import_idx = None;
        let mut notify_import_idx = None;

        let mut wait32_map = HashMap::new();
        let mut wait64_map = HashMap::new();
        let mut notify_map = HashMap::new();

        let rebinder = FuncRebinder {
            import_func_count,
            shift_offset: new_imports_count,
        };

        for payload in Parser::new(0).parse_all(input_wasm) {
            let payload = payload?;
            match &payload {
                Payload::ImportSection(s) => {
                    let mut import_sec = ImportSection::new();
                    for group in s.clone() {
                        for import in group? {
                            let (_, import) = import?;
                            let ty = match import.ty {
                                TypeRef::Func(f) => EntityType::Function(f),
                                TypeRef::Table(t) => EntityType::Table(translate_table_type(t, &DefaultRebinder)),
                                TypeRef::Memory(m) => EntityType::Memory(translate_memory_type(m)),
                                TypeRef::Global(g) => EntityType::Global(translate_global_type(g, &DefaultRebinder)),
                                TypeRef::Tag(t) => EntityType::Tag(crate::wasm_stream::translator::translate_tag_type(t)),
                                _ => unreachable!(),
                            };
                            import_sec.import(import.module, import.name, ty);
                        }
                    }
                    if let Some(ty) = wait32_import_ty {
                        import_sec.import(
                            "wasi_snapshot_preview1",
                            "__vfs_atomic_wait32",
                            EntityType::Function(ty),
                        );
                        wait32_import_idx = Some(func_count);
                        func_count += 1;
                    }
                    if let Some(ty) = wait64_import_ty {
                        import_sec.import(
                            "wasi_snapshot_preview1",
                            "__vfs_atomic_wait64",
                            EntityType::Function(ty),
                        );
                        wait64_import_idx = Some(func_count);
                        func_count += 1;
                    }
                    if let Some(ty) = notify_import_ty {
                        import_sec.import(
                            "wasi_snapshot_preview1",
                            "__vfs_atomic_notify",
                            EntityType::Function(ty),
                        );
                        notify_import_idx = Some(func_count);
                        func_count += 1;
                    }
                    module.section(&import_sec);
                }
                Payload::FunctionSection(s) => {
                    let mut func_sec = FunctionSection::new();
                    for f in s.clone() {
                        func_sec.function(f?);
                        func_count += 1;
                    }
                    for offset in &wait32_offsets {
                        func_sec.function(wait32_wrap_ty.unwrap());
                        wait32_map.insert(*offset, func_count);
                        func_count += 1;
                    }
                    for offset in &wait64_offsets {
                        func_sec.function(wait64_wrap_ty.unwrap());
                        wait64_map.insert(*offset, func_count);
                        func_count += 1;
                    }
                    for offset in &notify_offsets {
                        func_sec.function(notify_wrap_ty.unwrap());
                        notify_map.insert(*offset, func_count);
                        func_count += 1;
                    }
                    module.section(&func_sec);
                }
                Payload::TableSection(s) => {
                    let mut sec = wasm_encoder::TableSection::new();
                    for t in s.clone() {
                        sec.table(translate_table_type(t?.ty, &rebinder));
                    }
                    module.section(&sec);
                }
                Payload::MemorySection(s) => {
                    let mut sec = wasm_encoder::MemorySection::new();
                    for m in s.clone() {
                        sec.memory(translate_memory_type(m?));
                    }
                    module.section(&sec);
                }
                Payload::TagSection(s) => {
                    let mut sec = wasm_encoder::TagSection::new();
                    for t in s.clone() {
                        let t = t?;
                        sec.tag(wasm_encoder::TagType {
                            kind: wasm_encoder::TagKind::Exception,
                            func_type_idx: t.func_type_idx,
                        });
                    }
                    module.section(&sec);
                }
                Payload::GlobalSection(s) => {
                    let mut sec = GlobalSection::new();
                    for g in s.clone() {
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
                        sec.global(translate_global_type(g.ty, &rebinder), &init_expr);
                    }
                    module.section(&sec);
                }
                Payload::ExportSection(s) => {
                    let mut sec = ExportSection::new();
                    for e in s.clone() {
                        let e = e?;
                        let kind = match e.kind {
                            wasmparser::ExternalKind::Func
                            | wasmparser::ExternalKind::FuncExact => wasm_encoder::ExportKind::Func,
                            wasmparser::ExternalKind::Table => wasm_encoder::ExportKind::Table,
                            wasmparser::ExternalKind::Memory => wasm_encoder::ExportKind::Memory,
                            wasmparser::ExternalKind::Global => wasm_encoder::ExportKind::Global,
                            wasmparser::ExternalKind::Tag => wasm_encoder::ExportKind::Tag,

                        };
                        let idx = match e.kind {
                            wasmparser::ExternalKind::Func
                            | wasmparser::ExternalKind::FuncExact => rebinder.function(e.index),
                            _ => e.index,
                        };
                        sec.export(e.name, kind, idx);
                    }
                    module.section(&sec);
                }
                Payload::StartSection { func, .. } => {
                    module.section(&StartSection {
                        function_index: rebinder.function(*func),
                    });
                }
                Payload::ElementSection(s) => {
                    let mut sec = ElementSection::new();
                    for elem in s.clone() {
                        let elem = elem?;
                        let items = match elem.items {
                            wasmparser::ElementItems::Functions(f) => {
                                let funcs_vec = f
                                    .into_iter()
                                    .map(|idx| Ok(rebinder.function(idx?)))
                                    .collect::<Result<Vec<u32>, eyre::Error>>()?;
                                wasm_encoder::Elements::Functions(std::borrow::Cow::Owned(
                                    funcs_vec,
                                ))
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
                                let enc_ref_ty = translate_ref_type(ref_ty, &rebinder);
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
                    module.section(&sec);
                }
                Payload::DataCountSection { count, .. } => {
                    module.section(&DataCountSection { count: *count });
                }
                Payload::DataSection(s) => {
                    let mut sec = DataSection::new();
                    for d in s.clone() {
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
                    module.section(&sec);
                }
                Payload::CodeSectionStart { range, .. } => {
                    let reader = wasmparser::BinaryReader::new(
                        &input_wasm[range.start..range.end],
                        range.start,
                    );
                    let s = wasmparser::CodeSectionReader::new(reader)?;
                    let wait32_map_clone = wait32_map.clone();
                    let wait64_map_clone = wait64_map.clone();
                    let notify_map_clone = notify_map.clone();
                    let mut code_sec = par_process_code_section(s, move |_, func_body| {
                        let mut locals = Vec::new();
                        let mut locals_reader = func_body.get_locals_reader()?;
                        for _ in 0..locals_reader.get_count() {
                            let (count, ty) = locals_reader.read()?;
                            locals.push((
                                count,
                                crate::wasm_stream::translator::translate_val_type(ty, &rebinder),
                            ));
                        }
                        let mut func = Function::new(locals);
                        let mut reader = func_body.get_operators_reader()?;
                        while !reader.eof() {
                            let op = reader.read()?;
                            match op {
                                wasmparser::Operator::MemoryAtomicWait32 { memarg } => {
                                    func.instruction(&Instruction::Call(
                                        wait32_map_clone[&memarg.offset],
                                    ));
                                }
                                wasmparser::Operator::MemoryAtomicWait64 { memarg } => {
                                    func.instruction(&Instruction::Call(
                                        wait64_map_clone[&memarg.offset],
                                    ));
                                }
                                wasmparser::Operator::MemoryAtomicNotify { memarg } => {
                                    func.instruction(&Instruction::Call(
                                        notify_map_clone[&memarg.offset],
                                    ));
                                }
                                _ => {
                                    func.instruction(&translate(&op, &rebinder));
                                }
                            }
                        }
                        Ok(func)
                    })?;

                    let target_id = self.target_index as i32;

                    for offset in &wait32_offsets {
                        let mut func = Function::new(Vec::new());
                        func.instruction(&Instruction::I32Const(target_id));
                        func.instruction(&Instruction::LocalGet(0));
                        if *offset > 0 {
                            func.instruction(&Instruction::I32Const(*offset as i32));
                            func.instruction(&Instruction::I32Add);
                        }
                        func.instruction(&Instruction::LocalGet(1));
                        func.instruction(&Instruction::LocalGet(2));
                        func.instruction(&Instruction::Call(wait32_import_idx.unwrap()));
                        func.instruction(&Instruction::End);
                        code_sec.function(&func);
                    }
                    for offset in &wait64_offsets {
                        let mut func = Function::new(Vec::new());
                        func.instruction(&Instruction::I32Const(target_id));
                        func.instruction(&Instruction::LocalGet(0));
                        if *offset > 0 {
                            func.instruction(&Instruction::I32Const(*offset as i32));
                            func.instruction(&Instruction::I32Add);
                        }
                        func.instruction(&Instruction::LocalGet(1));
                        func.instruction(&Instruction::LocalGet(2));
                        func.instruction(&Instruction::Call(wait64_import_idx.unwrap()));
                        func.instruction(&Instruction::End);
                        code_sec.function(&func);
                    }
                    for offset in &notify_offsets {
                        let mut func = Function::new(Vec::new());
                        func.instruction(&Instruction::I32Const(target_id));
                        func.instruction(&Instruction::LocalGet(0));
                        if *offset > 0 {
                            func.instruction(&Instruction::I32Const(*offset as i32));
                            func.instruction(&Instruction::I32Add);
                        }
                        func.instruction(&Instruction::LocalGet(1));
                        func.instruction(&Instruction::Call(notify_import_idx.unwrap()));
                        func.instruction(&Instruction::End);
                        code_sec.function(&func);
                    }

                    module.section(&code_sec);
                }
                Payload::CustomSection(c) => {
                    module.section(&CustomSection {
                        name: c.name().into(),
                        data: std::borrow::Cow::Borrowed(c.data()),
                    });
                }
                _ => {}
            }
        }

        Ok(module.finish())
    }
}
