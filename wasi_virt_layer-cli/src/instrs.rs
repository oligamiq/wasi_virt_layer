use eyre::Context as _;
use walrus::ir::*;
use walrus::*;

/// Trait for rewriting or modifying instructions sequentially in a local function.
pub trait InstrRewrite {
    // todo!(); Change it to match "read"
    /// Rewrites instructions using a callback that returns `T`, tracking sequence IDs.
    fn rewrite<T>(
        &mut self,
        find: impl FnMut(&mut Instr, (usize, InstrSeqId)) -> T,
    ) -> eyre::Result<Vec<T>>;

    /// Retains instructions that match the specified callback condition.
    fn retain(&mut self, keep: impl FnMut(&Instr, (usize, InstrSeqId)) -> bool);
}

/// Trait for reading instructions sequentially in a local function without modifying them.
pub trait InstrRead {
    /// Reads and iterates over instructions using a callback, tracking sequence IDs.
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

fn rewrite_inner<'a, T>(
    builder: &mut InstrSeqBuilder<'a>,
    find: &mut impl FnMut(&mut Instr, (usize, InstrSeqId)) -> T,
) -> eyre::Result<Vec<T>> {
    let mut visited_instrs = std::collections::HashSet::new();
    let mut work_stack = vec![builder.id()];
    let mut ret = vec![];

    while let Some(seq_id) = work_stack.pop() {
        if visited_instrs.contains(&seq_id) {
            continue;
        }
        visited_instrs.insert(seq_id);

        let mut next_instrs = vec![];
        let mut builder_mut = builder.instr_seq(seq_id);
        let id = builder_mut.id();

        for (i, (instr, _)) in builder_mut.instrs_mut().iter_mut().enumerate() {
            ret.push(find(instr, (i, id)));
            gen_next_instrs(instr, &mut next_instrs);
        }

        // Push child sequences in reverse order to maintain processing order
        for instr_seq_id in next_instrs.into_iter().rev() {
            if !visited_instrs.contains(&instr_seq_id) {
                work_stack.push(instr_seq_id);
            }
        }
    }

    Ok(ret)
}

fn retain_inner<'a>(
    builder: &mut InstrSeqBuilder<'a>,
    keep: &mut impl FnMut(&Instr, (usize, InstrSeqId)) -> bool,
) -> eyre::Result<()> {
    let mut visited_instrs = std::collections::HashSet::new();
    let mut work_stack = vec![builder.id()];

    while let Some(seq_id) = work_stack.pop() {
        if visited_instrs.contains(&seq_id) {
            continue;
        }
        visited_instrs.insert(seq_id);

        let mut next_instrs = vec![];
        let mut instr_seq = builder.instr_seq(seq_id);
        let id = instr_seq.id();

        instr_seq.instrs_mut().retain(|(instr, _)| {
            let should_keep = keep(instr, (0, id));
            gen_next_instrs(instr, &mut next_instrs);
            should_keep
        });

        // Push child sequences in reverse order to maintain processing order
        for instr_seq_id in next_instrs.into_iter().rev() {
            if !visited_instrs.contains(&instr_seq_id) {
                work_stack.push(instr_seq_id);
            }
        }
    }

    Ok(())
}

impl<'a> InstrRewrite for InstrSeqBuilder<'a> {
    fn rewrite<T>(
        &mut self,
        mut find: impl FnMut(&mut Instr, (usize, InstrSeqId)) -> T,
    ) -> eyre::Result<Vec<T>> {
        rewrite_inner(self, &mut find).wrap_err_with(|| eyre::eyre!("Failed to rewrite instrs"))
    }

    fn retain(&mut self, mut keep: impl FnMut(&Instr, (usize, InstrSeqId)) -> bool) {
        retain_inner(self, &mut keep)
            .wrap_err_with(|| eyre::eyre!("Failed to retain instrs"))
            .unwrap();
    }
}
