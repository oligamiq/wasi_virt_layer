use eyre::Context as _;
use itertools::Itertools;
use walrus::InstrSeqBuilder;

use crate::{
    instrs::InstrRewrite,
    util::{ResultUtil as _, WalrusUtilFuncs as _, WalrusUtilModule as _},
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

pub fn readjust_debug_call_function(module: &mut walrus::Module) -> eyre::Result<bool> {
    let mut changed = 0;

    let debugger = module
        .exports
        .get_func("debug_call_function_start")
        .to_eyre()
        .wrap_err("Failed to get debug_call_function export")?;

    let finalize = module
        .exports
        .get_func("debug_call_function_end")
        .to_eyre()
        .wrap_err("Failed to get debug_call_function_end export")?;

    let excludes =
        gen_exclude_set(module, EXCLUDE_NAMES).wrap_err("Failed to generate exclude set")?;

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

            eprintln!("fid: {fid:?} entry_id: {entry_id:?}");

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
                        eprintln!("### pos: {pos}, seq: {seq_id:?}, removed unwanted call");
                    }
                })?;
            }

            let adjust_common =
                |seq: &mut InstrSeqBuilder<'_>,
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
    "debug_blind_print_etc_flag",
    "debug_call_function_start",
    "debug_call_function_end",
];

fn gen_exclude_set(
    module: &mut walrus::Module,
    names: &[&str],
) -> eyre::Result<Vec<walrus::FunctionId>> {
    names
        .iter()
        .filter_map(|name| {
            Some(
                get_fid(module, name)
                    .transpose()?
                    .map(|fid| module.funcs.find_children_with(fid))
                    .flatten(),
            )
        })
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

    let name = "debug_call_function_start";
    if let Some(e) = get_fid(module, name)?.map(|debugger| {
        let excludes =
            gen_exclude_set(module, EXCLUDE_NAMES).wrap_err("Failed to generate exclude set")?;

        let finalize_name = "debug_call_function_end";
        let finalize = get_fid(module, finalize_name)?.unwrap();

        log::info!("{name}, {finalize_name} function found. Enabling debug feature.");

        module
            .funcs
            .iter_local_mut()
            .filter(|(func, _)| !excludes.contains(func))
            .try_for_each(|(fid, func)| {
                use walrus::ir::*;

                let fid = fid.index() as i32;
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
