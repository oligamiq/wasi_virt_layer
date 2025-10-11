use std::{cmp, collections::HashMap};

use eyre::{Context as _, ContextCompat as _};
use itertools::Itertools;

use crate::{
    abi::Wasip1ABIFunc,
    generator::{Generator, ModuleExternal},
    instrs::{InstrRead as _, InstrRewrite},
    util::{WalrusFID, WalrusUtilFuncs as _, WalrusUtilModule as _},
};

macro_rules! get_instr {
    ($seq:expr, $idx:expr) => {{
        let length = $seq.instrs().len();
        #[allow(unused_comparisons)]
        let idx = if $idx < 0 {
            length as isize + $idx as isize
        } else {
            $idx as isize
        } as usize;
        $seq.instrs().get(idx).map(|(i, _)| i.clone())
    }};
}

/// readjust is broken but check is not broken I think.
pub fn readjust_debug_call_function(module: &mut walrus::Module) -> eyre::Result<bool> {
    let mut changed = 0;

    let debugger = "debug_call_function_start".get_fid(&module.exports)?;

    let finalize = "debug_call_function_end".get_fid(&module.exports)?;

    let excludes = gen_exclude_set(module).wrap_err("Failed to generate exclude set")?;

    module
        .funcs
        .iter_local_mut()
        .filter(|(func, _)| !excludes.contains(func))
        .try_for_each(|(fid, func)| {
            use walrus::ir::*;

            let fid = fid.index() as i32;

            // check debugger call at entry
            let entry_id = func.entry_block();
            let mut body = func.builder_mut().func_body();
            let mut entry_seq = body.instr_seq(entry_id);

            // eprintln!("fid: {fid:?} entry_id: {entry_id:?}");

            // remove call we don't want
            let len = entry_seq.instrs().len();
            let ret_trap = entry_seq
                .rewrite(|instr, (pos, seq_id)| {
                    if instr.is_return()
                        || instr.is_return_call()
                        || instr.is_return_call_indirect()
                    {
                        Some((pos, seq_id))
                    } else {
                        None
                    }
                })?
                .into_iter()
                .filter_map(|v| v)
                .sorted_by(|(a_pos, a_seq), (b_pos, b_seq)| b_seq.cmp(a_seq).then(b_pos.cmp(a_pos)))
                .collect_vec();

            {
                entry_seq.rewrite(|instr, (pos, seq_id)| {
                    if ret_trap
                        .iter()
                        .filter(|(_, s)| *s == seq_id)
                        .any(|(p, _)| pos == *p - 1)
                    {
                        return;
                    }

                    if let Instr::Call(Call { func }) = instr
                        && ((*func == debugger && seq_id == entry_id && pos != 1)
                            || (*func == finalize && seq_id != entry_id && pos != len - 1))
                    {
                        *instr = Instr::Drop(Drop {});
                        changed += 1;
                        // eprintln!("### pos: {pos}, seq: {seq_id:?}, removed unwanted call");
                    }
                })?;
            }

            let adjust_common =
                |seq: &mut walrus::InstrSeqBuilder<'_>,
                 pos: usize,
                 caller|
                 -> eyre::Result<(Option<i32>, Option<walrus::FunctionId>)> {
                    let before = match get_instr!(seq, pos) {
                        Some(Instr::Const(Const {
                            value: Value::I32(value),
                        })) if value == fid => Some(value),
                        _ => None,
                    };
                    let after = match get_instr!(seq, pos + 1) {
                        Some(Instr::Call(Call { func })) if func == caller => Some(func),
                        _ => None,
                    };

                    Ok((before, after))
                };

            let mut adjust_front = |seq_id: InstrSeqId, pos: usize, caller| {
                let mut seq = body.instr_seq(seq_id);
                let (before, after) = adjust_common(&mut seq, pos, caller)?;
                // eprintln!(
                //     "#### pos: {pos}, seq_id: {seq_id:?}, before: {before:?}, after: {after:?}"
                // );
                match (before, after) {
                    (Some(_), Some(_)) => {}
                    (None, None) => {
                        // eprintln!("pos: {pos}, seq: {seq:?}, added both");
                        seq.const_at(pos + 2, Value::I32(fid));
                        seq.call_at(pos + 3, caller);
                        changed += 1;
                    }
                    (None, Some(_)) => {
                        // eprintln!("pos: {pos}, seq: {seq:?}, added before");
                        seq.instrs_mut().remove(pos);
                        seq.const_at(pos, Value::I32(fid));
                        changed += 1;
                    }
                    (Some(_), None) => {
                        // eprintln!("pos: {pos}, seq: {seq:?}, added after");
                        seq.instrs_mut().remove(pos + 1);
                        seq.call_at(pos + 1, caller);
                        changed += 1;
                    }
                }
                eyre::Ok(())
            };

            adjust_front(entry_id, len - 2, finalize)?;
            for (pos, seq) in ret_trap.into_iter() {
                adjust_front(seq, pos - 2, finalize)?;
            }

            let mut adjust_first = |seq_id: InstrSeqId, pos: usize, caller| {
                let mut seq = body.instr_seq(seq_id);
                let (before, after) = adjust_common(&mut seq, pos, caller)?;
                // eprintln!(
                //     "#### pos: {pos}, seq_id: {seq_id:?}, before: {before:?}, after: {after:?}"
                // );
                match (before, after) {
                    (Some(_), Some(_)) => {}
                    (None, None) => {
                        // eprintln!("pos: {pos}, seq: {seq:?}, added both");
                        seq.const_at(pos, Value::I32(fid));
                        seq.call_at(pos + 1, caller);
                        changed += 1;
                    }
                    (None, Some(_)) => {
                        // eprintln!("pos: {pos}, seq: {seq:?}, added before");
                        seq.instrs_mut().remove(pos);
                        seq.const_at(pos, Value::I32(fid));
                        changed += 1;
                    }
                    (Some(_), None) => {
                        // eprintln!("pos: {pos}, seq: {seq:?}, added after");
                        seq.instrs_mut().remove(pos + 1);
                        seq.call_at(pos + 1, caller);
                        changed += 1;
                    }
                }
                eyre::Ok(())
            };

            adjust_first(entry_id, 0, debugger)?;

            eyre::Ok(())
        })?;

    eprintln!("Readjusted debug_call_function, changes made: {changed}");

    Ok(changed != 0)
}

const EXCLUDE_NAMES: &[&str] = &[
    "debug_call_indirect",
    "debug_atomic_wait",
    "debug_call_function_start",
    "debug_call_function_end",
    "debug_blind_print_etc_flag",
    "debug_loop",
];

fn gen_exclude_set(module: &mut walrus::Module) -> eyre::Result<Vec<walrus::FunctionId>> {
    let start = module.start;

    [
        "debug_call_function_start",
        "debug_call_function_end",
        "debug_loop",
    ]
    .iter()
    .filter_map(|name| get_fid(module, name).transpose())
    .chain(start.iter().copied().map(Ok))
    .collect::<eyre::Result<Vec<_>>>()?
    .into_iter()
    .map(|fid| module.funcs.find_children_with(fid, false))
    .flatten_ok()
    .try_collect::<_, Vec<_>, _>()
}

fn get_fid(module: &mut walrus::Module, name: &str) -> eyre::Result<Option<walrus::FunctionId>> {
    module
        .exports
        .iter()
        .find(|export| export.name == name)
        .map(|export| {
            let fid = match export.item {
                walrus::ExportItem::Function(fid) => fid,
                _ => eyre::bail!("{name} is not a function export"),
            };
            Ok(fid)
        })
        .transpose()
}

pub fn generate_debug_call_function(module: &mut walrus::Module) -> eyre::Result<()> {
    let name = "debug_call_indirect";
    if let Some(e) = get_fid(module, name)?.map(|fid| {
        module
            .debug_call_indirect(fid)
            .wrap_err("Failed to set debug_call_indirect")?;

        log::info!("{name} function found. Enabling debug feature.");

        eyre::Ok(())
    }) {
        e.wrap_err("Failed to enable debug_call_indirect")?;
    }

    let name = "debug_atomic_wait";
    if let Some(e) = get_fid(module, name)?.map(|fid| {
        use walrus::ValType::{I32, I64};

        log::info!("{name} function found. Enabling debug feature.");

        module
            .gen_inspect(fid, &[I32, I32, I64], &[fid], |instr| match instr {
                walrus::ir::Instr::AtomicWait(_) => Some([]),
                _ => None,
            })
            .wrap_err("Failed to set debug_atomic_wait")?;

        eyre::Ok(())
    }) {
        e.wrap_err("Failed to enable debug_atomic_wait")?;
    }

    let name = "debug_loop";
    if let Some(e) = get_fid(module, name)?.map(|fid| {
        log::info!("{name} function found. Enabling debug feature.");

        let mut count = 0;

        module
            .gen_inspect(fid, &[], &[fid], |instr| match instr {
                walrus::ir::Instr::Loop(_) => Some([{
                    count += 1;
                    count
                }]),
                _ => None,
            })
            .wrap_err("Failed to set debug_loop")?;

        eyre::Ok(())
    }) {
        e.wrap_err("Failed to enable debug_loop")?;
    }

    Ok(())
}

pub fn generate_debug_call_function_last(module: &mut walrus::Module) -> eyre::Result<()> {
    use walrus::ir::*;
    use walrus::*;

    let name = "debug_call_function_start";

    if let Some(e) = get_fid(module, name)?.map(|debugger| {
        let excludes = gen_exclude_set(module).wrap_err("Failed to generate exclude set")?;

        let finalize_name = "debug_call_function_end";
        let finalize = get_fid(module, finalize_name)?.unwrap();

        log::info!("{name}, {finalize_name} function found. Enabling debug feature.");

        let import_count = module
            .imports
            .iter()
            .filter(|imp| matches!(imp.kind, walrus::ImportKind::Function(_)))
            .count();

        let calc_future_size = |func: &LocalFunction, id: FunctionId| -> eyre::Result<u64> {
            if excludes.contains(&id) {
                Ok(func.size())
            } else {
                let mut size = func.size();
                size += 2; // start instrs
                size += 2; // end instrs
                size += func
                    .read(|instr, _| {
                        if instr.is_return()
                            || instr.is_return_call()
                            || instr.is_return_call_indirect()
                        {
                            2u64
                        } else {
                            0
                        }
                    })
                    .wrap_err("Failed to read return instructions")?
                    .into_iter()
                    .sum::<u64>();
                Ok(size)
            }
        };

        // walrus sort on size
        let fids_with_size: HashMap<FunctionId, usize> = module
            .funcs
            .iter_local()
            .map(|(fid, func)| {
                calc_future_size(func, fid)
                    .wrap_err("Failed to calc size")
                    .map(|size| (fid, size as usize))
            })
            .collect::<eyre::Result<Vec<_>>>()?
            .into_iter()
            .sorted_by_key(|(id, size)| (cmp::Reverse(*size), *id))
            .map(|(fid, _)| fid)
            .enumerate()
            .map(|(i, fid)| (fid, i + import_count))
            .collect();

        module
            .funcs
            .iter_local_mut()
            .filter(|(func, _)| !excludes.contains(func))
            .try_for_each(|(fid, func)| {
                let fid = fids_with_size
                    .get(&fid)
                    .copied()
                    .wrap_err("Failed to get function order id")? as i32;
                let entry_id = func.entry_block();
                let mut body = func.builder_mut().func_body();
                let mut entry_seq = body.instr_seq(entry_id);
                {
                    entry_seq.const_at(0, Value::I32(fid));
                    entry_seq.call_at(1, debugger);
                }

                {
                    // last instruction must be `Return`
                    entry_seq.i32_const(fid).call(finalize);

                    let pos = body
                        .rewrite(|instr, pos| {
                            if instr.is_return()
                                || instr.is_return_call()
                                || instr.is_return_call_indirect()
                            {
                                Some(pos)
                            } else {
                                None
                            }
                        })
                        .map(|v| {
                            v.into_iter()
                                .filter_map(|v| v)
                                .sorted_by(|(a_pos, a_seq), (b_pos, b_seq)| {
                                    b_seq.cmp(a_seq).then(b_pos.cmp(a_pos))
                                })
                                .collect_vec()
                        })
                        .wrap_err("Failed to find return instructions")?;

                    pos.into_iter().for_each(|(pos, seq)| {
                        let mut seq = body.instr_seq(seq);
                        seq.const_at(pos, Value::I32(fid))
                            .call_at(pos + 1, finalize);
                    });
                }

                eyre::Ok(())
            })?;

        eyre::Ok(())
    }) {
        e.wrap_err("Failed to enable debug_call_function")?;
    }

    Ok(())
}

pub fn has_debug(module: &walrus::Module) -> bool {
    module
        .exports
        .iter()
        .any(|export| EXCLUDE_NAMES.contains(&export.name.as_str()))
}

#[derive(Debug, Default)]
pub struct DebugCallMemoryGrow {
    has: Option<bool>,
}

impl Generator for DebugCallMemoryGrow {
    fn pre_vfs(
        &mut self,
        module: &mut walrus::Module,
        ctx: &crate::generator::GeneratorCtx,
    ) -> eyre::Result<()> {
        self.has = Some(false);

        if !ctx.threads || !ctx.unstable_print_debug {
            return Ok(());
        }

        if let Some((id, id2)) = "debug_call_memory_grow"
            .get_fid(&module.exports)
            .ok()
            .and_then(|id| {
                "debug_call_memory_grow_pre"
                    .get_fid(&module.exports)
                    .ok()
                    .map(|id2| (id, id2))
            })
        {
            self.has = Some(true);

            module
                .gen_inspect_with_finalize(
                    Some(id),
                    Some(id2),
                    &[walrus::ValType::I32],
                    &[walrus::ValType::I32],
                    &module.funcs.find_children_with(id, false).unwrap(),
                    |instr| {
                        if let walrus::ir::Instr::MemoryGrow(walrus::ir::MemoryGrow {
                            memory: _,
                            ..
                        }) = instr
                        {
                            static mut Z: i32 = 17;
                            unsafe { Z += 1 };
                            Some([0, unsafe { Z }])
                        } else {
                            None
                        }
                    },
                )
                .unwrap();
        }

        Ok(())
    }

    fn pre_target(
        &mut self,
        module: &mut walrus::Module,
        _: &crate::generator::GeneratorCtx,
        _: &ModuleExternal,
    ) -> eyre::Result<()> {
        let has = self.has.wrap_err("DebugCallMemoryGrow not initialized")?;
        if !has {
            return Ok(());
        }

        let func_ty = module.types.add(
            &[
                walrus::ValType::I32,
                walrus::ValType::I32,
                walrus::ValType::I32,
            ],
            &[],
        );
        let (id, _) =
            module.add_import_func("wasip1-vfs_debug", "debug_call_memory_grow_import", func_ty);
        let (id2, _) = module.add_import_func(
            "wasip1-vfs_debug",
            "debug_call_memory_grow_pre_import",
            func_ty,
        );

        module
            .gen_inspect_with_finalize(
                Some(id),
                Some(id2),
                &[walrus::ValType::I32],
                &[walrus::ValType::I32],
                &module.funcs.find_children_with(id, false).unwrap(),
                |instr| {
                    if let walrus::ir::Instr::MemoryGrow(walrus::ir::MemoryGrow {
                        memory: _, ..
                    }) = instr
                    {
                        static mut I: i32 = 117;
                        unsafe { I += 1 };
                        println!("Rewriting memory.grow to call debug function");
                        Some([1, unsafe { I }])
                    } else {
                        None
                    }
                },
            )
            .wrap_err("Failed to set debug_call_memory_grow")?;

        Ok(())
    }

    fn post_combine(
        &mut self,
        module: &mut walrus::Module,
        ctx: &crate::generator::GeneratorCtx,
    ) -> eyre::Result<()> {
        if !ctx.unstable_print_debug {
            return Ok(());
        }

        module
            .renew_call_fn(
                ("wasip1-vfs_debug", "debug_call_memory_grow_import"),
                "debug_call_memory_grow",
            )
            .ok();

        module
            .renew_call_fn(
                ("wasip1-vfs_debug", "debug_call_memory_grow_pre_import"),
                "debug_call_memory_grow_pre",
            )
            .ok();

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct DebugExportVFSFunctions;

impl Generator for DebugExportVFSFunctions {
    fn pre_vfs(
        &mut self,
        module: &mut walrus::Module,
        ctx: &crate::generator::GeneratorCtx,
    ) -> eyre::Result<()> {
        if !ctx.unstable_print_debug {
            return Ok(());
        }

        for wasm_name in &ctx.target_names {
            module
                .exports
                .iter()
                .filter(|export| {
                    export
                        .name
                        .starts_with(&format!("__wasip1_vfs_{wasm_name}_"))
                })
                .filter(|export| {
                    <Wasip1ABIFunc as strum::VariantNames>::VARIANTS.contains(
                        &export
                            .name
                            .as_str()
                            .trim_start_matches(&format!("__wasip1_vfs_{wasm_name}_")),
                    )
                })
                .filter_map(|export| match export.item {
                    walrus::ExportItem::Function(fid) => Some((export.name.clone(), fid)),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .into_iter()
                .for_each(|(name, old_fid)| {
                    module.exports.add(&format!("debug_{name}"), old_fid);
                });
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct DebugBase;

impl Generator for DebugBase {
    fn post_combine(
        &mut self,
        module: &mut walrus::Module,
        ctx: &crate::generator::GeneratorCtx,
    ) -> eyre::Result<()> {
        if !ctx.unstable_print_debug {
            return Ok(());
        }

        if let Some(id) = "debug_wasip1_vfs_pre_init".get_fid(&module.exports).ok() {
            let start = module.funcs.get_mut(module.start.unwrap());
            start
                .kind
                .unwrap_local_mut()
                .builder_mut()
                .func_body()
                .call(id);
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct DebugCallFunctionSmallScale;

impl Generator for DebugCallFunctionSmallScale {
    fn post_all_optimize(
        &mut self,
        module: &mut walrus::Module,
        ctx: &super::GeneratorCtx,
    ) -> eyre::Result<bool> {
        if !ctx.unstable_print_debug {
            return Ok(false);
        }

        generate_debug_call_function(module).wrap_err("Failed to generate debug_call_function")?;

        Ok(true)
    }
}

#[derive(Debug, Default)]
pub struct DebugCallFunctionMain;

impl Generator for DebugCallFunctionMain {
    fn post_all_optimize(
        &mut self,
        module: &mut walrus::Module,
        ctx: &super::GeneratorCtx,
    ) -> eyre::Result<bool> {
        if !ctx.unstable_print_debug {
            return Ok(false);
        }

        generate_debug_call_function_last(module)
            .wrap_err("Failed to generate debug_call_function_last")?;

        Ok(true)
    }
}
