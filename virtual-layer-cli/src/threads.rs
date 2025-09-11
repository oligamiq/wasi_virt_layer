use std::{borrow::Borrow, fs};

use camino::Utf8PathBuf;
use eyre::Context;
use itertools::Itertools;

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

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[allow(non_camel_case_types)]
    enum DebugFunction {
        Start((usize, InstrSeqId)),
        End((usize, InstrSeqId)),
    }

    impl DebugFunction {
        fn position(&self) -> (usize, InstrSeqId) {
            match self {
                DebugFunction::Start(pos) => *pos,
                DebugFunction::End(pos) => *pos,
            }
        }

        fn seq(&self) -> InstrSeqId {
            match self {
                DebugFunction::Start((_, seq)) => *seq,
                DebugFunction::End((_, seq)) => *seq,
            }
        }

        fn cmp_by_seq(&self, other: &Self) -> std::cmp::Ordering {
            let (self_pos, self_seq) = self.position();
            let (other_pos, other_seq) = other.position();

            match self_seq.cmp(&other_seq) {
                std::cmp::Ordering::Equal => self_pos.cmp(&other_pos),
                ord => ord,
            }
        }
    }
    use walrus::ir::*;
    use walrus::*;
    const MAX_LOOKAHEAD: usize = 15;

    enum LookaheadResult {
        FoundFunction(walrus::FunctionId),
        FoundBlock,
        MetStart,
        MetEnd,
    }

    impl LookaheadResult {
        fn id(&self) -> Option<walrus::FunctionId> {
            match self {
                LookaheadResult::FoundFunction(id) => Some(*id),
                _ => None,
            }
        }
    }

    fn take_and_check_instr<'a, T: 'a>(
        iter: impl IntoIterator<Item = &'a (Instr, T)>,
        start_fid: impl Borrow<walrus::FunctionId>,
        end_fid: impl Borrow<walrus::FunctionId>,
    ) -> Option<LookaheadResult> {
        let start_fid: FunctionId = *start_fid.borrow();
        let end_fid: FunctionId = *end_fid.borrow();

        iter.into_iter().take(MAX_LOOKAHEAD).find_map(|(instr, _)| {
            if let Instr::Call(call) = instr {
                if call.func == start_fid {
                    return Some(LookaheadResult::MetStart);
                }
                if call.func == end_fid {
                    return Some(LookaheadResult::MetEnd);
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

    #[derive(Debug, Clone, Copy)]
    struct DebugFunctionSet {
        outer_fid: walrus::FunctionId,
        seq: InstrSeqId,
        start_position: Option<usize>,
        end_position: Option<usize>,
        debugging_function_id: Option<walrus::FunctionId>,
    }

    impl DebugFunctionSet {
        fn cmp_by_fid_seq_pos(&self, other: &Self) -> std::cmp::Ordering {
            match self.outer_fid.cmp(&other.outer_fid) {
                std::cmp::Ordering::Equal => match self.seq.cmp(&other.seq) {
                    std::cmp::Ordering::Equal => self.cmp_pos().cmp(&other.cmp_pos()),
                    ord => ord,
                },
                ord => ord,
            }
        }

        fn cmp_pos(&self) -> Option<usize> {
            if let (Some(self_pos), Some(end_pos)) = (self.start_position, self.end_position) {
                if self_pos < end_pos {
                    return Some(self_pos);
                } else {
                    panic!("DebugFunctionSet start_position is not less than other start_position");
                }
            }
            self.start_position.or(self.end_position)
        }
    }

    let positions = module
        .funcs
        .iter_local_mut()
        .map(|(outer_fid, f)| {
            let calls = f
                .read(|instr, pos| {
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
                .sorted_by(|a, b| a.cmp_by_seq(b))
                .collect::<Vec<_>>();

            if calls.is_empty() {
                return Ok(Vec::<DebugFunctionSet>::new());
            }

            let set = calls
                .iter()
                .copied()
                .zip(calls.iter().copied().skip(1))
                .filter(|(call, _)| matches!(call, DebugFunction::Start(_)))
                .map(|(call, may_finalize)| {
                    if call.seq() == may_finalize.seq() {
                        (call, Some(may_finalize))
                    } else {
                        (call, None)
                    }
                })
                .flat_map(|(call, may_finalize)| {
                    let (pos, seq) = call.position();
                    let instr = f.block(seq);
                    let debugging_function_id =
                        take_and_check_instr(instr.iter().skip(pos + 1), debugger, finalize)
                            .and_then(|t| t.id());
                    let debugging_function_id_alt = may_finalize.and_then(|f| {
                        let pos = f.position().0;
                        take_and_check_instr(instr.iter().skip(pos - 1).rev(), debugger, finalize)
                            .and_then(|t| t.id())
                    });
                    let end_position = may_finalize.map(|f| f.position().0);
                    if debugging_function_id == debugging_function_id_alt {
                        vec![DebugFunctionSet {
                            outer_fid,
                            seq,
                            start_position: Some(pos),
                            end_position,
                            debugging_function_id,
                        }]
                    } else {
                        vec![
                            DebugFunctionSet {
                                outer_fid,
                                seq,
                                start_position: Some(pos),
                                end_position: None,
                                debugging_function_id,
                            },
                            DebugFunctionSet {
                                outer_fid,
                                seq,
                                start_position: None,
                                end_position,
                                debugging_function_id: debugging_function_id_alt,
                            },
                        ]
                    }
                })
                .chain(
                    calls
                        .iter()
                        .copied()
                        .skip(1)
                        .zip(calls.iter().copied())
                        .filter(|(call, _)| matches!(call, DebugFunction::End(_)))
                        .filter(|(_, may_finalize)| matches!(may_finalize, DebugFunction::End(_)))
                        .map(|(call, _)| {
                            let (pos, seq) = call.position();
                            let instr = f.block(seq);
                            let debugging_function_id = take_and_check_instr(
                                instr.iter().skip(pos - 1).rev(),
                                debugger,
                                finalize,
                            )
                            .and_then(|t| t.id());

                            DebugFunctionSet {
                                outer_fid,
                                seq,
                                start_position: None,
                                end_position: Some(pos),
                                debugging_function_id,
                            }
                        }),
                )
                .collect::<Vec<_>>();

            Ok(set)
        })
        .collect::<eyre::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .sorted_by(|a, b| a.cmp_by_fid_seq_pos(b))
        .rev()
        .collect::<Vec<_>>();

    #[derive(Debug, Clone, Copy)]
    struct PositionWithParams<'a> {
        call_pos: usize,
        before_value: Option<i32>,
        fids: &'a [FunctionId],
    }

    impl<'a> PositionWithParams<'a> {
        const fn new(call_pos: usize, before_value: Option<i32>, fids: &'a [FunctionId]) -> Self {
            Self {
                call_pos,
                before_value,
                fids,
            }
        }

        fn get_idx(&self, fid: FunctionId) -> i32 {
            self.fids.iter().position(|&f| f == fid).unwrap() as i32
        }

        fn apply(
            &self,
            instrs: &mut InstrSeqBuilder<'_>,
            changed: &mut bool,
            debugging_function_id: Option<FunctionId>,
        ) -> eyre::Result<()> {
            if let Some(debugging_function_id) = debugging_function_id {
                self.apply_by_id(instrs, changed, debugging_function_id)
            } else {
                self.rm(instrs, changed);
                Ok(())
            }
        }

        // non debugging_function_id so broken
        fn rm(&self, instrs: &mut InstrSeqBuilder<'_>, changed: &mut bool) {
            let PositionWithParams { call_pos, .. } = *self;

            instrs.instrs_mut().remove(call_pos);
            instrs.drop_at(call_pos);

            *changed = true;
        }

        fn apply_by_id(
            &self,
            instrs: &mut InstrSeqBuilder<'_>,
            changed: &mut bool,
            debugging_function_id: FunctionId,
        ) -> eyre::Result<()> {
            use walrus::ir::*;

            let PositionWithParams {
                call_pos,
                before_value,
                ..
            } = *self;

            // let debugging_function_id = debugging_function_id.index() as i32;
            let debugging_function_id = self.get_idx(debugging_function_id);

            // adjusting the value
            if let Some(v) = before_value {
                if debugging_function_id == v {
                    // already adjusted
                    return Ok(());
                }
                instrs.instrs_mut().remove(call_pos - 1);
                instrs.const_at(call_pos - 1, Value::I32(debugging_function_id));
            } else {
                instrs.drop_at(call_pos - 1);
                instrs.const_at(call_pos, Value::I32(debugging_function_id));
            }

            *changed = true;

            Ok(())
        }
    }

    let fids = module.funcs.iter().map(|f| f.id()).collect::<Vec<_>>();

    for DebugFunctionSet {
        outer_fid,
        seq,
        start_position,
        end_position,
        debugging_function_id,
    } in positions
    {
        let f = module.funcs.get_mut(outer_fid);
        let mut instr_seq = match f.kind {
            walrus::FunctionKind::Local(ref mut l) => l.builder_mut().instr_seq(seq),
            _ => continue,
        };

        let gen_before = |pos: usize| -> PositionWithParams {
            let instr = match &instr_seq.instrs().get(pos - 1) {
                Some((v, _)) => v,
                None => return PositionWithParams::new(pos, None, &fids),
            };
            if let Instr::Const(Const {
                value: Value::I32(v),
            }) = instr
            {
                PositionWithParams::new(pos, Some(*v), &fids)
            } else {
                PositionWithParams::new(pos, None, &fids)
            }
        };

        let start_position = start_position.map(gen_before);
        let end_position = end_position.map(gen_before);

        start_position
            .map(|p| p.apply(&mut instr_seq, &mut changed, debugging_function_id))
            .transpose()?;
        end_position
            .map(|p| p.apply(&mut instr_seq, &mut changed, debugging_function_id))
            .transpose()?;
    }

    Ok(changed)
}
