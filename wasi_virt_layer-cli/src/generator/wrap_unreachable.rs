use std::collections::HashSet;
use walrus::{ir::*, FunctionId, Module, ValType, ConstExpr, ExportItem};
use crate::{
    generator::{Generator, GeneratorCtx, ModuleExternal},
    instrs::InstrRewrite as _,
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
            let fix_name = format!("__wasip1_virt_layer_{target}_fix_main_raw_exit_code");
            let handle_name = format!("__wasip1_virt_layer_{target}_handle_thread_exit");

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

            if let Ok(import_fix) = ("__wasip1_virt_layer", fix_name.as_str()).get_fid(&module.imports) {
                if let Ok(export_fix) = fix_name.as_str().get_fid(&module.exports) {
                    module.renew_call_fn(import_fix, export_fix)?;
                    module.exports.erase_with(export_fix, ctx.unstable_print_debug)?;
                }
            }

            if let Ok(import_handle) = ("__wasip1_virt_layer", handle_name.as_str()).get_fid(&module.imports) {
                if let Ok(export_handle) = handle_name.as_str().get_fid(&module.exports) {
                    module.renew_call_fn(import_handle, export_handle)?;
                    module.exports.erase_with(export_handle, ctx.unstable_print_debug)?;
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

        // Import the handler functions from the VFS
        let fix_exit_code_type = module.types.add(&[ValType::I32], &[ValType::I32]);
        let fix_exit_code_import = module.add_import_func("__wasip1_virt_layer", format!("__wasip1_virt_layer_{}_fix_main_raw_exit_code", external.name).as_str(), fix_exit_code_type).0;

        let handle_thread_exit_type = module.types.add(&[ValType::I32], &[]);
        let handle_thread_exit_import = module.add_import_func("__wasip1_virt_layer", format!("__wasip1_virt_layer_{}_handle_thread_exit", external.name).as_str(), handle_thread_exit_type).0;

        // Process every EXISTING function to replace `unreachable` and hook `call`/`call_indirect`
        // IMPORTANT: This must happen BEFORE creating wrapper functions below,
        // so that the wrappers themselves are not processed by the call-hook loop.
        // If the wrappers were processed, the hook after `call(orig_func)` would
        // short-circuit with a dummy return, bypassing the fix_exit_code logic.
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

        // Wrap __main_void — created AFTER the call-hook loop above so the wrapper
        // itself is not processed (its internal call to orig_func must NOT be hooked).
        let main_void_export = module.exports.iter().find(|e| e.name == "__main_void").and_then(|e| if let ExportItem::Function(f) = e.item { Some((e.id(), f)) } else { None });
        if let Some((export_id, orig_func)) = main_void_export {
            let ret_local = module.locals.add(ValType::I32);
            let new_func = module.add_func(&[], &[ValType::I32], |builder, _| {
                let mut body = builder.func_body();
                body.call(orig_func);
                body.local_set(ret_local);
                body.global_get(flag_global);
                body.if_else(
                    ValType::I32,
                    |then| {
                        then.global_get(flag_global);
                        then.call(fix_exit_code_import);
                    },
                    |else_| {
                        else_.local_get(ret_local);
                    }
                );
                Ok(())
            })?;
            module.exports.delete(export_id);
            module.exports.add("__main_void", new_func);
            // Redirect all callers of orig_func to new_func, EXCEPT the wrapper itself.
            // We cannot use renew_call_fn because it would also rewrite the call inside
            // new_func, creating infinite recursion.
            let all_fids: Vec<FunctionId> = module.funcs.iter_local().map(|(id, _)| id).collect();
            for fid in all_fids {
                if fid == new_func {
                    continue;
                }
                let local = module.funcs.get_mut(fid).kind.unwrap_local_mut();
                local.builder_mut().func_body().rewrite(|instr, _| {
                    if let Instr::Call(call) = instr {
                        if call.func == orig_func {
                            call.func = new_func;
                        }
                    }
                });
            }
        }

        // Wrap wasi_thread_start — also created after the call-hook loop.
        let thread_start_export = module.exports.iter().find(|e| e.name == "wasi_thread_start").and_then(|e| if let ExportItem::Function(f) = e.item { Some((e.id(), f)) } else { None });
        if let Some((export_id, orig_func)) = thread_start_export {
            let new_func = module.add_func(&[ValType::I32, ValType::I32], &[], |builder, args| {
                let mut body = builder.func_body();
                body.local_get(args[0]);
                body.local_get(args[1]);
                body.call(orig_func);
                body.global_get(flag_global);
                body.if_else(
                    None,
                    |then| {
                        then.global_get(flag_global);
                        then.call(handle_thread_exit_import);
                    },
                    |_| {}
                );
                Ok(())
            })?;
            module.exports.delete(export_id);
            module.exports.add("wasi_thread_start", new_func);
            // Same pattern: redirect callers, excluding the wrapper itself.
            for fid in module.funcs.iter_local().map(|(id, _)| id).collect::<Vec<_>>() {
                if fid == new_func {
                    continue;
                }
                let local = module.funcs.get_mut(fid).kind.unwrap_local_mut();
                local.builder_mut().func_body().rewrite(|instr, _| {
                    if let Instr::Call(call) = instr {
                        if call.func == orig_func {
                            call.func = new_func;
                        }
                    }
                })?;
            }
        }

        Ok(())
    }
}
