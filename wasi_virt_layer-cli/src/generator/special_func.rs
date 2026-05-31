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
    pub fn flush(
        mut self,
        module: &mut walrus::Module,
        threads: bool,
    ) -> eyre::Result<Option<MemoryId>> {
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

        if self.current_size == 0 {
            // remove
            module.memories.delete(self.mem_id);
            return Ok(None);
        }

        Ok(Some(self.mem_id))
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

        for wasm in &ctx.target_names {
            let wasm_mem = ctx.target_used_memory_id.as_ref().unwrap()[wasm];
            let starter = ctx.starts.flesh_target_start[wasm].get_fid(&module.exports)?;

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

                        body.call(starter);
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

            module
                .replace_imported_func(reset_on_thread_once, |(builder, _)| {
                    builder.func_body().call(initializers);
                })
                .to_eyre()
                .wrap_err("Failed to replace reset_on_thread_once import")?;

            reset_on_thread
        } else {
            initializers
        };

        let save_target_memory = ctx.starts.save_target_memory.get_fid(&module.exports)?;

        module
            .funcs
            .get_mut(save_target_memory)
            .kind
            .unwrap_local_mut()
            .builder_mut()
            .func_body()
            .call(init_id);

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
        _module: &mut walrus::Module,
        _ctx: &GeneratorCtx,
        _external: &crate::generator::ModuleExternal,
    ) -> eyre::Result<()> {
        Ok(())
    }

    fn post_combine(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
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
        _module: &mut walrus::Module,
        _: &GeneratorCtx,
        _external: &ModuleExternal,
    ) -> eyre::Result<()> {
        Ok(())
    }

    fn post_combine(
        &mut self,
        _module: &mut walrus::Module,
        _ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        Ok(())
    }
}
