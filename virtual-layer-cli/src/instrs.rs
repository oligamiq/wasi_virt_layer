use eyre::Context as _;
use walrus::ir::*;
use walrus::*;

pub trait InstrRewrite {
    fn rewrite<T>(
        &mut self,
        find: impl FnMut(&mut Instr, (usize, InstrSeqId)) -> T,
    ) -> eyre::Result<Vec<T>>;
}

impl<'a> InstrRewrite for InstrSeqBuilder<'a> {
    fn rewrite<T>(
        &mut self,
        mut find: impl FnMut(&mut Instr, (usize, InstrSeqId)) -> T,
    ) -> eyre::Result<Vec<T>> {
        let mut visited_instrs = vec![];

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

        rewrite_inner(self, &mut find, &mut visited_instrs)
            .wrap_err_with(|| eyre::eyre!("Failed to rewrite instrs"))
    }
}
