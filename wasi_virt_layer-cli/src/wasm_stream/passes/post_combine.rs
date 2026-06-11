use crate::wasm_stream::passes::fn_in_starts::{FnInStarts, ResolvedStartFuncs};
use crate::wasm_stream::pipeline::StreamPass;
use crate::wasm_stream::translator::Rebind;
use eyre::{Context, Result};
use std::collections::{HashMap, HashSet};
use wasm_encoder::{
    CodeSection, EntityType, ExportSection, FunctionSection, ImportSection, Module, StartSection,
    TypeSection,
};

pub struct PostCombineStreamPass {
    pub vfs_name: String,
    pub target_names: Vec<String>,
    pub defined_funcs_counts: Vec<u32>,
    pub threads: bool,
}

impl PostCombineStreamPass {
    pub fn new(
        vfs_name: String,
        target_names: Vec<String>,
        defined_funcs_counts: Vec<u32>,
        threads: bool,
    ) -> Self {
        Self {
            vfs_name,
            target_names,
            defined_funcs_counts,
            threads,
        }
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
    pub main_void_funcs: HashSet<u32>,
    pub wasi_thread_starts: HashMap<String, u32>,
    reset_globals_funcs: HashMap<String, u32>,
    wrap_unreachable_targets: HashSet<String>,
    set_unreachable_flag_funcs: HashMap<String, u32>,
    original_data_count: u32,
    import_types: HashMap<u32, u32>,
    has_thread_spawn: bool,
    thread_spawn_type_idx: Option<u32>,
    host_thread_spawn_import: Option<u32>,
    host_proc_exit_import: Option<u32>,
    /// Start function indices resolved from export names.
    /// See [`FnInStarts`] for the canonical execution order definition.
    start_funcs: ResolvedStartFuncs,
    flesh_target_start_exports: HashMap<String, u32>,
    normalized_target_start_exports: HashMap<String, u32>,
    target_runtime_starts: HashMap<String, u32>,
}

fn resolve_host_export(name: &str, info: &ParsedInfo) -> Option<u32> {
    if let Some(&export_orig_idx) = info.exported_funcs.get(name) {
        return Some(export_orig_idx);
    }

    let self_suffix = name.strip_prefix("__wasip1_vfs___self_")?;
    if self_suffix == "proc_exit" {
        return None;
    }

    let compact_self_name = format!("__wasip1_vfs_self_{self_suffix}");
    if let Some(&export_orig_idx) = info.exported_funcs.get(&compact_self_name) {
        return Some(export_orig_idx);
    }

    None
}

fn data_segments_for_memory(
    data_segments: &[(u32, i32, Vec<u8>)],
    memory_index: u32,
) -> impl Iterator<Item = (usize, i32, &[u8])> {
    data_segments
        .iter()
        .enumerate()
        .filter_map(move |(segment_index, (mem_idx, offset, bytes))| {
            (*mem_idx == memory_index).then_some((segment_index, *offset, bytes.as_slice()))
        })
}

fn should_drop_export(name: &str) -> bool {
    if matches!(
        name,
        "_start"
            | "__flesh_vfs_start"
            | "__thread_patch"
            | "__save_target_memory"
            | "__simple_debug_wasip1_vfs_pre_init"
            | "simple_debug_wasip1_vfs_pre_init"
            | "__wasip1_vfs_wasi_thread_spawn_wrapper"
            | "__wasip1_vfs_wasi_thread_spawn___self"
            | "__wasip1_vfs_is_root_spawn"
            | "wasi_thread_initializer"
            | "__wasip1_vfs_thread_initializer"
            | "__wasip1_vfs_wasi_thread_start_entry"
    ) {
        return true;
    }

    if (name.starts_with("__flesh_") && name.ends_with("_start"))
        || (name.starts_with("__wasip1_virt_layer_") && name.ends_with("_wrap_unreachable"))
    {
        return true;
    }

    false
}

fn should_emit_custom_section(name: &str, emitted: &mut HashSet<String>) -> bool {
    if name.chars().any(char::is_control) {
        return false;
    }
    if matches!(name, "producers" | "target_features") {
        return emitted.insert(name.to_string());
    }
    true
}

fn wrap_unreachable_proc_exit_target<'a>(name: &'a str, info: &ParsedInfo) -> Option<&'a str> {
    let target_name = name
        .strip_prefix("__wasip1_vfs_")?
        .strip_suffix("_proc_exit")?;
    if target_name == "self" || target_name == "_self" {
        return None;
    }
    info.wrap_unreachable_targets
        .contains(target_name)
        .then_some(target_name)
}

impl StreamPass for PostCombineStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> eyre::Result<Vec<u8>> {
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
                                if import.module
                                    == "wasip1-vfs:host/virtual-file-system-wasip1-threads-import"
                                    && import.name == "[static]wasip1-threads.thread-spawn-import"
                                {
                                    info.host_thread_spawn_import = Some(func_import_count);
                                }
                                if import.module
                                    == "wasip1-vfs:host/virtual-file-system-wasip1-core"
                                    && import.name == "[static]wasip1.proc-exit-import"
                                {
                                    info.host_proc_exit_import = Some(func_import_count);
                                }

                                let is_special = (import.module == "wasip1-vfs"
                                    && import.name.starts_with("__wasip1_vfs_")
                                    && (import.name.ends_with("_memory_copy_from")
                                        || import.name.ends_with("_memory_copy_to")
                                        || import.name.ends_with("_memory_trap")
                                        || import.name.ends_with("_reset")
                                        || import.name.ends_with("__start")
                                        || import.name.ends_with("__main_void")
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
                                    || import.module == "wasip1-vfs"
                                    || import.module == "wasi_snapshot_preview1"
                                    || (import.module == "env"
                                        && import.name == "__wasip1_vfs_wasi_thread_spawn_wrapper")
                                {
                                    info.host_imports
                                        .insert(func_import_count, import.name.to_string());

                                    if import.module == "env"
                                        && import.name == "__wasip1_vfs_wasi_thread_spawn_wrapper"
                                    {
                                        info.has_thread_spawn = true;
                                        if let wasmparser::TypeRef::Func(type_index) = import.ty {
                                            info.thread_spawn_type_idx = Some(type_index);
                                        }
                                    }
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
                                "__flesh_vfs_start" => {
                                    info.start_funcs.flesh_vfs_start = Some(export.index)
                                }
                                "__thread_patch" => {
                                    info.start_funcs.thread_patch = Some(export.index)
                                }
                                "__init_offset_global" => {
                                    info.start_funcs.init_offset_global = Some(export.index)
                                }
                                "__save_target_memory" => {
                                    info.start_funcs.save_target_memory = Some(export.index)
                                }
                                "simple_debug_wasip1_vfs_pre_init" => {
                                    info.start_funcs.simple_debug_pre_init = Some(export.index)
                                }
                                "wasi_thread_initializer" | "__wasip1_vfs_thread_initializer" => {
                                    info.start_funcs.wasi_thread_initializer = Some(export.index)
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
                                    info.flesh_target_start_exports
                                        .entry(target_name)
                                        .or_insert(export.index);
                                }
                                name if name.starts_with("__wasip1_vfs_")
                                    && name.ends_with("__start") =>
                                {
                                    let target_name = name
                                        .strip_prefix("__wasip1_vfs_")
                                        .unwrap()
                                        .strip_suffix("__start")
                                        .unwrap()
                                        .to_string();
                                    info.normalized_target_start_exports
                                        .insert(target_name, export.index);
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
                                name if name.starts_with("__wasip1_virt_layer_")
                                    && name.ends_with("_wrap_unreachable") =>
                                {
                                    let target = name
                                        .strip_prefix("__wasip1_virt_layer_")
                                        .unwrap()
                                        .strip_suffix("_wrap_unreachable")
                                        .unwrap();
                                    info.wrap_unreachable_targets.insert(target.to_string());
                                }
                                name if name.starts_with("__wasip1_virt_layer_")
                                    && name.ends_with("_set_unreachable_flag") =>
                                {
                                    let target = name
                                        .strip_prefix("__wasip1_virt_layer_")
                                        .unwrap()
                                        .strip_suffix("_set_unreachable_flag")
                                        .unwrap();
                                    info.set_unreachable_flag_funcs
                                        .insert(target.to_string(), export.index);
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

        for (target_name, &flesh_idx) in &info.flesh_target_start_exports {
            info.start_funcs
                .flesh_target_starts
                .insert(target_name.clone(), flesh_idx);
        }
        for (target_name, &normalized_idx) in &info.normalized_target_start_exports {
            info.target_runtime_starts
                .insert(target_name.clone(), normalized_idx);
        }
        for (target_name, &flesh_idx) in &info.flesh_target_start_exports {
            info.target_runtime_starts
                .entry(target_name.clone())
                .or_insert(flesh_idx);
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
        let needs_host_proc_exit = info
            .host_imports
            .values()
            .any(|name| name == "__wasip1_vfs___self_proc_exit")
            && info.has_thread_spawn
            && !info.wrap_unreachable_targets.is_empty();

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

        let inject_host_thread_spawn =
            info.has_thread_spawn && info.host_thread_spawn_import.is_none();
        let mut real_thread_spawn_new_idx = info
            .host_thread_spawn_import
            .and_then(|idx| func_map.get(&idx).copied());
        if info.has_thread_spawn {
            if real_thread_spawn_new_idx.is_none() {
                real_thread_spawn_new_idx = Some(current_new_idx);
                current_new_idx += 1;
            }
        }
        let inject_host_proc_exit = needs_host_proc_exit && info.host_proc_exit_import.is_none();
        let mut real_proc_exit_new_idx = info
            .host_proc_exit_import
            .and_then(|idx| func_map.get(&idx).copied());
        if needs_host_proc_exit {
            if real_proc_exit_new_idx.is_none() {
                real_proc_exit_new_idx = Some(current_new_idx);
                current_new_idx += 1;
            }
        }

        for i in 0..defined_func_count {
            let orig_idx = func_import_count + i;
            func_map.insert(orig_idx, current_new_idx);
            current_new_idx += 1;
        }

        let mut unintentionally_dropped_names = Vec::new();
        for (import_idx, name) in &info.host_imports {
            if name == "__wasip1_vfs_wasi_thread_spawn_wrapper" {
                dropped_func_original_indices.push(*import_idx);
                info.dropped_imports.insert(*import_idx, name.clone());
            } else if name == "__wasip1_vfs___self_proc_exit" {
                dropped_func_original_indices.push(*import_idx);
                info.dropped_imports.insert(*import_idx, name.clone());
            } else if wrap_unreachable_proc_exit_target(name, &info).is_some() {
                dropped_func_original_indices.push(*import_idx);
                info.dropped_imports.insert(*import_idx, name.clone());
            } else if let Some(export_orig_idx) = resolve_host_export(name, &info) {
                let export_new_idx = func_map.get(&export_orig_idx).copied().unwrap();
                func_map.insert(*import_idx, export_new_idx);
            } else {
                dropped_func_original_indices.push(*import_idx);
                info.dropped_imports.insert(*import_idx, name.clone());
                unintentionally_dropped_names.push(name.as_str());
            }
        }
        crate::abi::is_valid::validate_unresolved_imports(
            &unintentionally_dropped_names,
            &self.target_names,
        )
        .wrap_err("Failed to translate Wasm to Component")?;
        // Newly injected functions (memory_copy, etc) go at the end
        for orig_idx in &dropped_func_original_indices {
            func_map.insert(*orig_idx, current_new_idx);
            current_new_idx += 1;
        }

        let new_start_idx = current_new_idx;
        current_new_idx += 1;

        let rebinder = PostCombineRebinder {
            func_map: func_map.clone(),
        };

        let mut type_mem_copy = None;
        let mut type_void_void = None;
        let mut type_void_i32 = None;
        let mut type_i32_void = None;
        let mut type_i32_i32_void = None;
        let mut type_i32_i32 = None;

        let mut data_section_opt = None;
        let mut is_after_code = false;
        let mut custom_sections_after_code: Vec<(String, Vec<u8>)> = Vec::new();
        let mut emitted_custom_sections = HashSet::new();

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
                                } else if f.params() == [wasmparser::ValType::I32]
                                    && f.results().is_empty()
                                {
                                    type_i32_void = Some(type_count);
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
                    if type_i32_void.is_none() {
                        type_i32_void = Some(type_count);
                        type_section
                            .ty()
                            .function(vec![wasm_encoder::ValType::I32], vec![]);
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
                    if inject_host_thread_spawn {
                        import_section.import(
                            "wasip1-vfs:host/virtual-file-system-wasip1-threads-import",
                            "[static]wasip1-threads.thread-spawn-import",
                            wasm_encoder::EntityType::Function(info.thread_spawn_type_idx.unwrap()),
                        );
                    }
                    if inject_host_proc_exit {
                        import_section.import(
                            "wasip1-vfs:host/virtual-file-system-wasip1-core",
                            "[static]wasip1.proc-exit-import",
                            wasm_encoder::EntityType::Function(type_i32_void.unwrap()),
                        );
                    }
                    if !import_section.is_empty() {
                        module.section(&import_section);
                    }
                }
                wasmparser::Payload::FunctionSection(s) => {
                    for func in s {
                        function_section.function(func?);
                    }

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
                        if should_drop_export(export.name) {
                            continue; // dropped
                        }
                        if export.kind == wasmparser::ExternalKind::Memory {
                            if export.index != 0 {
                                continue;
                            } // Keep only VFS memory (index 0)
                            export_section.export("memory", wasm_encoder::ExportKind::Memory, 0);
                            continue;
                        }
                        let mut actual_export_idx = match export.kind {
                            wasmparser::ExternalKind::Func => rebinder.function(export.index),
                            _ => export.index,
                        };

                        if export.name == "wasi_thread_start" {
                            if let Some(&entry_idx) = info
                                .exported_funcs
                                .get("__wasip1_vfs_wasi_thread_start_entry")
                            {
                                if let Some(&new_entry_idx) = func_map.get(&entry_idx) {
                                    actual_export_idx = new_entry_idx;
                                }
                            }
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
                            actual_export_idx,
                        );
                    }
                    export_section.export("_start", wasm_encoder::ExportKind::Func, new_start_idx);
                    module.section(&export_section);
                    if self.threads {
                        module.section(&StartSection {
                            function_index: new_start_idx,
                        });
                    }
                }
                wasmparser::Payload::StartSection { .. } => {
                    // The original start has been replaced by the synthesized combined start.
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
                    let current_idx_for_mem = current_defined_func_idx;
                    current_defined_func_idx += 1;
                    let is_start = info
                        .start_funcs
                        .flesh_target_starts
                        .values()
                        .chain(info.target_runtime_starts.values())
                        .any(|&idx| idx == func_orig_idx);

                    let mut locals = Vec::new();
                    for local in body.get_locals_reader()? {
                        let local = local?;
                        locals.push((
                            local.0,
                            crate::wasm_stream::translator::translate_val_type(local.1, &rebinder),
                        ));
                    }
                    let mut func = wasm_encoder::Function::new(locals);

                    let mut mem_idx = 0;
                    let mut cumulative = 0;
                    for (i, &count) in self.defined_funcs_counts.iter().enumerate() {
                        cumulative += count;
                        if current_idx_for_mem < cumulative {
                            mem_idx = i as u32;
                            break;
                        }
                    }

                    for op in body.get_operators_reader()? {
                        let op_unwrapped = op?;
                        let mut skip = false;

                        // Reset functions reuse passive data segments. This matches the walrus
                        // implementation, which removes data.drop from every local function.
                        if matches!(op_unwrapped, wasmparser::Operator::DataDrop { .. }) {
                            continue;
                        }

                        if let wasmparser::Operator::Call { function_index } = &op_unwrapped {
                            if is_start && info.main_void_funcs.contains(function_index) {
                                func.instruction(&wasm_encoder::Instruction::I32Const(0));
                                skip = true;
                            }
                        }

                        if !skip {
                            let mut enc_op =
                                crate::wasm_stream::translator::translate(&op_unwrapped, &rebinder);

                            if mem_idx > 0 {
                                if let Some(mem_info) =
                                    crate::wasm_stream::mem_info::memory_op_info(&op_unwrapped)
                                {
                                    if mem_info.memory == 0 {
                                        crate::wasm_stream::mem_info::set_memory_index(
                                            &mut enc_op,
                                            mem_idx,
                                        );
                                    }
                                }
                            }

                            func.instruction(&enc_op);
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
                    data_count_section.count =
                        original_data_count + info.data_segments.len() as u32;
                    has_data_count = true;
                    data_section_opt = Some(data_section);
                }
                wasmparser::Payload::CustomSection(s) => {
                    if !should_emit_custom_section(s.name(), &mut emitted_custom_sections) {
                        continue;
                    }
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

            if name == "__wasip1_vfs_wasi_thread_spawn_wrapper" {
                let is_root_spawn_orig_idx = info
                    .exported_funcs
                    .get("__wasip1_vfs_is_root_spawn")
                    .unwrap();
                let is_root_spawn_new_idx = func_map.get(is_root_spawn_orig_idx).unwrap();

                let self_thread_spawn_orig_idx = info
                    .exported_funcs
                    .get("__wasip1_vfs_wasi_thread_spawn___self")
                    .unwrap();
                let self_thread_spawn_new_idx = func_map.get(self_thread_spawn_orig_idx).unwrap();

                let real_thread_spawn_new_idx = real_thread_spawn_new_idx.unwrap();

                func.instruction(&wasm_encoder::Instruction::Call(*is_root_spawn_new_idx));
                func.instruction(&wasm_encoder::Instruction::If(
                    wasm_encoder::BlockType::Result(wasm_encoder::ValType::I32),
                ));
                func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                func.instruction(&wasm_encoder::Instruction::Call(real_thread_spawn_new_idx));
                func.instruction(&wasm_encoder::Instruction::Else);
                func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                func.instruction(&wasm_encoder::Instruction::Call(*self_thread_spawn_new_idx));
                func.instruction(&wasm_encoder::Instruction::End);
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name == "__wasip1_vfs___self_proc_exit" && needs_host_proc_exit {
                func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                func.instruction(&wasm_encoder::Instruction::Call(
                    real_proc_exit_new_idx.unwrap(),
                ));
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name.ends_with("_memory_copy_from") {
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
                    src_mem: 0,
                    dst_mem: wasm_mem,
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
                    .ok_or_else(|| {
                        eyre::eyre!(
                            "Target name '{}' not found in provided targets.",
                            target_name
                        )
                    })? as u32
                    + 1;
                func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                func.instruction(&wasm_encoder::Instruction::LocalGet(1));
                func.instruction(&wasm_encoder::Instruction::LocalGet(2));
                func.instruction(&wasm_encoder::Instruction::MemoryCopy {
                    src_mem: wasm_mem,
                    dst_mem: 0,
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
            } else if name == "__wasip1_vfs_wasi_thread_start_entry" {
                if let Some(&start_idx) = info.exported_funcs.get("wasi_thread_start") {
                    let start_new_idx = func_map.get(&start_idx).unwrap();
                    func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                    func.instruction(&wasm_encoder::Instruction::LocalGet(1));
                    func.instruction(&wasm_encoder::Instruction::Call(*start_new_idx));
                }
                func.instruction(&wasm_encoder::Instruction::End);
            } else if let Some(target_name) = wrap_unreachable_proc_exit_target(name, &info) {
                if let Some(&set_flag_idx) = info.set_unreachable_flag_funcs.get(target_name) {
                    func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                    func.instruction(&wasm_encoder::Instruction::Call(
                        rebinder.function(set_flag_idx),
                    ));
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
            } else if name.ends_with("__start") {
                let target_name = name
                    .strip_prefix("__wasip1_vfs_")
                    .unwrap()
                    .strip_suffix("__start")
                    .unwrap();
                if let Some(&orig_start_idx) = info.target_runtime_starts.get(target_name) {
                    func.instruction(&wasm_encoder::Instruction::Call(
                        rebinder.function(orig_start_idx),
                    ));
                }
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name.ends_with("__main_void") {
                let raw_target_name = name.strip_prefix("__wasip1_vfs_").unwrap();
                let target_name = raw_target_name
                    .strip_suffix("___main_void")
                    .or_else(|| {
                        raw_target_name
                            .strip_suffix("__main_void")
                            .map(|name| name.strip_suffix('_').unwrap_or(name))
                    })
                    .unwrap();
                let main_void_name = format!("__wasip1_vfs_{target_name}___main_void");
                if let Some(&orig_main_void_idx) = info.exported_funcs.get(&main_void_name) {
                    func.instruction(&wasm_encoder::Instruction::Call(
                        rebinder.function(orig_main_void_idx),
                    ));
                } else {
                    if let Some(&orig_start_idx) = info.target_runtime_starts.get(target_name) {
                        func.instruction(&wasm_encoder::Instruction::Call(
                            rebinder.function(orig_start_idx),
                        ));
                    }
                    func.instruction(&wasm_encoder::Instruction::I32Const(0));
                }
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name == "__wasip1_vfs_reset_on_thread_once" {
                // Walrus uses this once-only hook to snapshot target memory. Streaming resets
                // directly from retained passive data, so the hook must not reset shared target
                // memories while worker instances are starting concurrently.
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name == "__wvl_poll_atomic_wait" {
                if self.threads {
                    // WaitPoll computes the deadline itself, so use finite wait slices here.
                    // Treat the timeout as unsigned before clamping because a u64 duration
                    // above i64::MAX reaches this ABI as a negative i64 bit pattern.
                    const MAX_POLL_WAIT_NS: i64 = 100_000_000;
                    func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                    func.instruction(&wasm_encoder::Instruction::LocalGet(1));
                    func.instruction(&wasm_encoder::Instruction::LocalGet(2));
                    func.instruction(&wasm_encoder::Instruction::I64Const(MAX_POLL_WAIT_NS));
                    func.instruction(&wasm_encoder::Instruction::LocalGet(2));
                    func.instruction(&wasm_encoder::Instruction::I64Const(MAX_POLL_WAIT_NS));
                    func.instruction(&wasm_encoder::Instruction::I64LtU);
                    func.instruction(&wasm_encoder::Instruction::Select);
                    func.instruction(&wasm_encoder::Instruction::MemoryAtomicWait32(
                        wasm_encoder::MemArg {
                            align: 2,
                            offset: 0,
                            memory_index: 0,
                        },
                    ));
                } else {
                    // 2 is the WebAssembly atomic.wait "timed out" result.
                    func.instruction(&wasm_encoder::Instruction::I32Const(2));
                }
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name == "__wvl_atomic_wait32_vfs" {
                func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                func.instruction(&wasm_encoder::Instruction::LocalGet(1));
                func.instruction(&wasm_encoder::Instruction::LocalGet(2));
                func.instruction(&wasm_encoder::Instruction::MemoryAtomicWait32(
                    wasm_encoder::MemArg {
                        align: 2,
                        offset: 0,
                        memory_index: 0,
                    },
                ));
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name == "__wvl_atomic_notify_vfs" {
                func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                func.instruction(&wasm_encoder::Instruction::LocalGet(1));
                func.instruction(&wasm_encoder::Instruction::MemoryAtomicNotify(
                    wasm_encoder::MemArg {
                        align: 2,
                        offset: 0,
                        memory_index: 0,
                    },
                ));
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name == "__wvl_atomic_cmpxchg32_vfs" {
                func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                func.instruction(&wasm_encoder::Instruction::LocalGet(1));
                func.instruction(&wasm_encoder::Instruction::LocalGet(2));
                func.instruction(&wasm_encoder::Instruction::I32AtomicRmwCmpxchg(
                    wasm_encoder::MemArg {
                        align: 2,
                        offset: 0,
                        memory_index: 0,
                    },
                ));
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name == "__wvl_atomic_store32_vfs" {
                func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                func.instruction(&wasm_encoder::Instruction::LocalGet(1));
                func.instruction(&wasm_encoder::Instruction::I32AtomicStore(
                    wasm_encoder::MemArg {
                        align: 2,
                        offset: 0,
                        memory_index: 0,
                    },
                ));
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name == "__wvl_atomic_load32_target" || name == "__wvl_atomic_load64_target" {
                for i in 0..self.target_names.len() {
                    let mem_idx = (i + 1) as u32;
                    func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                    func.instruction(&wasm_encoder::Instruction::I32Const(i as i32));
                    func.instruction(&wasm_encoder::Instruction::I32Eq);
                    func.instruction(&wasm_encoder::Instruction::If(
                        wasm_encoder::BlockType::Empty,
                    ));
                    func.instruction(&wasm_encoder::Instruction::LocalGet(1));
                    if name == "__wvl_atomic_load32_target" {
                        func.instruction(&wasm_encoder::Instruction::I32AtomicLoad(
                            wasm_encoder::MemArg {
                                align: 2,
                                offset: 0,
                                memory_index: mem_idx,
                            },
                        ));
                    } else {
                        func.instruction(&wasm_encoder::Instruction::I64AtomicLoad(
                            wasm_encoder::MemArg {
                                align: 3,
                                offset: 0,
                                memory_index: mem_idx,
                            },
                        ));
                    }
                    func.instruction(&wasm_encoder::Instruction::Return);
                    func.instruction(&wasm_encoder::Instruction::End);
                }
                func.instruction(&wasm_encoder::Instruction::Unreachable);
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name.ends_with("_reset") {
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
                    func.instruction(&wasm_encoder::Instruction::Call(
                        rebinder.function(reset_globals_idx),
                    ));
                }

                let target_data_segments =
                    data_segments_for_memory(&info.data_segments, wasm_mem).collect::<Vec<_>>();
                // 2. Zero-fill memory
                // memory.size returns pages, so multiply by 64KB to get bytes.
                func.instruction(&wasm_encoder::Instruction::I32Const(0)); // dst
                func.instruction(&wasm_encoder::Instruction::I32Const(0)); // val
                func.instruction(&wasm_encoder::Instruction::MemorySize(wasm_mem)); // pages
                func.instruction(&wasm_encoder::Instruction::I32Const(65536));
                func.instruction(&wasm_encoder::Instruction::I32Mul); // length
                func.instruction(&wasm_encoder::Instruction::MemoryFill(wasm_mem));

                // 3. Restore active data. Passive data is restored by the flesh start below.
                for (segment_index, offset, bytes) in target_data_segments {
                    func.instruction(&wasm_encoder::Instruction::I32Const(offset)); // dst
                    func.instruction(&wasm_encoder::Instruction::I32Const(0)); // src
                    func.instruction(&wasm_encoder::Instruction::I32Const(bytes.len() as i32)); // size
                    func.instruction(&wasm_encoder::Instruction::MemoryInit {
                        data_index: info.original_data_count + segment_index as u32,
                        mem: wasm_mem,
                    });
                }

                // 4. Re-run the target's module start. Besides memory.init, this
                // restores once guards and other runtime state that cannot be
                // reconstructed from data segments alone.
                if let Some(&start_idx) = info.start_funcs.flesh_target_starts.get(target_name) {
                    func.instruction(&wasm_encoder::Instruction::Call(
                        rebinder.function(start_idx),
                    ));
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
        // The execution order of start functions is defined in FnInStarts.
        // See fn_in_starts.rs for the canonical order and detailed documentation.
        // FnInStarts::emit_start_body is the single source of truth for ordering.
        // -----------------------------------------------------------------------------
        let fn_in_starts = FnInStarts::new(&self.target_names);
        let start_func =
            fn_in_starts.emit_start_body(&info.start_funcs, &self.target_names, |idx| {
                rebinder.function(idx)
            });
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

        Ok(module.finish())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_encoder::{
        CodeSection, ExportKind, Function, FunctionSection, Instruction, Module, TypeSection,
    };

    #[test]
    fn emits_synthesized_start_section_and_export() -> eyre::Result<()> {
        let input = module_with_flesh_vfs_start();
        let mut pass = PostCombineStreamPass::new("vfs".to_string(), Vec::new(), vec![1], true);
        let output = pass.run(&input)?;

        let mut exported_start = None;
        let mut section_start = None;

        for payload in wasmparser::Parser::new(0).parse_all(&output) {
            match payload? {
                wasmparser::Payload::ExportSection(exports) => {
                    for export in exports {
                        let export = export?;
                        if export.name == "_start" {
                            exported_start = Some(export.index);
                        }
                    }
                }
                wasmparser::Payload::StartSection { func, .. } => {
                    section_start = Some(func);
                }
                _ => {}
            }
        }

        assert_eq!(exported_start, section_start);
        assert_eq!(section_start, Some(1));
        Ok(())
    }

    #[test]
    fn synthesized_start_prefers_vfs_thread_initializer_export() -> eyre::Result<()> {
        let input = module_with_thread_initializer();
        let mut pass = PostCombineStreamPass::new("vfs".to_string(), Vec::new(), vec![3], true);
        let output = pass.run(&input)?;

        let mut start = None;
        let mut calls = Vec::new();
        let mut func_idx = 0;

        for payload in wasmparser::Parser::new(0).parse_all(&output) {
            match payload? {
                wasmparser::Payload::StartSection { func, .. } => {
                    start = Some(func);
                }
                wasmparser::Payload::CodeSectionEntry(body) => {
                    if Some(func_idx) == start {
                        for op in body.get_operators_reader()? {
                            if let wasmparser::Operator::Call { function_index } = op? {
                                calls.push(function_index);
                            }
                        }
                    }
                    func_idx += 1;
                }
                _ => {}
            }
        }

        assert_eq!(start, Some(3));
        assert_eq!(calls, vec![0, 1]);
        Ok(())
    }

    #[test]
    fn synthesized_start_uses_flesh_target_start_export() -> eyre::Result<()> {
        let input = module_with_target_start_pair();
        let mut pass = PostCombineStreamPass::new(
            "vfs".to_string(),
            vec!["target".to_string()],
            vec![3],
            true,
        );
        let output = pass.run(&input)?;

        let calls = start_calls(&output)?;
        assert_eq!(calls, vec![0, 1]);
        Ok(())
    }

    #[test]
    fn reset_data_segments_are_scoped_to_the_target_memory() {
        let segments = vec![
            (1, 16, vec![1, 2]),
            (2, 32, vec![3, 4, 5]),
            (1, 48, vec![6]),
        ];

        let selected = data_segments_for_memory(&segments, 2).collect::<Vec<_>>();

        assert_eq!(selected, vec![(1, 32, &[3, 4, 5][..])]);
    }

    #[test]
    fn drops_pipeline_exports_but_keeps_late_link_exports() {
        assert!(should_drop_export("__flesh_target_name_start"));
        assert!(should_drop_export(
            "__wasip1_virt_layer_target_name_wrap_unreachable"
        ));
        assert!(!should_drop_export("__wasip1_vfs_target_name_args_get"));
        assert!(!should_drop_export(
            "__wasip1_vfs_target_name_wasi_thread_start"
        ));
    }

    #[test]
    fn filters_invalid_and_duplicate_custom_sections() {
        let mut emitted = HashSet::new();

        assert!(should_emit_custom_section("producers", &mut emitted));
        assert!(!should_emit_custom_section("producers", &mut emitted));
        assert!(!should_emit_custom_section("\u{8}l", &mut emitted));
        assert!(should_emit_custom_section("component-name", &mut emitted));
    }

    #[test]
    fn non_threaded_build_does_not_emit_start_section() -> eyre::Result<()> {
        let input = module_with_flesh_vfs_start();
        let mut pass = PostCombineStreamPass::new("vfs".to_string(), Vec::new(), vec![1], false);
        let output = pass.run(&input)?;

        let has_start_section =
            wasmparser::Parser::new(0)
                .parse_all(&output)
                .try_fold(false, |found, payload| {
                    Ok::<_, wasmparser::BinaryReaderError>(
                        found || matches!(payload?, wasmparser::Payload::StartSection { .. }),
                    )
                })?;

        assert!(!has_start_section);
        Ok(())
    }

    #[test]
    fn implements_poll_wait_with_atomic_wait_for_threads() -> eyre::Result<()> {
        let input = module_with_poll_wait_import();
        let mut pass = PostCombineStreamPass::new("vfs".to_string(), Vec::new(), vec![1], true);
        let output = pass.run(&input)?;

        let mut found_wait = false;
        let mut found_unsigned_clamp = false;
        for payload in wasmparser::Parser::new(0).parse_all(&output) {
            if let wasmparser::Payload::CodeSectionEntry(body) = payload? {
                let ops = body
                    .get_operators_reader()?
                    .into_iter()
                    .collect::<Result<Vec<_>, _>>()?;
                for op in &ops {
                    if matches!(
                        op,
                        wasmparser::Operator::MemoryAtomicWait32 {
                            memarg: wasmparser::MemArg { memory: 0, .. }
                        }
                    ) {
                        found_wait = true;
                    }
                }
                found_unsigned_clamp |= ops.windows(4).any(|ops| {
                    matches!(
                        ops,
                        [
                            wasmparser::Operator::LocalGet { local_index: 2 },
                            wasmparser::Operator::I64Const { value: 100_000_000 },
                            wasmparser::Operator::I64LtU,
                            wasmparser::Operator::Select
                        ]
                    )
                });
            }
        }

        assert!(found_wait);
        assert!(found_unsigned_clamp);
        Ok(())
    }

    #[test]
    fn implements_poll_wait_as_timeout_without_threads() -> eyre::Result<()> {
        let input = module_with_poll_wait_import();
        let mut pass = PostCombineStreamPass::new("vfs".to_string(), Vec::new(), vec![1], false);
        let output = pass.run(&input)?;

        let mut found = false;
        for payload in wasmparser::Parser::new(0).parse_all(&output) {
            if let wasmparser::Payload::CodeSectionEntry(body) = payload? {
                let ops = body
                    .get_operators_reader()?
                    .into_iter()
                    .collect::<Result<Vec<_>, _>>()?;
                if matches!(
                    ops.as_slice(),
                    [
                        wasmparser::Operator::I32Const { value: 2 },
                        wasmparser::Operator::End
                    ]
                ) {
                    found = true;
                }
            }
        }

        assert!(found);
        Ok(())
    }

    fn start_calls(output: &[u8]) -> eyre::Result<Vec<u32>> {
        let mut start = None;
        let mut calls = Vec::new();
        let mut func_idx = 0;

        for payload in wasmparser::Parser::new(0).parse_all(output) {
            match payload? {
                wasmparser::Payload::StartSection { func, .. } => {
                    start = Some(func);
                }
                wasmparser::Payload::CodeSectionEntry(body) => {
                    if Some(func_idx) == start {
                        for op in body.get_operators_reader()? {
                            if let wasmparser::Operator::Call { function_index } = op? {
                                calls.push(function_index);
                            }
                        }
                    }
                    func_idx += 1;
                }
                _ => {}
            }
        }

        Ok(calls)
    }

    fn module_with_flesh_vfs_start() -> Vec<u8> {
        let mut module = Module::new();

        let mut types = TypeSection::new();
        types.ty().function([], []);
        module.section(&types);

        let mut functions = FunctionSection::new();
        functions.function(0);
        module.section(&functions);

        let mut exports = ExportSection::new();
        exports.export("__flesh_vfs_start", ExportKind::Func, 0);
        module.section(&exports);

        let mut code = CodeSection::new();
        let mut func = Function::new([]);
        func.instruction(&Instruction::End);
        code.function(&func);
        module.section(&code);

        module.finish()
    }

    fn module_with_thread_initializer() -> Vec<u8> {
        let mut module = Module::new();

        let mut types = TypeSection::new();
        types.ty().function([], []);
        module.section(&types);

        let mut functions = FunctionSection::new();
        functions.function(0);
        functions.function(0);
        functions.function(0);
        module.section(&functions);

        let mut exports = ExportSection::new();
        exports.export("__flesh_vfs_start", ExportKind::Func, 0);
        exports.export("__wasip1_vfs_thread_initializer", ExportKind::Func, 1);
        exports.export("__thread_patch", ExportKind::Func, 2);
        module.section(&exports);

        let mut code = CodeSection::new();
        for _ in 0..3 {
            let mut func = Function::new([]);
            func.instruction(&Instruction::End);
            code.function(&func);
        }
        module.section(&code);

        module.finish()
    }

    fn module_with_target_start_pair() -> Vec<u8> {
        let mut module = Module::new();

        let mut types = TypeSection::new();
        types.ty().function([], []);
        module.section(&types);

        let mut functions = FunctionSection::new();
        functions.function(0);
        functions.function(0);
        functions.function(0);
        module.section(&functions);

        let mut exports = ExportSection::new();
        exports.export("__flesh_vfs_start", ExportKind::Func, 0);
        exports.export("__wasip1_vfs_target__start", ExportKind::Func, 2);
        exports.export("__flesh_target_start", ExportKind::Func, 1);
        module.section(&exports);

        let mut code = CodeSection::new();
        for _ in 0..3 {
            let mut func = Function::new([]);
            func.instruction(&Instruction::End);
            code.function(&func);
        }
        module.section(&code);

        module.finish()
    }

    fn module_with_poll_wait_import() -> Vec<u8> {
        let mut module = Module::new();

        let mut types = TypeSection::new();
        types.ty().function(
            [
                wasm_encoder::ValType::I32,
                wasm_encoder::ValType::I32,
                wasm_encoder::ValType::I64,
            ],
            [wasm_encoder::ValType::I32],
        );
        types.ty().function([], []);
        module.section(&types);

        let mut imports = wasm_encoder::ImportSection::new();
        imports.import(
            "wvl_poll",
            "__wvl_poll_atomic_wait",
            wasm_encoder::EntityType::Function(0),
        );
        module.section(&imports);

        let mut functions = FunctionSection::new();
        functions.function(1);
        module.section(&functions);

        let mut memories = wasm_encoder::MemorySection::new();
        memories.memory(wasm_encoder::MemoryType {
            minimum: 1,
            maximum: Some(1),
            memory64: false,
            shared: true,
            page_size_log2: None,
        });
        module.section(&memories);

        let mut exports = ExportSection::new();
        exports.export("__flesh_vfs_start", ExportKind::Func, 1);
        module.section(&exports);

        let mut code = CodeSection::new();
        let mut func = Function::new([]);
        func.instruction(&Instruction::End);
        code.function(&func);
        module.section(&code);

        module.finish()
    }
}
