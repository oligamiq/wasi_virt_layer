use eyre::Context;
use itertools::Itertools;
use strum::VariantNames;

use crate::{
    abi::{Wasip1ThreadsABIExportFunc, Wasip1ThreadsABIFunc},
    generator::{Generator, GeneratorCtx},
    instrs::InstrRewrite,
    unique_name::UniqueName,
    util::{
        WalrusFID as _, WalrusUtilExport as _, WalrusUtilImport as _, WalrusUtilModule as _,
        WasmName, gen_component_name,
    },
};

/// Constants and format builders for renaming Thread spawn imports and exports during patching.
#[derive(Debug, strum::AsRefStr, strum::EnumCount, Hash, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum ThreadsSpawnName<'a> {
    /// Custom spawn anchor injected for bridging definitions.
    ImportAnchor(&'a str),
    /// Flag indicating whether the current instance is the root spawner thread.
    IsRootSpawn,
    #[strum(serialize = "wasi_thread_spawn___self")]
    /// Function identifier for native WASI spawn capability.
    WasiThreadSpawnSelf,
    #[strum(serialize = "__self_wasi_thread_start")]
    /// The starting routine mapping on thread creation execution.
    SelfWasiThreadStart,
    #[strum(serialize = "__self_wasi_thread_start_anchor")]
    /// The underlying static bridge to anchor thread origins.
    SelfWasiThreadStartAnchor,
    /// Low level function signature to actually request system spawn.
    RealThreadSpawnFn,
    /// Entry stub function used when entering newly spawned routines.
    WasiThreadStartEntry,
    /// Specialized start function per WASM module target.
    WasiThreadStart(&'a WasmName),
    #[strum(serialize = "wasi_thread_start")]
    /// Target resolving memory start capabilities per component.
    WasiThreadStartDestination(&'a WasmName),
    /// Wraps dynamic spawn allocations over components.
    WasiThreadSpawn(&'a WasmName),
    /// Cross-linking anchor marking a spawn location.
    WasiThreadStartAnchor(&'a WasmName),
    /// Initializer function for thread memory limits scaling.
    ThreadInitializer,
    /// Reference to the existing deprecated `_start` handling.
    OldStart,
}

/// The thread spawn process itself within the VFS is also caught,
/// but processing is performed to exclude only the root spawn from this.
/// Relocate thread creation from root spawn to the outer layer
///
/// **Why this is needed:**
/// Per `IMPORTS_EXPORTS_EVOLUTION_DETAILED.md`, `wasi-threads:spawn` requires special handling
/// because thread creation from the "root spawn" (the original process invoking `_start`)
/// differs from spawning within explicitly created WASI threads.
///
/// This generator ensures that only the root spawn initiates the true thread start sequence
/// on the host by intercepting `wasi_thread_spawn`. Real execution is forwarded via
/// `__wasip1_vfs_real_thread_spawn_fn`, avoiding recursive or improper initializations.
#[derive(Debug, Default)]
pub struct ThreadsSpawn;

impl Generator for ThreadsSpawn {
    fn pre_vfs(&mut self, module: &mut walrus::Module, ctx: &GeneratorCtx) -> eyre::Result<()> {
        if !ctx.threads {
            return Ok(());
        }

        let name = <Wasip1ThreadsABIFunc as VariantNames>::VARIANTS
            .iter()
            .exactly_one()
            .wrap_err("Expected exactly one variant for Wasip1ThreadsABIFunc")?; // thread-spawn

        let component_name = gen_component_name(UniqueName::WASIP1_THREADS_ABI_MODULE_ALT, name);

        module.exports.erase_with(
            &UniqueName::ThreadsSpawn(&ThreadsSpawnName::ImportAnchor(name)),
            ctx.unstable_print_debug,
        )?;

        let real_thread_spawn_fn_id =
            (UniqueName::THREADS_MODULE_ROOT, &component_name).get_fid(&module.imports)?;

        let branch_fid =
            UniqueName::ThreadsSpawn(&ThreadsSpawnName::IsRootSpawn).get_fid(&module.exports)?;

        if let Some(normal_thread_spawn_fn_id) = (UniqueName::WASIP1_THREADS_ABI_MODULE, name)
            .get_fid(&module.imports)
            .ok()
        {
            let self_thread_spawn_fn_id =
                UniqueName::ThreadsSpawn(&ThreadsSpawnName::WasiThreadSpawnSelf)
                    .get_fid(&module.exports)?;

            module
                .exports
                .erase_with(self_thread_spawn_fn_id, ctx.unstable_print_debug)?;

            use walrus::ValType::I32;
            let real_thread_spawn_fn_id = module
                .add_func(&[I32], &[I32], |builder, args| {
                    let mut body = builder.func_body();
                    body.call(branch_fid)
                        .if_else(
                            I32,
                            |then| {
                                then.local_get(args[0]) // pass the argument to thread-spawn
                                    .call(real_thread_spawn_fn_id);
                            },
                            |else_| {
                                else_
                                    .local_get(args[0]) // pass the argument to thread-spawn
                                    .call(self_thread_spawn_fn_id); // call thread-spawn
                            },
                        )
                        .return_();

                    Ok(())
                })
                .wrap_err("Failed to add real thread spawn function")?;

            module
                .renew_call_fn(normal_thread_spawn_fn_id, real_thread_spawn_fn_id)
                .wrap_err("Failed to rewrite thread-spawn call")?;

            let start_name = Wasip1ThreadsABIExportFunc::VARIANTS
                .iter()
                .exactly_one()
                .wrap_err("Expected exactly one variant for Wasip1ThreadsABIExportFunc")?; // wasi-thread-start

            let exporting_thread_starter_id = start_name.get_fid(&module.exports)?;

            module
                .connect_func_alt_with_remove_export(
                    (
                        UniqueName::NAMESPACE,
                        &UniqueName::ThreadsSpawn(&ThreadsSpawnName::SelfWasiThreadStart),
                    ),
                    start_name,
                    ctx.unstable_print_debug,
                )
                .wrap_err("Failed to rewrite self_wasi_thread_start call in root spawn")?;

            module.exports.erase_with(
                &UniqueName::ThreadsSpawn(&ThreadsSpawnName::SelfWasiThreadStartAnchor),
                ctx.unstable_print_debug,
            )?;

            if ctx.unstable_print_debug {
                module.exports.add(
                    &UniqueName::ThreadsSpawn(&ThreadsSpawnName::RealThreadSpawnFn).to_string(),
                    real_thread_spawn_fn_id,
                );
            }

            // __wasip1_vfs_self_wasi_thread_start
            module
                .renew_call_fn(
                    (
                        UniqueName::NAMESPACE,
                        &UniqueName::ThreadsSpawn(&ThreadsSpawnName::WasiThreadStartEntry),
                    ),
                    exporting_thread_starter_id,
                )
                .wrap_err("Failed to connect wasip1-vfs.wasi_thread_start")?;

            module
                .exports
                .erase_with(branch_fid, ctx.unstable_print_debug)?;
        } else {
            log::warn!("No normal thread-spawn found, why are you not using threads?");

            let component_name =
                gen_component_name(UniqueName::WASIP1_THREADS_ABI_MODULE_ALT, name);

            let _real_thread_spawn_fn_id =
                (UniqueName::THREADS_MODULE_ROOT, &component_name).get_fid(&module.imports)?;

            module.exports.erase_with(
                &UniqueName::ThreadsSpawn(&ThreadsSpawnName::IsRootSpawn),
                ctx.unstable_print_debug,
            )?;

            let fake_start = module
                .add_func(
                    &[walrus::ValType::I32, walrus::ValType::I32],
                    &[],
                    |_, _| Ok(()),
                )
                .wrap_err("Failed to add fake thread spawn function")?;

            module
                .renew_call_fn(
                    (
                        UniqueName::NAMESPACE,
                        &UniqueName::ThreadsSpawn(&ThreadsSpawnName::WasiThreadStartEntry),
                    ),
                    fake_start,
                )
                .wrap_err("Failed to connect wasip1-vfs.wasi_thread_start")?;

            // println!("unstable_print_debug: {}", ctx.unstable_print_debug);
        }

        Ok(())
    }

    fn pre_target(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
        external: &crate::generator::ModuleExternal,
    ) -> eyre::Result<()> {
        if !ctx.threads {
            return Ok(());
        }

        let wasm = &external.name;

        let start_name = Wasip1ThreadsABIFunc::VARIANTS
            .iter()
            .exactly_one()
            .wrap_err("Expected exactly one variant for Wasip1ThreadsABIExportFunc")?; // thread-spawn

        module
            .imports
            .find_mut((UniqueName::WASIP1_THREADS_ABI_MODULE, start_name))
            .ok()
            .map(|import| {
                // import.name = format!("__wasip1_vfs_wasi_thread_spawn_{name}");
                import.name =
                    UniqueName::ThreadsSpawn(&ThreadsSpawnName::WasiThreadSpawn(wasm)).to_string();
            });

        let export_name = Wasip1ThreadsABIExportFunc::VARIANTS
            .iter()
            .exactly_one()
            .wrap_err("Expected exactly one variant for Wasip1ThreadsABIExportFunc")?; // wasi_thread_start

        module
            .exports
            .iter_mut()
            .find(|export| export.name == *export_name)
            .map(|export| {
                export.name =
                    UniqueName::ThreadsSpawn(&ThreadsSpawnName::WasiThreadStartDestination(wasm))
                        .to_string();
            });

        Ok(())
    }
}

/// https://github.com/rust-lang/rust/issues/146843
/// thread spawn is broken on wasm32-wasip1-threads for building library
///
/// **Why this is needed:**
/// Currently, thread initialization routines (e.g. `__wasm_init_tls`) can be improperly tied
/// or omitted during WASM generation (see rust-lang/rust#146843). This patching strategy
/// wraps the original `_start` with an injected `ThreadInitializer` call if one is exported.
#[derive(Debug, Default)]
pub struct ThreadsSpawnPatch;

impl Generator for ThreadsSpawnPatch {
    fn pre_vfs(&mut self, module: &mut walrus::Module, ctx: &GeneratorCtx) -> eyre::Result<()> {
        let initializer = UniqueName::ThreadsSpawn(&ThreadsSpawnName::ThreadInitializer)
            .get_fid(&module.exports)
            .ok();

        if let Some(init_id) = initializer {
            module
                .exports
                .erase_with(init_id, ctx.unstable_print_debug)?;

            let init = ctx.starts.thread_patch.get_fid(&module.exports)?;
            module
                .funcs
                .get_mut(init)
                .kind
                .unwrap_local_mut()
                .builder_mut()
                .func_body()
                .call(init_id);
        }

        Ok(())
    }
}

/// Rewrites `memory.atomic.wait` and `memory.atomic.notify` in target modules
/// to VFS alternate implementations to handle offset shifts during memory growth safely.
#[derive(Debug, Default)]
pub struct AtomicPatch;

impl AtomicPatch {
    /// Creates a wrapper function that adds a constant offset to the address argument,
    /// pushes the `wasm_id`, and then delegates to the base VFS import function.
    ///
    /// For wait32/wait64: `(addr, expected, timeout) -> (wasm_id, addr+offset, expected, timeout)`
    /// For notify:        `(addr, count)             -> (wasm_id, addr+offset, count)`
    fn make_offset_wrapper(
        module: &mut walrus::Module,
        base_func: walrus::FunctionId,
        wasm_id: u32,
        offset: u64,
        params: &[walrus::ValType],
        results: &[walrus::ValType],
    ) -> walrus::FunctionId {
        let mut builder = walrus::FunctionBuilder::new(&mut module.types, params, results);
        let args: Vec<walrus::LocalId> = params.iter().map(|ty| module.locals.add(*ty)).collect();

        {
            let mut body = builder.func_body();
            // wasm_id
            body.i32_const(wasm_id as i32);
            // addr + offset
            body.local_get(args[0]);
            if offset != 0 {
                body.i32_const(offset as i32)
                    .binop(walrus::ir::BinaryOp::I32Add);
            }
            // remaining args
            for arg in &args[1..] {
                body.local_get(*arg);
            }
            body.call(base_func);
        }

        builder.finish(args, &mut module.funcs)
    }
}

impl Generator for AtomicPatch {
    fn pre_target(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
        external: &crate::generator::ModuleExternal,
    ) -> eyre::Result<()> {
        if !ctx.threads {
            return Ok(());
        }

        let wasm_id = ctx
            .target_names
            .iter()
            .position(|n| n.as_str() == external.name.as_str())
            .unwrap_or(0) as u32;

        use walrus::ValType::{I32, I64};

        // Import base VFS atomic functions (take wasm_id as first arg now)
        let wait32_ty = module.types.add(&[I32, I32, I32, I64], &[I32]);
        let (wait32_import, _) =
            module.add_import_func("wasi_snapshot_preview1", "__vfs_atomic_wait32", wait32_ty);

        let wait64_ty = module.types.add(&[I32, I32, I64, I64], &[I32]);
        let (wait64_import, _) =
            module.add_import_func("wasi_snapshot_preview1", "__vfs_atomic_wait64", wait64_ty);

        let notify_ty = module.types.add(&[I32, I32, I32], &[I32]);
        let (notify_import, _) =
            module.add_import_func("wasi_snapshot_preview1", "__vfs_atomic_notify", notify_ty);

        // --- Pass 1: Collect unique offsets via DFS visitor ---
        let mut wait32_offsets = std::collections::BTreeSet::new();
        let mut wait64_offsets = std::collections::BTreeSet::new();
        let mut notify_offsets = std::collections::BTreeSet::new();

        {
            struct OffsetCollector<'a> {
                wait32: &'a mut std::collections::BTreeSet<u64>,
                wait64: &'a mut std::collections::BTreeSet<u64>,
                notify: &'a mut std::collections::BTreeSet<u64>,
            }

            impl<'instr> walrus::ir::Visitor<'instr> for OffsetCollector<'_> {
                fn start_instr_seq(&mut self, seq: &'instr walrus::ir::InstrSeq) {
                    for (instr, _) in seq.instrs.iter() {
                        match instr {
                            walrus::ir::Instr::AtomicWait(w) => {
                                if w.sixty_four {
                                    self.wait64.insert(w.arg.offset);
                                } else {
                                    self.wait32.insert(w.arg.offset);
                                }
                            }
                            walrus::ir::Instr::AtomicNotify(n) => {
                                self.notify.insert(n.arg.offset);
                            }
                            _ => {}
                        }
                    }
                }
            }

            for (_fid, func) in module.funcs.iter_local() {
                let entry = func.entry_block();
                let mut visitor = OffsetCollector {
                    wait32: &mut wait32_offsets,
                    wait64: &mut wait64_offsets,
                    notify: &mut notify_offsets,
                };
                walrus::ir::dfs_in_order(&mut visitor, func, entry);
            }
        }

        // --- Create wrapper functions for each unique offset ---
        let mut wait32_map: std::collections::HashMap<u64, walrus::FunctionId> =
            std::collections::HashMap::new();
        let mut wait64_map: std::collections::HashMap<u64, walrus::FunctionId> =
            std::collections::HashMap::new();
        let mut notify_map: std::collections::HashMap<u64, walrus::FunctionId> =
            std::collections::HashMap::new();

        for offset in wait32_offsets {
            let fid = Self::make_offset_wrapper(
                module,
                wait32_import,
                wasm_id,
                offset,
                &[I32, I32, I64],
                &[I32],
            );
            wait32_map.insert(offset, fid);
        }
        for offset in wait64_offsets {
            let fid = Self::make_offset_wrapper(
                module,
                wait64_import,
                wasm_id,
                offset,
                &[I32, I64, I64],
                &[I32],
            );
            wait64_map.insert(offset, fid);
        }
        for offset in notify_offsets {
            let fid = Self::make_offset_wrapper(
                module,
                notify_import,
                wasm_id,
                offset,
                &[I32, I32],
                &[I32],
            );
            notify_map.insert(offset, fid);
        }

        // --- Pass 2: Rewrite all atomic.wait/notify to calls ---
        let funcs: Vec<(walrus::FunctionId, &mut walrus::LocalFunction)> =
            module.funcs.iter_local_mut().collect();
        use rayon::prelude::*;
        funcs.into_par_iter().try_for_each(|(_fid, func)| {
            let mut body = func.builder_mut().func_body();

            body.rewrite(|instr, _pos| {
                let new_instr = match instr {
                    walrus::ir::Instr::AtomicWait(w) => {
                        if !w.sixty_four {
                            let target = wait32_map[&w.arg.offset];
                            Some(walrus::ir::Instr::Call(walrus::ir::Call { func: target }))
                        } else {
                            let target = wait64_map[&w.arg.offset];
                            Some(walrus::ir::Instr::Call(walrus::ir::Call { func: target }))
                        }
                    }
                    walrus::ir::Instr::AtomicNotify(n) => {
                        let target = notify_map[&n.arg.offset];
                        Some(walrus::ir::Instr::Call(walrus::ir::Call { func: target }))
                    }
                    _ => None,
                };
                if let Some(n) = new_instr {
                    *instr = n;
                }
            })
            .map_err(|e| eyre::eyre!("{e}"))?;

            eyre::Ok(())
        })?;

        Ok(())
    }

    fn post_combine(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        if !ctx.threads {
            return Ok(());
        }

        let vfs_mem = ctx.vfs_used_memory_id.unwrap();

        use crate::util::{WalrusFID, WalrusUtilModule};

        if let Some(id) = ("wvl_atomic", "__wvl_atomic_wait32_vfs")
            .get_fid(&module.imports)
            .ok()
        {
            module
                .replace_imported_func(id, |(builder, args)| {
                    builder
                        .func_body()
                        .local_get(args[0])
                        .local_get(args[1])
                        .local_get(args[2])
                        .atomic_wait(
                            vfs_mem,
                            walrus::ir::MemArg {
                                align: 4,
                                offset: 0,
                            },
                            false,
                        );
                })
                .map_err(|e| eyre::eyre!("{e}"))?;
        }

        if let Some(id) = ("wvl_atomic", "__wvl_atomic_notify_vfs")
            .get_fid(&module.imports)
            .ok()
        {
            module
                .replace_imported_func(id, |(builder, args)| {
                    builder
                        .func_body()
                        .local_get(args[0])
                        .local_get(args[1])
                        .atomic_notify(
                            vfs_mem,
                            walrus::ir::MemArg {
                                align: 4,
                                offset: 0,
                            },
                        );
                })
                .map_err(|e| eyre::eyre!("{e}"))?;
        }

        if let Some(id) = ("wvl_atomic", "__wvl_atomic_cmpxchg32_vfs")
            .get_fid(&module.imports)
            .ok()
        {
            module
                .replace_imported_func(id, |(builder, args)| {
                    builder
                        .func_body()
                        .local_get(args[0])
                        .local_get(args[1])
                        .local_get(args[2])
                        .cmpxchg(
                            vfs_mem,
                            walrus::ir::AtomicWidth::I32,
                            walrus::ir::MemArg {
                                align: 4,
                                offset: 0,
                            },
                        );
                })
                .map_err(|e| eyre::eyre!("{e}"))?;
        }

        if let Some(id) = ("wvl_atomic", "__wvl_atomic_store32_vfs")
            .get_fid(&module.imports)
            .ok()
        {
            module
                .replace_imported_func(id, |(builder, args)| {
                    builder
                        .func_body()
                        .local_get(args[0])
                        .local_get(args[1])
                        .store(
                            vfs_mem,
                            walrus::ir::StoreKind::I32 { atomic: true },
                            walrus::ir::MemArg {
                                align: 4,
                                offset: 0,
                            },
                        );
                })
                .map_err(|e| eyre::eyre!("{e}"))?;
        }

        if let Some(id) = ("wvl_atomic", "__wvl_atomic_load32_target")
            .get_fid(&module.imports)
            .ok()
        {
            module
                .replace_imported_func(id, |(builder, args)| {
                    let mut body = builder.func_body();
                    for (i, name) in ctx.target_names.iter().enumerate() {
                        if let Some(&mem) = ctx.target_used_memory_id.as_ref().unwrap().get(name) {
                            body.local_get(args[0])
                                .i32_const(i as i32)
                                .binop(walrus::ir::BinaryOp::I32Eq)
                                .if_else(
                                    None,
                                    |then| {
                                        then.local_get(args[1])
                                            .load(
                                                mem,
                                                walrus::ir::LoadKind::I32 { atomic: true },
                                                walrus::ir::MemArg {
                                                    align: 4,
                                                    offset: 0,
                                                },
                                            )
                                            .return_();
                                    },
                                    |_else| {},
                                );
                        }
                    }
                    body.unreachable();
                })
                .map_err(|e| eyre::eyre!("{e}"))?;
        }

        if let Some(id) = ("wvl_atomic", "__wvl_atomic_load64_target")
            .get_fid(&module.imports)
            .ok()
        {
            module
                .replace_imported_func(id, |(builder, args)| {
                    let mut body = builder.func_body();
                    for (i, name) in ctx.target_names.iter().enumerate() {
                        if let Some(&mem) = ctx.target_used_memory_id.as_ref().unwrap().get(name) {
                            body.local_get(args[0])
                                .i32_const(i as i32)
                                .binop(walrus::ir::BinaryOp::I32Eq)
                                .if_else(
                                    None,
                                    |then| {
                                        then.local_get(args[1])
                                            .load(
                                                mem,
                                                walrus::ir::LoadKind::I64 { atomic: true },
                                                walrus::ir::MemArg {
                                                    align: 8,
                                                    offset: 0,
                                                },
                                            )
                                            .return_();
                                    },
                                    |_else| {},
                                );
                        }
                    }
                    body.unreachable();
                })
                .map_err(|e| eyre::eyre!("{e}"))?;
        }

        Ok(())
    }
}
