use std::fs;

use camino::Utf8PathBuf;
use eyre::Context;
use walrus::ir::InstrSeqId;

use crate::{instrs::InstrRead, util::ResultUtil};

pub fn remove_unused_threads_function(wasm: &mut walrus::Module) -> eyre::Result<()> {
    // if wasm doesn't have wasi.thread-spawn on import,
    // wasm's export `wasi_thread_start` should be removed

    if !wasm
        .imports
        .iter()
        .any(|i| i.module == "wasi" && i.name == "thread-spawn")
    {
        if wasm.exports.iter().any(|e| e.name == "wasi_thread_start") {
            wasm.exports
                .remove("wasi_thread_start")
                .to_eyre()
                .wrap_err("Failed to remove wasi_thread_start export")?;
        } else {
            log::warn!(
                "wasi.thread-spawn is not imported, and wasi_thread_start is not exported. This expect multi-threaded Wasm. Is this non rust-lang wasm?"
            );
        }

        // todo!() check memory id used on thread-spawn function
        if let Some((mem, id)) = {
            wasm.imports
                .iter()
                .filter_map(|e| match e.kind {
                    walrus::ImportKind::Memory(mem) => Some((mem, e.id())),
                    _ => None,
                })
                .find(|_| true)
        } {
            wasm.imports.delete(id);
            wasm.memories
                .iter_mut()
                .find(|m| m.id() == mem)
                .unwrap()
                .import = None;
        } else {
            log::warn!(
                "wasi.thread-spawn is not imported, and shared memory is not exported. This expect multi-threaded Wasm. Is this non rust-lang wasm?"
            );
        }
    }

    Ok(())
}

pub fn adjust_core_wasm(
    path: &Utf8PathBuf,
    threads: bool,
    debug: bool,
) -> eyre::Result<(Utf8PathBuf, Option<Vec<(u64, u64)>>, bool)> {
    let mut module = walrus::Module::from_file(path)
        .to_eyre()
        .wrap_err("Failed to load module")?;

    let mem_size = if threads {
        module.memories.iter_mut().for_each(|mem| {
            mem.shared = true;
        });

        let mem_size = {
            module
                .memories
                .iter_mut()
                .enumerate()
                .map(|(count, mem)| {
                    let id = module.imports.add(
                        "env",
                        &mem.name.clone().unwrap_or_else(|| match count {
                            0 => "memory".to_string(),
                            n => format!("memory{n}"),
                        }),
                        walrus::ImportKind::Memory(mem.id()),
                    );

                    mem.import = Some(id);

                    (mem.initial, mem.maximum.unwrap_or(0))
                })
                .collect::<Vec<_>>()
        };
        Some(mem_size)
    } else {
        None
    };

    let changed = if debug {
        readjust_debug_call_function(&mut module)
            .wrap_err("Failed to readjust debug_call_function")?
    } else {
        false
    };

    let new_path = path.with_extension("adjusted.wasm");

    if fs::metadata(&new_path).is_ok() {
        fs::remove_file(&new_path).wrap_err("Failed to remove existing file")?;
    }

    module
        .emit_wasm_file(new_path.clone())
        .to_eyre()
        .wrap_err("Failed to write temporary wasm file")?;

    Ok((new_path, mem_size, changed))
}

pub fn readjust_debug_call_function(module: &mut walrus::Module) -> eyre::Result<bool> {
    let mut changed = false;

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

    #[allow(non_camel_case_types)]
    enum DebugFunction {
        Start((usize, InstrSeqId)),
        End((usize, InstrSeqId)),
    }

    let positions = module
        .funcs
        .iter_local_mut()
        .map(|(id, f)| {
            Ok(f.read(|instr, pos| {
                if let walrus::ir::Instr::Call(call) = instr {
                    if call.func == debugger {
                        return Some(DebugFunction::Start(pos));
                    }
                    if call.func == finalize {
                        return Some(DebugFunction::End(pos));
                    }
                }
                None
            })?
            .into_iter()
            .filter_map(|pos| pos)
            .map(move |pos| (id, pos)))
        })
        .collect::<eyre::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .rev()
        .collect::<Vec<_>>();

    // let fids = module.funcs.iter().map(|f| f.id()).collect::<Vec<_>>();

    for (outer_fid, pos) in positions {
        let func = match module.funcs.get_mut(outer_fid).kind {
            walrus::FunctionKind::Local(ref mut local_func) => local_func,
            _ => {
                return Err(eyre::eyre!("Function is not local"));
            }
        };
        let mut func_body = func.builder_mut().func_body();
        use walrus::ir::*;
        const MAX_LOOKAHEAD: usize = 15;

        enum LookaheadResult {
            FoundFunction(walrus::FunctionId),
            FoundBlock,
            MustBroken,
        }

        fn take_and_check_instr<'a, T: 'a>(
            iter: impl IntoIterator<Item = &'a (Instr, T)>,
            fids: &[walrus::FunctionId],
        ) -> Option<LookaheadResult> {
            iter.into_iter().take(MAX_LOOKAHEAD).find_map(|(instr, _)| {
                if let Instr::Call(call) = instr {
                    if fids.contains(&call.func) {
                        return Some(LookaheadResult::MustBroken);
                    }
                    Some(LookaheadResult::FoundFunction(call.func))
                    // I think optimizer would inline;
                } else if let Instr::Block(..) = instr {
                    Some(LookaheadResult::FoundBlock)
                } else {
                    None
                }
            })
        }

        fn check_instr(
            result: Option<LookaheadResult>,
            instr: &mut walrus::InstrSeqBuilder,
            fid: walrus::FunctionId,
            outer_fid: walrus::FunctionId,
            name: impl AsRef<str>,
            pos: usize,
            changed: &mut bool,
        ) -> eyre::Result<()> {
            let name = name.as_ref();
            match result {
                Some(LookaheadResult::FoundFunction(id)) => {
                    if let Instr::Const(Const {
                        value: Value::I32(v),
                    }) = instr.instrs()[pos - 1].0
                    {
                        // let id = fids.iter().copied().position(|f| f == id).unwrap() as i32;
                        let id = id.index() as i32;
                        if v != id {
                            instr.instrs_mut().remove(pos - 1);
                            instr.const_at(pos - 1, Value::I32(id));
                            *changed = true;
                        }
                    } else {
                        log::warn!(
                            "Expected I32 before {name}, found {:?} in {outer_fid:?} function",
                            instr.instrs()[pos - 1]
                        );
                        return check_instr(None, instr, fid, outer_fid, name, pos, changed);
                    }
                }
                Some(LookaheadResult::FoundBlock) => {}
                None | Some(LookaheadResult::MustBroken) => {
                    log::warn!(
                        "Could not find a function call after {name} within {MAX_LOOKAHEAD} instructions. Removing {name} call in {fid:?} function.",
                    );

                    let fail_before = if !matches!(
                        instr.instrs()[pos - 1].0,
                        Instr::Const(Const {
                            value: Value::I32(_)
                        })
                    ) {
                        log::warn!(
                            "Expected I32 before {name}, found {:?} in {outer_fid:?} function",
                            instr.instrs()[pos - 1]
                        );

                        true
                    } else {
                        false
                    };

                    let fail_after = if !matches!(
                        instr.instrs()[pos].0,
                        Instr::Call(Call { func: f }) if f == fid
                    ) {
                        log::warn!(
                            "Expected {name} after I32, found {:?} in {outer_fid:?} function",
                            instr.instrs()[pos]
                        );

                        true
                    } else {
                        false
                    };

                    match (fail_before, fail_after) {
                        (_, true) => {}
                        (true, false) => {
                            instr.instrs_mut().remove(pos);
                            instr.drop_at(pos);
                        }
                        (false, false) => {
                            instr.instrs_mut().remove(pos);
                            instr.instrs_mut().remove(pos - 1);
                        }
                    }

                    *changed = true;
                }
            }

            Ok(())
        }

        match pos {
            DebugFunction::Start((pos, seq)) => {
                let mut instr = func_body.instr_seq(seq);

                check_instr(
                    take_and_check_instr(
                        instr.instrs().iter().skip(pos + 1),
                        &[debugger, finalize],
                    ),
                    &mut instr,
                    debugger,
                    outer_fid,
                    "debug_call_function_start",
                    pos,
                    &mut changed,
                )?;
            }
            DebugFunction::End((pos, seq)) => {
                let mut instr = func_body.instr_seq(seq);

                check_instr(
                    take_and_check_instr(
                        instr.instrs().iter().skip(pos - 1).rev(),
                        &[debugger, finalize],
                    ),
                    &mut instr,
                    finalize,
                    outer_fid,
                    "debug_call_function_end",
                    pos,
                    &mut changed,
                )?;
            }
        }
    }

    Ok(changed)
}
