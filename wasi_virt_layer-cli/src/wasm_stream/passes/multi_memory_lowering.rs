use crate::wasm_stream::pipeline::{StreamPass, par_process_code_section};
use eyre::{Result, ContextCompat};
use wasmparser::{Parser, Payload, TypeRef};
use wasm_encoder::{Module, Section, RawSection, CodeSection, Function, Instruction, ValType};

#[derive(Debug, Default)]
pub struct MultiMemoryLoweringStreamPass {
    pub threads: bool,
}

impl MultiMemoryLoweringStreamPass {
    pub fn new(threads: bool) -> Self {
        Self { threads }
    }
}

impl StreamPass for MultiMemoryLoweringStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> Result<Vec<u8>> {
        let mut memory_count = 0;
        let mut total_initial = 0;
        let mut max_pages = None;
        let mut is_shared = false;
        let mut lock_acquire_fn = None;
        let mut lock_release_fn = None;
        let mut lock_write_acquire_fn = None;
        let mut lock_write_release_fn = None;

        let mut func_types = Vec::new();
        let mut types = Vec::new();
        let mut imported_funcs = 0;
        let mut global_count = 0;
        let mut memory_initials = Vec::new();
        let mut data_count = 0;

        for payload in Parser::new(0).parse_all(input_wasm) {
            match payload? {
                Payload::TypeSection(s) => {
                    for t in s {
                        for sub_ty in t?.into_types() {
                            types.push(sub_ty);
                        }
                    }
                }
                Payload::ImportSection(s) => {
                    for group in s {
                        for i in group?.into_iter() {
                            let (_, i) = i?;
                            match i.ty {
                                TypeRef::Func(_) => imported_funcs += 1,
                                TypeRef::Global(_) => global_count += 1,
                                _ => {}
                            }
                        }
                    }
                }
                Payload::FunctionSection(s) => {
                    for f in s {
                        func_types.push(f?);
                    }
                }
                Payload::GlobalSection(s) => {
                    for g in s {
                        let _ = g?;
                        global_count += 1;
                    }
                }
                Payload::MemorySection(s) => {
                    for mem in s {
                        let mem = mem?;
                        memory_count += 1;
                        memory_initials.push(mem.initial);
                        total_initial += mem.initial;
                        if let Some(max) = mem.maximum {
                            let cur_max = max_pages.unwrap_or(0);
                            max_pages = Some(std::cmp::min(cur_max + max, 65536));
                        }
                        if memory_count == 1 {
                            is_shared = mem.shared;
                        }
                    }
                }
                Payload::DataSection(s) => {
                    data_count = s.count();
                }
                Payload::ExportSection(s) => {
                    for export in s {
                        let export = export?;
                        if let wasmparser::ExternalKind::Func = export.kind {
                            match export.name {
                                "__wasip1_vfs_memory_lock_read_acquire" => lock_acquire_fn = Some(export.index),
                                "__wasip1_vfs_memory_lock_read_release" => lock_release_fn = Some(export.index),
                                "__wasip1_vfs_memory_lock_write_acquire" => lock_write_acquire_fn = Some(export.index),
                                "__wasip1_vfs_memory_lock_write_release" => lock_write_release_fn = Some(export.index),
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        if memory_count <= 1 {
            return Ok(input_wasm.to_vec());
        }

        let mut size_type_idx = None;
        let mut grow_type_idx = None;
        
        for (i, t) in types.iter().enumerate() {
            if let wasmparser::CompositeInnerType::Func(f) = &t.composite_type.inner {
                if f.params().is_empty() && f.results().len() == 1 && f.results()[0] == wasmparser::ValType::I32 {
                    size_type_idx = Some(i as u32);
                }
                if f.params().len() == 1 && f.params()[0] == wasmparser::ValType::I32 && f.results().len() == 1 && f.results()[0] == wasmparser::ValType::I32 {
                    grow_type_idx = Some(i as u32);
                }
            }
        }

        let mut encoder = Module::new();

        let threads = self.threads;

        let mut current_offset = 0;
        let mut memory_offsets = Vec::new();
        for initial in &memory_initials {
            memory_offsets.push(current_offset);
            current_offset += initial;
        }

        let orig_global_count = global_count;
        let orig_func_count = imported_funcs + func_types.len() as u32;

        let final_size_type_idx = size_type_idx.unwrap_or(types.len() as u32);
        let final_grow_type_idx = grow_type_idx.unwrap_or(if size_type_idx.is_none() { types.len() as u32 + 1 } else { types.len() as u32 });

        for payload in Parser::new(0).parse_all(input_wasm) {
            let payload = payload?;
            match payload {
                Payload::TypeSection(s) => {
                    let mut type_sec = wasm_encoder::TypeSection::new();
                    // We must re-encode types to append
                    for t in s {
                        let t = t?;
                        // For simplicity, we assume we only have Func types in this module.
                        // We can use translator
                        for sub_ty in t.into_types() {
                            match &sub_ty.composite_type.inner {
                                wasmparser::CompositeInnerType::Func(f) => {
                                    let params: Vec<_> = f.params().iter().map(|p| match p {
                                        wasmparser::ValType::I32 => wasm_encoder::ValType::I32,
                                        wasmparser::ValType::I64 => wasm_encoder::ValType::I64,
                                        wasmparser::ValType::F32 => wasm_encoder::ValType::F32,
                                        wasmparser::ValType::F64 => wasm_encoder::ValType::F64,
                                        wasmparser::ValType::V128 => wasm_encoder::ValType::V128,
                                        _ => unimplemented!(),
                                    }).collect();
                                    let results: Vec<_> = f.results().iter().map(|p| match p {
                                        wasmparser::ValType::I32 => wasm_encoder::ValType::I32,
                                        wasmparser::ValType::I64 => wasm_encoder::ValType::I64,
                                        wasmparser::ValType::F32 => wasm_encoder::ValType::F32,
                                        wasmparser::ValType::F64 => wasm_encoder::ValType::F64,
                                        wasmparser::ValType::V128 => wasm_encoder::ValType::V128,
                                        _ => unimplemented!(),
                                    }).collect();
                                    type_sec.ty().function(params, results);
                                }
                                _ => unimplemented!(),
                            }
                        }
                    }
                    if size_type_idx.is_none() {
                        type_sec.ty().function([], [wasm_encoder::ValType::I32]);
                    }
                    if grow_type_idx.is_none() {
                        type_sec.ty().function([wasm_encoder::ValType::I32], [wasm_encoder::ValType::I32]);
                    }
                    encoder.section(&type_sec);
                }
                Payload::MemorySection(_) => {
                    let mut mem_section = wasm_encoder::MemorySection::new();
                    mem_section.memory(wasm_encoder::MemoryType {
                        minimum: total_initial,
                        maximum: max_pages,
                        memory64: false,
                        shared: is_shared,
                        page_size_log2: None,
                    });
                    encoder.section(&mem_section);
                }
                Payload::GlobalSection(s) => {
                    let mut global_sec = wasm_encoder::GlobalSection::new();
                    for item in s {
                        let g = item?;
                        let ty = wasm_encoder::GlobalType {
                            val_type: match g.ty.content_type {
                                wasmparser::ValType::I32 => wasm_encoder::ValType::I32,
                                wasmparser::ValType::I64 => wasm_encoder::ValType::I64,
                                wasmparser::ValType::F32 => wasm_encoder::ValType::F32,
                                wasmparser::ValType::F64 => wasm_encoder::ValType::F64,
                                wasmparser::ValType::V128 => wasm_encoder::ValType::V128,
                                _ => unimplemented!(),
                            },
                            mutable: g.ty.mutable,
                            shared: g.ty.shared,
                        };
                        let expr = crate::wasm_stream::translator::translate_const_expr(&g.init_expr, &crate::wasm_stream::translator::DefaultRebinder)?;
                        global_sec.global(ty, &expr);
                    }
                    // Append offset globals
                    for offset in memory_offsets.iter().skip(1) {
                        global_sec.global(
                            wasm_encoder::GlobalType { val_type: wasm_encoder::ValType::I32, mutable: true, shared: false },
                            &wasm_encoder::ConstExpr::i32_const((*offset * 65536) as i32)
                        );
                    }
                    encoder.section(&global_sec);
                }
                Payload::FunctionSection(s) => {
                    let mut func_sec = wasm_encoder::FunctionSection::new();
                    for f in s {
                        func_sec.function(f?);
                    }
                    // size fns
                    for _ in 0..memory_count {
                        func_sec.function(final_size_type_idx);
                    }
                    // grow fns
                    for _ in 0..memory_count {
                        func_sec.function(final_grow_type_idx);
                    }
                    encoder.section(&func_sec);
                }
                Payload::ExportSection(s) => {
                    let mut exp_sec = wasm_encoder::ExportSection::new();
                    for e in s {
                        let e = e?;
                        let kind = match e.kind {
                            wasmparser::ExternalKind::Func => wasm_encoder::ExportKind::Func,
                            wasmparser::ExternalKind::FuncExact => wasm_encoder::ExportKind::Func,
                            wasmparser::ExternalKind::Table => wasm_encoder::ExportKind::Table,
                            wasmparser::ExternalKind::Memory => wasm_encoder::ExportKind::Memory,
                            wasmparser::ExternalKind::Global => wasm_encoder::ExportKind::Global,
                            wasmparser::ExternalKind::Tag => wasm_encoder::ExportKind::Tag,
                        };
                        // Note: If an export is memory and memory > 0, we could remap to 0. 
                        // But combined memory is 0.
                        let idx = if e.kind == wasmparser::ExternalKind::Memory { 0 } else { e.index };
                        exp_sec.export(e.name, kind, idx);
                    }
                    encoder.section(&exp_sec);
                }
                Payload::DataSection(s) => {
                    let mut data_sec = wasm_encoder::DataSection::new();
                    for d in s {
                        let d = d?;
                        match d.kind {
                            wasmparser::DataKind::Active { memory_index, offset_expr } => {
                                let mem_idx = memory_index as usize;
                                let mut expr = crate::wasm_stream::translator::translate_const_expr(&offset_expr, &crate::wasm_stream::translator::DefaultRebinder)?;
                                if mem_idx > 0 {
                                    // Replace `expr` by adding the initial offset
                                    // Actually, we can just use the global
                                    let gid = orig_global_count + mem_idx as u32 - 1;
                                    let mut new_expr = wasm_encoder::ConstExpr::global_get(gid);
                                    // Wasm-encoder doesn't allow complex const expressions yet?
                                    // Wait! ConstExpr in wasm-encoder is limited!
                                    // Actually, DataKind::Active offset can just be i32_const(val + offset * 65536)
                                    // if the original is just i32_const.
                                    // Let's assume it's just i32_const because multi-memory lowering runs at build time.
                                    let op = offset_expr.get_operators_reader().read()?;
                                    if let wasmparser::Operator::I32Const { value } = op {
                                        let new_val = value + (memory_offsets[mem_idx] * 65536) as i32;
                                        expr = wasm_encoder::ConstExpr::i32_const(new_val);
                                    } else {
                                        panic!("Data segment offset is not i32.const");
                                    }
                                }
                                data_sec.active(0, &expr, d.data.iter().copied());
                            }
                            wasmparser::DataKind::Passive => {
                                data_sec.passive(d.data.iter().copied());
                            }
                        }
                    }
                    encoder.section(&data_sec);
                }
                Payload::CodeSectionStart { count, range, size } => {
                    let reader = wasmparser::BinaryReader::new(&input_wasm[range.start..range.end], range.start);
                    let s = wasmparser::CodeSectionReader::new(reader)?;
                    let mut new_code_sec = par_process_code_section(s, |i, func_body| {
                        let func_type_idx = func_types[i] as usize;
                        let func_type = match &types[func_type_idx].composite_type.inner {
                            wasmparser::CompositeInnerType::Func(f) => f,
                            _ => unreachable!(),
                        };
                        let num_params = func_type.params().len() as u32;

                        let mut locals = Vec::new();
                        let mut locals_reader = func_body.get_locals_reader()?;
                        let mut original_locals_count = 0;
                        for _ in 0..locals_reader.get_count() {
                            let (count, ty) = locals_reader.read()?;
                            original_locals_count += count;
                            let enc_ty = match ty {
                                wasmparser::ValType::I32 => wasm_encoder::ValType::I32,
                                wasmparser::ValType::I64 => wasm_encoder::ValType::I64,
                                wasmparser::ValType::F32 => wasm_encoder::ValType::F32,
                                wasmparser::ValType::F64 => wasm_encoder::ValType::F64,
                                wasmparser::ValType::V128 => wasm_encoder::ValType::V128,
                                _ => unimplemented!("Unsupported local type {:?}", ty),
                            };
                            locals.push((count, enc_ty));
                        }
                        
                        let tmp_base = num_params + original_locals_count;
                        let tmp_addr = tmp_base;
                        let tmp_i32 = tmp_base + 1;
                        let tmp_i32_2 = tmp_base + 2;
                        let tmp_i64 = tmp_base + 3;
                        let tmp_f32 = tmp_base + 4;
                        let tmp_f64 = tmp_base + 5;
                        let tmp_v128 = tmp_base + 6;

                        locals.push((1, wasm_encoder::ValType::I32)); // tmp_addr
                        locals.push((1, wasm_encoder::ValType::I32)); // tmp_i32
                        locals.push((1, wasm_encoder::ValType::I32)); // tmp_i32_2
                        locals.push((1, wasm_encoder::ValType::I64)); // tmp_i64
                        locals.push((1, wasm_encoder::ValType::F32)); // tmp_f32
                        locals.push((1, wasm_encoder::ValType::F64)); // tmp_f64
                        locals.push((1, wasm_encoder::ValType::V128)); // tmp_v128

                        let mut func = Function::new(locals);
                        let mut reader = func_body.get_operators_reader()?;
                        while !reader.eof() {
                            let op = reader.read()?;
                            match op {
                                wasmparser::Operator::MemorySize { mem, .. } => {
                                    func.instruction(&wasm_encoder::Instruction::Call(orig_func_count + mem));
                                }
                                wasmparser::Operator::MemoryGrow { mem, .. } => {
                                    func.instruction(&wasm_encoder::Instruction::Call(orig_func_count + memory_count + mem));
                                }
                                wasmparser::Operator::MemoryCopy { dst_mem, src_mem } => {
                                    let d_idx = dst_mem as u32;
                                    let s_idx = src_mem as u32;
                                    if d_idx > 0 || s_idx > 0 {
                                        func.instruction(&wasm_encoder::Instruction::LocalSet(tmp_addr)); // len
                                        func.instruction(&wasm_encoder::Instruction::LocalSet(tmp_i32)); // src
                                        func.instruction(&wasm_encoder::Instruction::LocalSet(tmp_i32_2)); // dst

                                        func.instruction(&wasm_encoder::Instruction::LocalGet(tmp_i32_2));
                                        if d_idx > 0 {
                                            func.instruction(&wasm_encoder::Instruction::GlobalGet(orig_global_count + d_idx - 1));
                                            func.instruction(&wasm_encoder::Instruction::I32Add);
                                        }

                                        func.instruction(&wasm_encoder::Instruction::LocalGet(tmp_i32));
                                        if s_idx > 0 {
                                            func.instruction(&wasm_encoder::Instruction::GlobalGet(orig_global_count + s_idx - 1));
                                            func.instruction(&wasm_encoder::Instruction::I32Add);
                                        }

                                        func.instruction(&wasm_encoder::Instruction::LocalGet(tmp_addr));
                                        func.instruction(&wasm_encoder::Instruction::MemoryCopy { dst_mem: 0, src_mem: 0 });
                                    } else {
                                        func.instruction(&wasm_encoder::Instruction::MemoryCopy { dst_mem: 0, src_mem: 0 });
                                    }
                                }
                                wasmparser::Operator::MemoryFill { mem } => {
                                    let idx = mem as u32;
                                    if idx > 0 {
                                        func.instruction(&wasm_encoder::Instruction::LocalSet(tmp_addr)); // len
                                        func.instruction(&wasm_encoder::Instruction::LocalSet(tmp_i32)); // val
                                        func.instruction(&wasm_encoder::Instruction::LocalSet(tmp_i32_2)); // dst

                                        func.instruction(&wasm_encoder::Instruction::LocalGet(tmp_i32_2));
                                        func.instruction(&wasm_encoder::Instruction::GlobalGet(orig_global_count + idx - 1));
                                        func.instruction(&wasm_encoder::Instruction::I32Add);

                                        func.instruction(&wasm_encoder::Instruction::LocalGet(tmp_i32));
                                        func.instruction(&wasm_encoder::Instruction::LocalGet(tmp_addr));
                                        func.instruction(&wasm_encoder::Instruction::MemoryFill(0));
                                    } else {
                                        func.instruction(&wasm_encoder::Instruction::MemoryFill(0));
                                    }
                                }
                                wasmparser::Operator::MemoryInit { data_index, mem } => {
                                    let idx = mem as u32;
                                    if idx > 0 {
                                        func.instruction(&wasm_encoder::Instruction::LocalSet(tmp_addr)); // len
                                        func.instruction(&wasm_encoder::Instruction::LocalSet(tmp_i32)); // src
                                        func.instruction(&wasm_encoder::Instruction::LocalSet(tmp_i32_2)); // dst

                                        func.instruction(&wasm_encoder::Instruction::LocalGet(tmp_i32_2));
                                        func.instruction(&wasm_encoder::Instruction::GlobalGet(orig_global_count + idx - 1));
                                        func.instruction(&wasm_encoder::Instruction::I32Add);

                                        func.instruction(&wasm_encoder::Instruction::LocalGet(tmp_i32));
                                        func.instruction(&wasm_encoder::Instruction::LocalGet(tmp_addr));
                                        func.instruction(&wasm_encoder::Instruction::MemoryInit { data_index, mem: 0 });
                                    } else {
                                        func.instruction(&wasm_encoder::Instruction::MemoryInit { data_index, mem: 0 });
                                    }
                                }
                                _ => {
                                    if let Some((idx, is_store, val_ty)) = crate::wasm_stream::mem_info::memory_op_info(&op) {
                                        if idx > 0 {
                                            let offset_idx = orig_global_count + idx - 1;
                                            
                                            if !is_store {
                                                func.instruction(&wasm_encoder::Instruction::LocalSet(tmp_addr));
                                                if threads {
                                                    if let Some(acq) = lock_acquire_fn { func.instruction(&wasm_encoder::Instruction::Call(acq)); }
                                                }
                                                func.instruction(&wasm_encoder::Instruction::LocalGet(tmp_addr));
                                                func.instruction(&wasm_encoder::Instruction::GlobalGet(offset_idx));
                                                func.instruction(&wasm_encoder::Instruction::I32Add);
                                                
                                            } else {
                                                let tmp_val = match val_ty {
                                                    wasm_encoder::ValType::I32 => tmp_i32,
                                                    wasm_encoder::ValType::I64 => tmp_i64,
                                                    wasm_encoder::ValType::F32 => tmp_f32,
                                                    wasm_encoder::ValType::F64 => tmp_f64,
                                                    wasm_encoder::ValType::V128 => tmp_v128,
                                                    _ => unreachable!(),
                                                };
                                                func.instruction(&wasm_encoder::Instruction::LocalSet(tmp_val));
                                                func.instruction(&wasm_encoder::Instruction::LocalSet(tmp_addr));
                                                if threads {
                                                    if let Some(acq) = lock_acquire_fn { func.instruction(&wasm_encoder::Instruction::Call(acq)); }
                                                }
                                                func.instruction(&wasm_encoder::Instruction::LocalGet(tmp_addr));
                                                func.instruction(&wasm_encoder::Instruction::GlobalGet(offset_idx));
                                                func.instruction(&wasm_encoder::Instruction::I32Add);
                                                func.instruction(&wasm_encoder::Instruction::LocalGet(tmp_val));
                                            }
                                            
                                            let mut enc_op = crate::wasm_stream::translator::translate(&op, &crate::wasm_stream::translator::DefaultRebinder);
                                            crate::wasm_stream::mem_info::clear_memory_index(&mut enc_op);
                                            func.instruction(&enc_op);
                                            
                                            if threads {
                                                if let Some(rel) = lock_release_fn {
                                                    // if load, we need to save result, call release, push result
                                                    if !is_store {
                                                        let tmp_val = match val_ty {
                                                            wasm_encoder::ValType::I32 => tmp_i32,
                                                            wasm_encoder::ValType::I64 => tmp_i64,
                                                            wasm_encoder::ValType::F32 => tmp_f32,
                                                            wasm_encoder::ValType::F64 => tmp_f64,
                                                            wasm_encoder::ValType::V128 => tmp_v128,
                                                            _ => unreachable!(),
                                                        };
                                                        func.instruction(&wasm_encoder::Instruction::LocalSet(tmp_val));
                                                        func.instruction(&wasm_encoder::Instruction::Call(rel));
                                                        func.instruction(&wasm_encoder::Instruction::LocalGet(tmp_val));
                                                    } else {
                                                        func.instruction(&wasm_encoder::Instruction::Call(rel));
                                                    }
                                                }
                                            }
                                            continue;
                                        }
                                    }
                                    let mut enc_op = crate::wasm_stream::translator::translate(&op, &crate::wasm_stream::translator::DefaultRebinder);
                                    if let Some((_, _, _)) = crate::wasm_stream::mem_info::memory_op_info(&op) {
                                        crate::wasm_stream::mem_info::clear_memory_index(&mut enc_op);
                                    }
                                    func.instruction(&enc_op);
                                }
                            }
                        }
                        Ok(func)
                    })?;
                    
                    // Add the size and grow wrappers
                    for idx in 0..memory_count {
                        let mut func = Function::new(Vec::new());
                        if memory_count == 1 {
                            func.instruction(&Instruction::MemorySize(0));
                        } else if idx == 0 {
                            let next_gid = orig_global_count; // first offset global
                            func.instruction(&Instruction::GlobalGet(next_gid));
                            func.instruction(&Instruction::I32Const(16));
                            func.instruction(&Instruction::I32ShrU);
                        } else if idx == memory_count - 1 {
                            let this_gid = orig_global_count + idx - 1;
                            func.instruction(&Instruction::MemorySize(0));
                            func.instruction(&Instruction::GlobalGet(this_gid));
                            func.instruction(&Instruction::I32Const(16));
                            func.instruction(&Instruction::I32ShrU);
                            func.instruction(&Instruction::I32Sub);
                        } else {
                            let this_gid = orig_global_count + idx - 1;
                            let next_gid = orig_global_count + idx;
                            func.instruction(&Instruction::GlobalGet(next_gid));
                            func.instruction(&Instruction::GlobalGet(this_gid));
                            func.instruction(&Instruction::I32Sub);
                            func.instruction(&Instruction::I32Const(16));
                            func.instruction(&Instruction::I32ShrU);
                        }
                        func.instruction(&Instruction::End);
                        new_code_sec.function(&func);
                    }

                    for idx in 0..memory_count {
                        let mut func = Function::new(vec![
                            (1, wasm_encoder::ValType::I32), // return_size
                            (1, wasm_encoder::ValType::I32), // combined_size
                            (1, wasm_encoder::ValType::I32), // shift_bytes
                        ]);
                        let page_delta = 0;
                        let return_size = 1;
                        let combined_size = 2;
                        let shift_bytes = 3;

                        // return_size = memory_size_i()
                        func.instruction(&Instruction::Call(orig_func_count + idx));
                        func.instruction(&Instruction::LocalSet(return_size));

                        if threads {
                            if let Some(acq) = lock_write_acquire_fn { func.instruction(&Instruction::Call(acq)); }
                        }

                        func.instruction(&Instruction::MemorySize(0));
                        func.instruction(&Instruction::LocalSet(combined_size));

                        func.instruction(&Instruction::LocalGet(page_delta));
                        func.instruction(&Instruction::MemoryGrow(0));

                        func.instruction(&Instruction::I32Const(-1));
                        func.instruction(&Instruction::I32Eq);
                        func.instruction(&Instruction::If(wasm_encoder::BlockType::Empty));
                        if threads {
                            if let Some(rel) = lock_write_release_fn { func.instruction(&Instruction::Call(rel)); }
                        }
                        func.instruction(&Instruction::I32Const(-1));
                        func.instruction(&Instruction::Return);
                        func.instruction(&Instruction::End); // end if

                        let next_offset_global = if idx + 1 < memory_count { Some(orig_global_count + idx) } else { None };
                        if let Some(next_gid) = next_offset_global {
                            func.instruction(&Instruction::LocalGet(page_delta));
                            func.instruction(&Instruction::I32Const(16));
                            func.instruction(&Instruction::I32Shl);
                            func.instruction(&Instruction::LocalSet(shift_bytes));

                            // dst
                            func.instruction(&Instruction::GlobalGet(next_gid));
                            func.instruction(&Instruction::LocalGet(shift_bytes));
                            func.instruction(&Instruction::I32Add);
                            // src
                            func.instruction(&Instruction::GlobalGet(next_gid));
                            // len
                            func.instruction(&Instruction::LocalGet(combined_size));
                            func.instruction(&Instruction::I32Const(16));
                            func.instruction(&Instruction::I32Shl);
                            func.instruction(&Instruction::GlobalGet(next_gid));
                            func.instruction(&Instruction::I32Sub);

                            func.instruction(&Instruction::MemoryCopy { dst_mem: 0, src_mem: 0 });

                            let globals_to_update = (idx + 1)..memory_count;
                            for update_idx in globals_to_update {
                                let gid = orig_global_count + update_idx - 1;
                                func.instruction(&Instruction::GlobalGet(gid));
                                func.instruction(&Instruction::LocalGet(shift_bytes));
                                func.instruction(&Instruction::I32Add);
                                func.instruction(&Instruction::GlobalSet(gid));
                            }
                        }

                        if threads {
                            if let Some(rel) = lock_write_release_fn { func.instruction(&Instruction::Call(rel)); }
                        }

                        func.instruction(&Instruction::LocalGet(return_size));
                        func.instruction(&Instruction::End);

                        new_code_sec.function(&func);
                    }

                    encoder.section(&new_code_sec);
                }
                _ => {
                    if let Some((id, range)) = payload.as_section() {
                        encoder.section(&RawSection { id, data: &input_wasm[range.clone()] });
                    }
                }
            }
        }

        Ok(encoder.finish())
    }
}
