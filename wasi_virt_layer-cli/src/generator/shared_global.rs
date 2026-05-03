use eyre::Context as _;
use walrus::FunctionId;

use crate::{
    args::TargetMemoryType,
    generator::{
        Generator,
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
pub struct SharedGlobal {
    before_globals: Option<usize>,
    before_memories: Option<usize>,
}

/// Enum containing identifiers for alternative shared global function replacements and locker usages.
#[derive(Debug, strum::AsRefStr, strum::EnumCount, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
pub enum SharedGlobalFnsName {
    /// Wrapper replacing a global variable assignment internally without locking overhead.
    GlobalAltSet,
    GlobalAltSetWithLock,
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
        &self,
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

        let mut lockers = used_mem_id
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

        let offset_globals = module
            .globals
            .iter()
            .filter(|g| {
                g.mutable
                    && matches!(
                        g.kind,
                        walrus::GlobalKind::Local(walrus::ConstExpr::Value(
                            walrus::ir::Value::I32(_)
                        ))
                    )
            })
            .map(|g| g.id())
            .collect::<Vec<_>>();

        // print all globals and their initial values
        log::info!("All mutable i32 globals:");
        for global_id in &offset_globals {
            let global = module.globals.get(*global_id);
            let init = match global.kind {
                walrus::GlobalKind::Local(walrus::ConstExpr::Value(walrus::ir::Value::I32(value))) => value,
                _ => unreachable!(),
            };
            log::info!(" - {:?}: initial value = {}", global_id, init);
        }

        let target_globals = &offset_globals[self.before_globals.unwrap() as usize..];

        // フラグなどでちゃんとマッチングをとるべきだが、
        // 順序が関係ないので今は成り立っている
        // todo!();
        let mut global_mappings = Vec::new();
        for (i, name) in ctx.target_names.iter().enumerate() {
            global_mappings.push((target_globals[i], name.as_ref()));
        }
        if self.before_memories.unwrap() > ctx.target_names.len() {
            // External Memory Managerのメモリサイズが0ならば、グローバルは生成されない。
            if target_globals.len() == ctx.target_names.len() + 1 {
                global_mappings.push((
                    target_globals[ctx.target_names.len()],
                    "vfs_external_memory_manager",
                ));
            }
        } else {
            log::warn!(
                "The number of target globals exceeds the number of mutable i32 globals. This may lead to incorrect behavior. Please verify the target names and global variables."
            );
        }
        // print all globals and their initial values
        log::info!("Globals to be replaced:");
        for (global_id, target_name) in &global_mappings {
            let global = module.globals.get(*global_id);
            let init = match global.kind {
                walrus::GlobalKind::Local(walrus::ConstExpr::Value(walrus::ir::Value::I32(value))) => value,
                _ => unreachable!(),
            };
            log::info!(" - {}: initial value = {}", target_name, init);
        }

        let intrrupt_fn = ctx.starts.init_offset_global.get_fid(&module.exports)?;

        for (global_id, target_name) in global_mappings {
            let global = module.globals.get(global_id);
            let init = match global.kind {
                walrus::GlobalKind::Local(walrus::ConstExpr::Value(walrus::ir::Value::I32(
                    value,
                ))) => value,
                _ => unreachable!(),
            };

            let global_set_alt_without_lock = UniqueName::SharedGlobalFnsForTarget(
                &SharedGlobalFnsName::GlobalAltSet,
                target_name,
            )
            .get_fid(&module.exports)?;
            let global_set_alt_with_lock = UniqueName::SharedGlobalFnsForTarget(
                &SharedGlobalFnsName::GlobalAltSetWithLock,
                target_name,
            )
            .get_fid(&module.exports)?;
            let global_init_alt_without_lock_once = UniqueName::SharedGlobalFnsForTarget(
                &SharedGlobalFnsName::GlobalAltInitOnce,
                target_name,
            )
            .get_fid(&module.exports)?;
            let global_get_alt_with_lock = UniqueName::SharedGlobalFnsForTarget(
                &SharedGlobalFnsName::GlobalAltGet,
                target_name,
            )
            .get_fid(&module.exports)?;
            let global_get_alt_without_lock = UniqueName::SharedGlobalFnsForTarget(
                &SharedGlobalFnsName::GlobalAltGetNoWait,
                target_name,
            )
            .get_fid(&module.exports)?;
            let global_alt_pos = UniqueName::SharedGlobalFnsForTarget(
                &SharedGlobalFnsName::GlobalAltPos,
                target_name,
            )
            .get_fid(&module.exports)?;

            module
                .exports
                .erase_with(global_alt_pos, ctx.unstable_print_debug)?;

            let start_local = module.funcs.get_mut(intrrupt_fn).kind.unwrap_local_mut();
            start_local
                .builder_mut()
                .func_body()
                .i32_const(init)
                .call(global_init_alt_without_lock_once);

            // The locker is locked at the point it is called. So we can replace
            for (_, (locker_id, _)) in lockers.iter_mut() {
                use walrus::ir::*;
                let new_locker =
                    module.nested_copy_func(*locker_id, &[] as &[FunctionId], true, true)?;

                module.funcs.flat_rewrite(
                    |instr, _| match instr {
                        Instr::GlobalGet(GlobalGet { global }) if *global == global_id => {
                            *instr = Instr::Call(Call {
                                func: global_get_alt_without_lock,
                            });
                        }
                        Instr::GlobalSet(GlobalSet { global }) if *global == global_id => {
                            *instr = Instr::Call(Call {
                                func: global_set_alt_without_lock,
                            });
                        }
                        _ => {}
                    },
                    new_locker,
                    true,
                )?;

                module.renew_call_fn(*locker_id, new_locker)?;
                *locker_id = new_locker;
            }

            module
                .funcs
                .all_rewrite(
                    |instr, _| match instr {
                        walrus::ir::Instr::GlobalSet(walrus::ir::GlobalSet { global })
                            if *global == global_id =>
                        {
                            // log::warn!(
                            //     "Rewriting global set to nop for global {:?}",
                            //     global_id,
                            // );
                            // *instr = walrus::ir::Instr::Drop(walrus::ir::Drop {});
                            *instr = walrus::ir::Instr::Call(walrus::ir::Call {
                                func: global_set_alt_with_lock,
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
                    &lockers.values().map(|(v, _)| *v).collect::<Vec<_>>(),
                )
                .wrap_err("Failed to rewrite global set/get")?;

            module.globals.delete(global_id);

            module
                .exports
                .erase_with(global_set_alt_with_lock, ctx.unstable_print_debug)?;
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
        }

        for (_, (_, name)) in lockers {
            module.exports.erase_with(&name, ctx.unstable_print_debug)?;
        }

        Ok(())
    }
}

impl Generator for SharedGlobal {
    fn post_combine(
            &mut self,
            module: &mut walrus::Module,
            _: &super::GeneratorCtx,
        ) -> eyre::Result<()> {
        self.before_globals = Some(module.globals.iter()
            .filter(|g| {
                g.mutable
                    && matches!(
                        g.kind,
                        walrus::GlobalKind::Local(walrus::ConstExpr::Value(
                            walrus::ir::Value::I32(_)
                        ))
                    )
            })
            .count());

        self.before_memories = Some(module.memories.iter().count());

        Ok(())
    }

    fn post_lower_memory(
        &mut self,
        module: &mut walrus::Module,
        ctx: &crate::generator::GeneratorCtx,
    ) -> eyre::Result<()> {
        if !matches!(ctx.target_memory_type, TargetMemoryType::Single) {
            unreachable!();
        }

        if !ctx.threads {
            return Ok(());
        }

        self.post_lower_memory_inner(module, ctx)?;

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

        // print all exports
        log::info!("Current exports:");
        for export in module.exports.iter() {
            log::info!(" - {}", export.name);
        }

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
