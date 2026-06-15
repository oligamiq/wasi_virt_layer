use std::collections::HashMap;

use eyre::{ContextCompat as _, Result};
use wasm_encoder::{
    BlockType, CodeSection, ConstExpr, EntityType, ExportKind, ExportSection, Function,
    FunctionSection, GlobalSection, GlobalType, ImportSection, Instruction, Module, RawSection,
    TypeSection, ValType,
};
use wasmparser::{CompositeInnerType, ExternalKind, Payload, TypeRef};

use crate::wasm_stream::{
    pipeline::StreamPass,
    translator::{
        DefaultRebinder, Rebind, translate, translate_global_type, translate_sub_type,
        translate_val_type,
    },
};

const READY: i32 = 2;
const THREAD_MANAGED: i32 = 3;

fn is_thread_start(name: &str) -> bool {
    name == "wasi_thread_start"
        || name.ends_with("#wasi-thread-start")
        || name.contains("_wasi_thread_start")
}

fn should_protect(name: &str) -> bool {
    !is_thread_start(name)
        && !name.starts_with("__wasip1_vfs_stack_")
        && !name.starts_with("__flesh_")
        && !name.starts_with("__wasip1_virt_layer_")
        && !name.contains("reset_globals")
        && !name.ends_with("_resetter")
}

struct WrapperRebinder {
    import_funcs: u32,
    extra_imports: u32,
}

impl Rebind for WrapperRebinder {
    fn function(&self, index: u32) -> u32 {
        if index < self.import_funcs {
            index
        } else {
            index + self.extra_imports
        }
    }
}

fn find_type(
    types: &[wasmparser::SubType],
    params: &[wasmparser::ValType],
    results: &[wasmparser::ValType],
) -> Option<u32> {
    types
        .iter()
        .position(|ty| {
            matches!(
                &ty.composite_type.inner,
                CompositeInnerType::Func(f) if f.params() == params && f.results() == results
            )
        })
        .map(|i| i as u32)
}

fn look_for_type_or_append(
    types: &[wasmparser::SubType],
    base_idx: u32,
    params: &[wasmparser::ValType],
    results: &[wasmparser::ValType],
    appended: &mut bool,
) -> u32 {
    find_type(types, params, results).unwrap_or_else(|| {
        *appended = true;
        base_idx
    })
}

/// Generates export wrappers for multi-memory targets using the arena's slot acquire.
pub struct ExportStackMultiMemoryTargetStreamPass {
    target_name: String,
    vfs_name: String,
    arena_size: u32,
    stack_size: u32,
}

impl ExportStackMultiMemoryTargetStreamPass {
    pub fn new(target_name: String, vfs_name: String, arena_size: u32, stack_size: u32) -> Self {
        Self {
            target_name,
            vfs_name,
            arena_size,
            stack_size,
        }
    }
}

impl StreamPass for ExportStackMultiMemoryTargetStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> Result<Vec<u8>> {
        let arena_size = self.arena_size;
        let arena_off_i32 = arena_size as i32;

        // --- First pass: collect info ---
        let mut types = Vec::new();
        let mut function_types = Vec::new();
        let mut import_func_count = 0_u32;
        let mut import_global_count = 0_u32;
        let mut local_global_count = 0_u32;
        let mut defined_func_count = 0_u32;
        let mut stack_pointer = None;
        let mut has_import_section = false;
        let mut raw_exports = Vec::new();
        let mut has_memory = false;

        for payload in wasmparser::Parser::new(0).parse_all(input_wasm) {
            match payload? {
                Payload::TypeSection(s) => {
                    for g in s {
                        types.extend(g?.into_types());
                    }
                }
                Payload::ImportSection(s) => {
                    has_import_section = true;
                    for g in s {
                        for import in g?.into_iter() {
                            let (_, import) = import?;
                            match import.ty {
                                TypeRef::Func(i) | TypeRef::FuncExact(i) => {
                                    import_func_count += 1;
                                    function_types.push(i);
                                }
                                TypeRef::Global(_) => import_global_count += 1,
                                TypeRef::Memory(_) => has_memory = true,
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
                Payload::MemorySection(_) => has_memory = true,
                Payload::GlobalSection(s) => local_global_count = s.count(),
                Payload::ExportSection(s) => {
                    for export in s {
                        let export = export?;
                        if export.name == "__stack_pointer" && export.kind == ExternalKind::Global {
                            stack_pointer = Some(export.index);
                        }
                        raw_exports.push((export.name.to_string(), export.kind, export.index));
                    }
                }
                _ => {}
            }
        }

        let Some(stack_pointer) = stack_pointer else {
            log::warn!(
                "multi-memory export wrapper for target `{}`: __stack_pointer not found",
                self.target_name
            );
            return Ok(input_wasm.to_vec());
        };
        if !has_import_section || !has_memory {
            log::warn!(
                "multi-memory export wrapper for target `{}`: no imports or no memory",
                self.target_name
            );
            return Ok(input_wasm.to_vec());
        }

        // Collect protected exports
        let mut exports = Vec::new();
        for (name, kind, index) in &raw_exports {
            if matches!(kind, ExternalKind::Func | ExternalKind::FuncExact)
                && (should_protect(name) || is_thread_start(name))
            {
                let type_index = *function_types
                    .get(*index as usize)
                    .wrap_err_with(|| format!("missing function type for export `{name}`"))?;
                exports.push((name.clone(), *index, type_index, is_thread_start(name)));
            }
        }
        if exports.is_empty() {
            log::warn!(
                "multi-memory export wrapper for target `{}`: no protectable exports",
                self.target_name
            );
            return Ok(input_wasm.to_vec());
        }

        let base_type_idx = types.len() as u32;
        let mut appended_ensure = false;
        let mut appended_void = false;
        let ensure_type = look_for_type_or_append(
            &types,
            base_type_idx,
            &[],
            &[wasmparser::ValType::I32],
            &mut appended_ensure,
        );
        let void_type = look_for_type_or_append(
            &types,
            if appended_ensure {
                base_type_idx + 1
            } else {
                base_type_idx
            },
            &[],
            &[],
            &mut appended_void,
        );

        // Indices:
        //   import_func_count + 3 new imports = new_import_base
        //   functions: [existing defined_func_count] [ensure_idx] [wrapper_0] ...
        let new_import_base = import_func_count + 3;
        let new_import_count = 3_u32;
        let ensure_idx = new_import_base + defined_func_count;
        let wrapper_bases: HashMap<String, u32> = exports
            .iter()
            .enumerate()
            .map(|(i, (name, _, _, _))| (name.clone(), ensure_idx + 1 + i as u32))
            .collect();

        let rebinder = WrapperRebinder {
            import_funcs: import_func_count,
            extra_imports: new_import_count,
        };

        let mut module = Module::new();
        let mut saw_code = false;

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
                    if appended_ensure {
                        out.ty().function([], [ValType::I32]);
                    }
                    if appended_void {
                        out.ty().function([], []);
                    }
                    module.section(&out);
                }
                Payload::ImportSection(s) => {
                    let mut out = ImportSection::new();
                    // Re-emit existing imports
                    for g in s {
                        for import in g?.into_iter() {
                            let (_, import) = import?;
                            let entity = match import.ty {
                                TypeRef::Func(i) | TypeRef::FuncExact(i) => EntityType::Function(i),
                                TypeRef::Table(t) => EntityType::Table(
                                    crate::wasm_stream::translator::translate_table_type(
                                        t,
                                        &DefaultRebinder,
                                    ),
                                ),
                                TypeRef::Memory(m) => EntityType::Memory(
                                    crate::wasm_stream::translator::translate_memory_type(m),
                                ),
                                TypeRef::Global(g) => {
                                    EntityType::Global(translate_global_type(g, &DefaultRebinder))
                                }
                                TypeRef::Tag(t) => EntityType::Tag(
                                    crate::wasm_stream::translator::translate_tag_type(t),
                                ),
                            };
                            out.import(import.module, import.name, entity);
                        }
                    }
                    // New imports from VFS
                    out.import(
                        &self.vfs_name,
                        "__wasip1_vfs_stack_ensure_vfs",
                        EntityType::Function(ensure_type),
                    );
                    out.import(
                        &self.vfs_name,
                        "__wasip1_vfs_memory_lock_read_acquire",
                        EntityType::Function(void_type),
                    );
                    out.import(
                        &self.vfs_name,
                        "__wasip1_vfs_memory_lock_read_release",
                        EntityType::Function(void_type),
                    );
                    module.section(&out);
                }
                Payload::FunctionSection(s) => {
                    let mut out = FunctionSection::new();
                    for f in s {
                        out.function(f?);
                    }
                    out.function(ensure_type); // ensure_target
                    for (_, _, type_idx, _) in &exports {
                        out.function(*type_idx);
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
                    // 5 new instance-local globals
                    for _ in 0..5 {
                        out.global(
                            GlobalType {
                                val_type: ValType::I32,
                                mutable: true,
                                shared: false,
                            },
                            &ConstExpr::i32_const(0),
                        );
                    }
                    module.section(&out);
                }
                Payload::ExportSection(s) => {
                    let mut out = ExportSection::new();
                    for export in s {
                        let export = export?;
                        let idx = wrapper_bases
                            .get(export.name)
                            .copied()
                            .unwrap_or_else(|| rebinder.function(export.index));
                        let kind = match export.kind {
                            ExternalKind::Func | ExternalKind::FuncExact => ExportKind::Func,
                            ExternalKind::Table => ExportKind::Table,
                            ExternalKind::Memory => ExportKind::Memory,
                            ExternalKind::Global => ExportKind::Global,
                            ExternalKind::Tag => ExportKind::Tag,
                        };
                        out.export(export.name, kind, idx);
                    }
                    let ensure_export_name =
                        format!("__wasip1_vfs_{}_stack_ensure", self.target_name);
                    out.export(&ensure_export_name, ExportKind::Func, ensure_idx);
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
                Payload::CodeSectionStart { range, .. } => {
                    saw_code = true;
                    let mut out = CodeSection::new();

                    let reader = wasmparser::BinaryReader::new(
                        &input_wasm[range.start..range.end],
                        range.start,
                    );
                    let code_reader = wasmparser::CodeSectionReader::new(reader)?;
                    for body in code_reader {
                        let body = body?;
                        let locals = body
                            .get_locals_reader()?
                            .into_iter()
                            .map(|l| {
                                let (c, t) = l?;
                                Ok((c, translate_val_type(t, &DefaultRebinder)))
                            })
                            .collect::<Result<Vec<_>>>()?;
                        let mut func = Function::new(locals);
                        for op in body.get_operators_reader()? {
                            func.instruction(&translate(&op?, &rebinder));
                        }
                        out.function(&func);
                    }

                    let first_new_global = import_global_count + local_global_count;
                    let state_global = first_new_global;
                    let current_base_global = first_new_global + 1;
                    let current_end_global = first_new_global + 2;
                    let depth_global = first_new_global + 3;
                    let generation_global = first_new_global + 4;

                    // ensure_vfs import index
                    let ensure_vfs = import_func_count;
                    let lock_acquire = import_func_count + 1;
                    let lock_release = import_func_count + 2;
                    // slot_acquire is 3rd of 4 arena-added functions: size(+0), grow(+1), slot_acquire(+2), slot_release(+3)
                    // Original defined before arena = defined_func_count - 4
                    let slot_acquire_idx =
                        (import_func_count + new_import_count) + defined_func_count - 2;

                    // Build ensure_target function: () -> i32
                    // Calls: ensure_vfs, slot_acquire, sets up stack pointer
                    // Locals: slot_packed(i64)
                    let mut ensure_fn = Function::new([(1, ValType::I64)]);

                    // Call ensure_vfs
                    ensure_fn.instruction(&Instruction::Call(ensure_vfs));
                    ensure_fn.instruction(&Instruction::If(BlockType::Empty));
                    ensure_fn.instruction(&Instruction::I32Const(3));
                    ensure_fn.instruction(&Instruction::Return);
                    ensure_fn.instruction(&Instruction::End);

                    // Check if already ready
                    for st in [READY, THREAD_MANAGED] {
                        ensure_fn.instruction(&Instruction::GlobalGet(state_global));
                        ensure_fn.instruction(&Instruction::I32Const(st));
                        ensure_fn.instruction(&Instruction::I32Eq);
                        ensure_fn.instruction(&Instruction::If(BlockType::Empty));
                        ensure_fn.instruction(&Instruction::I32Const(0));
                        ensure_fn.instruction(&Instruction::Return);
                        ensure_fn.instruction(&Instruction::End);
                    }

                    // Mark initializing
                    ensure_fn.instruction(&Instruction::I32Const(1));
                    ensure_fn.instruction(&Instruction::GlobalSet(state_global));

                    // Call slot_acquire() -> i64 packed(base, end)
                    ensure_fn.instruction(&Instruction::Call(slot_acquire_idx));
                    ensure_fn.instruction(&Instruction::LocalTee(0));
                    ensure_fn.instruction(&Instruction::I64Eqz);
                    ensure_fn.instruction(&Instruction::If(BlockType::Empty));
                    ensure_fn.instruction(&Instruction::I32Const(0));
                    ensure_fn.instruction(&Instruction::GlobalSet(state_global));
                    ensure_fn.instruction(&Instruction::I32Const(1));
                    ensure_fn.instruction(&Instruction::Return);
                    ensure_fn.instruction(&Instruction::End);

                    // Extract base (low 32) and end (high 32)
                    ensure_fn.instruction(&Instruction::LocalGet(0));
                    ensure_fn.instruction(&Instruction::I32WrapI64);
                    ensure_fn.instruction(&Instruction::GlobalSet(current_base_global));

                    ensure_fn.instruction(&Instruction::LocalGet(0));
                    ensure_fn.instruction(&Instruction::I64Const(32));
                    ensure_fn.instruction(&Instruction::I64ShrU);
                    ensure_fn.instruction(&Instruction::I32WrapI64);
                    ensure_fn.instruction(&Instruction::GlobalSet(current_end_global));

                    // Stack pointer = end - arena_size
                    ensure_fn.instruction(&Instruction::GlobalGet(current_end_global));
                    ensure_fn.instruction(&Instruction::I32Const(arena_off_i32));
                    ensure_fn.instruction(&Instruction::I32Sub);
                    ensure_fn.instruction(&Instruction::GlobalSet(stack_pointer));

                    ensure_fn.instruction(&Instruction::I32Const(READY));
                    ensure_fn.instruction(&Instruction::GlobalSet(state_global));
                    ensure_fn.instruction(&Instruction::I32Const(0));
                    ensure_fn.instruction(&Instruction::End);
                    out.function(&ensure_fn);

                    // Build wrappers
                    let ensure_index = ensure_idx;
                    let lock_acquire = import_func_count + 1;
                    let lock_release = import_func_count + 2;

                    for (name, original_idx, type_idx, thread_start) in &exports {
                        let original_func_idx = *original_idx;
                        let func_ty = match &types[*type_idx as usize].composite_type.inner {
                            CompositeInnerType::Func(f) => f,
                            _ => eyre::bail!("unexpected non-func type for export {name}"),
                        };
                        let params = func_ty.params();
                        let results = func_ty.results();
                        let result_count = results.len() as u32;
                        let first_result_local = params.len() as u32;

                        let result_locals: Vec<_> = results
                            .iter()
                            .copied()
                            .map(|t| (1, translate_val_type(t, &DefaultRebinder)))
                            .collect();
                        let mut wrapper = Function::new(result_locals);

                        if *thread_start {
                            wrapper.instruction(&Instruction::I32Const(THREAD_MANAGED));
                            wrapper.instruction(&Instruction::GlobalSet(state_global));
                        } else {
                            // Call ensure
                            wrapper.instruction(&Instruction::Call(ensure_index));
                            wrapper.instruction(&Instruction::If(BlockType::Empty));
                            wrapper.instruction(&Instruction::Unreachable);
                            wrapper.instruction(&Instruction::End);

                            // Depth++
                            wrapper.instruction(&Instruction::GlobalGet(depth_global));
                            wrapper.instruction(&Instruction::I32Const(1));
                            wrapper.instruction(&Instruction::I32Add);
                            wrapper.instruction(&Instruction::GlobalSet(depth_global));

                            // Layout lock acquire
                            wrapper.instruction(&Instruction::Call(lock_acquire));
                        }

                        // Forward params and call original
                        for i in 0..params.len() as u32 {
                            wrapper.instruction(&Instruction::LocalGet(i));
                        }
                        wrapper
                            .instruction(&Instruction::Call(rebinder.function(original_func_idx)));

                        if !*thread_start {
                            // Layout lock release
                            wrapper.instruction(&Instruction::Call(lock_release));

                            // Save results
                            for i in (0..result_count).rev() {
                                wrapper.instruction(&Instruction::LocalSet(first_result_local + i));
                            }
                            // Depth--
                            wrapper.instruction(&Instruction::GlobalGet(depth_global));
                            wrapper.instruction(&Instruction::I32Const(1));
                            wrapper.instruction(&Instruction::I32Sub);
                            wrapper.instruction(&Instruction::GlobalSet(depth_global));
                            // Restore results
                            for i in 0..result_count {
                                wrapper.instruction(&Instruction::LocalGet(first_result_local + i));
                            }
                        }
                        wrapper.instruction(&Instruction::End);
                        out.function(&wrapper);
                    }
                    module.section(&out);
                }
                Payload::CodeSectionEntry(_) => {}
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

        if !saw_code {
            eyre::bail!("target has no code section");
        }

        Ok(module.finish())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_encoder::{GlobalSection, MemorySection, MemoryType};

    fn fixture_correct() -> Vec<u8> {
        let mut module = Module::new();

        // Types: 0=() -> i32, 1=() -> (), 2=(i32) -> i32, 3=() -> i64, 4=(i32) -> ()
        let mut types = TypeSection::new();
        types.ty().function([], [ValType::I32]); // 0
        types.ty().function([], []); // 1
        types.ty().function([ValType::I32], [ValType::I32]); // 2
        types.ty().function([], [ValType::I64]); // 3
        types.ty().function([ValType::I32], []); // 4
        module.section(&types);

        let imports = ImportSection::new();
        module.section(&imports);

        // 2 original + 4 arena functions
        let mut functions = FunctionSection::new();
        functions.function(0); // run: () -> i32
        functions.function(1); // thread_start: () -> ()
        functions.function(0); // size_fn: () -> i32
        functions.function(2); // grow_fn: (i32) -> i32
        functions.function(3); // slot_acquire: () -> i64
        functions.function(4); // slot_release: (i32) -> ()
        module.section(&functions);

        let mut memories = MemorySection::new();
        memories.memory(MemoryType {
            minimum: 7,
            maximum: Some(16),
            memory64: false,
            shared: false,
            page_size_log2: None,
        });
        module.section(&memories);

        let mut globals = GlobalSection::new();
        globals.global(
            GlobalType {
                val_type: ValType::I32,
                mutable: true,
                shared: false,
            },
            &ConstExpr::i32_const(1048576),
        );
        module.section(&globals);

        let mut exports = wasm_encoder::ExportSection::new();
        exports.export("__stack_pointer", ExportKind::Global, 0);
        exports.export("run", ExportKind::Func, 0);
        exports.export("wasi_thread_start", ExportKind::Func, 1);
        module.section(&exports);

        let mut code = CodeSection::new();
        let mut run = Function::new([]);
        run.instruction(&Instruction::I32Const(42));
        run.instruction(&Instruction::End);
        code.function(&run);
        let mut ts = Function::new([]);
        ts.instruction(&Instruction::End);
        code.function(&ts);
        let mut sz = Function::new([]);
        sz.instruction(&Instruction::MemorySize(0));
        sz.instruction(&Instruction::I32Const(3));
        sz.instruction(&Instruction::I32Sub);
        sz.instruction(&Instruction::End);
        code.function(&sz);
        let mut gw = Function::new([(1, ValType::I32)]);
        gw.instruction(&Instruction::LocalGet(0));
        gw.instruction(&Instruction::MemoryGrow(0));
        gw.instruction(&Instruction::End);
        code.function(&gw);
        let mut sa = Function::new([]);
        sa.instruction(&Instruction::I64Const(0x2000000100000));
        sa.instruction(&Instruction::End);
        code.function(&sa);
        let mut sr = Function::new([(1, ValType::I32)]);
        sr.instruction(&Instruction::LocalGet(0));
        sr.instruction(&Instruction::Drop);
        sr.instruction(&Instruction::End);
        code.function(&sr);
        module.section(&code);

        module.finish()
    }

    #[test]
    fn wrapper_pass_validates() -> Result<()> {
        let fixture = fixture_correct();
        // Validate the fixture first
        wasmparser::Validator::new_with_features(wasmparser::WasmFeatures::all())
            .validate_all(&fixture)?;

        let mut pass = ExportStackMultiMemoryTargetStreamPass::new(
            "target".to_string(),
            "vfs".to_string(),
            196608, // 3 pages arena
            65536,
        );
        let output = pass.run(&fixture)?;
        wasmparser::Validator::new_with_features(wasmparser::WasmFeatures::all())
            .validate_all(&output)?;

        // Check that the ensure export exists
        let mut found_ensure = false;
        for payload in wasmparser::Parser::new(0).parse_all(&output) {
            if let Payload::ExportSection(s) = payload? {
                for export in s {
                    if export?.name == "__wasip1_vfs_target_stack_ensure" {
                        found_ensure = true;
                    }
                }
            }
        }
        assert!(found_ensure);
        Ok(())
    }

    #[test]
    fn wrapper_pass_skips_without_exports() -> Result<()> {
        // Module with memory but no exports to protect
        let mut module = Module::new();
        let mut types = TypeSection::new();
        types.ty().function([], [ValType::I32]);
        module.section(&types);
        module.section(&ImportSection::new());
        let mut functions = FunctionSection::new();
        functions.function(0);
        functions.function(0);
        functions.function(0);
        functions.function(0);
        module.section(&functions);
        let mut memories = MemorySection::new();
        memories.memory(MemoryType {
            minimum: 7,
            maximum: Some(16),
            memory64: false,
            shared: false,
            page_size_log2: None,
        });
        module.section(&memories);
        let mut globals = GlobalSection::new();
        globals.global(
            GlobalType {
                val_type: ValType::I32,
                mutable: true,
                shared: false,
            },
            &ConstExpr::i32_const(65536),
        );
        module.section(&globals);
        let mut exports = wasm_encoder::ExportSection::new();
        exports.export("__stack_pointer", ExportKind::Global, 0);
        // Only export "memory" — no protectable function exports
        exports.export("memory", ExportKind::Memory, 0);
        module.section(&exports);
        let mut code = CodeSection::new();
        for _ in 0..4 {
            let mut f = Function::new([]);
            f.instruction(&Instruction::I32Const(0));
            f.instruction(&Instruction::End);
            code.function(&f);
        }
        module.section(&code);
        let fixture = module.finish();

        let mut pass = ExportStackMultiMemoryTargetStreamPass::new(
            "target".to_string(),
            "vfs".to_string(),
            196608,
            65536,
        );
        let output = pass.run(&fixture)?;
        // Should return unchanged since no exports to protect
        assert_eq!(output, fixture);
        Ok(())
    }
}
