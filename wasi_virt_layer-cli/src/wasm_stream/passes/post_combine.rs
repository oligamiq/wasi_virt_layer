use crate::wasm_stream::pipeline::StreamPass;
use crate::wasm_stream::translator::Rebind;
use eyre::Result;
use std::collections::HashMap;
use wasm_encoder::{
    CodeSection, EntityType, ExportSection, FunctionSection, ImportSection, Module, TypeSection,
};

pub struct PostCombineStreamPass {
    pub target_names: Vec<String>,
}

impl PostCombineStreamPass {
    pub fn new(target_names: Vec<String>) -> Self {
        Self { target_names }
    }
}

struct PostCombineRebinder {
    func_map: HashMap<u32, u32>,
}

impl Rebind for PostCombineRebinder {
    fn function(&self, index: u32) -> u32 {
        self.func_map.get(&index).copied().unwrap_or(index)
    }
}

#[derive(Default, Debug)]
struct ParsedInfo {
    dropped_imports: HashMap<u32, String>,
    host_imports: HashMap<u32, String>,
    exported_funcs: HashMap<String, u32>,
    mutable_globals: HashMap<u32, i64>, // index -> initial value
    data_segments: Vec<(u32, i32, Vec<u8>)>, // mem_idx, offset, data bytes
    flesh_vfs_start: Option<u32>,
    pub main_void_funcs: std::collections::HashSet<u32>,
    pub thread_patch: Option<u32>,
    pub wasi_thread_starts: HashMap<String, u32>,
    init_offset_global: Option<u32>,
    save_target_memory: Option<u32>,
    flesh_target_starts: HashMap<String, u32>,
    reset_globals_funcs: HashMap<String, u32>,
    simple_debug_pre_init: Option<u32>,
    original_data_count: u32,
    import_types: HashMap<u32, u32>,
    wasi_thread_initializer: Option<u32>,
}

impl StreamPass for PostCombineStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> eyre::Result<Vec<u8>> {
        std::fs::write("debug_pre_combine.wasm", input_wasm).unwrap();
        let mut info = ParsedInfo::default();
        let mut func_import_count = 0;
        let mut global_count = 0;
        let mut type_count = 0;
        let mut defined_func_count = 0;
        let mut memory_count = 0;
        let mut memory_import_count = 0;

        // Pass 1: Gather Information
        for payload in wasmparser::Parser::new(0).parse_all(input_wasm) {
            match payload? {
                wasmparser::Payload::ImportSection(s) => {
                    for group in s {
                        for i in group?.into_iter() {
                            let (_, import) = i?;
                            if matches!(import.ty, wasmparser::TypeRef::Func(_)) {
                                let is_special = (import.module == "wasip1-vfs"
                                    && import.name.starts_with("__wasip1_vfs_")
                                    && (import.name.ends_with("_memory_copy_from")
                                        || import.name.ends_with("_memory_copy_to")
                                        || import.name.ends_with("_memory_trap")
                                        || import.name.ends_with("_reset")
                                        || import.name.ends_with("__start")
                                        || import.name.ends_with("__main_void")
                                        || import.name.ends_with("_wasi_thread_start")
                                        || import.name.ends_with("_memory_director")
                                        || import.name.ends_with("_reset_on_thread_once")))
                                    || (import.module == "wasip1-vfs_single_memory"
                                        && import.name == "__wasip1_vfs_memory_grow_alt")
                                    || import.module == "wvl_poll"
                                    || import.module == "wvl_atomic";

                                if is_special {
                                    info.dropped_imports
                                        .insert(func_import_count, import.name.to_string());
                                } else if import.module == "__wasip1_vfs-host"
                                    || import.module == "__wasip1_virt_layer"
                                    || (import.module == "env" && import.name == "__wasip1_vfs_wasi_thread_spawn_wrapper")
                                {
                                    info.host_imports
                                        .insert(func_import_count, import.name.to_string());
                                }
                                if let wasmparser::TypeRef::Func(type_index) = import.ty {
                                    info.import_types.insert(func_import_count, type_index);
                                }
                                func_import_count += 1;
                            }
                            if matches!(import.ty, wasmparser::TypeRef::Global(_)) {
                                global_count += 1;
                            }
                            if matches!(import.ty, wasmparser::TypeRef::Memory(_)) {
                                memory_import_count += 1;
                                memory_count += 1;
                            }
                        }
                    }
                }
                wasmparser::Payload::TypeSection(s) => {
                    for ty in s {
                        for _ in ty?.into_types() {
                            type_count += 1;
                        }
                    }
                }
                wasmparser::Payload::FunctionSection(s) => {
                    defined_func_count = s.count();
                }
                wasmparser::Payload::MemorySection(s) => {
                    memory_count += s.count();
                }
                wasmparser::Payload::ExportSection(s) => {
                    for export in s {
                        let export = export?;
                        if export.kind == wasmparser::ExternalKind::Func {
                            info.exported_funcs
                                .insert(export.name.to_string(), export.index);
                            match export.name {
                                "__flesh_vfs_start" => info.flesh_vfs_start = Some(export.index),
                                "__thread_patch" => info.thread_patch = Some(export.index),
                                "__init_offset_global" => {
                                    info.init_offset_global = Some(export.index)
                                }
                                "__save_target_memory" => {
                                    info.save_target_memory = Some(export.index)
                                }
                                "simple_debug_wasip1_vfs_pre_init" => {
                                    info.simple_debug_pre_init = Some(export.index)
                                }
                                "wasi_thread_initializer" => {
                                    info.wasi_thread_initializer = Some(export.index)
                                }
                                name if name.starts_with("__flesh_")
                                    && name.ends_with("_start") =>
                                {
                                    let target_name = name
                                        .strip_prefix("__flesh_")
                                        .unwrap()
                                        .strip_suffix("_start")
                                        .unwrap()
                                        .to_string();
                                    info.flesh_target_starts.insert(target_name, export.index);
                                }
                                name if name.starts_with("__wasip1_vfs_")
                                    && name.ends_with("_wasi_thread_start_anchor") =>
                                {
                                    let target_name = name
                                        .strip_prefix("__wasip1_vfs_")
                                        .unwrap()
                                        .strip_suffix("_wasi_thread_start_anchor")
                                        .unwrap();
                                    info.wasi_thread_starts
                                        .insert(target_name.to_string(), export.index);
                                }
                                name if name.ends_with("__main_void") => {
                                    info.main_void_funcs.insert(export.index);
                                }
                                name if name.starts_with("__wasip1_vfs_")
                                    && name.ends_with("_reset_globals") =>
                                {
                                    let target = name
                                        .strip_prefix("__wasip1_vfs_")
                                        .unwrap()
                                        .strip_suffix("_reset_globals")
                                        .unwrap();
                                    info.reset_globals_funcs
                                        .insert(target.replace("_", "-"), export.index);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                wasmparser::Payload::GlobalSection(s) => {
                    for global in s {
                        let global = global?;
                        if global.ty.mutable {
                            let mut reader = global.init_expr.get_operators_reader();
                            let op = reader.read()?;
                            match op {
                                wasmparser::Operator::I32Const { value } => {
                                    info.mutable_globals.insert(global_count, value as i64);
                                }
                                wasmparser::Operator::I64Const { value } => {
                                    info.mutable_globals.insert(global_count, value);
                                }
                                _ => {}
                            }
                        }
                        global_count += 1;
                    }
                }
                wasmparser::Payload::DataSection(s) => {
                    info.original_data_count = s.count();
                    for data in s {
                        let data = data?;
                        if let wasmparser::DataKind::Active {
                            memory_index,
                            offset_expr,
                        } = data.kind
                        {
                            let mut reader = offset_expr.get_operators_reader();
                            let op = reader.read()?;
                            if let wasmparser::Operator::I32Const { value } = op {
                                info.data_segments
                                    .push((memory_index, value, data.data.to_vec()));
                            }
                        }
                    }
                }
                wasmparser::Payload::DataCountSection { count, .. } => {
                    info.original_data_count = count;
                }
                _ => {}
            }
        }

        // Pass 2: Re-emit and modify
        let mut module = Module::new();

        let mut type_section = TypeSection::new();
        let mut import_section = ImportSection::new();
        let mut function_section = FunctionSection::new();
        let mut memory_section = wasm_encoder::MemorySection::new();
        let mut global_section = wasm_encoder::GlobalSection::new();
        let mut export_section = ExportSection::new();
        let mut code_section = CodeSection::new();
        let mut data_count_section = wasm_encoder::DataCountSection { count: 0 };
        let mut has_data_count = false;

        let mut data_count = 0;

        // Build comprehensive func_map
        let mut func_map = HashMap::new();
        let mut current_new_idx = 0;
        let mut dropped_func_original_indices = Vec::new();

        for i in 0..func_import_count {
            if info.host_imports.contains_key(&i) {
                // Do not assign new index yet! Will route to export's new index below.
            } else if info.dropped_imports.contains_key(&i) {
                dropped_func_original_indices.push(i);
            } else {
                func_map.insert(i, current_new_idx);
                current_new_idx += 1;
            }
        }

        for i in 0..defined_func_count {
            let orig_idx = func_import_count + i;
            func_map.insert(orig_idx, current_new_idx);
            current_new_idx += 1;
        }

        for (import_idx, name) in &info.host_imports {
            if let Some(&export_orig_idx) = info.exported_funcs.get(name) {
                let export_new_idx = func_map.get(&export_orig_idx).copied().unwrap();
                func_map.insert(*import_idx, export_new_idx);
            } else {
                dropped_func_original_indices.push(*import_idx);
                info.dropped_imports
                    .insert(*import_idx, name.clone());
            }
        }

        // Newly injected functions (memory_copy, etc) go at the end
        for orig_idx in &dropped_func_original_indices {
            func_map.insert(*orig_idx, current_new_idx);
            current_new_idx += 1;
        }

        let new_start_idx = current_new_idx;
        current_new_idx += 1;

        let rebinder = PostCombineRebinder { func_map };

        let mut type_mem_copy = None;
        let mut type_void_void = None;
        let mut type_void_i32 = None;
        let mut type_i32_i32_void = None;
        let mut type_i32_i32 = None;

        let mut data_section_opt = None;
        let mut is_after_code = false;
        let mut custom_sections_after_code: Vec<(String, Vec<u8>)> = Vec::new();

        let mut current_defined_func_idx = 0;

        for payload in wasmparser::Parser::new(0).parse_all(input_wasm) {
            match payload? {
                wasmparser::Payload::TypeSection(s) => {
                    let mut type_count = 0;
                    for ty in s {
                        let ty = ty?;
                        for sub_ty in ty.clone().into_types() {
                            if let wasmparser::CompositeInnerType::Func(f) =
                                &sub_ty.composite_type.inner
                            {
                                if f.params()
                                    == [
                                        wasmparser::ValType::I32,
                                        wasmparser::ValType::I32,
                                        wasmparser::ValType::I32,
                                    ]
                                    && f.results().is_empty()
                                {
                                    type_mem_copy = Some(type_count);
                                } else if f.params().is_empty() && f.results().is_empty() {
                                    type_void_void = Some(type_count);
                                } else if f.params().is_empty()
                                    && f.results() == [wasmparser::ValType::I32]
                                {
                                    type_void_i32 = Some(type_count);
                                } else if f.params()
                                    == [wasmparser::ValType::I32, wasmparser::ValType::I32]
                                    && f.results().is_empty()
                                {
                                    type_i32_i32_void = Some(type_count);
                                } else if f.params() == [wasmparser::ValType::I32]
                                    && f.results() == [wasmparser::ValType::I32]
                                {
                                    type_i32_i32 = Some(type_count);
                                }
                            }
                        }
                        if ty.is_explicit_rec_group() {
                            let rec_types = ty
                                .into_types()
                                .map(|sub_ty| {
                                    crate::wasm_stream::translator::translate_sub_type(
                                        &sub_ty, &rebinder,
                                    )
                                })
                                .collect::<Vec<_>>();
                            type_section.ty().rec(rec_types);
                        } else {
                            for sub_ty in ty.into_types() {
                                type_section.ty().subtype(
                                    &crate::wasm_stream::translator::translate_sub_type(
                                        &sub_ty, &rebinder,
                                    ),
                                );
                            }
                        }
                        type_count += 1;
                    }

                    if type_mem_copy.is_none() {
                        type_mem_copy = Some(type_count);
                        type_section.ty().function(
                            vec![
                                wasm_encoder::ValType::I32,
                                wasm_encoder::ValType::I32,
                                wasm_encoder::ValType::I32,
                            ],
                            vec![],
                        );
                        type_count += 1;
                    }
                    if type_void_void.is_none() {
                        type_void_void = Some(type_count);
                        type_section.ty().function(vec![], vec![]);
                        type_count += 1;
                    }
                    if type_void_i32.is_none() {
                        type_void_i32 = Some(type_count);
                        type_section
                            .ty()
                            .function(vec![], vec![wasm_encoder::ValType::I32]);
                        type_count += 1;
                    }
                    if type_i32_i32_void.is_none() {
                        type_i32_i32_void = Some(type_count);
                        type_section.ty().function(
                            vec![wasm_encoder::ValType::I32, wasm_encoder::ValType::I32],
                            vec![],
                        );
                        type_count += 1;
                    }
                    if type_i32_i32.is_none() {
                        type_i32_i32 = Some(type_count);
                        type_section.ty().function(
                            vec![wasm_encoder::ValType::I32],
                            vec![wasm_encoder::ValType::I32],
                        );
                        type_count += 1;
                    }

                    module.section(&type_section);
                }
                wasmparser::Payload::ImportSection(s) => {
                    let mut idx = 0;
                    for group in s {
                        for i in group?.into_iter() {
                            let (_, import) = i?;
                            if matches!(import.ty, wasmparser::TypeRef::Func(_)) {
                                if !info.dropped_imports.contains_key(&idx)
                                    && !info.host_imports.contains_key(&idx)
                                {
                                    let entity = match import.ty {
                                        wasmparser::TypeRef::Func(i) => EntityType::Function(i),
                                        _ => unreachable!(),
                                    };
                                    import_section.import(import.module, import.name, entity);
                                }
                                idx += 1;
                            } else {
                                let entity = match import.ty {
                                    wasmparser::TypeRef::Table(t) => EntityType::Table(
                                        crate::wasm_stream::translator::translate_table_type(
                                            t, &rebinder,
                                        ),
                                    ),
                                    wasmparser::TypeRef::Memory(t) => EntityType::Memory(
                                        crate::wasm_stream::translator::translate_memory_type(t),
                                    ),
                                    wasmparser::TypeRef::Global(g) => EntityType::Global(
                                        crate::wasm_stream::translator::translate_global_type(
                                            g, &rebinder,
                                        ),
                                    ),
                                    wasmparser::TypeRef::Tag(t) => EntityType::Tag(
                                        crate::wasm_stream::translator::translate_tag_type(t),
                                    ),
                                    _ => unimplemented!(),
                                };
                                import_section.import(import.module, import.name, entity);
                            }
                        }
                    }
                    if !import_section.is_empty() {
                        module.section(&import_section);
                    }
                }
                wasmparser::Payload::FunctionSection(s) => {
                    for func in s {
                        function_section.function(func?);
                    }

                    println!("Dropped imports: {:?}", info.dropped_imports);

                    for orig_idx in &dropped_func_original_indices {
                        let type_index = info.import_types.get(orig_idx).unwrap();
                        function_section.function(*type_index);
                    }
                    function_section.function(type_void_void.unwrap()); // start

                    module.section(&function_section);
                }
                wasmparser::Payload::TableSection(s) => {
                    let mut tables = wasm_encoder::TableSection::new();
                    for table in s {
                        tables.table(crate::wasm_stream::translator::translate_table_type(
                            table?.ty, &rebinder,
                        ));
                    }
                    module.section(&tables);
                }
                wasmparser::Payload::MemorySection(s) => {
                    for mem in s {
                        memory_section
                            .memory(crate::wasm_stream::translator::translate_memory_type(mem?));
                    }
                    // TODO: append VFS External Memory here
                    module.section(&memory_section);
                }
                wasmparser::Payload::GlobalSection(s) => {
                    for global in s {
                        let global = global?;
                        let mut instrs = Vec::new();
                        for op in global.init_expr.get_operators_reader() {
                            let op = op?;
                            if matches!(op, wasmparser::Operator::End) {
                                continue;
                            }
                            instrs.push(crate::wasm_stream::translator::translate(&op, &rebinder));
                        }
                        let init_expr = wasm_encoder::ConstExpr::extended(instrs);
                        global_section.global(
                            crate::wasm_stream::translator::translate_global_type(
                                global.ty, &rebinder,
                            ),
                            &init_expr,
                        );
                    }
                    module.section(&global_section);
                }
                wasmparser::Payload::ExportSection(s) => {
                    for export in s {
                        let export = export?;
                        if matches!(
                            export.name,
                            "_start"
                                | "__flesh_vfs_start"
                                | "__thread_patch"
                                | "__init_offset_global"
                                | "__save_target_memory"
                                | "__simple_debug_wasip1_vfs_pre_init"
                                | "__wasip1_vfs_wasi_thread_spawn_wrapper"
                                | "wasi_thread_initializer"
                        ) || (export.name.starts_with("__flesh_")
                            && export.name.ends_with("_start"))
                          || (export.name.starts_with("__wasip1_virt_layer_")
                            && export.name.ends_with("_wrap_unreachable"))
                        {
                            continue; // dropped
                        }
                        if export.kind == wasmparser::ExternalKind::Memory {
                            if export.index != 0 {
                                continue;
                            } // Keep only VFS memory (index 0)
                            export_section.export("memory", wasm_encoder::ExportKind::Memory, 0);
                            continue;
                        }
                        export_section.export(
                            export.name,
                            match export.kind {
                                wasmparser::ExternalKind::Func => wasm_encoder::ExportKind::Func,
                                wasmparser::ExternalKind::Table => wasm_encoder::ExportKind::Table,
                                wasmparser::ExternalKind::Memory => {
                                    wasm_encoder::ExportKind::Memory
                                }
                                wasmparser::ExternalKind::Global => {
                                    wasm_encoder::ExportKind::Global
                                }
                                wasmparser::ExternalKind::Tag => wasm_encoder::ExportKind::Tag,
                                _ => unimplemented!(),
                            },
                            match export.kind {
                                wasmparser::ExternalKind::Func => rebinder.function(export.index),
                                _ => export.index, // other indices are untouched for now
                            },
                        );
                    }
                    export_section.export("_start", wasm_encoder::ExportKind::Func, new_start_idx);
                    module.section(&export_section);
                }
                wasmparser::Payload::StartSection { .. } => {
                    // Do not emit original start, we exported our custom `_start` instead.
                }
                wasmparser::Payload::ElementSection(s) => {
                    let mut elements = wasm_encoder::ElementSection::new();
                    for elem in s {
                        let elem = elem?;
                        let mut funcs_vec = Vec::new();
                        let items = match elem.items {
                            wasmparser::ElementItems::Functions(f) => {
                                funcs_vec = f
                                    .into_iter()
                                    .map(|idx| Ok(rebinder.function(idx?)))
                                    .collect::<Result<Vec<u32>, eyre::Error>>()?;
                                wasm_encoder::Elements::Functions(std::borrow::Cow::Borrowed(
                                    &funcs_vec,
                                ))
                            }
                            wasmparser::ElementItems::Expressions(_e, _) => {
                                unimplemented!(
                                    "ElementItems::Expressions not fully implemented in post_combine rebinder"
                                );
                            }
                        };
                        match elem.kind {
                            wasmparser::ElementKind::Passive => {
                                elements.passive(items);
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
                                    instrs.push(crate::wasm_stream::translator::translate(
                                        &op, &rebinder,
                                    ));
                                }
                                let offset = wasm_encoder::ConstExpr::extended(instrs);
                                elements.active(table_index, &offset, items);
                            }
                            wasmparser::ElementKind::Declared => {
                                elements.declared(items);
                            }
                        }
                    }
                    module.section(&elements);
                }
                wasmparser::Payload::DataCountSection { count, .. } => {
                    data_count_section.count = count + info.data_segments.len() as u32;
                    data_count = count; // original count
                    has_data_count = true;
                }
                wasmparser::Payload::CodeSectionStart { .. } => {
                    is_after_code = true;
                }
                wasmparser::Payload::CodeSectionEntry(body) => {
                    let func_orig_idx = func_import_count + current_defined_func_idx;
                    current_defined_func_idx += 1;
                    let is_start = info.flesh_target_starts.values().any(|&idx| idx == func_orig_idx);

                    let mut locals = Vec::new();
                    for local in body.get_locals_reader()? {
                        let local = local?;
                        locals.push((
                            local.0,
                            crate::wasm_stream::translator::translate_val_type(local.1, &rebinder),
                        ));
                    }
                    let mut func = wasm_encoder::Function::new(locals);
                    for op in body.get_operators_reader()? {
                        let op_unwrapped = op?;
                        let mut skip = false;

                        if let wasmparser::Operator::Call { function_index } = &op_unwrapped {
                            if is_start && info.main_void_funcs.contains(function_index) {
                                func.instruction(&wasm_encoder::Instruction::I32Const(0));
                                skip = true;
                            }
                        }

                        if !skip {
                            func.instruction(&crate::wasm_stream::translator::translate(
                                &op_unwrapped, &rebinder,
                            ));
                        }
                    }
                    code_section.function(&func);
                }
                wasmparser::Payload::DataSection(s) => {
                    let mut data_section = wasm_encoder::DataSection::new();
                    let original_data_count = s.count();
                    for data in s {
                        let data = data?;
                        match data.kind {
                            wasmparser::DataKind::Passive => {
                                data_section.passive(data.data.iter().copied());
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
                                    instrs.push(crate::wasm_stream::translator::translate(
                                        &op, &rebinder,
                                    ));
                                }
                                let offset = wasm_encoder::ConstExpr::extended(instrs);
                                data_section.active(
                                    memory_index,
                                    &offset,
                                    data.data.iter().copied(),
                                );
                            }
                        }
                    }
                    for (_, _, bytes) in &info.data_segments {
                        data_section.passive(bytes.iter().copied());
                    }
                    data_count_section.count = original_data_count + info.data_segments.len() as u32;
                    has_data_count = true;
                    data_section_opt = Some(data_section);
                }
                wasmparser::Payload::CustomSection(s) => {
                    if is_after_code {
                        custom_sections_after_code.push((s.name().to_string(), s.data().to_vec()));
                    } else {
                        module.section(&wasm_encoder::CustomSection {
                            name: s.name().into(),
                            data: std::borrow::Cow::Borrowed(s.data()),
                        });
                    }
                }
                _ => {} // Other sections we pass through or ignore
            }
        }
        for orig_idx in &dropped_func_original_indices {
            let name = info.dropped_imports.get(orig_idx).unwrap();
            let mut func = wasm_encoder::Function::new(vec![]);

            if name.ends_with("_memory_copy_from") {
                let target_name = name
                    .strip_prefix("__wasip1_vfs_")
                    .unwrap()
                    .strip_suffix("_memory_copy_from")
                    .unwrap();
                let wasm_mem = self
                    .target_names
                    .iter()
                    .position(|n| n.replace("-", "_") == target_name)
                    .ok_or_else(|| eyre::eyre!("Target name '{}' not found in provided targets. Ensure the VFS import matches the provided Wasm filename.", target_name))? as u32
                    + 1;
                func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                func.instruction(&wasm_encoder::Instruction::LocalGet(1));
                func.instruction(&wasm_encoder::Instruction::LocalGet(2));
                func.instruction(&wasm_encoder::Instruction::MemoryCopy {
                    src_mem: wasm_mem,
                    dst_mem: 0,
                });
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name.ends_with("_memory_copy_to") {
                let target_name = name
                    .strip_prefix("__wasip1_vfs_")
                    .unwrap()
                    .strip_suffix("_memory_copy_to")
                    .unwrap();
                let wasm_mem = self
                    .target_names
                    .iter()
                    .position(|n| n.replace("-", "_") == target_name)
                    .ok_or_else(|| eyre::eyre!("Target name '{}' not found in provided targets.", target_name))? as u32
                    + 1;
                func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                func.instruction(&wasm_encoder::Instruction::LocalGet(1));
                func.instruction(&wasm_encoder::Instruction::LocalGet(2));
                func.instruction(&wasm_encoder::Instruction::MemoryCopy {
                    src_mem: 0,
                    dst_mem: wasm_mem,
                });
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name.ends_with("_memory_director") {
                let target_name = name
                    .strip_prefix("__wasip1_vfs_")
                    .unwrap()
                    .strip_suffix("_memory_director")
                    .unwrap();
                let wasm_mem = self
                    .target_names
                    .iter()
                    .position(|n| n.replace("-", "_") == target_name)
                    .unwrap() as u32
                    + 1;
                
                func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                
                if wasm_mem > 0 {
                    for i in 0..wasm_mem {
                        func.instruction(&wasm_encoder::Instruction::MemorySize(i));
                        if i > 0 {
                            func.instruction(&wasm_encoder::Instruction::I32Add);
                        }
                    }
                    func.instruction(&wasm_encoder::Instruction::I32Const(65536));
                    func.instruction(&wasm_encoder::Instruction::I32Mul);
                    func.instruction(&wasm_encoder::Instruction::I32Add);
                }
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name.ends_with("_memory_trap") {
                // Just return the ptr to satisfy the anchor
                func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name.ends_with("_memory_grow_alt") {
                func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                func.instruction(&wasm_encoder::Instruction::MemoryGrow(0));
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name.ends_with("__start") || name.ends_with("__main_void") {
                let target_name = if name.ends_with("__start") {
                    name.strip_prefix("__wasip1_vfs_")
                        .unwrap()
                        .strip_suffix("__start")
                        .unwrap()
                } else {
                    name.strip_prefix("__wasip1_vfs_")
                        .unwrap()
                        .strip_suffix("__main_void")
                        .unwrap()
                };
                if let Some(&orig_start_idx) = info.flesh_target_starts.get(target_name) {
                    func.instruction(&wasm_encoder::Instruction::Call(
                        rebinder.function(orig_start_idx),
                    ));
                }
                if name.ends_with("__main_void") {
                    func.instruction(&wasm_encoder::Instruction::I32Const(0));
                }
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name.ends_with("_wasi_thread_start") {
                let target_name = name
                    .strip_prefix("__wasip1_vfs_")
                    .unwrap()
                    .strip_suffix("_wasi_thread_start")
                    .unwrap();
                if let Some(&anchor_idx) = info.wasi_thread_starts.get(target_name) {
                    func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                    func.instruction(&wasm_encoder::Instruction::LocalGet(1));
                    func.instruction(&wasm_encoder::Instruction::Call(
                        rebinder.function(anchor_idx),
                    ));
                }
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name == "__wasip1_vfs_reset_on_thread_once" {
                eprintln!("[START SEQUENCE] processing global reset_on_thread_once");
                // The global reset_on_thread_once simply calls all target-specific reset functions.
                for target_name in &self.target_names {
                    let target_reset_name = format!("__wasip1_vfs_{}_reset", target_name.replace("-", "_"));
                    let mut found_idx = None;
                    for (orig_idx, d_name) in &info.dropped_imports {
                        if d_name == &target_reset_name {
                            found_idx = Some(*orig_idx);
                            break;
                        }
                    }
                    if let Some(idx) = found_idx {
                        func.instruction(&wasm_encoder::Instruction::Call(rebinder.function(idx)));
                    } else {
                        eprintln!("[START SEQUENCE] Warning: target reset {} not found in dropped imports", target_reset_name);
                    }
                }
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name == "__wvl_atomic_wait32_vfs" {
                func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                func.instruction(&wasm_encoder::Instruction::LocalGet(1));
                func.instruction(&wasm_encoder::Instruction::LocalGet(2));
                func.instruction(&wasm_encoder::Instruction::MemoryAtomicWait32(
                    wasm_encoder::MemArg { align: 2, offset: 0, memory_index: 0 },
                ));
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name == "__wvl_atomic_notify_vfs" {
                func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                func.instruction(&wasm_encoder::Instruction::LocalGet(1));
                func.instruction(&wasm_encoder::Instruction::MemoryAtomicNotify(
                    wasm_encoder::MemArg { align: 2, offset: 0, memory_index: 0 },
                ));
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name == "__wvl_atomic_cmpxchg32_vfs" {
                func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                func.instruction(&wasm_encoder::Instruction::LocalGet(1));
                func.instruction(&wasm_encoder::Instruction::LocalGet(2));
                func.instruction(&wasm_encoder::Instruction::I32AtomicRmwCmpxchg(
                    wasm_encoder::MemArg { align: 2, offset: 0, memory_index: 0 },
                ));
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name == "__wvl_atomic_store32_vfs" {
                func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                func.instruction(&wasm_encoder::Instruction::LocalGet(1));
                func.instruction(&wasm_encoder::Instruction::I32AtomicStore(
                    wasm_encoder::MemArg { align: 2, offset: 0, memory_index: 0 },
                ));
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name == "__wvl_atomic_load32_target" || name == "__wvl_atomic_load64_target" {
                for i in 0..self.target_names.len() {
                    let mem_idx = (i + 1) as u32;
                    func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                    func.instruction(&wasm_encoder::Instruction::I32Const(i as i32));
                    func.instruction(&wasm_encoder::Instruction::I32Eq);
                    func.instruction(&wasm_encoder::Instruction::If(wasm_encoder::BlockType::Empty));
                    func.instruction(&wasm_encoder::Instruction::LocalGet(1));
                    if name == "__wvl_atomic_load32_target" {
                        func.instruction(&wasm_encoder::Instruction::I32AtomicLoad(
                            wasm_encoder::MemArg { align: 2, offset: 0, memory_index: mem_idx },
                        ));
                    } else {
                        func.instruction(&wasm_encoder::Instruction::I64AtomicLoad(
                            wasm_encoder::MemArg { align: 3, offset: 0, memory_index: mem_idx },
                        ));
                    }
                    func.instruction(&wasm_encoder::Instruction::Return);
                    func.instruction(&wasm_encoder::Instruction::End);
                }
                func.instruction(&wasm_encoder::Instruction::Unreachable);
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name.ends_with("_reset") {
                eprintln!("[START SEQUENCE] processing reset function: {}", name);
                let target_name = name
                    .strip_prefix("__wasip1_vfs_")
                    .unwrap_or_else(|| panic!("Failed prefix strip: {}", name))
                    .strip_suffix("_reset")
                    .unwrap_or_else(|| panic!("Failed to strip suffix from {}", name));
                let wasm_mem = self
                    .target_names
                    .iter()
                    .position(|n| n.replace("-", "_") == target_name)
                    .unwrap() as u32
                    + 1;

                // 1. Reset globals
                if let Some(&reset_globals_idx) = info.reset_globals_funcs.get(target_name) {
                    func.instruction(&wasm_encoder::Instruction::Call(rebinder.function(reset_globals_idx)));
                }

                // 2. Zero-fill memory
                // To do this simply, we zero-fill the entire target memory `wasm_mem`.
                // memory.size returns pages, so we multiply by 64KB (65536) to get bytes.
                func.instruction(&wasm_encoder::Instruction::I32Const(0)); // dst
                func.instruction(&wasm_encoder::Instruction::I32Const(0)); // val
                func.instruction(&wasm_encoder::Instruction::MemorySize(wasm_mem)); // pages
                func.instruction(&wasm_encoder::Instruction::I32Const(65536));
                func.instruction(&wasm_encoder::Instruction::I32Mul); // length
                func.instruction(&wasm_encoder::Instruction::MemoryFill(wasm_mem));

                // 3. Initialize memory from passive data segments
                let mut data_idx_offset = info.original_data_count;
                for (mem_idx, offset, bytes) in &info.data_segments {
                    if *mem_idx == wasm_mem {
                        func.instruction(&wasm_encoder::Instruction::I32Const(*offset)); // dst
                        func.instruction(&wasm_encoder::Instruction::I32Const(0)); // src
                        func.instruction(&wasm_encoder::Instruction::I32Const(bytes.len() as i32)); // size
                        func.instruction(&wasm_encoder::Instruction::MemoryInit {
                            data_index: data_idx_offset,
                            mem: wasm_mem,
                        });
                    }
                    data_idx_offset += 1;
                }

                // 4. Call target's original start
                if let Some(&start_idx) = info.flesh_target_starts.get(target_name) {
                    func.instruction(&wasm_encoder::Instruction::Call(rebinder.function(start_idx)));
                }
                
                func.instruction(&wasm_encoder::Instruction::End);
            } else {
                func.instruction(&wasm_encoder::Instruction::Unreachable);
                func.instruction(&wasm_encoder::Instruction::End);
            }
            code_section.function(&func);
        }

        // -----------------------------------------------------------------------------
        // [START SEQUENCE]
        // The execution order of start functions is extremely important for virtualization.
        // In the legacy walrus pipeline, this was managed by creating empty dummy functions
        // and later injecting their bodies (via starts.rs and ResetFunc).
        // In the streaming pipeline, we achieve much better developer clarity by directly
        // building the `_start` function here with an explicit, sequentially ordered list of calls.
        // -----------------------------------------------------------------------------
        let mut start_func = wasm_encoder::Function::new(vec![]);
        
        eprintln!("[START SEQUENCE DEBUG] new_start_idx = {new_start_idx}");
        eprintln!("[START SEQUENCE DEBUG] flesh_vfs_start = {:?}", info.flesh_vfs_start);
        eprintln!("[START SEQUENCE DEBUG] thread_patch = {:?}", info.thread_patch);
        eprintln!("[START SEQUENCE DEBUG] init_offset_global = {:?}", info.init_offset_global);
        eprintln!("[START SEQUENCE DEBUG] save_target_memory = {:?}", info.save_target_memory);
        eprintln!("[START SEQUENCE DEBUG] flesh_target_starts = {:?}", info.flesh_target_starts);
        eprintln!("[START SEQUENCE DEBUG] simple_debug_pre_init = {:?}", info.simple_debug_pre_init);
        eprintln!("[START SEQUENCE DEBUG] self.target_names = {:?}", self.target_names);
        eprintln!("[START SEQUENCE DEBUG] reset_globals_funcs = {:?}", info.reset_globals_funcs);
        
        // 1. Initialize the VFS internal state.
        if let Some(idx) = info.flesh_vfs_start {
            eprintln!("[START SEQUENCE] 1. flesh_vfs_start: orig={idx} -> rebind={}", rebinder.function(idx));
            start_func.instruction(&wasm_encoder::Instruction::Call(rebinder.function(idx)));
        }
        
        // 2. Patch thread-local behaviors (if threads are enabled).
        // The VFS or Target might export wasi_thread_initializer (TLS init).
        // Since we removed __thread_patch, we just call it directly here.
        if let Some(idx) = info.wasi_thread_initializer {
            eprintln!("[START SEQUENCE] 2. thread_patch (wasi_thread_initializer): orig={idx} -> rebind={}", rebinder.function(idx));
            start_func.instruction(&wasm_encoder::Instruction::Call(rebinder.function(idx)));
        } else if let Some(idx) = info.thread_patch {
            // Fallback for older __thread_patch logic if it ever gets re-added
            eprintln!("[START SEQUENCE] 2. thread_patch: orig={idx} -> rebind={}", rebinder.function(idx));
            start_func.instruction(&wasm_encoder::Instruction::Call(rebinder.function(idx)));
        }
        
        // 3. Initialize global offsets for memory merging.
        if let Some(idx) = info.init_offset_global {
            eprintln!("[START SEQUENCE] 3. init_offset_global: orig={idx} -> rebind={}", rebinder.function(idx));
            start_func.instruction(&wasm_encoder::Instruction::Call(rebinder.function(idx)));
        }
        
        // 4. Save the initial memory state of the target (for later resets).
        // Note: In the streaming pipeline, memory backup is handled efficiently by preserving
        // passive data segments in the DataSection, so this function may be a no-op empty function.
        if let Some(idx) = info.save_target_memory {
            eprintln!("[START SEQUENCE] 4. save_target_memory: orig={idx} -> rebind={}", rebinder.function(idx));
            start_func.instruction(&wasm_encoder::Instruction::Call(rebinder.function(idx)));
        }
        
        // 5. Finally, invoke the target modules' native start functions.
        for target_name in &self.target_names {
            if let Some(&idx) = info.flesh_target_starts.get(target_name) {
                eprintln!("[START SEQUENCE] 5. flesh_target_start[{target_name}]: orig={idx} -> rebind={}", rebinder.function(idx));
                start_func.instruction(&wasm_encoder::Instruction::Call(rebinder.function(idx)));
            } else {
                eprintln!("[START SEQUENCE] 5. flesh_target_start[{target_name}]: NOT FOUND");
                eprintln!("[START SEQUENCE]    Available keys: {:?}", info.flesh_target_starts.keys().collect::<Vec<_>>());
            }
        }
        
        // 6. Post-initialization debug hooks.
        if let Some(idx) = info.simple_debug_pre_init {
            eprintln!("[START SEQUENCE] 6. simple_debug_pre_init: orig={idx} -> rebind={}", rebinder.function(idx));
            start_func.instruction(&wasm_encoder::Instruction::Call(rebinder.function(idx)));
        }
        
        start_func.instruction(&wasm_encoder::Instruction::End);
        code_section.function(&start_func);

        if has_data_count {
            module.section(&data_count_section);
        }

        module.section(&code_section);

        if let Some(ds) = data_section_opt {
            module.section(&ds);
        }
        for (name, custom_data) in custom_sections_after_code {
            module.section(&wasm_encoder::CustomSection {
                name: name.into(),
                data: std::borrow::Cow::Borrowed(&custom_data),
            });
        }

        let output_wasm = module.finish();
        std::fs::write("debug_stream_post_combine.wasm", &output_wasm).unwrap();
        Ok(output_wasm)
    }
}
