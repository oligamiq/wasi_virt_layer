use std::collections::HashSet;
use walrus::{ir::*, FunctionId, Module, ValType, ConstExpr};
use crate::{
    generator::{Generator, GeneratorCtx, ModuleExternal},
    util::{WalrusUtilModule, WalrusFID, WalrusUtilExport},
};

/// Generator that replaces WebAssembly unreachable instructions and handles unwinding
/// call stacks gracefully by injecting a global flag and modifying call sites.
#[derive(Debug, Default)]
pub struct WrapUnreachableGenerator {
    targets: HashSet<String>,
}

impl Generator for WrapUnreachableGenerator {
    fn pre_vfs(&mut self, module: &mut Module, ctx: &GeneratorCtx) -> eyre::Result<()> {
        for target in ctx.target_names.iter() {
            let export_name = format!("__wasip1_virt_layer_{target}_wrap_unreachable");
            if module.exports.iter().any(|e| e.name == export_name) {
                self.targets.insert(target.to_string());
            }
        }
        Ok(())
    }

    fn post_combine(
        &mut self,
        module: &mut Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        for target in &self.targets {
            let get_name = format!("__wasip1_virt_layer_{target}_get_unreachable_flag");
            let set_name = format!("__wasip1_virt_layer_{target}_set_unreachable_flag");

            if let Ok(import_get) = ("__wasip1_virt_layer", get_name.as_str()).get_fid(&module.imports) {
                if let Ok(export_get) = get_name.as_str().get_fid(&module.exports) {
                    module.renew_call_fn(import_get, export_get)?;
                    module.exports.erase_with(export_get, ctx.unstable_print_debug)?;
                }
            }

            if let Ok(import_set) = ("__wasip1_virt_layer", set_name.as_str()).get_fid(&module.imports) {
                if let Ok(export_set) = set_name.as_str().get_fid(&module.exports) {
                    module.renew_call_fn(import_set, export_set)?;
                    module.exports.erase_with(export_set, ctx.unstable_print_debug)?;
                }
            }
            
            let export_marker = format!("__wasip1_virt_layer_{target}_wrap_unreachable");
            if let Ok(export_id) = export_marker.as_str().get_fid(&module.exports) {
                module.exports.erase_with(export_id, ctx.unstable_print_debug)?;
            }
        }
        Ok(())
    }

    fn pre_target(
        &mut self,
        module: &mut Module,
        _: &GeneratorCtx,
        external: &ModuleExternal,
    ) -> eyre::Result<()> {
        if !self.targets.contains(&external.name.to_string()) {
            return Ok(());
        }

        // Add global `unreachable_flag`
        let flag_global = module.globals.add_local(
            ValType::I32,
            true,
            false,
            ConstExpr::Value(Value::I32(0)),
        );

        // Export getter
        let getter = module.add_func(&[], &[ValType::I32], |builder, _| {
            builder.func_body().global_get(flag_global);
            Ok(())
        })?;
        module.exports.add(&format!("__wasip1_virt_layer_{}_get_unreachable_flag", external.name), getter);

        // Export setter
        let setter = module.add_func(&[ValType::I32], &[], |builder, args| {
            builder.func_body().local_get(args[0]).global_set(flag_global);
            Ok(())
        })?;
        module.exports.add(&format!("__wasip1_virt_layer_{}_set_unreachable_flag", external.name), setter);

        // We will process every function to replace `unreachable` and hook `call`/`call_indirect`
        let func_ids: Vec<FunctionId> = module.funcs.iter_local().map(|(id, _)| id).collect();

        for fid in func_ids {
            // Get the return type of this function so we know what dummy values to push
            let func_type = module.types.get(module.funcs.get(fid).ty());
            let return_types = func_type.results().to_vec();

            let mut instructions_to_replace = Vec::new();
            let mut calls_to_hook = Vec::new();

            {
                let func = module.funcs.get(fid);
                let local_func = func.kind.unwrap_local();
                let entry_block = local_func.entry_block();

                // Helper to recurse instructions and find things
                fn visit_seq(
                    seq_id: InstrSeqId,
                    module: &Module,
                    func: &walrus::LocalFunction,
                    instructions_to_replace: &mut Vec<(InstrSeqId, usize)>,
                    calls_to_hook: &mut Vec<(InstrSeqId, usize)>,
                ) {
                    let seq = func.block(seq_id);
                    for (i, (instr, _)) in seq.instrs.iter().enumerate() {
                        match instr {
                            Instr::Unreachable(_) => {
                                instructions_to_replace.push((seq_id, i));
                            }
                            Instr::Call(_) | Instr::CallIndirect(_) => {
                                calls_to_hook.push((seq_id, i));
                            }
                            Instr::Block(b) => {
                                visit_seq(b.seq, module, func, instructions_to_replace, calls_to_hook);
                            }
                            Instr::Loop(l) => {
                                visit_seq(l.seq, module, func, instructions_to_replace, calls_to_hook);
                            }
                            Instr::IfElse(i_e) => {
                                visit_seq(i_e.consequent, module, func, instructions_to_replace, calls_to_hook);
                                visit_seq(i_e.alternative, module, func, instructions_to_replace, calls_to_hook);
                            }
                            _ => {}
                        }
                    }
                }

                visit_seq(entry_block, module, local_func, &mut instructions_to_replace, &mut calls_to_hook);
            }

            // We must apply replacements starting from the end of each block so indices don't shift.
            // Or we can rebuild the instructions.
            // walrus `builder.instr_seq(seq_id)` allows us to rewrite? No, it just gives a builder.
            // Actually, we can use `module.funcs.get_mut(fid).kind.unwrap_local_mut()` and modify `seq.instrs` directly if we are careful,
            // but walrus has a `dfs_in_order_mut` or we can just rebuild the instruction sequences.
            // A safer way is to just replace instructions by replacing them in the `instrs` vector in reverse order.
            
            // Sort by seq_id and then reverse index
            instructions_to_replace.sort_by(|a, b| b.cmp(a));
            calls_to_hook.sort_by(|a, b| b.cmp(a));

            let func_mut = module.funcs.get_mut(fid).kind.unwrap_local_mut();

            // Replace unreachable
            for (seq_id, idx) in instructions_to_replace {
                let seq = func_mut.block_mut(seq_id);
                // Remove unreachable
                let _unreachable_instr = seq.instrs.remove(idx);
                let loc = _unreachable_instr.1;

                // Insert dummy values and return
                let mut new_instrs = Vec::new();
                new_instrs.push((Instr::Const(Const { value: Value::I32(1) }), loc));
                new_instrs.push((Instr::GlobalSet(GlobalSet { global: flag_global }), loc));

                for ty in &return_types {
                    let val = match ty {
                        ValType::I32 => Value::I32(0),
                        ValType::I64 => Value::I64(0),
                        ValType::F32 => Value::F32(0.0),
                        ValType::F64 => Value::F64(0.0),
                        ValType::V128 => Value::V128(0),
                        ValType::Ref(_) => unimplemented!("Ref types not supported for dummy returns"),
                    };
                    new_instrs.push((Instr::Const(Const { value: val }), loc));
                }
                new_instrs.push((Instr::Return(Return {}), loc));

                for (j, instr) in new_instrs.into_iter().enumerate() {
                    seq.instrs.insert(idx + j, instr);
                }
            }

            // Hook calls
            for (seq_id, idx) in calls_to_hook {
                // insert after the call
                let insert_idx = idx + 1;
                let seq = func_mut.block_mut(seq_id);
                // Try to get loc of the call
                let loc = seq.instrs[idx].1;

                // We need to create an `if` block.
                let consequent_seq = func_mut.builder_mut().dangling_instr_seq(None).id(); // Create a new seq
                // Fill the consequent block
                let consequent = func_mut.block_mut(consequent_seq);
                for ty in &return_types {
                    let val = match ty {
                        ValType::I32 => Value::I32(0),
                        ValType::I64 => Value::I64(0),
                        ValType::F32 => Value::F32(0.0),
                        ValType::F64 => Value::F64(0.0),
                        ValType::V128 => Value::V128(0),
                        ValType::Ref(_) => unimplemented!("Ref types not supported for dummy returns"),
                    };
                    consequent.instrs.push((Instr::Const(Const { value: val }), loc));
                }
                consequent.instrs.push((Instr::Return(Return {}), loc));

                let alternative_seq = func_mut.builder_mut().dangling_instr_seq(None).id(); // empty block

                let seq = func_mut.block_mut(seq_id);
                let mut new_instrs = Vec::new();
                new_instrs.push((Instr::GlobalGet(GlobalGet { global: flag_global }), loc));
                new_instrs.push((Instr::IfElse(IfElse {
                    consequent: consequent_seq,
                    alternative: alternative_seq,
                }), loc));

                for (j, instr) in new_instrs.into_iter().enumerate() {
                    seq.instrs.insert(insert_idx + j, instr);
                }
            }
        }

        Ok(())
    }
}
