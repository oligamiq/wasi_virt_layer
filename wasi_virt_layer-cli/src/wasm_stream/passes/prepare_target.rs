use crate::wasm_stream::pipeline::StreamPass;
use wasm_encoder::{
    CodeSection, EntityType, ExportKind, ExportSection, Function, FunctionSection, GlobalSection,
    ImportSection, Instruction, Module, RawSection, TypeSection, ValType,
};
use wasmparser::{Parser, Payload};

pub struct PrepareTargetStreamPass;

impl PrepareTargetStreamPass {
    pub fn new() -> Self {
        Self
    }
}

impl StreamPass for PrepareTargetStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> eyre::Result<Vec<u8>> {
        let mut type_section = TypeSection::new();
        let mut import_section = ImportSection::new();
        let mut function_section = FunctionSection::new();
        let mut global_section = GlobalSection::new();
        let mut export_section = ExportSection::new();
        let mut code_section = CodeSection::new();

        let mut func_import_count = 0;
        let mut original_start_idx = None;
        let mut memory_params = None;

        let mut next_type_idx = 0;

        // Pass 1: Gather info
        for payload in Parser::new(0).parse_all(input_wasm) {
            match payload? {
                Payload::TypeSection(s) => {
                    for ty in s {
                        for sub_ty in ty?.into_types() {
                            type_section.ty().subtype(&crate::wasm_stream::translator::translate_sub_type(
                                &sub_ty,
                                &crate::wasm_stream::translator::DefaultRebinder,
                            ));
                            next_type_idx += 1;
                        }
                    }
                }
                Payload::ImportSection(s) => {
                    for group in s {
                        for import in group? {
                            let (_, import) = import?;
                            if let wasmparser::TypeRef::Func(ty_idx) = import.ty {
                                import_section.import(import.module, import.name, EntityType::Function(ty_idx));
                                func_import_count += 1;
                            } else {
                                // Pass through other imports
                                import_section.import(
                                    import.module,
                                    import.name,
                                    match import.ty {
                                        wasmparser::TypeRef::Table(t) => EntityType::Table(crate::wasm_stream::translator::translate_table_type(t, &crate::wasm_stream::translator::DefaultRebinder)),
                                        wasmparser::TypeRef::Memory(m) => EntityType::Memory(crate::wasm_stream::translator::translate_memory_type(m)),
                                        wasmparser::TypeRef::Global(g) => EntityType::Global(crate::wasm_stream::translator::translate_global_type(g, &crate::wasm_stream::translator::DefaultRebinder)),
                                        wasmparser::TypeRef::Tag(t) => EntityType::Tag(crate::wasm_stream::translator::translate_tag_type(t)),
                                    }
                                );
                            }
                        }
                    }
                }
                Payload::ExportSection(s) => {
                    for export in s {
                        let export = export?;
                        if export.name == "_start" && export.kind == wasmparser::ExternalKind::Func {
                            original_start_idx = Some(export.index);
                        }
                    }
                }
                Payload::MemorySection(s) => {
                    if let Some(mem) = s.into_iter().next() {
                        let mem = mem?;
                        memory_params = Some((mem.initial as i32, mem.maximum.map(|m| m as i32).unwrap_or(0)));
                    }
                }
                _ => {}
            }
        }

        // Define types for ABI functions
        let type_idx_register = next_type_idx;
        type_section.ty().function(vec![ValType::I32, ValType::I32, ValType::I32], vec![ValType::I32]);
        next_type_idx += 1;

        let type_idx_get_lock_ptr = next_type_idx;
        type_section.ty().function(vec![ValType::I32], vec![ValType::I32]);
        next_type_idx += 1;

        let type_idx_grow = next_type_idx;
        type_section.ty().function(vec![ValType::I32, ValType::I32], vec![ValType::I32]);
        next_type_idx += 1;

        let type_idx_void_void = next_type_idx;
        type_section.ty().function(vec![], vec![]);
        next_type_idx += 1;

        let type_idx_i32_i32 = next_type_idx;
        type_section.ty().function(vec![ValType::I32], vec![ValType::I32]);
        next_type_idx += 1;

        // Add ABI imports
        let func_idx_register = func_import_count;
        import_section.import("env", "wasip1_vfs_register_shared_memory_target", EntityType::Function(type_idx_register));
        func_import_count += 1;

        let func_idx_get_lock_ptr = func_import_count;
        import_section.import("env", "wasip1_vfs_shared_memory_get_lock_ptr", EntityType::Function(type_idx_get_lock_ptr));
        func_import_count += 1;

        let func_idx_grow = func_import_count;
        import_section.import("env", "wasip1_vfs_shared_memory_grow", EntityType::Function(type_idx_grow));
        func_import_count += 1;

        // Count existing globals
        let mut global_count = 0;
        for payload in Parser::new(0).parse_all(input_wasm) {
            match payload? {
                Payload::ImportSection(s) => {
                    for group in s {
                        for import in group? {
                            let (_, import) = import?;
                            if let wasmparser::TypeRef::Global(_) = import.ty {
                                global_count += 1;
                            }
                        }
                    }
                }
                Payload::GlobalSection(s) => {
                    global_count += s.count();
                }
                _ => {}
            }
        }

        let metadata_ptr_global_idx = global_count;
        let lock_ptr_global_idx = global_count + 1;

        // Re-emit everything
        let mut module = Module::new();
        module.section(&type_section);
        module.section(&import_section);

        let mut func_count = func_import_count;
        let mut original_defined_func_count = 0;

        for payload in Parser::new(0).parse_all(input_wasm) {
            match payload? {
                Payload::FunctionSection(s) => {
                    original_defined_func_count = s.count();
                    for ty_idx in s {
                        function_section.function(ty_idx?);
                        func_count += 1;
                    }
                }
                Payload::TableSection(s) => {
                    let mut section = wasm_encoder::TableSection::new();
                    for table in s {
                        let table = table?;
                        section.table(crate::wasm_stream::translator::translate_table_type(
                            table.ty,
                            &crate::wasm_stream::translator::DefaultRebinder,
                        ));
                    }
                    module.section(&section);
                }
                Payload::MemorySection(s) => {
                    let mut section = wasm_encoder::MemorySection::new();
                    for mem in s {
                        section.memory(crate::wasm_stream::translator::translate_memory_type(mem?));
                    }
                    module.section(&section);
                }
                Payload::GlobalSection(s) => {
                    for global in s {
                        let global = global?;
                        let mut instrs = Vec::new();
                        for op in global.init_expr.get_operators_reader() {
                            let op = op?;
                            if matches!(op, wasmparser::Operator::End) { continue; }
                            instrs.push(crate::wasm_stream::translator::translate(&op, &crate::wasm_stream::translator::DefaultRebinder));
                        }
                        global_section.global(
                            crate::wasm_stream::translator::translate_global_type(global.ty, &crate::wasm_stream::translator::DefaultRebinder),
                            &wasm_encoder::ConstExpr::extended(instrs),
                        );
                    }
                }
                _ => {}
            }
        }

        // Add new globals
        global_section.global(
            wasm_encoder::GlobalType { val_type: ValType::I32, mutable: true, shared: false },
            &wasm_encoder::ConstExpr::i32_const(0),
        );
        global_section.global(
            wasm_encoder::GlobalType { val_type: ValType::I32, mutable: true, shared: false },
            &wasm_encoder::ConstExpr::i32_const(0),
        );
        module.section(&global_section);

        // Add new functions (init and grow wrapper)
        let grow_wrapper_idx = func_count;
        function_section.function(type_idx_i32_i32);
        func_count += 1;

        let new_start_idx = func_count;
        function_section.function(type_idx_void_void);
        func_count += 1;

        module.section(&function_section);

        // Exports
        for payload in Parser::new(0).parse_all(input_wasm) {
            if let Payload::ExportSection(s) = payload? {
                for export in s {
                    let export = export?;
                    if export.name == "_start" && export.kind == wasmparser::ExternalKind::Func {
                        continue; // skip original start, will export new one
                    }
                    export_section.export(
                        export.name,
                        match export.kind {
                            wasmparser::ExternalKind::Func => ExportKind::Func,
                            wasmparser::ExternalKind::Table => ExportKind::Table,
                            wasmparser::ExternalKind::Memory => ExportKind::Memory,
                            wasmparser::ExternalKind::Global => ExportKind::Global,
                            wasmparser::ExternalKind::Tag => ExportKind::Tag,
                        },
                        match export.kind {
                            wasmparser::ExternalKind::Func => export.index + 3, // shifted by 3 imports
                            _ => export.index,
                        }
                    );
                }
            }
        }
        
        if original_start_idx.is_some() {
            export_section.export("_start", ExportKind::Func, new_start_idx);
        } else {
            export_section.export("__wvl_init", ExportKind::Func, new_start_idx);
        }
        module.section(&export_section);

        // Element section
        for payload in Parser::new(0).parse_all(input_wasm) {
            if let Payload::ElementSection(s) = payload? {
                let mut section = wasm_encoder::ElementSection::new();
                for elem in s {
                    let elem = elem?;
                    let items = match elem.items {
                        wasmparser::ElementItems::Functions(f) => {
                            let shifted = f.into_iter().map(|idx| Ok(idx? + 3)).collect::<eyre::Result<Vec<u32>>>()?;
                            wasm_encoder::Elements::Functions(std::borrow::Cow::Owned(shifted))
                        }
                        _ => unimplemented!(),
                    };
                    match elem.kind {
                        wasmparser::ElementKind::Passive => section.passive(items),
                        wasmparser::ElementKind::Active { table_index, offset_expr } => {
                            let mut instrs = Vec::new();
                            for op in offset_expr.get_operators_reader() {
                                let op = op?;
                                if matches!(op, wasmparser::Operator::End) { continue; }
                                instrs.push(crate::wasm_stream::translator::translate(&op, &crate::wasm_stream::translator::DefaultRebinder));
                            }
                            section.active(table_index, &wasm_encoder::ConstExpr::extended(instrs), items);
                        }
                        wasmparser::ElementKind::Declared => section.declared(items),
                    }
                }
                module.section(&section);
            }
        }

        // Code section
        let (initial_pages, max_pages) = memory_params.ok_or_else(|| eyre::eyre!("No memory found"))?;

        struct ShiftRebinder;
        impl crate::wasm_stream::translator::Rebind for ShiftRebinder {
            fn function(&self, index: u32) -> u32 {
                index + 3 // Shift by 3 new imports
            }
        }

        for payload in Parser::new(0).parse_all(input_wasm) {
            if let Payload::CodeSectionEntry(body) = payload? {
                let mut locals = Vec::new();
                for local in body.get_locals_reader()? {
                    let local = local?;
                    locals.push((local.0, crate::wasm_stream::translator::translate_val_type(local.1, &ShiftRebinder)));
                }
                let mut func = Function::new(locals);
                for op in body.get_operators_reader()? {
                    let op = op?;
                    match op {
                        wasmparser::Operator::MemoryGrow { .. } => {
                            func.instruction(&Instruction::Call(grow_wrapper_idx));
                        }
                        _ => {
                            func.instruction(&crate::wasm_stream::translator::translate(&op, &ShiftRebinder));
                        }
                    }
                }
                code_section.function(&func);
            }
        }

        // Add grow wrapper body
        let mut grow_wrapper = Function::new(vec![]);
        grow_wrapper.instruction(&Instruction::GlobalGet(metadata_ptr_global_idx));
        grow_wrapper.instruction(&Instruction::LocalGet(0));
        grow_wrapper.instruction(&Instruction::Call(func_idx_grow));
        grow_wrapper.instruction(&Instruction::End);
        code_section.function(&grow_wrapper);

        // Add init body
        let mut init_func = Function::new(vec![]);
        init_func.instruction(&Instruction::I32Const(0)); // base_ptr
        init_func.instruction(&Instruction::I32Const(initial_pages));
        init_func.instruction(&Instruction::I32Const(max_pages));
        init_func.instruction(&Instruction::Call(func_idx_register));
        init_func.instruction(&Instruction::GlobalSet(metadata_ptr_global_idx));
        init_func.instruction(&Instruction::GlobalGet(metadata_ptr_global_idx));
        init_func.instruction(&Instruction::Call(func_idx_get_lock_ptr));
        init_func.instruction(&Instruction::GlobalSet(lock_ptr_global_idx));
        
        if let Some(orig_idx) = original_start_idx {
            init_func.instruction(&Instruction::Call(orig_idx + 3));
        }
        init_func.instruction(&Instruction::End);
        code_section.function(&init_func);

        module.section(&code_section);

        // Data section
        for payload in Parser::new(0).parse_all(input_wasm) {
            if let Payload::DataSection(s) = payload? {
                let mut section = wasm_encoder::DataSection::new();
                for data in s {
                    let data = data?;
                    match data.kind {
                        wasmparser::DataKind::Passive => section.passive(data.data.iter().copied()),
                        wasmparser::DataKind::Active { memory_index, offset_expr } => {
                            let mut instrs = Vec::new();
                            for op in offset_expr.get_operators_reader() {
                                let op = op?;
                                if matches!(op, wasmparser::Operator::End) { continue; }
                                instrs.push(crate::wasm_stream::translator::translate(&op, &crate::wasm_stream::translator::DefaultRebinder));
                            }
                            section.active(memory_index, &wasm_encoder::ConstExpr::extended(instrs), data.data.iter().copied());
                        }
                    }
                }
                module.section(&section);
            }
        }

        // Custom sections
        for payload in Parser::new(0).parse_all(input_wasm) {
            if let Payload::CustomSection(s) = payload? {
                module.section(&wasm_encoder::CustomSection {
                    name: s.name().into(),
                    data: std::borrow::Cow::Borrowed(s.data()),
                });
            }
        }

        Ok(module.finish())
    }
}
