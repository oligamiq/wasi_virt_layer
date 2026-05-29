use eyre::Result;
use wasm_encoder::{Module, Section, RawSection, ExportSection, ExportKind, FunctionSection, CodeSection, ImportSection, EntityType, TypeSection};
use std::collections::HashMap;
use crate::wasm_stream::pipeline::StreamPass;
use crate::wasm_stream::tracker::IndexTracker;
use crate::wasm_stream::translator::{translate, Rebind};
use crate::unique_name::UniqueName;
use crate::generator::memory::MemoryUniqueName;
use crate::generator::special_func::SpecialFuncUniqueName;

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
    data_segments: Vec<(u32, i32, usize)>, // mem_idx, offset, length
    flesh_vfs_start: Option<u32>,
    pub thread_patch: Option<u32>,
    pub wasi_thread_starts: HashMap<String, u32>,
    init_offset_global: Option<u32>,
    save_target_memory: Option<u32>,
    flesh_target_starts: HashMap<String, u32>,
    simple_debug_pre_init: Option<u32>,
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
                                let is_special = (import.module == "wasip1-vfs" && import.name.starts_with("__wasip1_vfs_") && (
                                    import.name.ends_with("_memory_copy_from") ||
                                    import.name.ends_with("_memory_copy_to") ||
                                    import.name.ends_with("_memory_trap") ||
                                    import.name.ends_with("_reset") ||
                                    import.name.ends_with("__start") ||
                                    import.name.ends_with("__main_void") ||
                                    import.name.ends_with("_wasi_thread_start") ||
                                    import.name.ends_with("_reset_on_thread_once")
                                )) || (import.module == "wasip1-vfs_single_memory" && import.name == "__wasip1_vfs_memory_grow_alt");
                                
                                if is_special {
                                    info.dropped_imports.insert(func_import_count, import.name.to_string());
                                } else if import.module == "__wasip1_vfs-host" {
                                    info.host_imports.insert(func_import_count, import.name.to_string());
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
                            info.exported_funcs.insert(export.name.to_string(), export.index);
                            match export.name {
                                "__flesh_vfs_start" => info.flesh_vfs_start = Some(export.index),
                                "__thread_patch" => info.thread_patch = Some(export.index),
                                "__init_offset_global" => info.init_offset_global = Some(export.index),
                                "__save_target_memory" => info.save_target_memory = Some(export.index),
                                "__simple_debug_wasip1_vfs_pre_init" => info.simple_debug_pre_init = Some(export.index),
                                name if name.starts_with("__flesh_") && name.ends_with("_start") => {
                                    let target_name = name.strip_prefix("__flesh_").unwrap().strip_suffix("_start").unwrap().to_string();
                                    info.flesh_target_starts.insert(target_name, export.index);
                                }
                                name if name.starts_with("__wasip1_vfs_") && name.ends_with("_wasi_thread_start_anchor") => {
                                    let target_name = name.strip_prefix("__wasip1_vfs_").unwrap().strip_suffix("_wasi_thread_start_anchor").unwrap();
                                    info.wasi_thread_starts.insert(target_name.to_string(), export.index);
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
                    for data in s {
                        let data = data?;
                        if let wasmparser::DataKind::Active { memory_index, offset_expr } = data.kind {
                            let mut reader = offset_expr.get_operators_reader();
                            let op = reader.read()?;
                            if let wasmparser::Operator::I32Const { value } = op {
                                info.data_segments.push((memory_index, value, data.data.len()));
                            }
                        }
                    }
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
                info.dropped_imports.insert(*import_idx, "_memory_trap".to_string());
            }
        }

        // Newly injected functions (memory_copy, etc) go at the end
        for orig_idx in &dropped_func_original_indices {
            func_map.insert(*orig_idx, current_new_idx);
            current_new_idx += 1;
        }

        let new_start_idx = current_new_idx;
        current_new_idx += 1;

        let rebinder = PostCombineRebinder {
            func_map,
        };
        
        let mut type_mem_copy = None;
        let mut type_void_void = None;
        let mut type_void_i32 = None;
        let mut type_i32_i32_void = None;
        let mut type_i32_i32 = None;
        let mut has_start_section_emitted = false;
        
        let mut data_section_opt = None;
        let mut is_after_code = false;
        let mut custom_sections_after_code: Vec<(String, Vec<u8>)> = Vec::new();
        
        for payload in wasmparser::Parser::new(0).parse_all(input_wasm) {
            match payload? {
                wasmparser::Payload::TypeSection(s) => {
                    let mut type_count = 0;
                    for ty in s {
                        let ty = ty?;
                        for sub_ty in ty.clone().into_types() {
                            if let wasmparser::CompositeInnerType::Func(f) = &sub_ty.composite_type.inner {
                                if f.params() == [wasmparser::ValType::I32, wasmparser::ValType::I32, wasmparser::ValType::I32] && f.results().is_empty() {
                                    type_mem_copy = Some(type_count);
                                } else if f.params().is_empty() && f.results().is_empty() {
                                    type_void_void = Some(type_count);
                                } else if f.params().is_empty() && f.results() == [wasmparser::ValType::I32] {
                                    type_void_i32 = Some(type_count);
                                } else if f.params() == [wasmparser::ValType::I32, wasmparser::ValType::I32] && f.results().is_empty() {
                                    type_i32_i32_void = Some(type_count);
                                } else if f.params() == [wasmparser::ValType::I32] && f.results() == [wasmparser::ValType::I32] {
                                    type_i32_i32 = Some(type_count);
                                }
                            }
                        }
                        if ty.is_explicit_rec_group() {
                            let rec_types = ty.into_types().map(|sub_ty| {
                                crate::wasm_stream::translator::translate_sub_type(&sub_ty, &rebinder)
                            }).collect::<Vec<_>>();
                            type_section.ty().rec(rec_types);
                        } else {
                            for sub_ty in ty.into_types() {
                                type_section.ty().subtype(&crate::wasm_stream::translator::translate_sub_type(&sub_ty, &rebinder));
                            }
                        }
                        type_count += 1;
                    }
                    
                    if type_mem_copy.is_none() {
                        type_mem_copy = Some(type_count);
                        type_section.ty().function(vec![wasm_encoder::ValType::I32, wasm_encoder::ValType::I32, wasm_encoder::ValType::I32], vec![]);
                        type_count += 1;
                    }
                    if type_void_void.is_none() {
                        type_void_void = Some(type_count);
                        type_section.ty().function(vec![], vec![]);
                        type_count += 1;
                    }
                    if type_void_i32.is_none() {
                        type_void_i32 = Some(type_count);
                        type_section.ty().function(vec![], vec![wasm_encoder::ValType::I32]);
                        type_count += 1;
                    }
                    if type_i32_i32_void.is_none() {
                        type_i32_i32_void = Some(type_count);
                        type_section.ty().function(vec![wasm_encoder::ValType::I32, wasm_encoder::ValType::I32], vec![]);
                        type_count += 1;
                    }
                    if type_i32_i32.is_none() {
                        type_i32_i32 = Some(type_count);
                        type_section.ty().function(vec![wasm_encoder::ValType::I32], vec![wasm_encoder::ValType::I32]);
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
                                if !info.dropped_imports.contains_key(&idx) {
                                    let entity = match import.ty {
                                        wasmparser::TypeRef::Func(i) => EntityType::Function(i),
                                        _ => unreachable!(),
                                    };
                                    import_section.import(import.module, import.name, entity);
                                }
                                idx += 1;
                            } else {
                                let entity = match import.ty {
                                    wasmparser::TypeRef::Table(t) => EntityType::Table(crate::wasm_stream::translator::translate_table_type(t, &rebinder)),
                                    wasmparser::TypeRef::Memory(t) => EntityType::Memory(crate::wasm_stream::translator::translate_memory_type(t)),
                                    wasmparser::TypeRef::Global(g) => EntityType::Global(crate::wasm_stream::translator::translate_global_type(g, &rebinder)),
                                    wasmparser::TypeRef::Tag(t) => EntityType::Tag(crate::wasm_stream::translator::translate_tag_type(t)),
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
                        let name = info.dropped_imports.get(orig_idx).unwrap();
                        if name.contains("memory_copy") {
                            function_section.function(type_mem_copy.unwrap());
                        } else if name.ends_with("_memory_grow_alt") {
                            function_section.function(type_i32_i32.unwrap());
                        } else if name.ends_with("___main_void") {
                            function_section.function(type_void_i32.unwrap());
                        } else if name.ends_with("_wasi_thread_start") {
                            function_section.function(type_i32_i32_void.unwrap());
                        } else {
                            function_section.function(type_void_void.unwrap());
                        }
                    }
                    function_section.function(type_void_void.unwrap()); // start
                    
                    module.section(&function_section);
                }
                wasmparser::Payload::TableSection(s) => {
                    let mut tables = wasm_encoder::TableSection::new();
                    for table in s {
                        tables.table(crate::wasm_stream::translator::translate_table_type(table?.ty, &rebinder));
                    }
                    module.section(&tables);
                }
                wasmparser::Payload::MemorySection(s) => {
                    for mem in s {
                        memory_section.memory(crate::wasm_stream::translator::translate_memory_type(mem?));
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
                            if matches!(op, wasmparser::Operator::End) { continue; }
                            instrs.push(crate::wasm_stream::translator::translate(&op, &rebinder));
                        }
                        let init_expr = wasm_encoder::ConstExpr::extended(instrs);
                        global_section.global(crate::wasm_stream::translator::translate_global_type(global.ty, &rebinder), &init_expr);
                    }
                    module.section(&global_section);
                }
                wasmparser::Payload::ExportSection(s) => {
                    for export in s {
                        let export = export?;
                        if matches!(export.name, "__flesh_vfs_start" | "__thread_patch" | "__init_offset_global" | "__save_target_memory" | "__simple_debug_wasip1_vfs_pre_init") || (export.name.starts_with("__flesh_") && export.name.ends_with("_start")) {
                            continue; // dropped
                        }
                        if export.kind == wasmparser::ExternalKind::Memory {
                            if export.index != 0 { continue; } // Keep only VFS memory (index 0)
                            export_section.export("memory", wasm_encoder::ExportKind::Memory, 0);
                            continue;
                        }
                        export_section.export(export.name, match export.kind {
                            wasmparser::ExternalKind::Func => wasm_encoder::ExportKind::Func,
                            wasmparser::ExternalKind::Table => wasm_encoder::ExportKind::Table,
                            wasmparser::ExternalKind::Memory => wasm_encoder::ExportKind::Memory,
                            wasmparser::ExternalKind::Global => wasm_encoder::ExportKind::Global,
                            wasmparser::ExternalKind::Tag => wasm_encoder::ExportKind::Tag,
                            _ => unimplemented!(),
                        }, match export.kind {
                            wasmparser::ExternalKind::Func => rebinder.function(export.index),
                            _ => export.index, // other indices are untouched for now
                        });
                    }
                    module.section(&export_section);
                    module.section(&wasm_encoder::StartSection { function_index: new_start_idx });
                    has_start_section_emitted = true;
                }
                wasmparser::Payload::StartSection { .. } => {
                    // Do not emit original start, we already emitted our custom one right after ExportSection.
                }
                wasmparser::Payload::ElementSection(s) => {
                    let mut elements = wasm_encoder::ElementSection::new();
                    for elem in s {
                        let elem = elem?;
                        let mut funcs_vec = Vec::new();
                        let items = match elem.items {
                            wasmparser::ElementItems::Functions(f) => {
                                funcs_vec = f.into_iter().map(|idx| Ok(rebinder.function(idx?))).collect::<Result<Vec<u32>, eyre::Error>>()?;
                                wasm_encoder::Elements::Functions(std::borrow::Cow::Borrowed(&funcs_vec))
                            }
                            wasmparser::ElementItems::Expressions(e, _) => {
                                unimplemented!("ElementItems::Expressions not fully implemented in post_combine rebinder");
                            }
                        };
                        match elem.kind {
                            wasmparser::ElementKind::Passive => {
                                elements.passive(items);
                            }
                            wasmparser::ElementKind::Active { table_index, offset_expr } => {
                                let mut instrs = Vec::new();
                                for op in offset_expr.get_operators_reader() {
                                    let op = op?;
                                    if matches!(op, wasmparser::Operator::End) { continue; }
                                    instrs.push(crate::wasm_stream::translator::translate(&op, &rebinder));
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
                    data_count_section.count = count;
                    data_count = count;
                    has_data_count = true;
                }
                wasmparser::Payload::CodeSectionStart { .. } => {
                    is_after_code = true;
                }
                wasmparser::Payload::CodeSectionEntry(body) => {
                    let mut locals = Vec::new();
                    for local in body.get_locals_reader()? {
                        let local = local?;
                        locals.push((local.0, crate::wasm_stream::translator::translate_val_type(local.1, &rebinder)));
                    }
                    let mut func = wasm_encoder::Function::new(locals);
                    for op in body.get_operators_reader()? {
                        func.instruction(&crate::wasm_stream::translator::translate(&op?, &rebinder));
                    }
                    code_section.function(&func);
                }
                wasmparser::Payload::DataSection(s) => {
                    let mut data_section = wasm_encoder::DataSection::new();
                    for data in s {
                        let data = data?;
                        match data.kind {
                            wasmparser::DataKind::Passive => {
                                data_section.passive(data.data.iter().copied());
                            }
                            wasmparser::DataKind::Active { memory_index, offset_expr } => {
                                let mut instrs = Vec::new();
                                for op in offset_expr.get_operators_reader() {
                                    let op = op?;
                                    if matches!(op, wasmparser::Operator::End) { continue; }
                                    instrs.push(crate::wasm_stream::translator::translate(&op, &rebinder));
                                }
                                let offset = wasm_encoder::ConstExpr::extended(instrs);
                                data_section.active(memory_index, &offset, data.data.iter().copied());
                            }
                        }
                    }
                    // TODO: append new reset data segment here
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
                let target_name = name.strip_prefix("__wasip1_vfs_").unwrap().strip_suffix("_memory_copy_from").unwrap();
                let wasm_mem = self.target_names.iter().position(|n| n == target_name).unwrap() as u32 + 1;
                func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                func.instruction(&wasm_encoder::Instruction::LocalGet(1));
                func.instruction(&wasm_encoder::Instruction::LocalGet(2));
                func.instruction(&wasm_encoder::Instruction::MemoryCopy { src_mem: wasm_mem, dst_mem: 0 });
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name.ends_with("_memory_copy_to") {
                let target_name = name.strip_prefix("__wasip1_vfs_").unwrap().strip_suffix("_memory_copy_to").unwrap();
                let wasm_mem = self.target_names.iter().position(|n| n == target_name).unwrap() as u32 + 1;
                func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                func.instruction(&wasm_encoder::Instruction::LocalGet(1));
                func.instruction(&wasm_encoder::Instruction::LocalGet(2));
                func.instruction(&wasm_encoder::Instruction::MemoryCopy { src_mem: 0, dst_mem: wasm_mem });
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name.ends_with("_memory_trap") {
                func.instruction(&wasm_encoder::Instruction::Unreachable);
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name.ends_with("_memory_grow_alt") {
                func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                func.instruction(&wasm_encoder::Instruction::MemoryGrow(0));
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name.ends_with("__start") || name.ends_with("__main_void") {
                let target_name = if name.ends_with("__start") {
                    name.strip_prefix("__wasip1_vfs_").unwrap().strip_suffix("__start").unwrap()
                } else {
                    name.strip_prefix("__wasip1_vfs_").unwrap().strip_suffix("__main_void").unwrap()
                };
                if let Some(&orig_start_idx) = info.flesh_target_starts.get(target_name) {
                    func.instruction(&wasm_encoder::Instruction::Call(rebinder.function(orig_start_idx)));
                }
                if name.ends_with("__main_void") {
                    func.instruction(&wasm_encoder::Instruction::I32Const(0));
                }
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name.ends_with("_wasi_thread_start") {
                let target_name = name.strip_prefix("__wasip1_vfs_").unwrap().strip_suffix("_wasi_thread_start").unwrap();
                func.instruction(&wasm_encoder::Instruction::LocalGet(0));
                func.instruction(&wasm_encoder::Instruction::LocalGet(1));
                if let Some(&anchor_idx) = info.wasi_thread_starts.get(target_name) {
                    func.instruction(&wasm_encoder::Instruction::Call(rebinder.function(anchor_idx)));
                }
                func.instruction(&wasm_encoder::Instruction::End);
            } else if name.ends_with("_reset") || name.ends_with("_reset_on_thread_once") {
                func.instruction(&wasm_encoder::Instruction::End);
            } else {
                func.instruction(&wasm_encoder::Instruction::Unreachable);
                func.instruction(&wasm_encoder::Instruction::End);
            }
            code_section.function(&func);
        }

        let mut start_func = wasm_encoder::Function::new(vec![]);
        if let Some(idx) = info.flesh_vfs_start { start_func.instruction(&wasm_encoder::Instruction::Call(rebinder.function(idx))); }
        if let Some(idx) = info.thread_patch { start_func.instruction(&wasm_encoder::Instruction::Call(rebinder.function(idx))); }
        if let Some(idx) = info.init_offset_global { start_func.instruction(&wasm_encoder::Instruction::Call(rebinder.function(idx))); }
        if let Some(idx) = info.save_target_memory { start_func.instruction(&wasm_encoder::Instruction::Call(rebinder.function(idx))); }
        for target_name in &self.target_names {
            if let Some(&idx) = info.flesh_target_starts.get(target_name) {
                start_func.instruction(&wasm_encoder::Instruction::Call(rebinder.function(idx)));
            }
        }
        if let Some(idx) = info.simple_debug_pre_init { start_func.instruction(&wasm_encoder::Instruction::Call(rebinder.function(idx))); }
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
        
        let finished = module.finish();
        std::fs::write("debug_post_combine.wasm", &finished).unwrap();
        Ok(finished)
    }
}
