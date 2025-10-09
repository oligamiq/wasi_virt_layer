use eyre::{Context as _, ContextCompat as _};
use walrus::FunctionId;

use crate::{
    args::TargetMemoryType,
    generator::Generator,
    instrs::InstrRewrite as _,
    util::{WalrusFID as _, WalrusUtilFuncs as _, WalrusUtilModule as _},
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
#[derive(Debug, Default)]
pub struct SharedGlobal;

impl Generator for SharedGlobal {
    fn post_combine(
        &mut self,
        module: &mut walrus::Module,
        ctx: &crate::generator::GeneratorCtx,
    ) -> eyre::Result<()> {
        if !matches!(ctx.target_memory_type, TargetMemoryType::Single) {
            return Ok(());
        }

        if !ctx.threads {
            return Ok(());
        }

        use std::collections::{HashMap, HashSet};
        use walrus::ir::*;

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
                Self::gen_custom_locker(module, mem_id)
                    .wrap_err("Failed to generate custom locker function")
                    .map(|locker_id| (mem_id, locker_id))
            })
            .collect::<eyre::Result<HashMap<_, _>>>()?;

        Self::remove_gen_custom_locker_base(module, ctx.unstable_print_debug)
            .wrap_err("Failed to remove base locker function")?;

        module.funcs.all_rewrite(
            |instr, _| {
                if let Instr::MemoryGrow(MemoryGrow { memory, .. }) = instr {
                    *instr = Instr::Call(Call {
                        func: lockers.get(memory).unwrap().to_owned(),
                    });
                }
            },
            &lockers.values().cloned().collect::<Vec<_>>(),
        )?;

        Ok(())
    }

    fn post_lower_memory(
        &mut self,
        module: &mut walrus::Module,
        ctx: &crate::generator::GeneratorCtx,
    ) -> eyre::Result<()> {
        if !matches!(ctx.target_memory_type, TargetMemoryType::Single) {
            return Ok(());
        }

        if !ctx.threads {
            return Ok(());
        }

        let global_set_alt_without_lock =
            "__wasip1_vfs_memory_grow_global_alt_set".get_fid(&module.exports)?;
        let global_init_alt_without_lock_once =
            "__wasip1_vfs_memory_grow_global_alt_init_once".get_fid(&module.exports)?;
        let global_get_alt_with_lock =
            "__wasip1_vfs_memory_grow_global_alt_get".get_fid(&module.exports)?;
        let global_get_alt_without_lock =
            "__wasip1_vfs_memory_grow_global_alt_get_no_wait".get_fid(&module.exports)?;

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
        // let global_alt_pos = "__wasip1_vfs_memory_grow_global_alt_pos".get_fid(&module.exports)?;
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

        // check global set in start section function
        let start_id = if let Some(id) = module.start {
            module.nested_copy_func(id, &[] as &[FunctionId], false, false)?
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

        let lockers = module
            .exports
            .iter()
            .filter_map(|e| {
                if e.name.starts_with("__wasip1_vfs_memory_grow_locker_") {
                    if let walrus::ExportItem::Function(fid) = e.item {
                        Some(fid)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // The locker is locked at the point it is called. So we can replace
        for locker_id in lockers {
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

        Ok(())
    }
}

impl SharedGlobal {
    fn gen_custom_locker(
        module: &mut walrus::Module,
        mem_id: walrus::MemoryId,
    ) -> eyre::Result<walrus::FunctionId> {
        let alt_id = ("wasip1-vfs_single_memory", "__wasip1_vfs_memory_grow_alt")
            .get_fid(&module.imports)?;
        let base_locker = "__wasip1_vfs_memory_grow_locker".get_fid(&module.exports)?;

        let locker_id = module.copy_func(base_locker)?;
        module.exports.add(
            &format!("__wasip1_vfs_memory_grow_locker_{}", mem_id.index()),
            locker_id,
        );
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

        Ok(locker_id)
    }

    fn remove_gen_custom_locker_base(module: &mut walrus::Module, debug: bool) -> eyre::Result<()> {
        use walrus::ir::*;

        let alt_id = ("wasip1-vfs_single_memory", "__wasip1_vfs_memory_grow_alt")
            .get_fid(&module.imports)?;
        let base_locker = "__wasip1_vfs_memory_grow_locker".get_fid(&module.exports)?;
        if !debug {
            module.funcs.delete(base_locker);
            module.funcs.delete(alt_id);

            module
                .exports
                .remove("__wasip1_vfs_memory_grow_locker")
                .unwrap();
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

        module
            .imports
            .remove("wasip1-vfs_single_memory", "__wasip1_vfs_memory_grow_alt")
            .unwrap();

        Ok(())
    }
}
