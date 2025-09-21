use eyre::Context as _;
use walrus::ir::*;
use walrus::*;

pub trait InstrRewrite {
    // todo!(); Change it to match "read"
    fn rewrite<T>(
        &mut self,
        find: impl FnMut(&mut Instr, (usize, InstrSeqId)) -> T,
    ) -> eyre::Result<Vec<T>>;

    fn retain(&mut self, keep: impl FnMut(&Instr, (usize, InstrSeqId)) -> bool);
}

pub trait InstrRead {
    fn read<T>(&self, find: impl FnMut(&Instr, (usize, InstrSeqId)) -> T) -> eyre::Result<Vec<T>>;
}

impl InstrRead for LocalFunction {
    fn read<T>(
        &self,
        mut find: impl FnMut(&Instr, (usize, InstrSeqId)) -> T,
    ) -> eyre::Result<Vec<T>> {
        fn add<'a>(
            func: &'a LocalFunction,
            next_instrs: &mut std::collections::VecDeque<(InstrSeqId, &'a Instr, usize)>,
            id: InstrSeqId,
        ) -> eyre::Result<()> {
            for (i, (blk_id, _)) in func.block(id).instrs.iter().enumerate() {
                next_instrs.push_back((id, blk_id, i));
            }
            Ok(())
        }

        let mut visited_instrs = vec![];
        let mut next_instrs = std::collections::VecDeque::new();
        let mut ret = vec![];

        add(self, &mut next_instrs, self.entry_block())?;

        while let Some((blk_id, instr, instr_idx)) = next_instrs.pop_front() {
            if visited_instrs.contains(&(blk_id, instr_idx)) {
                continue;
            } else {
                visited_instrs.push((blk_id, instr_idx));
            }

            ret.push(find(instr, (instr_idx, blk_id)));

            match instr {
                Instr::Block(block) => {
                    add(self, &mut next_instrs, block.seq)?;
                }
                Instr::Loop(r#loop) => {
                    add(self, &mut next_instrs, r#loop.seq)?;
                }
                Instr::Br(br) => {
                    add(self, &mut next_instrs, br.block)?;
                }
                Instr::BrIf(br_if) => {
                    add(self, &mut next_instrs, br_if.block)?;
                }
                Instr::IfElse(if_else) => {
                    add(self, &mut next_instrs, if_else.consequent)?;
                    add(self, &mut next_instrs, if_else.alternative)?;
                }
                Instr::BrTable(br_table) => {
                    add(self, &mut next_instrs, br_table.default)?;
                    for block in &br_table.blocks {
                        add(self, &mut next_instrs, *block)?;
                    }
                }
                _ => {}
            }
        }

        Ok(ret)
    }
}

fn gen_next_instrs(instr: &Instr, next_instrs: &mut Vec<InstrSeqId>) {
    match instr {
        Instr::Block(block) => {
            next_instrs.push(block.seq);
        }
        Instr::Loop(r#loop) => {
            next_instrs.push(r#loop.seq);
        }
        Instr::Br(br) => {
            next_instrs.push(br.block);
        }
        Instr::BrIf(br_if) => {
            next_instrs.push(br_if.block);
        }
        Instr::IfElse(if_else) => {
            next_instrs.push(if_else.consequent);
            next_instrs.push(if_else.alternative);
        }
        Instr::BrTable(br_table) => {
            next_instrs.push(br_table.default);
            next_instrs.extend_from_slice(&br_table.blocks);
        }
        _ => {}
    }
}

#[inline(never)]
fn rewrite_inner<'a, T>(
    builder: &mut InstrSeqBuilder<'a>,
    find: &mut impl FnMut(&mut Instr, (usize, InstrSeqId)) -> T,
    visited_instrs: &mut Vec<InstrSeqId>,
) -> eyre::Result<Vec<T>> {
    let mut next_instrs = vec![];
    let mut ret = vec![];

    let id = builder.id();

    for (i, (instr, _)) in builder.instrs_mut().iter_mut().enumerate() {
        ret.push(find(instr, (i, id)));

        gen_next_instrs(instr, &mut next_instrs);
    }

    for instr_seq_id in next_instrs.into_iter() {
        if visited_instrs.contains(&instr_seq_id) {
            continue;
        }
        visited_instrs.push(instr_seq_id);

        let mut instr_seq = builder.instr_seq(instr_seq_id);
        ret.extend(rewrite_inner(&mut instr_seq, find, visited_instrs)?);
    }
    Ok(ret)
}

#[inline(never)]
fn retain_inner<'a>(
    builder: &mut InstrSeqBuilder<'a>,
    keep: &mut impl FnMut(&Instr, (usize, InstrSeqId)) -> bool,
    visited_instrs: &mut Vec<InstrSeqId>,
) -> eyre::Result<()> {
    let mut next_instrs = vec![];

    let id = builder.id();

    builder.instrs_mut().retain(|(instr, _)| {
        let idx = keep(instr, (0, id));

        gen_next_instrs(instr, &mut next_instrs);

        idx
    });

    for instr_seq_id in next_instrs.into_iter() {
        if visited_instrs.contains(&instr_seq_id) {
            continue;
        }
        visited_instrs.push(instr_seq_id);

        let mut instr_seq = builder.instr_seq(instr_seq_id);
        retain_inner(&mut instr_seq, keep, visited_instrs)?;
    }
    Ok(())
}

impl<'a> InstrRewrite for InstrSeqBuilder<'a> {
    fn rewrite<T>(
        &mut self,
        mut find: impl FnMut(&mut Instr, (usize, InstrSeqId)) -> T,
    ) -> eyre::Result<Vec<T>> {
        let mut visited_instrs = vec![];

        rewrite_inner(self, &mut find, &mut visited_instrs)
            .wrap_err_with(|| eyre::eyre!("Failed to rewrite instrs"))
    }

    fn retain(&mut self, mut keep: impl FnMut(&Instr, (usize, InstrSeqId)) -> bool) {
        let mut visited_instrs = vec![];

        retain_inner(self, &mut keep, &mut visited_instrs)
            .wrap_err_with(|| eyre::eyre!("Failed to retain instrs"))
            .unwrap();
    }
}
