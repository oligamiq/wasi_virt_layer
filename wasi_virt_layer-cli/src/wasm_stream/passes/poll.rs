use crate::wasm_stream::pipeline::{par_process_code_section, StreamPass};
use wasm_encoder::{Function, Instruction, Module, RawSection};
use wasmparser::{Parser, Payload};

pub struct PollWaitStreamPass {
    pub threads: bool,
    pub vfs_mem: u32,
}

impl PollWaitStreamPass {
    pub fn new(threads: bool, vfs_mem: u32) -> Self {
        Self { threads, vfs_mem }
    }
}

impl StreamPass for PollWaitStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> eyre::Result<Vec<u8>> {
        let mut target_func_idx = None;

        // 1. Find the import index of `__wvl_poll_atomic_wait`
        for payload in Parser::new(0).parse_all(input_wasm) {
            let payload = payload?;
            if let Payload::ImportSection(s) = payload {
                let mut current_func_idx = 0;
                for group in s {
                    for import_res in group? {
                        let (_, import) = import_res?;
                        if let wasmparser::TypeRef::Func(_) = import.ty {
                            if import.module == "wvl_poll" && import.name == "__wvl_poll_atomic_wait" {
                                target_func_idx = Some(current_func_idx);
                            }
                            current_func_idx += 1;
                        }
                    }
                }
            }
        }

        let Some(target_idx) = target_func_idx else {
            return Ok(input_wasm.to_vec());
        };

        let mut module = Module::new();

        // 2. Rewrite the code section
        for payload in Parser::new(0).parse_all(input_wasm) {
            let payload = payload?;
            match payload {
                Payload::CodeSectionStart { count: _, range, size: _ } => {
                    let reader = wasmparser::BinaryReader::new(&input_wasm[range.start..range.end], range.start);
                    let s = wasmparser::CodeSectionReader::new(reader)?;
                    let code_sec = par_process_code_section(s, |_, func_body| {
                        let mut locals = Vec::new();
                        let mut locals_reader = func_body.get_locals_reader()?;
                        for _ in 0..locals_reader.get_count() {
                            let (count, ty) = locals_reader.read()?;
                            let enc_ty = crate::wasm_stream::translator::translate_val_type(
                                ty,
                                &crate::wasm_stream::translator::DefaultRebinder,
                            );
                            locals.push((count, enc_ty));
                        }

                        let mut func = Function::new(locals);
                        let mut reader = func_body.get_operators_reader()?;
                        while !reader.eof() {
                            let op = reader.read()?;
                            match op {
                                wasmparser::Operator::Call { function_index } if function_index == target_idx => {
                                    if self.threads {
                                        func.instruction(&Instruction::MemoryAtomicWait32(wasm_encoder::MemArg {
                                            align: 2,
                                            offset: 0,
                                            memory_index: self.vfs_mem,
                                        }));
                                    } else {
                                        func.instruction(&Instruction::Drop);
                                        func.instruction(&Instruction::Drop);
                                        func.instruction(&Instruction::Drop);
                                        func.instruction(&Instruction::I32Const(2));
                                    }
                                }
                                _ => {
                                    func.instruction(&crate::wasm_stream::translator::translate(
                                        &op,
                                        &crate::wasm_stream::translator::DefaultRebinder,
                                    ));
                                }
                            }
                        }
                        Ok(func)
                    })?;
                    module.section(&code_sec);
                }
                Payload::CustomSection(c) => {
                    module.section(&wasm_encoder::CustomSection {
                        name: c.name().into(),
                        data: std::borrow::Cow::Borrowed(c.data()),
                    });
                }
                _ => {
                    if let Some((id, range)) = payload.as_section() {
                        if id != 10 { // code section
                            module.section(&RawSection {
                                id,
                                data: &input_wasm[range.clone()],
                            });
                        }
                    }
                }
            }
        }

        Ok(module.finish())
    }
}
