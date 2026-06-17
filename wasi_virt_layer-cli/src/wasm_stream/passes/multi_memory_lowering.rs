use crate::wasm_stream::pipeline::{StreamPass, par_process_code_section};
use crate::wasm_stream::translator::Rebind;
use eyre::Result;
use wasm_encoder::{Function, Instruction, Module, RawSection};
use wasmparser::{Parser, Payload, TypeRef};

const LOWERING_HELPERS_SECTION: &str = "wvl.multi_memory_lowering.helpers.v1";
struct ImportRebinder {
    pub func_map: std::collections::HashMap<u32, u32>,
    pub own_memory_removed_imports: u32,
    pub original_imported_funcs: u32,
}

impl crate::wasm_stream::translator::Rebind for ImportRebinder {
    fn function(&self, index: u32) -> u32 {
        if index >= self.original_imported_funcs {
            index - self.own_memory_removed_imports
        } else {
            *self.func_map.get(&index).unwrap()
        }
    }
}

#[derive(Debug, Default)]
pub struct MultiMemoryLoweringStreamPass {
    pub threads: bool,
    pub own_memory: bool,
    pub target_names: Vec<String>,
    pub lower_memory: bool,
}

impl MultiMemoryLoweringStreamPass {
    pub fn new(
        threads: bool,
        own_memory: bool,
        target_names: Vec<String>,
        lower_memory: bool,
    ) -> Self {
        Self {
            threads,
            own_memory,
            target_names,
            lower_memory,
        }
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
        let mut init_offset_global_fid = None;

        let mut own_memory_size_get = std::collections::HashMap::new();
        let mut own_memory_size_set = std::collections::HashMap::new();
        let mut own_memory_size_init = std::collections::HashMap::new();

        let mut func_types = Vec::new();
        let mut types = Vec::new();
        let mut imported_funcs = 0;
        let mut global_count = 0;
        let mut memory_initials = Vec::new();
        let mut data_count = 0;

        let mut func_map = std::collections::HashMap::new();
        let mut own_memory_removed_imports = 0;
        let mut own_memory_grow_imports = std::collections::HashMap::new();
        let mut original_imported_funcs = 0;
        let mut own_memory_size_imports = std::collections::HashMap::new();

        let is_own_memory_size = |name: &str| name.starts_with("__wasip1_vfs_own_memory_size_");
        let is_own_memory_grow = |name: &str| name.starts_with("__wasip1_vfs_own_memory_grow_");
        let get_target_name = |name: &str| {
            if is_own_memory_size(name) {
                Some(
                    name.trim_start_matches("__wasip1_vfs_own_memory_size_")
                        .to_string(),
                )
            } else if is_own_memory_grow(name) {
                Some(
                    name.trim_start_matches("__wasip1_vfs_own_memory_grow_")
                        .to_string(),
                )
            } else {
                None
            }
        };

        if self.own_memory {
            let mut current_idx = 0;
            for payload in Parser::new(0).parse_all(input_wasm) {
                if let Ok(Payload::ImportSection(s)) = payload {
                    for group in s {
                        if let Ok(group) = group {
                            for i in group.into_iter() {
                                if let Ok(i) = i {
                                    if matches!(i.1.ty, TypeRef::Func(_) | TypeRef::FuncExact(_)) {
                                        let name = i.1.name;
                                        if is_own_memory_size(name) || is_own_memory_grow(name) {
                                            own_memory_removed_imports += 1;
                                            if is_own_memory_size(name) {
                                                own_memory_size_imports.insert(
                                                    get_target_name(name).unwrap(),
                                                    current_idx,
                                                );
                                            } else {
                                                own_memory_grow_imports.insert(
                                                    get_target_name(name).unwrap(),
                                                    current_idx,
                                                );
                                            }
                                        } else {
                                            func_map.insert(
                                                current_idx,
                                                current_idx - own_memory_removed_imports,
                                            );
                                        }
                                        current_idx += 1;
                                        original_imported_funcs += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            for target in &self.target_names {
                if !own_memory_grow_imports.contains_key(target)
                    || !own_memory_size_imports.contains_key(target)
                {
                    panic!(
                        "VFS module is missing `own_memory!` configuration for target Wasm: {}. Make sure you pass all target wasms to `own_memory!` in your VFS.",
                        target
                    );
                }
            }
        }

        let mut own_memory_size_idx_to_mem = std::collections::HashMap::new();
        let mut own_memory_grow_idx_to_mem = std::collections::HashMap::new();
        if self.own_memory {
            for (target, idx) in &own_memory_size_imports {
                let normalized = target.replace("-", "_");
                if let Some(pos) = self.target_names.iter().position(|t| t == &normalized) {
                    own_memory_size_idx_to_mem.insert(*idx, (pos + 1) as u32);
                }
            }
            for (target, idx) in &own_memory_grow_imports {
                let normalized = target.replace("-", "_");
                if let Some(pos) = self.target_names.iter().position(|t| t == &normalized) {
                    own_memory_grow_idx_to_mem.insert(*idx, (pos + 1) as u32);
                }
            }
        }

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
                                TypeRef::Func(_) | TypeRef::FuncExact(_) => imported_funcs += 1,
                                TypeRef::Global(_) => global_count += 1,
                                TypeRef::Memory(m) => {
                                    memory_count += 1;
                                    memory_initials.push(m.initial);
                                    total_initial += m.initial;
                                    if let Some(max) = m.maximum {
                                        let cur_max = max_pages.unwrap_or(0);
                                        max_pages = Some(std::cmp::min(cur_max + max, 65536));
                                    }
                                    if memory_count == 1 {
                                        is_shared = m.shared;
                                    }
                                }
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
                            if export.name == "__wasip1_vfs_host_own_memory_size_get" {
                                own_memory_size_get.insert(0, export.index);
                            } else if export.name == "__wasip1_vfs_host_own_memory_size_set" {
                                own_memory_size_set.insert(0, export.index);
                            } else if export.name == "__wasip1_vfs_host_own_memory_size_init" {
                                own_memory_size_init.insert(0, export.index);
                            } else if export.name.starts_with("__wasip1_vfs_")
                                && export.name.ends_with("_own_memory_size_get")
                            {
                                let target_name = export
                                    .name
                                    .trim_start_matches("__wasip1_vfs_")
                                    .trim_end_matches("_own_memory_size_get");
                                if let Some(pos) = self
                                    .target_names
                                    .iter()
                                    .position(|t| t.replace("-", "_") == target_name)
                                {
                                    own_memory_size_get.insert((pos + 1) as u32, export.index);
                                }
                            } else if export.name.starts_with("__wasip1_vfs_")
                                && export.name.ends_with("_own_memory_size_set")
                            {
                                let target_name = export
                                    .name
                                    .trim_start_matches("__wasip1_vfs_")
                                    .trim_end_matches("_own_memory_size_set");
                                if let Some(pos) = self
                                    .target_names
                                    .iter()
                                    .position(|t| t.replace("-", "_") == target_name)
                                {
                                    own_memory_size_set.insert((pos + 1) as u32, export.index);
                                }
                            } else if export.name.starts_with("__wasip1_vfs_")
                                && export.name.ends_with("_own_memory_size_init")
                            {
                                let target_name = export
                                    .name
                                    .trim_start_matches("__wasip1_vfs_")
                                    .trim_end_matches("_own_memory_size_init");
                                if let Some(pos) = self
                                    .target_names
                                    .iter()
                                    .position(|t| t.replace("-", "_") == target_name)
                                {
                                    own_memory_size_init.insert((pos + 1) as u32, export.index);
                                }
                            }

                            match export.name {
                                "__wasip1_vfs_memory_lock_read_acquire" => {
                                    lock_acquire_fn = Some(export.index)
                                }
                                "__wasip1_vfs_memory_lock_read_release" => {
                                    lock_release_fn = Some(export.index)
                                }
                                "__wasip1_vfs_memory_lock_write_acquire" => {
                                    lock_write_acquire_fn = Some(export.index)
                                }
                                "__wasip1_vfs_memory_lock_write_release" => {
                                    lock_write_release_fn = Some(export.index)
                                }
                                "__init_offset_global" => {
                                    init_offset_global_fid = Some(export.index)
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // TODO: Skip early return if `self.own_memory` is true
        if memory_count <= 1 {
            return Ok(input_wasm.to_vec());
        }

        let mut size_type_idx = None;
        let mut grow_type_idx = None;
        let mut set_size_type_idx = None;

        for (i, t) in types.iter().enumerate() {
            if let wasmparser::CompositeInnerType::Func(f) = &t.composite_type.inner {
                if f.params().is_empty()
                    && f.results().len() == 1
                    && f.results()[0] == wasmparser::ValType::I32
                {
                    size_type_idx = Some(i as u32);
                }
                if f.params().len() == 1
                    && f.params()[0] == wasmparser::ValType::I32
                    && f.results().len() == 1
                    && f.results()[0] == wasmparser::ValType::I32
                {
                    grow_type_idx = Some(i as u32);
                }
                if f.params().len() == 1
                    && f.params()[0] == wasmparser::ValType::I32
                    && f.results().is_empty()
                {
                    set_size_type_idx = Some(i as u32);
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
        let new_func_count = orig_func_count - own_memory_removed_imports;

        if self.own_memory {
            for (target, orig_idx) in own_memory_size_imports.iter() {
                let target_idx =
                    self.target_names.iter().position(|n| n == target).unwrap() as u32 + 1;
                func_map.insert(*orig_idx, new_func_count + target_idx);
            }
            for (target, orig_idx) in own_memory_grow_imports.iter() {
                let target_idx =
                    self.target_names.iter().position(|n| n == target).unwrap() as u32 + 1;
                func_map.insert(*orig_idx, new_func_count + memory_count + target_idx);
            }
        }
        let rebinder = ImportRebinder {
            func_map,
            own_memory_removed_imports,
            original_imported_funcs,
        };

        let lock_acquire_fn = lock_acquire_fn
            .map(|idx| crate::wasm_stream::translator::Rebind::function(&rebinder, idx));
        let lock_release_fn = lock_release_fn
            .map(|idx| crate::wasm_stream::translator::Rebind::function(&rebinder, idx));
        let lock_write_acquire_fn = lock_write_acquire_fn
            .map(|idx| crate::wasm_stream::translator::Rebind::function(&rebinder, idx));
        let lock_write_release_fn = lock_write_release_fn
            .map(|idx| crate::wasm_stream::translator::Rebind::function(&rebinder, idx));

        let final_size_type_idx = size_type_idx.unwrap_or(types.len() as u32);
        let final_grow_type_idx = grow_type_idx.unwrap_or(if size_type_idx.is_none() {
            types.len() as u32 + 1
        } else {
            types.len() as u32
        });
        let mut added_types = 0;
        if size_type_idx.is_none() {
            added_types += 1;
        }
        if grow_type_idx.is_none() {
            added_types += 1;
        }
        let final_set_size_type_idx = set_size_type_idx.unwrap_or(types.len() as u32 + added_types);

        let mut emitted_memory = false;

        for payload in Parser::new(0).parse_all(input_wasm) {
            let payload = payload?;
            match payload {
                Payload::TypeSection(s) => {
                    let mut type_sec = wasm_encoder::TypeSection::new();
                    // We must re-encode types to append
                    for t in s {
                        let t = t?;
                        if t.is_explicit_rec_group() {
                            let rec_types = t
                                .into_types()
                                .map(|sub_ty| {
                                    crate::wasm_stream::translator::translate_sub_type(
                                        &sub_ty, &rebinder,
                                    )
                                })
                                .collect::<Vec<_>>();
                            type_sec.ty().rec(rec_types);
                        } else {
                            for sub_ty in t.into_types() {
                                type_sec.ty().subtype(
                                    &crate::wasm_stream::translator::translate_sub_type(
                                        &sub_ty, &rebinder,
                                    ),
                                );
                            }
                        }
                    }
                    if size_type_idx.is_none() {
                        type_sec.ty().function([], [wasm_encoder::ValType::I32]);
                    }
                    if grow_type_idx.is_none() {
                        type_sec
                            .ty()
                            .function([wasm_encoder::ValType::I32], [wasm_encoder::ValType::I32]);
                    }
                    if set_size_type_idx.is_none() {
                        type_sec.ty().function([wasm_encoder::ValType::I32], []);
                    }
                    encoder.section(&type_sec);
                }
                Payload::ImportSection(s) => {
                    let mut imports = wasm_encoder::ImportSection::new();
                    for group in s {
                        for i in group?.into_iter() {
                            let (_, i) = i?;
                            if self.own_memory {
                                if matches!(i.ty, TypeRef::Func(_) | TypeRef::FuncExact(_)) {
                                    if is_own_memory_size(i.name) || is_own_memory_grow(i.name) {
                                        println!("SKIPPING IMPORT: {}::{}", i.module, i.name);
                                        continue;
                                    }
                                }
                            }
                            match i.ty {
                                TypeRef::Func(_)
                                | TypeRef::FuncExact(_)
                                | TypeRef::Global(_)
                                | TypeRef::Table(_)
                                | TypeRef::Tag(_) => {
                                    let entity = match i.ty {
                                        TypeRef::Func(idx) | TypeRef::FuncExact(idx) => {
                                            wasm_encoder::EntityType::Function(idx)
                                        }
                                        TypeRef::Table(t) => wasm_encoder::EntityType::Table(
                                            crate::wasm_stream::translator::translate_table_type(
                                                t, &rebinder,
                                            ),
                                        ),
                                        TypeRef::Global(g) => wasm_encoder::EntityType::Global(
                                            crate::wasm_stream::translator::translate_global_type(
                                                g, &rebinder,
                                            ),
                                        ),
                                        TypeRef::Tag(t) => wasm_encoder::EntityType::Tag(
                                            crate::wasm_stream::translator::translate_tag_type(t),
                                        ),
                                        _ => unreachable!(),
                                    };
                                    imports.import(i.module, i.name, entity);
                                }
                                TypeRef::Memory(_) => {
                                    // Drop memory imports (they are merged into locally defined memory)
                                }
                            }
                        }
                    }
                    if !imports.is_empty() {
                        encoder.section(&imports);
                    }
                }
                Payload::MemorySection(s) => {
                    if !self.lower_memory {
                        encoder.section(&RawSection {
                            id: 5,
                            data: &input_wasm[s.range()],
                        });
                        continue;
                    }
                    if !emitted_memory {
                        emitted_memory = true;
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
                }
                Payload::GlobalSection(s) => {
                    if self.lower_memory {
                        if !emitted_memory {
                            emitted_memory = true;
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
                    }
                    let mut global_sec = wasm_encoder::GlobalSection::new();
                    for item in s {
                        let g = item?;
                        let ty = wasm_encoder::GlobalType {
                            val_type: crate::wasm_stream::translator::translate_val_type(
                                g.ty.content_type,
                                &rebinder,
                            ),
                            mutable: g.ty.mutable,
                            shared: g.ty.shared,
                        };
                        let expr = crate::wasm_stream::translator::translate_const_expr(
                            &g.init_expr,
                            &rebinder,
                        )?;
                        global_sec.global(ty, &expr);
                    }
                    // Append offset globals
                    if self.lower_memory {
                        for offset in memory_offsets.iter().skip(1) {
                            global_sec.global(
                                wasm_encoder::GlobalType {
                                    val_type: wasm_encoder::ValType::I32,
                                    mutable: true,
                                    shared: false,
                                },
                                &wasm_encoder::ConstExpr::i32_const((*offset * 65536) as i32),
                            );
                        }
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
                    if self.own_memory {
                        // logical grow fns
                        for _ in 0..memory_count {
                            func_sec.function(final_grow_type_idx);
                        }
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
                        let idx = if e.kind == wasmparser::ExternalKind::Memory {
                            0
                        } else if e.kind == wasmparser::ExternalKind::Func
                            || e.kind == wasmparser::ExternalKind::FuncExact
                        {
                            crate::wasm_stream::translator::Rebind::function(&rebinder, e.index)
                        } else {
                            e.index
                        };
                        exp_sec.export(e.name, kind, idx);
                    }
                    encoder.section(&exp_sec);
                }
                Payload::StartSection { func, range: _ } => {
                    let idx = crate::wasm_stream::translator::Rebind::function(&rebinder, func);
                    encoder.section(&wasm_encoder::StartSection {
                        function_index: idx,
                    });
                }
                Payload::ElementSection(s) => {
                    let mut elem_sec = wasm_encoder::ElementSection::new();
                    for elem in s {
                        let elem = elem?;
                        let offset;
                        let mode = match elem.kind {
                            wasmparser::ElementKind::Active {
                                table_index,
                                offset_expr,
                            } => {
                                offset = crate::wasm_stream::translator::translate_const_expr(
                                    &offset_expr,
                                    &rebinder,
                                )?;
                                wasm_encoder::ElementMode::Active {
                                    table: Some(rebinder.table(table_index.unwrap_or(0))),
                                    offset: &offset,
                                }
                            }
                            wasmparser::ElementKind::Passive => wasm_encoder::ElementMode::Passive,
                            wasmparser::ElementKind::Declared => {
                                wasm_encoder::ElementMode::Declared
                            }
                        };
                        let elements = match elem.items {
                            wasmparser::ElementItems::Functions(f) => {
                                let mut funcs = Vec::new();
                                for f in f {
                                    funcs.push(rebinder.function(f?));
                                }
                                wasm_encoder::Elements::Functions(std::borrow::Cow::Owned(funcs))
                            }
                            wasmparser::ElementItems::Expressions(ty, e) => {
                                let mut exprs = Vec::new();
                                for e in e {
                                    exprs.push(
                                        crate::wasm_stream::translator::translate_const_expr(
                                            &e?, &rebinder,
                                        )?,
                                    );
                                }
                                wasm_encoder::Elements::Expressions(
                                    crate::wasm_stream::translator::translate_ref_type(
                                        ty, &rebinder,
                                    ),
                                    std::borrow::Cow::Owned(exprs),
                                )
                            }
                        };
                        elem_sec.segment(wasm_encoder::ElementSegment { mode, elements });
                    }
                    encoder.section(&elem_sec);
                }
                Payload::DataCountSection { count, range } => {
                    if !self.lower_memory {
                        encoder.section(&RawSection {
                            id: 12,
                            data: &input_wasm[range],
                        });
                    } else {
                        encoder.section(&wasm_encoder::DataCountSection { count });
                    }
                }
                Payload::DataSection(s) => {
                    if !self.lower_memory {
                        encoder.section(&RawSection {
                            id: 11,
                            data: &input_wasm[s.range()],
                        });
                        continue;
                    }
                    let mut data_sec = wasm_encoder::DataSection::new();
                    for d in s {
                        let d = d?;
                        match d.kind {
                            wasmparser::DataKind::Active {
                                memory_index,
                                offset_expr,
                            } => {
                                let mem_idx = memory_index as usize;
                                let mut expr =
                                    crate::wasm_stream::translator::translate_const_expr(
                                        &offset_expr,
                                        &rebinder,
                                    )?;
                                if mem_idx > 0 {
                                    // Replace `expr` by adding the initial offset
                                    // Actually, we can just use the global
                                    let gid = orig_global_count + mem_idx as u32 - 1;
                                    let _new_expr = wasm_encoder::ConstExpr::global_get(gid);
                                    // Wasm-encoder doesn't allow complex const expressions yet?
                                    // Wait! ConstExpr in wasm-encoder is limited!
                                    // Actually, DataKind::Active offset can just be i32_const(val + offset * 65536)
                                    // if the original is just i32_const.
                                    // Let's assume it's just i32_const because multi-memory lowering runs at build time.
                                    let op = offset_expr.get_operators_reader().read()?;
                                    if let wasmparser::Operator::I32Const { value } = op {
                                        let new_val =
                                            value + (memory_offsets[mem_idx] * 65536) as i32;
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
                Payload::CodeSectionStart {
                    count: _,
                    range,
                    size: _,
                } => {
                    let reader = wasmparser::BinaryReader::new(
                        &input_wasm[range.start..range.end],
                        range.start,
                    );
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
                            let enc_ty =
                                crate::wasm_stream::translator::translate_val_type(ty, &rebinder);
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
                        let tmp_i64_2 = tmp_base + 7;

                        locals.push((1, wasm_encoder::ValType::I32)); // tmp_addr
                        locals.push((1, wasm_encoder::ValType::I32)); // tmp_i32
                        locals.push((1, wasm_encoder::ValType::I32)); // tmp_i32_2
                        locals.push((1, wasm_encoder::ValType::I64)); // tmp_i64
                        locals.push((1, wasm_encoder::ValType::F32)); // tmp_f32
                        locals.push((1, wasm_encoder::ValType::F64)); // tmp_f64
                        locals.push((1, wasm_encoder::ValType::V128)); // tmp_v128
                        locals.push((1, wasm_encoder::ValType::I64)); // tmp_i64_2

                        let mut func = Function::new(locals);

                        if self.own_memory
                            && Some(imported_funcs + i as u32) == init_offset_global_fid
                        {
                            // Call init for host
                            if let Some(&init_fn_idx) = own_memory_size_init.get(&0) {
                                func.instruction(&Instruction::I32Const(memory_initials[0] as i32));
                                func.instruction(&Instruction::Call(
                                    crate::wasm_stream::translator::Rebind::function(
                                        &rebinder,
                                        init_fn_idx,
                                    ),
                                ));
                            }

                            // Call init for targets
                            for target_idx in 1..memory_count {
                                if let Some(&init_fn_idx) =
                                    own_memory_size_init.get(&(target_idx as u32))
                                {
                                    func.instruction(&Instruction::I32Const(
                                        memory_initials[target_idx as usize] as i32,
                                    ));
                                    func.instruction(&Instruction::Call(
                                        crate::wasm_stream::translator::Rebind::function(
                                            &rebinder,
                                            init_fn_idx,
                                        ),
                                    ));
                                }
                            }
                        }

                        let mut reader = func_body.get_operators_reader()?;
                        while !reader.eof() {
                            let op = reader.read()?;
                            match op {
                                wasmparser::Operator::MemorySize { mem, .. } => {
                                    func.instruction(&wasm_encoder::Instruction::Call(
                                        new_func_count + mem,
                                    ));
                                }
                                wasmparser::Operator::MemoryGrow { mem, .. } => {
                                    if self.own_memory && mem != 0 {
                                        func.instruction(&wasm_encoder::Instruction::Call(
                                            new_func_count + 2 * memory_count + mem as u32,
                                        ));
                                    } else {
                                        func.instruction(&wasm_encoder::Instruction::Call(
                                            new_func_count + memory_count + mem as u32,
                                        ));
                                    }
                                }
                                wasmparser::Operator::MemoryCopy { dst_mem, src_mem } => {
                                    let d_idx = dst_mem as u32;
                                    let s_idx = src_mem as u32;
                                    if self.lower_memory && (d_idx > 0 || s_idx > 0) {
                                        func.instruction(&wasm_encoder::Instruction::LocalSet(
                                            tmp_addr,
                                        )); // len
                                        func.instruction(&wasm_encoder::Instruction::LocalSet(
                                            tmp_i32,
                                        )); // src
                                        func.instruction(&wasm_encoder::Instruction::LocalSet(
                                            tmp_i32_2,
                                        )); // dst

                                        // Acquire read lock if threaded
                                        if threads {
                                            if let Some(acq) = lock_acquire_fn {
                                                func.instruction(&wasm_encoder::Instruction::Call(
                                                    acq,
                                                ));
                                            }
                                        }

                                        func.instruction(&wasm_encoder::Instruction::LocalGet(
                                            tmp_i32_2,
                                        ));
                                        if d_idx > 0 {
                                            func.instruction(
                                                &wasm_encoder::Instruction::GlobalGet(
                                                    orig_global_count + d_idx - 1,
                                                ),
                                            );
                                            func.instruction(&wasm_encoder::Instruction::I32Add);
                                        }

                                        func.instruction(&wasm_encoder::Instruction::LocalGet(
                                            tmp_i32,
                                        ));
                                        if s_idx > 0 {
                                            func.instruction(
                                                &wasm_encoder::Instruction::GlobalGet(
                                                    orig_global_count + s_idx - 1,
                                                ),
                                            );
                                            func.instruction(&wasm_encoder::Instruction::I32Add);
                                        }

                                        func.instruction(&wasm_encoder::Instruction::LocalGet(
                                            tmp_addr,
                                        ));
                                        func.instruction(&wasm_encoder::Instruction::MemoryCopy {
                                            dst_mem: 0,
                                            src_mem: 0,
                                        });

                                        // Release read lock if threaded
                                        if threads {
                                            if let Some(rel) = lock_release_fn {
                                                func.instruction(&wasm_encoder::Instruction::Call(
                                                    rel,
                                                ));
                                            }
                                        }
                                    } else {
                                        let mut enc_op = crate::wasm_stream::translator::translate(
                                            &op, &rebinder,
                                        );
                                        if self.lower_memory {
                                            crate::wasm_stream::mem_info::clear_memory_index(
                                                &mut enc_op,
                                            );
                                        }
                                        func.instruction(&enc_op);
                                    }
                                }
                                wasmparser::Operator::MemoryFill { mem } => {
                                    let idx = mem as u32;
                                    if self.lower_memory && idx > 0 {
                                        func.instruction(&wasm_encoder::Instruction::LocalSet(
                                            tmp_addr,
                                        )); // len
                                        func.instruction(&wasm_encoder::Instruction::LocalSet(
                                            tmp_i32,
                                        )); // val
                                        func.instruction(&wasm_encoder::Instruction::LocalSet(
                                            tmp_i32_2,
                                        )); // dst

                                        // Acquire read lock if threaded
                                        if threads {
                                            if let Some(acq) = lock_acquire_fn {
                                                func.instruction(&wasm_encoder::Instruction::Call(
                                                    acq,
                                                ));
                                            }
                                        }

                                        func.instruction(&wasm_encoder::Instruction::LocalGet(
                                            tmp_i32_2,
                                        ));
                                        func.instruction(&wasm_encoder::Instruction::GlobalGet(
                                            orig_global_count + idx - 1,
                                        ));
                                        func.instruction(&wasm_encoder::Instruction::I32Add);

                                        func.instruction(&wasm_encoder::Instruction::LocalGet(
                                            tmp_i32,
                                        ));
                                        func.instruction(&wasm_encoder::Instruction::LocalGet(
                                            tmp_addr,
                                        ));
                                        func.instruction(&wasm_encoder::Instruction::MemoryFill(0));

                                        // Release read lock if threaded
                                        if threads {
                                            if let Some(rel) = lock_release_fn {
                                                func.instruction(&wasm_encoder::Instruction::Call(
                                                    rel,
                                                ));
                                            }
                                        }
                                    } else {
                                        let mut enc_op = crate::wasm_stream::translator::translate(
                                            &op, &rebinder,
                                        );
                                        if self.lower_memory {
                                            crate::wasm_stream::mem_info::clear_memory_index(
                                                &mut enc_op,
                                            );
                                        }
                                        func.instruction(&enc_op);
                                    }
                                }
                                wasmparser::Operator::MemoryInit { data_index, mem } => {
                                    let idx = mem as u32;
                                    if self.lower_memory && idx > 0 {
                                        func.instruction(&wasm_encoder::Instruction::LocalSet(
                                            tmp_addr,
                                        )); // len
                                        func.instruction(&wasm_encoder::Instruction::LocalSet(
                                            tmp_i32,
                                        )); // src
                                        func.instruction(&wasm_encoder::Instruction::LocalSet(
                                            tmp_i32_2,
                                        )); // dst

                                        // Acquire read lock if threaded
                                        if threads {
                                            if let Some(acq) = lock_acquire_fn {
                                                func.instruction(&wasm_encoder::Instruction::Call(
                                                    acq,
                                                ));
                                            }
                                        }

                                        func.instruction(&wasm_encoder::Instruction::LocalGet(
                                            tmp_i32_2,
                                        ));
                                        func.instruction(&wasm_encoder::Instruction::GlobalGet(
                                            orig_global_count + idx - 1,
                                        ));
                                        func.instruction(&wasm_encoder::Instruction::I32Add);

                                        func.instruction(&wasm_encoder::Instruction::LocalGet(
                                            tmp_i32,
                                        ));
                                        func.instruction(&wasm_encoder::Instruction::LocalGet(
                                            tmp_addr,
                                        ));
                                        func.instruction(&wasm_encoder::Instruction::MemoryInit {
                                            data_index,
                                            mem: 0,
                                        });

                                        // Release read lock if threaded
                                        if threads {
                                            if let Some(rel) = lock_release_fn {
                                                func.instruction(&wasm_encoder::Instruction::Call(
                                                    rel,
                                                ));
                                            }
                                        }
                                    } else {
                                        let mut enc_op = crate::wasm_stream::translator::translate(
                                            &op, &rebinder,
                                        );
                                        if self.lower_memory {
                                            crate::wasm_stream::mem_info::clear_memory_index(
                                                &mut enc_op,
                                            );
                                        }
                                        func.instruction(&enc_op);
                                    }
                                }
                                _ => {
                                    if let Some(info) =
                                        crate::wasm_stream::mem_info::memory_op_info(&op)
                                    {
                                        if self.lower_memory && info.memory > 0 {
                                            let offset_idx = orig_global_count + info.memory - 1;

                                            // Helper closure-like mapping for temp locals.
                                            // For two-operand cases we need distinct temp slots
                                            // per type.  The "second slot" indices:
                                            //   i32 -> tmp_i32_2, i64 -> tmp_i64_2
                                            // (f32/f64/v128 never appear as second operand in
                                            //  practice, but we panic if they do.)
                                            let tmp_for = |ty: wasm_encoder::ValType, secondary: bool| -> u32 {
                                                if !secondary {
                                                    match ty {
                                                        wasm_encoder::ValType::I32 => tmp_i32,
                                                        wasm_encoder::ValType::I64 => tmp_i64,
                                                        wasm_encoder::ValType::F32 => tmp_f32,
                                                        wasm_encoder::ValType::F64 => tmp_f64,
                                                        wasm_encoder::ValType::V128 => tmp_v128,
                                                        _ => unreachable!(),
                                                    }
                                                } else {
                                                    match ty {
                                                        wasm_encoder::ValType::I32 => tmp_i32_2,
                                                        wasm_encoder::ValType::I64 => tmp_i64_2,
                                                        _ => unreachable!("secondary temp for {:?}", ty),
                                                    }
                                                }
                                            };

                                            // Save value operands (top-of-stack first, i.e.
                                            // in reverse order of value_operands).
                                            // Stack before: [... addr val0 val1]
                                            //   LocalSet(tmp_for(val1, secondary=true))
                                            //   LocalSet(tmp_for(val0, secondary=false))
                                            //   LocalSet(tmp_addr)
                                            let _n = info.value_operands.len();
                                            for (i, ty) in
                                                info.value_operands.iter().enumerate().rev()
                                            {
                                                let secondary = i > 0; // first operand uses primary, rest use secondary
                                                func.instruction(
                                                    &wasm_encoder::Instruction::LocalSet(tmp_for(
                                                        *ty, secondary,
                                                    )),
                                                );
                                            }
                                            func.instruction(&wasm_encoder::Instruction::LocalSet(
                                                tmp_addr,
                                            ));

                                            // Acquire lock if threaded
                                            if threads {
                                                if let Some(acq) = lock_acquire_fn {
                                                    func.instruction(
                                                        &wasm_encoder::Instruction::Call(acq),
                                                    );
                                                }
                                            }

                                            // Push adjusted address
                                            func.instruction(&wasm_encoder::Instruction::LocalGet(
                                                tmp_addr,
                                            ));
                                            func.instruction(
                                                &wasm_encoder::Instruction::GlobalGet(offset_idx),
                                            );
                                            func.instruction(&wasm_encoder::Instruction::I32Add);

                                            // Restore value operands in original order
                                            for (i, ty) in info.value_operands.iter().enumerate() {
                                                let secondary = i > 0;
                                                func.instruction(
                                                    &wasm_encoder::Instruction::LocalGet(tmp_for(
                                                        *ty, secondary,
                                                    )),
                                                );
                                            }

                                            // Emit the instruction with memory index 0
                                            let mut enc_op =
                                                crate::wasm_stream::translator::translate(
                                                    &op, &rebinder,
                                                );
                                            crate::wasm_stream::mem_info::clear_memory_index(
                                                &mut enc_op,
                                            );
                                            func.instruction(&enc_op);

                                            // Release lock if threaded
                                            if threads {
                                                if let Some(rel) = lock_release_fn {
                                                    // If there's a result, save it, release, restore
                                                    if let Some(res_ty) = info.result_type {
                                                        let tmp_res = tmp_for(res_ty, false);
                                                        func.instruction(
                                                            &wasm_encoder::Instruction::LocalSet(
                                                                tmp_res,
                                                            ),
                                                        );
                                                        func.instruction(
                                                            &wasm_encoder::Instruction::Call(rel),
                                                        );
                                                        func.instruction(
                                                            &wasm_encoder::Instruction::LocalGet(
                                                                tmp_res,
                                                            ),
                                                        );
                                                    } else {
                                                        func.instruction(
                                                            &wasm_encoder::Instruction::Call(rel),
                                                        );
                                                    }
                                                }
                                            }
                                            continue;
                                        }
                                    }
                                    let mut enc_op =
                                        crate::wasm_stream::translator::translate(&op, &rebinder);
                                    if self.lower_memory
                                        && crate::wasm_stream::mem_info::memory_op_info(&op)
                                            .is_some()
                                    {
                                        crate::wasm_stream::mem_info::clear_memory_index(
                                            &mut enc_op,
                                        );
                                    }
                                    func.instruction(&enc_op);
                                }
                                wasmparser::Operator::Call { function_index } => {
                                    if self.own_memory {
                                        if let Some(&mem_idx) =
                                            own_memory_size_idx_to_mem.get(&function_index)
                                        {
                                            func.instruction(&wasm_encoder::Instruction::Call(
                                                new_func_count + mem_idx,
                                            ));
                                            continue;
                                        }
                                        if let Some(&mem_idx) =
                                            own_memory_grow_idx_to_mem.get(&function_index)
                                        {
                                            func.instruction(&wasm_encoder::Instruction::Call(
                                                new_func_count + memory_count + mem_idx,
                                            ));
                                            continue;
                                        }
                                    }
                                    let mut enc_op =
                                        crate::wasm_stream::translator::translate(&op, &rebinder);
                                    if self.lower_memory
                                        && crate::wasm_stream::mem_info::memory_op_info(&op)
                                            .is_some()
                                    {
                                        crate::wasm_stream::mem_info::clear_memory_index(
                                            &mut enc_op,
                                        );
                                    }
                                    func.instruction(&enc_op);
                                }
                                _ => {
                                    let mut enc_op =
                                        crate::wasm_stream::translator::translate(&op, &rebinder);
                                    if self.lower_memory
                                        && crate::wasm_stream::mem_info::memory_op_info(&op)
                                            .is_some()
                                    {
                                        crate::wasm_stream::mem_info::clear_memory_index(
                                            &mut enc_op,
                                        );
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
                        if self.own_memory {
                            let logical_size_get_fn =
                                crate::wasm_stream::translator::Rebind::function(
                                    &rebinder,
                                    *own_memory_size_get.get(&(idx as u32)).unwrap(),
                                );
                            func.instruction(&Instruction::Call(logical_size_get_fn));
                        } else if !self.lower_memory || memory_count == 1 {
                            func.instruction(&Instruction::MemorySize(idx as u32));
                        } else if idx == 0 {
                            let next_gid = orig_global_count; // first offset global
                            func.instruction(&Instruction::GlobalGet(next_gid));
                            func.instruction(&Instruction::I32Const(16));
                            func.instruction(&Instruction::I32ShrU);
                        } else if idx == memory_count - 1 {
                            let this_gid = orig_global_count + idx as u32 - 1;
                            func.instruction(&Instruction::MemorySize(0));
                            func.instruction(&Instruction::GlobalGet(this_gid));
                            func.instruction(&Instruction::I32Const(16));
                            func.instruction(&Instruction::I32ShrU);
                            func.instruction(&Instruction::I32Sub);
                        } else {
                            let this_gid = orig_global_count + idx as u32 - 1;
                            let next_gid = orig_global_count + idx as u32;
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
                        func.instruction(&Instruction::Call(new_func_count + idx));
                        func.instruction(&Instruction::LocalSet(return_size));

                        if threads {
                            if let Some(acq) = lock_write_acquire_fn {
                                func.instruction(&Instruction::Call(acq));
                            }
                        }

                        if self.lower_memory {
                            func.instruction(&Instruction::MemorySize(0));
                            func.instruction(&Instruction::LocalSet(combined_size));

                            func.instruction(&Instruction::LocalGet(page_delta));
                            func.instruction(&Instruction::MemoryGrow(0));

                            func.instruction(&Instruction::I32Const(-1));
                            func.instruction(&Instruction::I32Eq);
                            func.instruction(&Instruction::If(wasm_encoder::BlockType::Empty));
                            if threads {
                                if let Some(rel) = lock_write_release_fn {
                                    func.instruction(&Instruction::Call(rel));
                                }
                            }
                            func.instruction(&Instruction::I32Const(-1));
                            func.instruction(&Instruction::Return);
                            func.instruction(&Instruction::End); // end if

                            let next_offset_global = if idx + 1 < memory_count {
                                Some(orig_global_count + idx)
                            } else {
                                None
                            };
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

                                func.instruction(&Instruction::MemoryCopy {
                                    dst_mem: 0,
                                    src_mem: 0,
                                });

                                // Zero the gap that was just created, as dlmalloc expects newly allocated pages to be zeroed
                                // dst: next_gid
                                func.instruction(&Instruction::GlobalGet(next_gid));
                                // val: 0
                                func.instruction(&Instruction::I32Const(0));
                                // len: page_delta * 65536
                                func.instruction(&Instruction::LocalGet(page_delta));
                                func.instruction(&Instruction::I32Const(16));
                                func.instruction(&Instruction::I32Shl);
                                func.instruction(&Instruction::MemoryFill(0));

                                let globals_to_update = (idx + 1)..memory_count;
                                for update_idx in globals_to_update {
                                    let gid = orig_global_count + update_idx - 1;
                                    func.instruction(&Instruction::GlobalGet(gid));
                                    func.instruction(&Instruction::LocalGet(shift_bytes));
                                    func.instruction(&Instruction::I32Add);
                                    func.instruction(&Instruction::GlobalSet(gid));
                                }
                            }
                        } else {
                            func.instruction(&Instruction::LocalGet(page_delta));
                            func.instruction(&Instruction::MemoryGrow(idx as u32));
                            func.instruction(&Instruction::Drop); // We only care about logical size
                        }

                        if threads {
                            if let Some(rel) = lock_write_release_fn {
                                func.instruction(&Instruction::Call(rel));
                            }
                        }

                        func.instruction(&Instruction::LocalGet(return_size));
                        func.instruction(&Instruction::End);

                        new_code_sec.function(&func);
                    }
                    if self.own_memory {
                        for idx in 0..memory_count {
                            let mut func = Function::new(vec![
                                (1, wasm_encoder::ValType::I32), // return_size
                            ]);
                            let page_delta = 0;
                            let return_size = 1;

                            let logical_size_get_fn =
                                crate::wasm_stream::translator::Rebind::function(
                                    &rebinder,
                                    *own_memory_size_get.get(&(idx as u32)).unwrap(),
                                );
                            let logical_size_set_fn =
                                crate::wasm_stream::translator::Rebind::function(
                                    &rebinder,
                                    *own_memory_size_set.get(&(idx as u32)).unwrap(),
                                );

                            // Acquire lock if threaded
                            if threads {
                                if let Some(acq) = lock_write_acquire_fn {
                                    func.instruction(&Instruction::Call(acq));
                                }
                            }

                            // return_size = logical_size_get()
                            func.instruction(&Instruction::Call(logical_size_get_fn));
                            func.instruction(&Instruction::LocalSet(return_size));

                            // max_allowed
                            if !self.lower_memory || memory_count == 1 {
                                func.instruction(&Instruction::MemorySize(idx as u32));
                            } else if idx == 0 {
                                let next_gid = orig_global_count; // first offset global
                                func.instruction(&Instruction::GlobalGet(next_gid));
                                func.instruction(&Instruction::I32Const(16));
                                func.instruction(&Instruction::I32ShrU);
                            } else if idx == memory_count - 1 {
                                let this_gid = orig_global_count + idx as u32 - 1;
                                func.instruction(&Instruction::MemorySize(0));
                                func.instruction(&Instruction::GlobalGet(this_gid));
                                func.instruction(&Instruction::I32Const(16));
                                func.instruction(&Instruction::I32ShrU);
                                func.instruction(&Instruction::I32Sub);
                            } else {
                                let this_gid = orig_global_count + idx as u32 - 1;
                                let next_gid = orig_global_count + idx as u32;
                                func.instruction(&Instruction::GlobalGet(next_gid));
                                func.instruction(&Instruction::GlobalGet(this_gid));
                                func.instruction(&Instruction::I32Sub);
                                func.instruction(&Instruction::I32Const(16));
                                func.instruction(&Instruction::I32ShrU);
                            }

                            // Stack: [max_allowed]
                            func.instruction(&Instruction::LocalGet(return_size));
                            func.instruction(&Instruction::LocalGet(page_delta));
                            func.instruction(&Instruction::I32Add);
                            // Stack: [max_allowed, new_size]
                            func.instruction(&Instruction::I32GeU);

                            func.instruction(&Instruction::If(wasm_encoder::BlockType::Empty));
                            // success
                            func.instruction(&Instruction::Call(logical_size_get_fn));
                            func.instruction(&Instruction::LocalGet(page_delta));
                            func.instruction(&Instruction::I32Add);
                            func.instruction(&Instruction::Call(logical_size_set_fn));

                            if threads {
                                if let Some(rel) = lock_write_release_fn {
                                    func.instruction(&Instruction::Call(rel));
                                }
                            }
                            func.instruction(&Instruction::LocalGet(return_size));
                            func.instruction(&Instruction::Return);
                            func.instruction(&Instruction::End); // end if

                            // failure
                            if threads {
                                if let Some(rel) = lock_write_release_fn {
                                    func.instruction(&Instruction::Call(rel));
                                }
                            }
                            func.instruction(&Instruction::I32Const(-1));
                            func.instruction(&Instruction::End);

                            new_code_sec.function(&func);
                        }
                    }

                    if threads {
                        let mut data = Vec::with_capacity(8);
                        data.extend_from_slice(&new_func_count.to_le_bytes());
                        data.extend_from_slice(&memory_count.to_le_bytes());
                        encoder.section(&wasm_encoder::CustomSection {
                            name: LOWERING_HELPERS_SECTION.into(),
                            data: std::borrow::Cow::Owned(data),
                        });
                    }
                    encoder.section(&new_code_sec);
                }
                Payload::CustomSection(c) => {
                    encoder.section(&wasm_encoder::CustomSection {
                        name: c.name().into(),
                        data: std::borrow::Cow::Borrowed(c.data()),
                    });
                }
                _ => {
                    if let Some((id, range)) = payload.as_section() {
                        encoder.section(&RawSection {
                            id,
                            data: &input_wasm[range.clone()],
                        });
                    }
                }
            }
        }

        Ok(encoder.finish())
    }
}
