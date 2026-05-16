//! Reimplementation of binaryen's `--multi-memory-lowering` pass using walrus.
//!
//! This generator condenses a module with multiple memories into a single
//! combined memory, adjusting all memory-related instructions accordingly.
//!
//! In threaded mode, every memory access (load/store/atomic/copy/fill) is
//! wrapped with a read-lock acquire/release pair so that a concurrent
//! `memory.grow` (which takes a write lock) cannot invalidate offsets
//! mid-access.

use std::collections::HashMap;

use eyre::OptionExt as _;
use walrus::{
    ir::*, ConstExpr, DataKind, FunctionId, GlobalId, MemoryId, Module, ValType,
};

use crate::{
    generator::{Generator, GeneratorCtx},
    util::WalrusUtilModule as _,
};

/// Information about the combined memory after lowering.
#[derive(Debug, Clone)]
pub struct LoweringResult {
    /// The single combined memory ID.
    pub combined_memory: MemoryId,
    /// Offset global IDs for each original memory (index 0 is always `None`).
    pub offset_globals: Vec<Option<GlobalId>>,
}

/// Generator that lowers multiple memories into a single memory.
///
/// Must be registered **before** `SharedGlobal` so that `SharedGlobal` can
/// consume [`LoweringResult`] to identify the offset globals precisely.
#[derive(Debug, Default)]
pub struct MultiMemoryLowering {
    /// Result of the lowering pass, available after `post_combine`.
    result: Option<LoweringResult>,
    /// Number of memories before lowering (set during post_combine).
    before_memory_count: usize,
    /// memory.size helper function IDs (one per original memory).
    memory_size_fns: Vec<FunctionId>,
    /// memory.grow helper function IDs (one per original memory).
    memory_grow_fns: Vec<FunctionId>,
    /// Map from original MemoryId to its index.
    memory_idx_map: HashMap<MemoryId, usize>,
    /// Lock acquire/release function IDs for threaded mode.
    lock_read_acquire: Option<FunctionId>,
    lock_read_release: Option<FunctionId>,
    lock_write_acquire: Option<FunctionId>,
    lock_write_release: Option<FunctionId>,
}

impl MultiMemoryLowering {
    /// Creates a new `MultiMemoryLowering` generator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the lowering result after the pass has run.
    pub fn result(&self) -> Option<&LoweringResult> {
        self.result.as_ref()
    }

    /// Main entry point: lowers multiple memories into one.
    pub fn lower_memory(
        &mut self,
        module: &mut Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        let mem_count = module.memories.iter().count();
        if mem_count <= 1 {
            log::info!("MultiMemoryLowering: skipping (only {} memory)", mem_count);
            return Ok(());
        }
        self.before_memory_count = mem_count;

        // Build index map
        self.build_memory_idx_map(module);

        // Resolve lock functions for threaded mode
        if ctx.threads {
            self.resolve_lock_fns(module)?;
        }

        // 1. Compute combined memory parameters
        let (combined_initial, combined_max, is_shared, import_info) =
            Self::compute_combined_params(module)?;

        // 2. Create offset globals
        let offset_globals = self.make_offset_globals(module)?;

        // 3. Adjust active data segment offsets
        self.adjust_data_segments(module, &offset_globals)?;

        // 4. Create memory.size helper functions
        self.create_memory_size_fns(module, &offset_globals)?;

        // 5. Create memory.grow helper functions
        self.create_memory_grow_fns(module, &offset_globals, ctx.threads)?;

        // 6. Rewrite all memory instructions
        self.rewrite_instructions(module, &offset_globals, ctx.threads)?;

        // 7. Remove old memories and add combined memory
        let combined_id = self.replace_memories(
            module,
            combined_initial,
            combined_max,
            is_shared,
            import_info,
        )?;

        // Store result
        self.result = Some(LoweringResult {
            combined_memory: combined_id,
            offset_globals,
        });

        Ok(())
    }

    /// Builds a map from MemoryId to sequential index.
    fn build_memory_idx_map(&mut self, module: &Module) {
        self.memory_idx_map.clear();
        for (i, mem) in module.memories.iter().enumerate() {
            self.memory_idx_map.insert(mem.id(), i);
        }
    }

    /// Resolves lock acquire/release functions from VFS exports.
    fn resolve_lock_fns(&mut self, module: &Module) -> eyre::Result<()> {
        for export in module.exports.iter() {
            match export.name.as_str() {
                "__wasip1_vfs_memory_lock_read_acquire" => {
                    if let walrus::ExportItem::Function(fid) = export.item {
                        self.lock_read_acquire = Some(fid);
                    }
                }
                "__wasip1_vfs_memory_lock_read_release" => {
                    if let walrus::ExportItem::Function(fid) = export.item {
                        self.lock_read_release = Some(fid);
                    }
                }
                "__wasip1_vfs_memory_lock_write_acquire" => {
                    if let walrus::ExportItem::Function(fid) = export.item {
                        self.lock_write_acquire = Some(fid);
                    }
                }
                "__wasip1_vfs_memory_lock_write_release" => {
                    if let walrus::ExportItem::Function(fid) = export.item {
                        self.lock_write_release = Some(fid);
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn compute_combined_params(
        module: &Module,
    ) -> eyre::Result<(u64, Option<u64>, bool, Option<(String, String)>)> {
        let mut total_initial: u64 = 0;
        let mut total_max: u64 = 0;
        let mut has_max = false;
        let mut is_shared = false;
        let mut import_info = None;

        for (i, mem) in module.memories.iter().enumerate() {
            total_initial += mem.initial;
            if let Some(max) = mem.maximum {
                total_max += max;
                has_max = true;
            }
            if i == 0 {
                is_shared = mem.shared;
                if let Some(import_id) = mem.import {
                    let import = module.imports.get(import_id);
                    import_info =
                        Some((import.module.to_string(), import.name.to_string()));
                }
            }
        }

        // Cap at 65536 pages (4GB) – the Wasm spec maximum for 32-bit memories.
        // When merging multiple memories the sum of their max pages can exceed this.
        let max = if has_max {
            Some(total_max.min(65536))
        } else {
            None
        };
        Ok((total_initial, max, is_shared, import_info))
    }

    /// Creates mutable i32 offset globals for each memory (except memory 0).
    fn make_offset_globals(
        &self,
        module: &mut Module,
    ) -> eyre::Result<Vec<Option<GlobalId>>> {
        let mut offset_globals = Vec::new();
        let mut running_offset: u64 = 0;

        let initials: Vec<u64> = module.memories.iter().map(|m| m.initial).collect();

        for (i, initial) in initials.into_iter().enumerate() {
            if i == 0 {
                offset_globals.push(None);
            } else {
                let byte_offset = (running_offset * 65536) as i32;
                let gid = module.globals.add_local(
                    ValType::I32,
                    true,
                    false,
                    ConstExpr::Value(Value::I32(byte_offset)),
                );
                offset_globals.push(Some(gid));
            }
            running_offset += initial;
        }
        Ok(offset_globals)
    }

    /// Adjusts active data segment offsets.
    fn adjust_data_segments(
        &self,
        module: &mut Module,
        offset_globals: &[Option<GlobalId>],
    ) -> eyre::Result<()> {
        let adjustments: Vec<_> = module
            .data
            .iter()
            .filter_map(|data| {
                if let DataKind::Active { memory, offset: _ } = &data.kind {
                    let idx = self.memory_idx_map.get(memory)?;
                    if *idx > 0 {
                        let byte_offset =
                            self.get_initial_offset_from_globals(module, offset_globals, *idx);
                        Some((data.id(), byte_offset as i32))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        for (data_id, add_offset) in adjustments {
            let data = module.data.get_mut(data_id);
            if let DataKind::Active { offset, .. } = &mut data.kind {
                if let ConstExpr::Value(Value::I32(v)) = offset {
                    *v += add_offset;
                }
            }
        }
        Ok(())
    }

    /// Returns the initial byte offset for the memory at `idx`.
    fn get_initial_offset_from_globals(
        &self,
        module: &Module,
        offset_globals: &[Option<GlobalId>],
        idx: usize,
    ) -> u64 {
        if idx == 0 {
            return 0;
        }
        if let Some(Some(gid)) = offset_globals.get(idx) {
            if let walrus::GlobalKind::Local(ConstExpr::Value(Value::I32(v))) =
                &module.globals.get(*gid).kind
            {
                return *v as u64;
            }
        }
        0
    }

    /// Creates memory.size replacement functions.
    fn create_memory_size_fns(
        &mut self,
        module: &mut Module,
        offset_globals: &[Option<GlobalId>],
    ) -> eyre::Result<()> {
        self.memory_size_fns.clear();
        let mem_count = self.before_memory_count;

        for idx in 0..mem_count {
            let is_last = idx == mem_count - 1;

            let fid = if mem_count == 1 {
                // Single memory: placeholder (will be patched)
                module.add_func(&[], &[ValType::I32], |builder, _| {
                    builder.func_body().i32_const(0);
                    Ok(())
                })?
            } else if idx == 0 {
                let next_gid = offset_globals[1]
                    .ok_or_eyre("Expected offset global for memory 1")?;
                module.add_func(&[], &[ValType::I32], |builder, _| {
                    builder
                        .func_body()
                        .global_get(next_gid)
                        .i32_const(16)
                        .binop(BinaryOp::I32ShrU);
                    Ok(())
                })?
            } else if is_last {
                let this_gid = offset_globals[idx]
                    .ok_or_eyre("Expected offset global for last memory")?;
                // Placeholder for memory.size(combined) — will need patching
                let dummy_mem_id = module.memories.iter().next().unwrap().id();
                module.add_func(&[], &[ValType::I32], move |builder, _| {
                    builder
                        .func_body()
                        .memory_size(dummy_mem_id)
                        .global_get(this_gid)
                        .i32_const(16)
                        .binop(BinaryOp::I32ShrU)
                        .binop(BinaryOp::I32Sub);
                    Ok(())
                })?
            } else {
                let this_gid = offset_globals[idx]
                    .ok_or_eyre("Expected offset global")?;
                let next_gid = offset_globals[idx + 1]
                    .ok_or_eyre("Expected next offset global")?;
                module.add_func(&[], &[ValType::I32], |builder, _| {
                    builder
                        .func_body()
                        .global_get(next_gid)
                        .global_get(this_gid)
                        .binop(BinaryOp::I32Sub)
                        .i32_const(16)
                        .binop(BinaryOp::I32ShrU);
                    Ok(())
                })?
            };

            self.memory_size_fns.push(fid);
        }
        Ok(())
    }

    /// Creates memory.grow replacement functions.
    fn create_memory_grow_fns(
        &mut self,
        module: &mut Module,
        offset_globals: &[Option<GlobalId>],
        threads: bool,
    ) -> eyre::Result<()> {
        self.memory_grow_fns.clear();
        let mem_count = self.before_memory_count;
        let size_fns = self.memory_size_fns.clone();

        for idx in 0..mem_count {
            let size_fn = size_fns[idx];

            let globals_to_update: Vec<GlobalId> = ((idx + 1)..offset_globals.len())
                .filter_map(|i| offset_globals[i])
                .collect();

            let next_offset_global = if idx + 1 < mem_count { offset_globals[idx + 1] } else { None };

            let lock_write_acquire = if threads { self.lock_write_acquire } else { None };
            let lock_write_release = if threads { self.lock_write_release } else { None };
            let dummy_mem_id = module.memories.iter().next().unwrap().id();

            let return_size = module.locals.add(ValType::I32);
            let combined_size = module.locals.add(ValType::I32);
            let shift_bytes = module.locals.add(ValType::I32);

            let fid = module.add_func(&[ValType::I32], &[ValType::I32], move |mut builder, args| {
                let page_delta = args[0];
                let mut body = builder.func_body();

                // return_size = memory_size_i()
                body.call(size_fn).local_set(return_size);

                // lock write acquire
                if let Some(acq) = lock_write_acquire { body.call(acq); }

                // combined_size = memory.size(combined)
                body.memory_size(dummy_mem_id).local_set(combined_size);

                // result = memory.grow(combined, page_delta)
                body.local_get(page_delta).memory_grow(dummy_mem_id);

                // check if grow failed
                body.i32_const(-1).binop(BinaryOp::I32Eq);
                body.if_else(None, |then| {
                    if let Some(rel) = lock_write_release { then.call(rel); }
                    then.i32_const(-1).return_();
                }, |_| {});

                // shift data
                if let Some(next_gid) = next_offset_global {
                    body.local_get(page_delta).i32_const(16).binop(BinaryOp::I32Shl).local_set(shift_bytes);

                    // memory.copy(dst = next_gid + shift_bytes, src = next_gid, len = combined_size * 65536 - next_gid)
                    // dst
                    body.global_get(next_gid).local_get(shift_bytes).binop(BinaryOp::I32Add);
                    // src
                    body.global_get(next_gid);
                    // len
                    body.local_get(combined_size).i32_const(16).binop(BinaryOp::I32Shl).global_get(next_gid).binop(BinaryOp::I32Sub);

                    body.memory_copy(dummy_mem_id, dummy_mem_id);

                    // update globals
                    for gid in &globals_to_update {
                        body.global_get(*gid).local_get(shift_bytes).binop(BinaryOp::I32Add).global_set(*gid);
                    }
                }

                if let Some(rel) = lock_write_release { body.call(rel); }

                body.local_get(return_size);
                Ok(())
            })?;

            self.memory_grow_fns.push(fid);
        }
        Ok(())
    }

    /// Rewrites all memory-related instructions.
    fn rewrite_instructions(
        &self,
        module: &mut Module,
        offset_globals: &[Option<GlobalId>],
        threads: bool,
    ) -> eyre::Result<()> {
        let memory_idx_map = self.memory_idx_map.clone();
        let grow_fns = self.memory_grow_fns.clone();
        let size_fns = self.memory_size_fns.clone();

        let func_ids: Vec<_> = module.funcs.iter_local().map(|(id, _)| id).collect();

        let skip_fids: std::collections::HashSet<_> = grow_fns
            .iter()
            .chain(size_fns.iter())
            .copied()
            .collect();

        for fid in func_ids {
            if skip_fids.contains(&fid) {
                continue;
            }

            // Pre-allocate locals for rewriting
            let tmp_i32_1 = module.locals.add(ValType::I32);
            let tmp_i32_2 = module.locals.add(ValType::I32);
            let tmp_i32_3 = module.locals.add(ValType::I32);
            let tmp_i64_1 = module.locals.add(ValType::I64);
            let tmp_i64_2 = module.locals.add(ValType::I64);
            let tmp_f32_1 = module.locals.add(ValType::F32);
            let tmp_f64_1 = module.locals.add(ValType::F64);
            let tmp_v128_1 = module.locals.add(ValType::V128);

            let func = module.funcs.get_mut(fid).kind.unwrap_local_mut();
            Self::rewrite_func_instructions(
                func,
                &memory_idx_map,
                offset_globals,
                &grow_fns,
                &size_fns,
                threads,
                self.lock_read_acquire,
                self.lock_read_release,
                [tmp_i32_1, tmp_i32_2, tmp_i32_3],
                [tmp_i64_1, tmp_i64_2],
                tmp_f32_1,
                tmp_f64_1,
                tmp_v128_1,
            )?;
        }

        Ok(())
    }

    /// Returns the ValType associated with the StoreKind.
    fn store_kind_val_type(kind: &StoreKind) -> ValType {
        match kind {
            StoreKind::I32 { .. } | StoreKind::I32_8 { .. } | StoreKind::I32_16 { .. } => ValType::I32,
            StoreKind::I64 { .. } | StoreKind::I64_8 { .. } | StoreKind::I64_16 { .. } | StoreKind::I64_32 { .. } => ValType::I64,
            StoreKind::F32 => ValType::F32,
            StoreKind::F64 => ValType::F64,
            StoreKind::V128 => ValType::V128,
        }
    }

    /// Returns the ValType associated with the LoadKind.
    fn load_kind_val_type(kind: &LoadKind) -> ValType {
        match kind {
            LoadKind::I32 { .. } | LoadKind::I32_8 { .. } | LoadKind::I32_16 { .. } => ValType::I32,
            LoadKind::I64 { .. } | LoadKind::I64_8 { .. } | LoadKind::I64_16 { .. } | LoadKind::I64_32 { .. } => ValType::I64,
            LoadKind::F32 => ValType::F32,
            LoadKind::F64 => ValType::F64,
            LoadKind::V128 => ValType::V128,
        }
    }

    /// Returns the ValType associated with the AtomicWidth.
    fn atomic_width_val_type(width: &AtomicWidth) -> ValType {
        match width {
            AtomicWidth::I32 | AtomicWidth::I32_8 | AtomicWidth::I32_16 => ValType::I32,
            AtomicWidth::I64 | AtomicWidth::I64_8 | AtomicWidth::I64_16 | AtomicWidth::I64_32 => ValType::I64,
        }
    }

    /// Rewrites memory instructions within a single function.
    fn rewrite_func_instructions(
        func: &mut walrus::LocalFunction,
        memory_idx_map: &HashMap<MemoryId, usize>,
        offset_globals: &[Option<GlobalId>],
        grow_fns: &[FunctionId],
        size_fns: &[FunctionId],
        threads: bool,
        lock_acquire: Option<FunctionId>,
        lock_release: Option<FunctionId>,
        tmp_i32: [walrus::LocalId; 3],
        tmp_i64: [walrus::LocalId; 2],
        tmp_f32: walrus::LocalId,
        tmp_f64: walrus::LocalId,
        tmp_v128: walrus::LocalId,
    ) -> eyre::Result<()> {
        let get_tmp1 = |val_type: ValType| match val_type {
            ValType::I32 => tmp_i32[0],
            ValType::I64 => tmp_i64[0],
            ValType::F32 => tmp_f32,
            ValType::F64 => tmp_f64,
            ValType::V128 => tmp_v128,
            _ => unreachable!(),
        };
        let get_tmp2 = |val_type: ValType| match val_type {
            ValType::I32 => tmp_i32[1],
            ValType::I64 => tmp_i64[1],
            _ => unreachable!(),
        };
        let get_tmp3 = |val_type: ValType| match val_type {
            ValType::I32 => tmp_i32[2],
            _ => unreachable!(),
        };

        let mut seq_ids = vec![func.entry_block()];
        let mut visited = std::collections::HashSet::new();

        while let Some(seq_id) = seq_ids.pop() {
            if !visited.insert(seq_id) {
                continue;
            }

            let seq = func.block_mut(seq_id);
            let mut new_instrs = Vec::with_capacity(seq.instrs.len() * 2);

            for (instr, loc) in std::mem::take(&mut seq.instrs) {
                // track child seqs
                match &instr {
                    Instr::Block(b) => seq_ids.push(b.seq),
                    Instr::Loop(l) => seq_ids.push(l.seq),
                    Instr::IfElse(ie) => {
                        seq_ids.push(ie.consequent);
                        seq_ids.push(ie.alternative);
                    }
                    Instr::BrIf(bi) => seq_ids.push(bi.block),
                    Instr::Br(b) => seq_ids.push(b.block),
                    Instr::BrTable(bt) => {
                        seq_ids.push(bt.default);
                        for blk in &bt.blocks {
                            seq_ids.push(*blk);
                        }
                    }
                    _ => {}
                }

                match &instr {
                    Instr::MemoryGrow(MemoryGrow { memory }) => {
                        if let Some(&idx) = memory_idx_map.get(memory) {
                            new_instrs.push((Instr::Call(Call { func: grow_fns[idx] }), loc));
                        } else {
                            new_instrs.push((instr.clone(), loc));
                        }
                    }
                    Instr::MemorySize(MemorySize { memory }) => {
                        if let Some(&idx) = memory_idx_map.get(memory) {
                            new_instrs.push((Instr::Call(Call { func: size_fns[idx] }), loc));
                        } else {
                            new_instrs.push((instr.clone(), loc));
                        }
                    }
                    Instr::Load(Load { memory, kind, .. }) => {
                        let idx = memory_idx_map.get(memory).copied().unwrap_or(0);
                        if idx > 0 {
                            if let Some(Some(offset_gid)) = offset_globals.get(idx) {
                                let tmp_addr = get_tmp1(ValType::I32);
                                new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_addr }), loc));

                                if threads {
                                    if let Some(acq) = lock_acquire {
                                        new_instrs.push((Instr::Call(Call { func: acq }), loc));
                                    }
                                }
                                new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_addr }), loc));
                                new_instrs.push((Instr::GlobalGet(GlobalGet { global: *offset_gid }), loc));
                                new_instrs.push((Instr::Binop(Binop { op: BinaryOp::I32Add }), loc));
                                new_instrs.push((instr.clone(), loc));

                                let tmp_val = get_tmp2(Self::load_kind_val_type(kind));
                                new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_val }), loc));

                                if threads {
                                    if let Some(rel) = lock_release {
                                        new_instrs.push((Instr::Call(Call { func: rel }), loc));
                                    }
                                }
                                new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_val }), loc));
                                continue;
                            }
                        }
                        new_instrs.push((instr.clone(), loc));
                    }
                    Instr::LoadSimd(LoadSimd { memory, .. }) => {
                        let idx = memory_idx_map.get(memory).copied().unwrap_or(0);
                        if idx > 0 {
                            if let Some(Some(offset_gid)) = offset_globals.get(idx) {
                                let tmp_addr = get_tmp1(ValType::I32);
                                new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_addr }), loc));

                                if threads {
                                    if let Some(acq) = lock_acquire {
                                        new_instrs.push((Instr::Call(Call { func: acq }), loc));
                                    }
                                }
                                new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_addr }), loc));
                                new_instrs.push((Instr::GlobalGet(GlobalGet { global: *offset_gid }), loc));
                                new_instrs.push((Instr::Binop(Binop { op: BinaryOp::I32Add }), loc));
                                new_instrs.push((instr.clone(), loc));

                                let tmp_val = get_tmp1(ValType::V128);
                                new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_val }), loc));

                                if threads {
                                    if let Some(rel) = lock_release {
                                        new_instrs.push((Instr::Call(Call { func: rel }), loc));
                                    }
                                }
                                new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_val }), loc));
                                continue;
                            }
                        }
                        new_instrs.push((instr.clone(), loc));
                    }
                    Instr::Store(Store { memory, kind, .. }) => {
                        let idx = memory_idx_map.get(memory).copied().unwrap_or(0);
                        if idx > 0 {
                            if let Some(Some(offset_gid)) = offset_globals.get(idx) {
                                let tmp_val = get_tmp1(Self::store_kind_val_type(kind));
                                let tmp_addr = get_tmp2(ValType::I32);
                                
                                new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_val }), loc));
                                new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_addr }), loc));

                                if threads {
                                    if let Some(acq) = lock_acquire {
                                        new_instrs.push((Instr::Call(Call { func: acq }), loc));
                                    }
                                }
                                new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_addr }), loc));
                                new_instrs.push((Instr::GlobalGet(GlobalGet { global: *offset_gid }), loc));
                                new_instrs.push((Instr::Binop(Binop { op: BinaryOp::I32Add }), loc));
                                new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_val }), loc));
                                new_instrs.push((instr.clone(), loc));

                                if threads {
                                    if let Some(rel) = lock_release {
                                        new_instrs.push((Instr::Call(Call { func: rel }), loc));
                                    }
                                }
                                continue;
                            }
                        }
                        new_instrs.push((instr.clone(), loc));
                    }

                    Instr::AtomicRmw(AtomicRmw { memory, width, .. }) => {
                        let idx = memory_idx_map.get(memory).copied().unwrap_or(0);
                        if idx > 0 {
                            if let Some(Some(offset_gid)) = offset_globals.get(idx) {
                                let tmp_val = get_tmp1(Self::atomic_width_val_type(width));
                                let tmp_addr = get_tmp2(ValType::I32);
                                
                                new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_val }), loc));
                                new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_addr }), loc));

                                if threads {
                                    if let Some(acq) = lock_acquire {
                                        new_instrs.push((Instr::Call(Call { func: acq }), loc));
                                    }
                                }
                                new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_addr }), loc));
                                new_instrs.push((Instr::GlobalGet(GlobalGet { global: *offset_gid }), loc));
                                new_instrs.push((Instr::Binop(Binop { op: BinaryOp::I32Add }), loc));
                                new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_val }), loc));
                                new_instrs.push((instr.clone(), loc));

                                let tmp_res = tmp_val;
                                new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_res }), loc));

                                if threads {
                                    if let Some(rel) = lock_release {
                                        new_instrs.push((Instr::Call(Call { func: rel }), loc));
                                    }
                                }
                                new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_res }), loc));
                                continue;
                            }
                        }
                        new_instrs.push((instr.clone(), loc));
                    }
                    Instr::Cmpxchg(Cmpxchg { memory, width, .. }) => {
                        let idx = memory_idx_map.get(memory).copied().unwrap_or(0);
                        if idx > 0 {
                            if let Some(Some(offset_gid)) = offset_globals.get(idx) {
                                let val_type = Self::atomic_width_val_type(width);
                                let tmp_repl = get_tmp1(val_type);
                                let tmp_exp = get_tmp2(val_type);
                                let tmp_addr = get_tmp3(ValType::I32);
                                
                                new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_repl }), loc));
                                new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_exp }), loc));
                                new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_addr }), loc));

                                if threads {
                                    if let Some(acq) = lock_acquire {
                                        new_instrs.push((Instr::Call(Call { func: acq }), loc));
                                    }
                                }
                                new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_addr }), loc));
                                new_instrs.push((Instr::GlobalGet(GlobalGet { global: *offset_gid }), loc));
                                new_instrs.push((Instr::Binop(Binop { op: BinaryOp::I32Add }), loc));
                                new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_exp }), loc));
                                new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_repl }), loc));
                                new_instrs.push((instr.clone(), loc));

                                let tmp_res = tmp_repl;
                                new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_res }), loc));

                                if threads {
                                    if let Some(rel) = lock_release {
                                        new_instrs.push((Instr::Call(Call { func: rel }), loc));
                                    }
                                }
                                new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_res }), loc));
                                continue;
                            }
                        }
                        new_instrs.push((instr.clone(), loc));
                    }
                    Instr::MemoryCopy(MemoryCopy { src, dst }) => {
                        let src_idx = memory_idx_map.get(src).copied().unwrap_or(0);
                        let dst_idx = memory_idx_map.get(dst).copied().unwrap_or(0);
                        let src_off = offset_globals.get(src_idx).copied().flatten();
                        let dst_off = offset_globals.get(dst_idx).copied().flatten();
                        
                        if src_off.is_some() || dst_off.is_some() {
                            let tmp_len = get_tmp1(ValType::I32);
                            let tmp_src = get_tmp2(ValType::I32);
                            let tmp_dst = get_tmp3(ValType::I32);
                            
                            new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_len }), loc));
                            new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_src }), loc));
                            new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_dst }), loc));

                            if threads {
                                if let Some(acq) = lock_acquire {
                                    new_instrs.push((Instr::Call(Call { func: acq }), loc));
                                }
                            }
                            new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_dst }), loc));
                            if let Some(d_off) = dst_off {
                                new_instrs.push((Instr::GlobalGet(GlobalGet { global: d_off }), loc));
                                new_instrs.push((Instr::Binop(Binop { op: BinaryOp::I32Add }), loc));
                            }

                            new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_src }), loc));
                            if let Some(s_off) = src_off {
                                new_instrs.push((Instr::GlobalGet(GlobalGet { global: s_off }), loc));
                                new_instrs.push((Instr::Binop(Binop { op: BinaryOp::I32Add }), loc));
                            }

                            new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_len }), loc));
                            new_instrs.push((instr.clone(), loc));

                            if threads {
                                if let Some(rel) = lock_release {
                                    new_instrs.push((Instr::Call(Call { func: rel }), loc));
                                }
                            }
                            continue;
                        }
                        new_instrs.push((instr.clone(), loc));
                    }
                    Instr::MemoryFill(MemoryFill { memory }) => {
                        let idx = memory_idx_map.get(memory).copied().unwrap_or(0);
                        if idx > 0 {
                            if let Some(Some(offset_gid)) = offset_globals.get(idx) {
                                let tmp_len = get_tmp1(ValType::I32);
                                let tmp_val = get_tmp2(ValType::I32);
                                let tmp_dst = get_tmp3(ValType::I32);
                                
                                new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_len }), loc));
                                new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_val }), loc));
                                new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_dst }), loc));

                                if threads {
                                    if let Some(acq) = lock_acquire {
                                        new_instrs.push((Instr::Call(Call { func: acq }), loc));
                                    }
                                }
                                new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_dst }), loc));
                                new_instrs.push((Instr::GlobalGet(GlobalGet { global: *offset_gid }), loc));
                                new_instrs.push((Instr::Binop(Binop { op: BinaryOp::I32Add }), loc));

                                new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_val }), loc));
                                new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_len }), loc));
                                new_instrs.push((instr.clone(), loc));

                                if threads {
                                    if let Some(rel) = lock_release {
                                        new_instrs.push((Instr::Call(Call { func: rel }), loc));
                                    }
                                }
                                continue;
                            }
                        }
                        new_instrs.push((instr.clone(), loc));
                    }
                    Instr::MemoryInit(MemoryInit { memory, data: _ }) => {
                        let idx = memory_idx_map.get(memory).copied().unwrap_or(0);
                        if idx > 0 {
                            if let Some(Some(offset_gid)) = offset_globals.get(idx) {
                                let tmp_len = get_tmp1(ValType::I32);
                                let tmp_src = get_tmp2(ValType::I32);
                                let tmp_dst = get_tmp3(ValType::I32);
                                
                                new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_len }), loc));
                                new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_src }), loc));
                                new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_dst }), loc));

                                if threads {
                                    if let Some(acq) = lock_acquire {
                                        new_instrs.push((Instr::Call(Call { func: acq }), loc));
                                    }
                                }
                                new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_dst }), loc));
                                new_instrs.push((Instr::GlobalGet(GlobalGet { global: *offset_gid }), loc));
                                new_instrs.push((Instr::Binop(Binop { op: BinaryOp::I32Add }), loc));

                                new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_src }), loc));
                                new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_len }), loc));
                                new_instrs.push((instr.clone(), loc));

                                if threads {
                                    if let Some(rel) = lock_release {
                                        new_instrs.push((Instr::Call(Call { func: rel }), loc));
                                    }
                                }
                                continue;
                            }
                        }
                        new_instrs.push((instr.clone(), loc));
                    }
                    Instr::AtomicWait(AtomicWait { memory, sixty_four, .. }) => {
                        let idx = memory_idx_map.get(memory).copied().unwrap_or(0);
                        if idx > 0 {
                            if let Some(Some(offset_gid)) = offset_globals.get(idx) {
                                let tmp_timeout = get_tmp1(ValType::I64);
                                let tmp_expected = if *sixty_four {
                                    get_tmp2(ValType::I64)
                                } else {
                                    get_tmp1(ValType::I32)
                                };
                                let tmp_addr = get_tmp2(ValType::I32);

                                new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_timeout }), loc));
                                new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_expected }), loc));
                                new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_addr }), loc));

                                if threads {
                                    if let Some(acq) = lock_acquire {
                                        new_instrs.push((Instr::Call(Call { func: acq }), loc));
                                    }
                                }
                                new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_addr }), loc));
                                new_instrs.push((Instr::GlobalGet(GlobalGet { global: *offset_gid }), loc));
                                new_instrs.push((Instr::Binop(Binop { op: BinaryOp::I32Add }), loc));

                                new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_expected }), loc));
                                new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_timeout }), loc));
                                new_instrs.push((instr.clone(), loc));

                                let tmp_res = get_tmp3(ValType::I32);
                                new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_res }), loc));

                                if threads {
                                    if let Some(rel) = lock_release {
                                        new_instrs.push((Instr::Call(Call { func: rel }), loc));
                                    }
                                }
                                new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_res }), loc));
                                continue;
                            }
                        }
                        new_instrs.push((instr.clone(), loc));
                    }
                    Instr::AtomicNotify(AtomicNotify { memory, .. }) => {
                        let idx = memory_idx_map.get(memory).copied().unwrap_or(0);
                        if idx > 0 {
                            if let Some(Some(offset_gid)) = offset_globals.get(idx) {
                                let tmp_count = get_tmp1(ValType::I32);
                                let tmp_addr = get_tmp2(ValType::I32);

                                new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_count }), loc));
                                new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_addr }), loc));

                                if threads {
                                    if let Some(acq) = lock_acquire {
                                        new_instrs.push((Instr::Call(Call { func: acq }), loc));
                                    }
                                }
                                new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_addr }), loc));
                                new_instrs.push((Instr::GlobalGet(GlobalGet { global: *offset_gid }), loc));
                                new_instrs.push((Instr::Binop(Binop { op: BinaryOp::I32Add }), loc));

                                new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_count }), loc));
                                new_instrs.push((instr.clone(), loc));

                                let tmp_res = tmp_count;
                                new_instrs.push((Instr::LocalSet(LocalSet { local: tmp_res }), loc));

                                if threads {
                                    if let Some(rel) = lock_release {
                                        new_instrs.push((Instr::Call(Call { func: rel }), loc));
                                    }
                                }
                                new_instrs.push((Instr::LocalGet(LocalGet { local: tmp_res }), loc));
                                continue;
                            }
                        }
                        new_instrs.push((instr.clone(), loc));
                    }
                    _ => {
                        new_instrs.push((instr.clone(), loc));
                    }
                }
            }
            func.block_mut(seq_id).instrs = new_instrs;
        }

        Ok(())
    }

    /// Removes all existing memories and adds the combined memory.
    fn replace_memories(
        &self,
        module: &mut Module,
        initial: u64,
        max: Option<u64>,
        shared: bool,
        import_info: Option<(String, String)>,
    ) -> eyre::Result<MemoryId> {
        let mem_ids: Vec<MemoryId> = module.memories.iter().map(|m| m.id()).collect();

        // Remove memory exports
        let export_ids: Vec<_> = module
            .exports
            .iter()
            .filter(|e| matches!(e.item, walrus::ExportItem::Memory(_)))
            .map(|e| e.id())
            .collect();
        let had_exports = !export_ids.is_empty();
        for eid in export_ids {
            module.exports.delete(eid);
        }

        // Remove memory imports
        let import_ids: Vec<_> = module
            .imports
            .iter()
            .filter(|i| matches!(i.kind, walrus::ImportKind::Memory(_)))
            .map(|i| i.id())
            .collect();
        for iid in import_ids {
            module.imports.delete(iid);
        }

        // Remove old memories
        for mid in &mem_ids {
            module.memories.delete(*mid);
        }

        // Add combined memory
        let combined = module.memories.add_local(shared, false, initial, max, None);

        if had_exports {
            module.exports.add("memory", combined);
        }

        if let Some((mod_name, base_name)) = import_info {
            module.imports.add(&mod_name, &base_name, combined);
        }
        
        // Fix up ALL functions: all memory references must point to the combined memory.
        // rewrite_instructions already adjusted offsets for non-zero memories but left
        // the MemoryId fields unchanged. Those old IDs are now deleted, so we need to
        // remap every memory reference to the new combined memory.
        let all_local_fids: Vec<_> = module.funcs.iter_local().map(|(id, _)| id).collect();
        for fid in all_local_fids {
            if let walrus::FunctionKind::Local(func) = &mut module.funcs.get_mut(fid).kind {
                let mut seq_ids = vec![func.entry_block()];
                let mut visited = std::collections::HashSet::new();

                while let Some(seq_id) = seq_ids.pop() {
                    if !visited.insert(seq_id) { continue; }
                    let seq = func.block_mut(seq_id);
                    for (instr, _) in &mut seq.instrs {
                        match instr {
                            Instr::Block(b) => seq_ids.push(b.seq),
                            Instr::Loop(l) => seq_ids.push(l.seq),
                            Instr::IfElse(ie) => { seq_ids.push(ie.consequent); seq_ids.push(ie.alternative); }
                            Instr::BrIf(bi) => seq_ids.push(bi.block),
                            Instr::Br(b) => seq_ids.push(b.block),
                            Instr::BrTable(bt) => { seq_ids.push(bt.default); seq_ids.extend(&bt.blocks); }
                            Instr::MemorySize(m) => m.memory = combined,
                            Instr::MemoryGrow(m) => m.memory = combined,
                            Instr::MemoryCopy(m) => { m.src = combined; m.dst = combined; },
                            Instr::MemoryFill(m) => m.memory = combined,
                            Instr::MemoryInit(m) => m.memory = combined,
                            Instr::Load(m) => m.memory = combined,
                            Instr::Store(m) => m.memory = combined,
                            Instr::AtomicRmw(m) => m.memory = combined,
                            Instr::Cmpxchg(m) => m.memory = combined,
                            Instr::AtomicWait(m) => m.memory = combined,
                            Instr::AtomicNotify(m) => m.memory = combined,
                            Instr::LoadSimd(m) => m.memory = combined,
                            Instr::AtomicFence(_) => {},
                            _ => {}
                        }
                    }
                }
            }
        }
        
        // Debug Validation: check if any instruction still refers to a dead memory
        let all_local_fids: Vec<_> = module.funcs.iter_local().map(|(id, _)| id).collect();
        for fid in all_local_fids {
            if let walrus::FunctionKind::Local(func) = &module.funcs.get(fid).kind {
                let mut seq_ids = vec![func.entry_block()];
                let mut visited = std::collections::HashSet::new();

                while let Some(seq_id) = seq_ids.pop() {
                    if !visited.insert(seq_id) { continue; }
                    let seq = func.block(seq_id);
                    for (instr, _) in &seq.instrs {
                        match instr {
                            Instr::Block(b) => seq_ids.push(b.seq),
                            Instr::Loop(l) => seq_ids.push(l.seq),
                            Instr::IfElse(ie) => { seq_ids.push(ie.consequent); seq_ids.push(ie.alternative); }
                            Instr::BrIf(bi) => seq_ids.push(bi.block),
                            Instr::Br(b) => seq_ids.push(b.block),
                            Instr::BrTable(bt) => { seq_ids.push(bt.default); seq_ids.extend(&bt.blocks); }
                            Instr::MemorySize(m) => { if mem_ids.contains(&m.memory) { panic!("Leaked MemoryId in MemorySize"); } },
                            Instr::MemoryGrow(m) => { if mem_ids.contains(&m.memory) { panic!("Leaked MemoryId in MemoryGrow"); } },
                            Instr::MemoryInit(m) => { if mem_ids.contains(&m.memory) { panic!("Leaked MemoryId in MemoryInit"); } },
                            Instr::MemoryFill(m) => { if mem_ids.contains(&m.memory) { panic!("Leaked MemoryId in MemoryFill"); } },
                            Instr::Load(m) => { if mem_ids.contains(&m.memory) { panic!("Leaked MemoryId in Load"); } },
                            Instr::Store(m) => { if mem_ids.contains(&m.memory) { panic!("Leaked MemoryId in Store"); } },
                            Instr::AtomicRmw(m) => { if mem_ids.contains(&m.memory) { panic!("Leaked MemoryId in AtomicRmw"); } },
                            Instr::Cmpxchg(m) => { if mem_ids.contains(&m.memory) { panic!("Leaked MemoryId in Cmpxchg"); } },
                            Instr::AtomicWait(m) => { if mem_ids.contains(&m.memory) { panic!("Leaked MemoryId in AtomicWait"); } },
                            Instr::AtomicNotify(m) => { if mem_ids.contains(&m.memory) { panic!("Leaked MemoryId in AtomicNotify"); } },
                            Instr::LoadSimd(m) => { if mem_ids.contains(&m.memory) { panic!("Leaked MemoryId in LoadSimd"); } },
                            Instr::MemoryCopy(m) => {
                                if mem_ids.contains(&m.src) { panic!("Leaked src in MemoryCopy"); }
                                if mem_ids.contains(&m.dst) { panic!("Leaked dst in MemoryCopy"); }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        // Patch data segments to point to the combined memory
        let data_ids: Vec<_> = module.data.iter().map(|d| d.id()).collect();
        for did in data_ids {
            let data = module.data.get_mut(did);
            if let DataKind::Active { memory, .. } = &mut data.kind {
                *memory = combined;
            }
        }

        Ok(combined)
    }
}

impl Generator for MultiMemoryLowering {}
