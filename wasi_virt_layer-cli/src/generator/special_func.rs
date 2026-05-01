use std::collections::HashSet;

use eyre::Context as _;
use walrus::*;

use crate::{
    generator::{Generator, GeneratorCtx, ModuleExternal},
    instrs::InstrRewrite as _,
    unique_name::UniqueName,
    util::{ResultUtil as _, WalrusFID, WalrusUtilFuncs as _, WalrusUtilModule as _, WasmName},
};

/// Defines unique names associated with special life-cycle functions (startup, reset, main execution).
#[derive(Debug, strum::AsRefStr, strum::EnumCount, Hash, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum SpecialFuncUniqueName<'a> {
    /// Function that initializes memory resets.
    Resetter(&'a WasmName),
    /// Function that handles thread resets.
    ResetOnThread,
    /// Function that ensures thread reset occurs precisely once.
    ResetOnThreadOnce,
    /// Preserved original initialization function.
    StartInitOld,
    /// The primary start wrapper routine function.
    Start(&'a WasmName),
    /// Early application termination safe wrapper function.
    MainVoid(&'a WasmName),
    /// State reset function generated for given module.
    Reset(&'a WasmName),
}

/// To enable the reset function,
/// a memory area shall be provided
/// to retain memory information at startup.
pub struct VFSExternalMemoryManager {
    external_size: usize,
    current_size: usize, // * 64KiB
    mem_id: MemoryId,
}

impl VFSExternalMemoryManager {
    /// Creates a new active memory manager segment and allocates a local memory space.
    pub fn new(module: &mut walrus::Module) -> Self {
        let mem_id = module.memories.add_local(true, false, 0, None, None);

        Self {
            external_size: 0,
            current_size: 0,
            mem_id,
        }
    }

    /// Gets the allocated Memory ID managed by this external manager.
    pub fn memory_id(&self) -> MemoryId {
        self.mem_id
    }

    /// Provisions requested memory size and returns the beginning pointer/offset.
    pub fn alloc(&mut self, size: usize) -> usize {
        let ptr = self.current_size * 64 * 1024 + self.external_size;
        self.external_size += size;

        ptr
    }

    /// Commits size configurations on the generated memory instance, optionally enabling thread sharing.
    pub fn flush(mut self, module: &mut walrus::Module, threads: bool) -> eyre::Result<MemoryId> {
        let external_size = (0..=0x10000)
            .find(|i| *i * 64 * 1024 >= self.external_size)
            .ok_or_else(|| eyre::eyre!("Failed to find external size in 0..=0x10000"))?;

        self.current_size += external_size;

        let mem = module.memories.get_mut(self.mem_id);

        mem.initial = self.current_size as u64;
        mem.shared = threads;

        if threads {
            mem.maximum = Some(mem.initial);
        }

        Ok(self.mem_id)
    }
}

/// Handles the generation of memory state reset functionalities.
#[derive(Debug, Default)]
pub struct ResetFunc;

impl Generator for ResetFunc {
    fn post_combine(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        let mut mem_manager = VFSExternalMemoryManager::new(module);

        let initializers = module
            .add_func(&[], &[], |_, _| Ok(()))
            .wrap_err_with(|| eyre::eyre!("Failed to add initializer function"))?;

        let tmp_start_section_id = module.add_func(&[], &[], |_, _| Ok(()))?;

        for wasm in &ctx.target_names {
            let wasm_mem = ctx.target_used_memory_id.as_ref().unwrap()[wasm];

            if let Some(reset) = (
                UniqueName::NAMESPACE,
                &UniqueName::SpecialFunc(&SpecialFuncUniqueName::Reset(wasm)),
            )
                .get_fid(&module.imports)
                .ok()
            {
                let global = ctx.target_used_global_id.as_ref().unwrap()[wasm]
                    .iter()
                    .copied()
                    .map(|g| module.globals.get(g))
                    .filter(|g| g.mutable)
                    .filter_map(|g| {
                        if let GlobalKind::Local(ConstExpr::Value(v)) = g.kind {
                            Some((g.id(), v.clone()))
                        } else {
                            log::warn!(
                            "Global segment {:?} is not a value, we support only local variables",
                            g.kind
                        );
                            None
                        }
                    })
                    .collect::<Box<_>>();

                let data_range = module
                    .data
                    .iter()
                    .filter_map(|data| {
                        match &data.kind {
                            DataKind::Active { memory, offset } if *memory == wasm_mem => {
                                if let ConstExpr::Value(v) = offset {
                                    if let ir::Value::I32(offset) = v {
                                        Some((*offset, data.value.len()))
                                    } else {
                                        log::warn!(
                                            "Data segment {:?} is not i32, we support only i32",
                                            offset
                                        );
                                        None
                                    }
                                } else {
                                    log::warn!(
                                        "Data segment {:?} is not a value, we support only i32",
                                        offset
                                    );
                                    None
                                }
                            }
                            // Passive is across memories so ignore on now
                            _ => None,
                        }
                    })
                    .collect::<Box<_>>();

                let zero_range = std::iter::once(Some(0i32))
                    .chain(
                        data_range
                            .iter()
                            .flat_map(|(offset, len)| [Some(*offset), Some(*offset + *len as i32)]),
                    )
                    .chain(std::iter::once(None))
                    .collect::<Vec<_>>()
                    .chunks(2)
                    .map(|chunk| (chunk[0].unwrap(), chunk[1]))
                    .collect::<Box<_>>();

                let mem_init = data_range
                    .into_iter()
                    .map(|(offset, len)| (offset, len, mem_manager.alloc(len)))
                    .collect::<Box<_>>();

                let reset_area_mem_id = mem_manager.memory_id();

                let resetter = module
                    .replace_imported_func(reset, |(builder, _)| {
                        let mut body = builder.func_body();

                        for (id, value) in global.iter() {
                            body.const_(*value).global_set(*id);
                        }
                        for (start, end) in zero_range.iter() {
                            // ptr
                            body.i32_const(*start)
                                // value
                                .i32_const(0);

                            // len
                            if let Some(end) = end {
                                body.i32_const(*end - *start);
                            } else {
                                body.memory_size(wasm_mem);

                                // asserter.as_mut().unwrap()(&mut body).unwrap();

                                body.i32_const(64 * 1024)
                                    .binop(ir::BinaryOp::I32Mul)
                                    .i32_const(*start)
                                    .binop(ir::BinaryOp::I32Sub);
                            }
                            body.memory_fill(wasm_mem);
                        }
                        for (mem_offset, mem_len, mem_ptr) in mem_init.iter() {
                            body.i32_const(*mem_offset) // dst
                                .i32_const(*mem_ptr as i32) // src
                                .i32_const(*mem_len as i32) // len
                                .memory_copy(reset_area_mem_id, wasm_mem);
                        }

                        body.call(tmp_start_section_id);
                    })
                    .to_eyre()
                    .wrap_err_with(|| eyre::eyre!("Failed to replace reset function for {wasm}"))?;

                if ctx.unstable_print_debug {
                    module.exports.add(
                        &UniqueName::SpecialFunc(&SpecialFuncUniqueName::Resetter(wasm))
                            .to_string(),
                        resetter,
                    );
                }

                let mut func_body = module
                    .funcs
                    .get_mut(initializers)
                    .kind
                    .unwrap_local_mut()
                    .builder_mut()
                    .func_body();

                for (offset, len, ptr) in mem_init {
                    func_body
                        .i32_const(ptr as i32) // dst
                        .i32_const(offset) // src
                        .i32_const(len as i32) // len
                        .memory_copy(wasm_mem, reset_area_mem_id);
                }
            }
        }

        let _ = mem_manager.flush(module, ctx.threads)?;

        // Saves the memory state upon initial startup.
        // As the start section is also invoked when spawning threads,
        // ensure it is called only once if threads are enabled.
        let init_id = if ctx.threads {
            let reset_on_thread = UniqueName::SpecialFunc(&SpecialFuncUniqueName::ResetOnThread)
                .get_fid(&module.exports)?;
            let reset_on_thread_once = (
                UniqueName::NAMESPACE,
                &UniqueName::SpecialFunc(&SpecialFuncUniqueName::ResetOnThreadOnce),
            )
                .get_fid(&module.imports)?;

            // module.imports.erase(reset_on_thread_once)?;

            // module.exports.erase(reset_on_thread)?;
            // module.funcs.delete(reset_on_thread);

            module.renew_call_fn(reset_on_thread_once, initializers)?;

            reset_on_thread
        } else {
            initializers
        };

        let old_start = module.start.clone();
        let new_start = module
            .add_func(&[], &[], |builder, _| {
                let mut body = builder.func_body();
                body.call(init_id); // save environment
                if let Some(old_start) = old_start {
                    body.call(old_start);
                }
                Ok(())
            })
            .wrap_err_with(|| eyre::eyre!("Failed to add new start function"))?;

        module.start = Some(new_start);

        module.renew_call_fn(tmp_start_section_id, new_start)?;

        if let Some(start) = old_start {
            if ctx.unstable_print_debug {
                module.exports.add(
                    &UniqueName::SpecialFunc(&SpecialFuncUniqueName::StartInitOld).to_string(),
                    start,
                );
            }
        }

        // memory_init(memory, data)
        // fn(&mut self, Id<Memory>, Id<Data>)
        // data_drop(&mut self, data: DataId)
        // so we remove all data_drop sections.
        // Prevent the active segment from being deleted
        // so that it can be called upon as many times as required.
        module
            .funcs
            .iter_mut()
            .map(|func| {
                match &mut func.kind {
                    walrus::FunctionKind::Local(l) => {
                        l.builder_mut()
                            .func_body()
                            .retain(|instr, _| !instr.is_data_drop());
                    }
                    _ => {}
                }
                Ok(())
            })
            .collect::<eyre::Result<Vec<_>>>()?;

        Ok(())
    }
}

/// Manages redirection of `_start` calls from targets into safe wrappers.
#[derive(Debug, Default)]
pub struct StartFunc;

impl Generator for StartFunc {
    fn pre_target(
        &mut self,
        module: &mut walrus::Module,
        _: &GeneratorCtx,
        external: &ModuleExternal,
    ) -> eyre::Result<()> {
        let id = "_start".get_fid(&module.exports)?;

        module
            .exports
            .get_mut(module.exports.get_exported_func(id).unwrap().id())
            .name =
            UniqueName::SpecialFunc(&SpecialFuncUniqueName::Start(&external.name)).to_string();

        Ok(())
    }

    fn post_combine(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        for wasm in &ctx.target_names {
            // NOTE: The import was created by the import_wasm! macro using the
            // Rust identifier name (with underscores), but ctx.target_names has
            // the package name (with dashes). We need to normalize to underscores
            // to match what the macro generated.
            let normalized_wasm = wasm.as_ref().replace('-', "_");
            let import_name = format!("__wasip1_vfs_{normalized_wasm}__start");

            module.renew_call_fn(
                (UniqueName::NAMESPACE, &import_name).get_fid(&module.imports)?,
                ctx.start_func_id.as_ref().unwrap()[wasm],
                // Export already removed by StartFuncIdVisitor
            )?;
        }

        Ok(())
    }
}

/// Redirects early main exit functionalities to internal no-op executions seamlessly.
///
/// For C/C++ compiled Wasm that lacks a `__main_void` export, this generator
/// synthesizes a wrapper: `fn __main_void() -> i32 { _start(); 0 }`.
/// This allows the rest of the pipeline to operate uniformly regardless of
/// the source language.
#[derive(Debug, Default)]
pub struct MainVoidFunc {
    /// Tracks target names where `__main_void` was synthesized (i.e. C/C++ targets).
    /// For these targets, `post_combine` skips call-graph rewriting because `_start`
    /// never calls `__main_void` — the relationship is reversed.
    /// Uses `String` instead of `WasmName` to avoid holding references past
    /// `WasmNameHolder` lifetime.
    synthesized_targets: HashSet<String>,
}

impl Generator for MainVoidFunc {
    fn pre_target(
        &mut self,
        module: &mut walrus::Module,
        _: &GeneratorCtx,
        external: &ModuleExternal,
    ) -> eyre::Result<()> {
        let id = match "__main_void".get_fid(&module.exports).ok() {
            Some(id) => id,
            None => {
                // C/C++ Wasm: synthesize __main_void from _start.
                log::info!(
                    "No `__main_void` export found for target `{}`; synthesizing wrapper from `_start`.",
                    external.name
                );

                // StartFunc runs before MainVoidFunc and renames `_start` to a
                // unique name. Try the renamed export first, then fall back to
                // the original `_start` name.
                let renamed_start = UniqueName::SpecialFunc(
                    &SpecialFuncUniqueName::Start(&external.name),
                )
                .to_string();

                let start_fid = renamed_start
                    .as_str()
                    .get_fid(&module.exports)
                    .or_else(|_| "_start".get_fid(&module.exports))
                    .wrap_err_with(|| {
                        eyre::eyre!(
                            "Target `{}` has neither `__main_void` nor `_start` export",
                            external.name
                        )
                    })?;

                // Create: fn __main_void() -> i32 { _start(); 0 }
                let wrapper = module
                    .add_func(&[], &[ValType::I32], |builder, _| {
                        builder.func_body().call(start_fid).i32_const(0);
                        Ok(())
                    })
                    .wrap_err("Failed to create synthetic __main_void wrapper")?;

                module.exports.add("__main_void", wrapper);

                self.synthesized_targets.insert(external.name.to_string());

                wrapper
            }
        };

        module
            .exports
            .get_mut(module.exports.get_exported_func(id).unwrap().id())
            .name =
            UniqueName::SpecialFunc(&SpecialFuncUniqueName::MainVoid(&external.name)).to_string();

        Ok(())
    }

    fn post_combine(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        for wasm in &ctx.target_names {
            if let Some(fid) = (
                UniqueName::NAMESPACE,
                &UniqueName::SpecialFunc(&SpecialFuncUniqueName::MainVoid(wasm)),
            )
                .get_fid(&module.imports)
                .ok()
            {
                let main_void_func_name =
                    UniqueName::SpecialFunc(&SpecialFuncUniqueName::MainVoid(wasm)).to_string();

                // For synthesized targets (C/C++), skip the call-graph rewriting.
                // In Rust, `_start` calls `__main_void` internally, and this code
                // replaces that call with a fake function. For C/C++ targets, our
                // synthesized `__main_void` calls `_start` instead (reversed
                // direction), so there are no calls to rewrite.
                if self.synthesized_targets.contains(wasm.as_ref()) {
                    log::info!(
                        "Skipping main_void call-graph rewriting for synthesized target `{wasm}`."
                    );
                    module.connect_func_alt_with_remove_export(
                        fid,
                        main_void_func_name,
                        ctx.unstable_print_debug,
                    )?;
                    continue;
                }

                let main_void_func_id = main_void_func_name.get_fid(&module.exports)?;
                let start_fn_id = ctx.start_func_id.as_ref().unwrap()[wasm];

                let fake_fn_id = module.add_func(&[], &[walrus::ValType::I32], |func, _| {
                    func.func_body().i32_const(0).return_();

                    Ok(())
                })?;

                let call_main_void: i32 = module
                    .funcs
                    .rewrite(
                        |instr, _| {
                            if let walrus::ir::Instr::Call(c) = instr {
                                if c.func == main_void_func_id {
                                    c.func = fake_fn_id;
                                    1
                                } else {
                                    0
                                }
                            } else {
                                0
                            }
                        },
                        start_fn_id,
                    )
                    .wrap_err("Failed to read main_void calls")?
                    .into_iter()
                    .sum();

                if call_main_void == 0 {
                    let call_count = module
                        .funcs
                        .flat_read(
                            |instr, _| {
                                if let walrus::ir::Instr::Call(c) = instr {
                                    if c.func == main_void_func_id { 1 } else { 0 }
                                } else {
                                    0
                                }
                            },
                            start_fn_id,
                        )
                        .wrap_err("Failed to read main_void calls")?
                        .into_iter()
                        .count();

                    if call_count == 1 {
                        log::warn!(
                            "main_void is not called directly in start function, but called in nested function. we replaced once call to a fake function that returns 0."
                        );
                    } else {
                        if call_count > 1 {
                            log::warn!(
                                "main_void is not called directly in start function, and called in nested function. main_void called multiple times in start function, rust's default is once."
                            );
                        } else {
                            log::warn!(
                                "main_void is not called in nested start function, we think call_indirect is used. we replaced all calls to a fake function that returns 0."
                            );
                            // Strictly speaking, it should be limited to functions called within start_fn,
                            // but since the main_void function is only called inside start_fn and through export,
                            // it is acceptable to modify it in this function.
                            module
                                .renew_call_fn(main_void_func_id, fake_fn_id)
                                .wrap_err("Failed to rewrite main_void call in start")?;
                        }
                    }
                    let start_fn_id =
                        module.nested_copy_func(start_fn_id, &[start_fn_id], true, true)?;
                    module
                        .funcs
                        .flat_rewrite(
                            |instr, _| {
                                if let walrus::ir::Instr::Call(c) = instr {
                                    if c.func == main_void_func_id {
                                        c.func = fake_fn_id;
                                    }
                                }
                            },
                            start_fn_id,
                            false,
                        )
                        .wrap_err("Failed to read main_void calls")?;
                } else if call_main_void > 1 {
                    log::warn!(
                        "main_void called multiple times in start function, rust's default is once. we replaced all calls to a fake function that returns 0."
                    );
                }

                module.connect_func_alt_with_remove_export(
                    fid,
                    main_void_func_name,
                    ctx.unstable_print_debug,
                )?;
            } else {
                log::warn!(
                    "No main_void found for {wasm}. You can use {} directly",
                    UniqueName::SpecialFunc(&SpecialFuncUniqueName::MainVoid(wasm)).to_string()
                );
            }
        }

        Ok(())
    }
}

