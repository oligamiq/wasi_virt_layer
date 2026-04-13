use eyre::{Context as _, ContextCompat as _};
use walrus::FunctionId;

use crate::{
    args::TargetMemoryType,
    generator::{
        Generator,
        start_section::{StartFnInfo, StartFnPriority, StartSource},
    },
    instrs::InstrRewrite as _,
    unique_name::UniqueName,
    util::{
        WalrusFID as _, WalrusUtilExport, WalrusUtilFuncs as _, WalrusUtilImport as _,
        WalrusUtilModule as _,
    },
};

/// https://github.com/WebAssembly/binaryen/issues/7916
/// 0: Failed to load Wasm file: ./dist\threads_vfs.core.opt.adjusted.wasm
/// 1: failed to parse global section
/// 2: malformed mutability -- or shared globals require the shared-everything-threads proposal (at offset 0x49f)
///
/// The Globals causing errors during memory expansion are those generated
/// by wasm-opt --multi-memory-lowering,
/// so for now we will only address these.
/// When a newly created thread is executed,
/// it will use the always-executable VFS code and memory,
/// which are based on an address that never changes,
/// and perform operations on them atomically.
/// Operations on Global variables are replaced,
/// and before memory unification,
/// memory.grow is modified to be an atomic operation.
/// Since this Global variable should only be modified internally,
/// this approach should be sufficient.
/// module
///     .globals
///     .iter()
///     .map(|g| g.id())
///     .collect::<Vec<_>>()
///     .iter()
///     .for_each(|g| {
///         let g = module.globals.get_mut(*g);
///         if let walrus::GlobalKind::Local(_) = g.kind {
///             if g.mutable {
///                 g.shared = true;
///             }
///         }
///     });
/// Generator responsible for managing globally shared variables to enable multi-threading atomic modifications.
#[derive(Debug, Default)]
pub struct SharedGlobal;

/// Enum containing identifiers for alternative shared global function replacements and locker usages.
#[derive(Debug, strum::AsRefStr, strum::EnumCount, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
pub enum SharedGlobalFnsName {
    /// Wrapper replacing a global variable assignment internally without locking overhead.
    GlobalAltSet,
    /// Thread-safe wrapper evaluating and retrieving the global value.
    GlobalAltGet,
    /// Fast wrapper evaluating the global value bypassing thread synchronization lock operations.
    GlobalAltGetNoWait,
    /// Singleton wrapper assigning an initializing value solely on a one-time startup sequence.
    GlobalAltInitOnce,
    /// Utility function yielding the exact offset index memory location storing the global proxy.
    GlobalAltPos,
    /// Locking function instance controlling concurrent memory accesses for a specific table identified by index.
    Locker(usize),
    #[strum(serialize = "locker")]
    /// Uniquely identified primary initial lock mechanism controlling baseline single-memory environments.
    LockerBase,
    #[strum(serialize = "alt")]
    /// Replaced function logic hooking WebAssembly natively executed `memory.grow` allocation algorithms.
    MemoryGrowAlt,
}

impl SharedGlobalFnsName {
    /// Checks whether the provided string identifier maps to a locker operation, extracting its index.
    pub fn check_locker(str: impl AsRef<str>) -> Option<SharedGlobalFnsName> {
        let s = str.as_ref();
        let prefix = crate::unique_name::fmt!(SharedGlobalFns; "{}", SharedGlobalFnsName::Locker(0).as_ref());
        if s.starts_with(&prefix) && s.len() > prefix.len() {
            let index_str = &s[prefix.len() + 1..];
            if let Ok(index) = index_str.parse::<usize>() {
                return Some(SharedGlobalFnsName::Locker(index));
            }
        }
        None
    }
}

impl SharedGlobal {
    fn post_lower_memory_inner(
        module: &mut walrus::Module,
        ctx: &crate::generator::GeneratorCtx,
    ) -> eyre::Result<()> {
        use std::collections::{HashMap, HashSet};
        use walrus::ir::*;

        if !matches!(ctx.target_memory_type, TargetMemoryType::Single) {
            unreachable!();
        }

        if !ctx.threads {
            return Ok(());
        }

        let used_mem_id = module
            .funcs
            .all_read(
                |instr, _| {
                    if let Instr::MemoryGrow(MemoryGrow { memory, .. }) = instr {
                        Some(*memory)
                    } else {
                        None
                    }
                },
                &[] as &[walrus::FunctionId],
            )?
            .into_iter()
            .filter_map(|v| v)
            .collect::<HashSet<_>>();

        let lockers = used_mem_id
            .into_iter()
            .map(|mem_id| {
                Self::gen_custom_locker(module, mem_id, ctx.unstable_print_debug)
                    .wrap_err("Failed to generate custom locker function")
                    .map(|(locker_id, export_name)| (mem_id, (locker_id, export_name)))
            })
            .collect::<eyre::Result<HashMap<_, _>>>()?;

        Self::remove_gen_custom_locker_base(module, ctx.unstable_print_debug)
            .wrap_err("Failed to remove base locker function")?;

        module.funcs.all_rewrite(
            |instr, _| {
                if let Instr::MemoryGrow(MemoryGrow { memory, .. }) = instr {
                    *instr = Instr::Call(Call {
                        func: lockers.get(memory).unwrap().to_owned().0,
                    });
                }
            },
            &lockers.values().map(|(v, _)| *v).collect::<Vec<_>>(),
        )?;

        let global_set_alt_without_lock =
            UniqueName::SharedGlobalFns(&SharedGlobalFnsName::GlobalAltSet)
                .get_fid(&module.exports)?;
        let global_init_alt_without_lock_once =
            UniqueName::SharedGlobalFns(&SharedGlobalFnsName::GlobalAltInitOnce)
                .get_fid(&module.exports)?;
        let global_get_alt_with_lock =
            UniqueName::SharedGlobalFns(&SharedGlobalFnsName::GlobalAltGet)
                .get_fid(&module.exports)?;
        let global_get_alt_without_lock =
            UniqueName::SharedGlobalFns(&SharedGlobalFnsName::GlobalAltGetNoWait)
                .get_fid(&module.exports)?;

        let global = module
            .globals
            .iter()
            .last()
            .map(|g| g)
            .wrap_err_with(|| eyre::eyre!("Failed to get global ID"))?;

        let init = match global.kind {
            walrus::GlobalKind::Local(walrus::ConstExpr::Value(walrus::ir::Value::I32(value))) => {
                value
            }
            _ => unreachable!(),
        };

        let global_id = global.id();

        // Obtain the location within memory.
        let global_alt_pos = UniqueName::SharedGlobalFns(&SharedGlobalFnsName::GlobalAltPos)
            .get_fid(&module.exports)?;
        // let global_alt_pos = module.funcs.get(global_alt_pos).kind.unwrap_local();
        // let global_alt_pos = if let walrus::ir::Instr::Const(walrus::ir::Const {
        //     value: walrus::ir::Value::I32(value),
        // }) = global_alt_pos
        //     .block(global_alt_pos.entry_block())
        //     .instrs
        //     .first()
        //     .unwrap()
        //     .0
        // {
        //     value
        // } else {
        //     unreachable!()
        // };
        module
            .exports
            .erase_with(global_alt_pos, ctx.unstable_print_debug)?;

        // check global set in start section function
        let start_id = if let Some(id) = module.start {
            module
                .nested_copy_func(id, &[id], false, false)
                .wrap_err("Failed to create start function copy")?
        } else {
            // create a new start function
            module.add_func(&[], &[], |_, _| Ok(()))?
        };
        module.start = Some(start_id);

        if 0usize
            < module
                .funcs
                .flat_rewrite(
                    |instr, _| match instr {
                        walrus::ir::Instr::GlobalSet(walrus::ir::GlobalSet { global })
                            if *global == global_id =>
                        {
                            1usize
                        }
                        walrus::ir::Instr::GlobalGet(walrus::ir::GlobalGet { global })
                            if *global == global_id =>
                        {
                            *instr = walrus::ir::Instr::Const(walrus::ir::Const {
                                value: walrus::ir::Value::I32(init),
                            });
                            // println!("Rewrote global get to const i32 {init}");
                            0usize
                        }
                        _ => 0usize,
                    },
                    start_id,
                    false,
                )?
                .into_iter()
                .sum()
        {
            eyre::bail!(
                "The start section already contains a global set instruction. \
                Please remove it manually and try again."
            );
        }

        let start_local = module.funcs.get_mut(start_id).kind.unwrap_local_mut();
        start_local
            .builder_mut()
            .func_body()
            .i32_const(init)
            .call(global_init_alt_without_lock_once);

        // The locker is locked at the point it is called. So we can replace
        for (_, (locker_id, name)) in lockers {
            // println!("Rewriting locker: {:?}", locker_id);
            use walrus::ir::*;
            let new_locker =
                // module.nested_copy_func(locker_id, &[] as &[FunctionId], false, false)?;
                module.nested_copy_func(locker_id, &[] as &[FunctionId], true, true)?;

            module.funcs.flat_rewrite(
                |instr, _| match instr {
                    Instr::GlobalGet(GlobalGet { global }) if *global == global_id => {
                        *instr = Instr::Call(Call {
                            func: global_get_alt_without_lock,
                        });
                    }
                    _ => {}
                },
                new_locker,
                true,
            )?;

            module.renew_call_fn(locker_id, new_locker)?;

            module.exports.erase_with(&name, ctx.unstable_print_debug)?;
        }

        module
            .funcs
            .all_rewrite(
                |instr, _| match instr {
                    walrus::ir::Instr::GlobalSet(walrus::ir::GlobalSet { global })
                        if *global == global_id =>
                    {
                        *instr = walrus::ir::Instr::Call(walrus::ir::Call {
                            func: global_set_alt_without_lock,
                        });
                    }
                    walrus::ir::Instr::GlobalGet(walrus::ir::GlobalGet { global })
                        if *global == global_id =>
                    {
                        *instr = walrus::ir::Instr::Call(walrus::ir::Call {
                            func: global_get_alt_with_lock,
                        });
                    }
                    _ => {}
                },
                &[] as &[walrus::FunctionId],
            )
            .wrap_err("Failed to rewrite global set/get")?;

        module.globals.delete(global_id);

        module
            .exports
            .erase_with(global_set_alt_without_lock, ctx.unstable_print_debug)?;
        module
            .exports
            .erase_with(global_init_alt_without_lock_once, ctx.unstable_print_debug)?;
        module
            .exports
            .erase_with(global_get_alt_with_lock, ctx.unstable_print_debug)?;
        module
            .exports
            .erase_with(global_get_alt_without_lock, ctx.unstable_print_debug)?;

        Ok(())
    }
}

impl Generator for SharedGlobal {
    fn post_lower_memory(
        &mut self,
        module: &mut walrus::Module,
        ctx: &crate::generator::GeneratorCtx,
    ) -> eyre::Result<()> {
        use std::collections::{HashMap, HashSet};
        use walrus::ir::*;

        if !matches!(ctx.target_memory_type, TargetMemoryType::Single) {
            unreachable!();
        }

        if !ctx.threads {
            return Ok(());
        }

        ctx.start_section_builder
            .as_ref()
            .unwrap()
            .add_start_fn(StartFnInfo {
                priority: StartFnPriority::AfterAll,
                source: StartSource::Rewrite(Some(Box::new(|module, ctx| {
                    SharedGlobal::post_lower_memory_inner(module, ctx)
                }))),
            });

        Ok(())
    }
}

impl SharedGlobal {
    #[allow(unused_variables)]
    fn gen_custom_locker(
        module: &mut walrus::Module,
        mem_id: walrus::MemoryId,
        is_debug: bool,
    ) -> eyre::Result<(walrus::FunctionId, String)> {
        let alt_id = (
            "wasip1-vfs_single_memory",
            &UniqueName::SharedGlobalFns(&SharedGlobalFnsName::MemoryGrowAlt),
        )
            .get_fid(&module.imports)?;
        let base_locker = UniqueName::SharedGlobalFns(&SharedGlobalFnsName::LockerBase)
            .get_fid(&module.exports)?;

        let locker_id = module.copy_func(base_locker)?;

        let export_name =
            UniqueName::SharedGlobalFns(&SharedGlobalFnsName::Locker(mem_id.index())).to_string();
        // todo!(); This is essential for it to function.
        {
            module.exports.add(&export_name, locker_id);
        }

        let locker = module.funcs.get_mut(locker_id);

        use walrus::ir::*;

        locker
            .kind
            .unwrap_local_mut()
            .builder_mut()
            .func_body()
            .rewrite(|instr, _| {
                if let Instr::Call(Call { func }) = instr {
                    if *func == alt_id {
                        *instr = Instr::MemoryGrow(MemoryGrow { memory: mem_id });
                    }
                }
            })?;

        Ok((locker_id, export_name))
    }

    fn remove_gen_custom_locker_base(module: &mut walrus::Module, debug: bool) -> eyre::Result<()> {
        use walrus::ir::*;

        let alt_id = (
            "wasip1-vfs_single_memory",
            &UniqueName::SharedGlobalFns(&SharedGlobalFnsName::MemoryGrowAlt),
        )
            .get_fid(&module.imports)?;
        let base_locker = UniqueName::SharedGlobalFns(&SharedGlobalFnsName::LockerBase)
            .get_fid(&module.exports)?;
        if !debug {
            module.funcs.delete(base_locker);
            module.funcs.delete(alt_id);

            module.exports.erase_with(base_locker, debug)?;
        } else {
            let mem_id = module.memories.iter().next().unwrap().id();

            module
                .funcs
                .get_mut(base_locker)
                .kind
                .unwrap_local_mut()
                .builder_mut()
                .func_body()
                .rewrite(|instr, _| {
                    if let Instr::Call(Call { func }) = instr {
                        if *func == alt_id {
                            *instr = Instr::MemoryGrow(MemoryGrow { memory: mem_id });
                        }
                    }
                })?;
        }

        module.imports.erase(alt_id)?;

        Ok(())
    }
}
