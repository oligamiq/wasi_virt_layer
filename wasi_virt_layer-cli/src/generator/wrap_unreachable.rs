use std::collections::HashSet;

use walrus::{ConstExpr, ExportItem, FunctionId, Module, ValType, ir::*};

use crate::{
    generator::{Generator, GeneratorCtx, ModuleExternal},
    instrs::InstrRewrite as _,
    util::{WalrusFID, WalrusUtilModule, WasmName},
};

// ── Naming ────────────────────────────────────────────────────────────

/// Import module used by the `wrap_unreachable!` macro on the library side.
///
/// This is ABI-locked in `wasi_virt_layer::wasi::wrap_unreachable` and must
/// stay in sync with the library macro.
const WRAP_UNREACHABLE_MODULE: &str = "__wasip1_virt_layer";

/// Structured names associated with the wrap-unreachable feature for a
/// specific target module.
///
/// Follows the same pattern as [`SpecialFuncUniqueName`] and other
/// per-generator name enums, but uses the `__wasip1_virt_layer_` prefix
/// because these names are ABI-locked with the library-side macro.
///
/// [`SpecialFuncUniqueName`]: crate::generator::special_func::SpecialFuncUniqueName
#[derive(Debug, strum::AsRefStr, strum::EnumCount, Hash, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum WrapUnreachableName<'a> {
    /// Marker export that opts a target into unreachable wrapping.
    WrapUnreachable(&'a WasmName),
    /// Getter for the per-target `unreachable_flag` global.
    GetUnreachableFlag(&'a WasmName),
    /// Setter for the per-target `unreachable_flag` global.
    SetUnreachableFlag(&'a WasmName),
    /// VFS handler: maps the flag value to a proper exit code.
    FixMainRawExitCode(&'a WasmName),
    /// VFS handler: propagates thread exit when the flag is set.
    HandleThreadExit(&'a WasmName),
}

impl WrapUnreachableName<'_> {
    /// The prefix used for all wrap-unreachable names, matching the library macro.
    const PREFIX: &'static str = "__wasip1_virt_layer_";

    /// Formats this name into the full ABI-locked string.
    fn to_name(&self) -> String {
        let variant = self.as_ref();
        match self {
            Self::WrapUnreachable(wasm)
            | Self::GetUnreachableFlag(wasm)
            | Self::SetUnreachableFlag(wasm)
            | Self::FixMainRawExitCode(wasm)
            | Self::HandleThreadExit(wasm) => {
                format!("{}{wasm}_{variant}", Self::PREFIX)
            }
        }
    }
}

impl std::fmt::Display for WrapUnreachableName<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_name())
    }
}

// ── Dummy-value helpers ───────────────────────────────────────────────

/// Returns a zero-valued [`Value`] matching the given Wasm value type.
///
/// # Panics
///
/// Panics on `ValType::Ref`, which is not supported.
fn dummy_value(ty: &ValType) -> Value {
    match ty {
        ValType::I32 => Value::I32(0),
        ValType::I64 => Value::I64(0),
        ValType::F32 => Value::F32(0.0),
        ValType::F64 => Value::F64(0.0),
        ValType::V128 => Value::V128(0),
        ValType::Ref(_) => unimplemented!("Ref types not supported for dummy returns"),
    }
}

/// Appends dummy-constant instructions for every return type followed by a
/// `return` instruction.
fn push_dummy_return(
    instrs: &mut Vec<(Instr, InstrLocId)>,
    return_types: &[ValType],
    loc: InstrLocId,
) {
    for ty in return_types {
        instrs.push((
            Instr::Const(Const {
                value: dummy_value(ty),
            }),
            loc,
        ));
    }
    instrs.push((Instr::Return(Return {}), loc));
}

// ── Instruction scanning ──────────────────────────────────────────────

/// Positions of interesting instructions found during a recursive scan.
struct InstrScanResult {
    /// `(seq_id, index)` pairs for `Unreachable` instructions.
    unreachables: Vec<(InstrSeqId, usize)>,
    /// `(seq_id, index)` pairs for `Call` / `CallIndirect` instructions.
    calls: Vec<(InstrSeqId, usize)>,
}

/// Walks every instruction sequence within `func`, recording
/// positions of `unreachable` and `call` / `call_indirect` instructions.
fn scan_instructions(func: &walrus::LocalFunction) -> InstrScanResult {
    let mut result = InstrScanResult {
        unreachables: Vec::new(),
        calls: Vec::new(),
    };

    let mut visited = HashSet::new();
    let mut work_stack = vec![func.entry_block()];

    while let Some(seq_id) = work_stack.pop() {
        if visited.contains(&seq_id) {
            continue;
        }
        visited.insert(seq_id);

        for (i, (instr, _)) in func.block(seq_id).instrs.iter().enumerate() {
            match instr {
                Instr::Unreachable(_) => result.unreachables.push((seq_id, i)),
                Instr::Call(_) | Instr::CallIndirect(_) => result.calls.push((seq_id, i)),
                Instr::Block(b) => {
                    if !visited.contains(&b.seq) {
                        work_stack.push(b.seq);
                    }
                }
                Instr::Loop(l) => {
                    if !visited.contains(&l.seq) {
                        work_stack.push(l.seq);
                    }
                }
                Instr::IfElse(ie) => {
                    if !visited.contains(&ie.consequent) {
                        work_stack.push(ie.consequent);
                    }
                    if !visited.contains(&ie.alternative) {
                        work_stack.push(ie.alternative);
                    }
                }
                _ => {}
            }
        }
    }

    result
}

// ── Instruction patching ──────────────────────────────────────────────

/// Replaces every `unreachable` instruction at the recorded positions with
/// a *set-flag + dummy-return* sequence.
///
/// Processes sites in reverse index order so that earlier indices remain valid.
fn patch_unreachables(
    func: &mut walrus::LocalFunction,
    sites: &mut [(InstrSeqId, usize)],
    flag_global: walrus::GlobalId,
    return_types: &[ValType],
) {
    sites.sort_unstable_by(|a, b| b.cmp(a));

    for &(seq_id, idx) in sites.iter() {
        let seq = func.block_mut(seq_id);
        let (_removed, loc) = seq.instrs.remove(idx);

        let mut new_instrs = Vec::with_capacity(return_types.len() + 3);
        new_instrs.push((
            Instr::Const(Const {
                value: Value::I32(1),
            }),
            loc,
        ));
        new_instrs.push((
            Instr::GlobalSet(GlobalSet {
                global: flag_global,
            }),
            loc,
        ));
        push_dummy_return(&mut new_instrs, return_types, loc);

        for (j, instr) in new_instrs.into_iter().enumerate() {
            seq.instrs.insert(idx + j, instr);
        }
    }
}

/// Inserts a post-call flag-check after every `call` / `call_indirect` at the
/// recorded positions.
///
/// The check is: `if (flag) { push dummy values; return; } else { /* nop */ }`.
/// Processes sites in reverse index order.
fn hook_calls(
    func: &mut walrus::LocalFunction,
    sites: &mut [(InstrSeqId, usize)],
    flag_global: walrus::GlobalId,
    return_types: &[ValType],
) {
    sites.sort_unstable_by(|a, b| b.cmp(a));

    for &(seq_id, idx) in sites.iter() {
        let insert_idx = idx + 1;

        // Read the source location from the call instruction (NLL drops this borrow).
        let loc = func.block(seq_id).instrs[idx].1;

        // Build the consequent (flag-is-set) branch: dummy return.
        let consequent_seq = func.builder_mut().dangling_instr_seq(None).id();
        {
            let consequent = func.block_mut(consequent_seq);
            push_dummy_return(&mut consequent.instrs, return_types, loc);
        }

        // Empty alternative (flag-is-clear) branch.
        let alternative_seq = func.builder_mut().dangling_instr_seq(None).id();

        // Insert the check after the call.
        let seq = func.block_mut(seq_id);
        let check = [
            (
                Instr::GlobalGet(GlobalGet {
                    global: flag_global,
                }),
                loc,
            ),
            (
                Instr::IfElse(IfElse {
                    consequent: consequent_seq,
                    alternative: alternative_seq,
                }),
                loc,
            ),
        ];

        for (j, instr) in check.into_iter().enumerate() {
            seq.instrs.insert(insert_idx + j, instr);
        }
    }
}

// ── Export wrapping ───────────────────────────────────────────────────

/// Wraps the `__main_void` export so that, after calling the original
/// function, the unreachable flag is checked and forwarded to
/// `fix_main_raw_exit_code` when set.
fn wrap_main_void(
    module: &mut Module,
    flag_global: walrus::GlobalId,
    fix_exit_code_import: FunctionId,
) -> eyre::Result<()> {
    let Some((export_id, orig_func)) = find_export_func(module, "__main_void") else {
        return Ok(());
    };

    let ret_local = module.locals.add(ValType::I32);
    let wrapper = module.add_func(&[], &[ValType::I32], |builder, _| {
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
            },
        );
        Ok(())
    })?;

    module.exports.delete(export_id);
    module.exports.add("__main_void", wrapper);
    redirect_callers(module, orig_func, wrapper)?;

    Ok(())
}

/// Wraps the `wasi_thread_start` export so that `handle_thread_exit` is
/// invoked when the unreachable flag is set after the original entry point
/// returns.
fn wrap_thread_start(
    module: &mut Module,
    flag_global: walrus::GlobalId,
    handle_thread_exit_import: FunctionId,
) -> eyre::Result<()> {
    let Some((export_id, orig_func)) = find_export_func(module, "wasi_thread_start") else {
        return Ok(());
    };

    let wrapper = module.add_func(&[ValType::I32, ValType::I32], &[], |builder, args| {
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
            |_| {},
        );
        Ok(())
    })?;

    module.exports.delete(export_id);
    module.exports.add("wasi_thread_start", wrapper);
    redirect_callers(module, orig_func, wrapper)?;

    Ok(())
}

// ── Shared utilities ──────────────────────────────────────────────────

/// Finds a function export by name, returning its export ID and function ID.
fn find_export_func(module: &Module, name: &str) -> Option<(walrus::ExportId, FunctionId)> {
    module
        .exports
        .iter()
        .find(|e| e.name == name)
        .and_then(|e| match e.item {
            ExportItem::Function(f) => Some((e.id(), f)),
            _ => None,
        })
}

/// Redirects all callers of `old_func` to `new_func`, **excluding** calls
/// inside `new_func` itself (to prevent infinite recursion in wrappers).
fn redirect_callers(
    module: &mut Module,
    old_func: FunctionId,
    new_func: FunctionId,
) -> eyre::Result<()> {
    let all_fids: Vec<FunctionId> = module.funcs.iter_local().map(|(id, _)| id).collect();

    for fid in all_fids {
        if fid == new_func {
            continue;
        }
        let local = module.funcs.get_mut(fid).kind.unwrap_local_mut();
        local.builder_mut().func_body().rewrite(|instr, _| {
            if let Instr::Call(call) = instr {
                if call.func == old_func {
                    call.func = new_func;
                }
            }
        })?;
    }

    Ok(())
}



// ── Generator implementation ──────────────────────────────────────────

/// Generator that replaces WebAssembly `unreachable` instructions and handles
/// unwinding call stacks gracefully by injecting a global flag and modifying
/// call sites.
///
/// When a VFS target module opts-in by exporting a
/// `__wasip1_virt_layer_{target}_wrap_unreachable` marker (generated by the
/// [`wrap_unreachable!`] macro), this generator:
///
/// 1. Adds a per-target global `unreachable_flag` to the target module.
/// 2. Replaces every `unreachable` instruction with a flag-set + dummy-return
///    sequence.
/// 3. Hooks every `call` / `call_indirect` site with a post-call flag check
///    that short-circuits with a dummy return when the flag is set.
/// 4. Wraps `__main_void` and `wasi_thread_start` exports so the flag is
///    forwarded to the VFS for proper exit-code / thread-exit handling.
///
/// [`wrap_unreachable!`]: wasi_virt_layer::wrap_unreachable
#[derive(Debug, Default)]
pub struct WrapUnreachableGenerator {
    /// Target module names that opted in via the marker export.
    targets: HashSet<String>,
}

impl Generator for WrapUnreachableGenerator {
    fn pre_vfs(&mut self, module: &mut Module, ctx: &GeneratorCtx) -> eyre::Result<()> {
        for target in ctx.target_names.iter() {
            let marker = WrapUnreachableName::WrapUnreachable(&target).to_string();
            println!(
                "Looking for marker: {}, found: {}",
                marker,
                module.exports.iter().any(|e| e.name == marker)
            );
            if module.exports.iter().any(|e| e.name == marker) {
                self.targets.insert(target.to_string());
            }
        }
        Ok(())
    }

    fn post_combine(&mut self, module: &mut Module, ctx: &GeneratorCtx) -> eyre::Result<()> {
        Ok(())
    }

    fn pre_target(
        &mut self,
        _module: &mut Module,
        _: &GeneratorCtx,
        _external: &ModuleExternal,
    ) -> eyre::Result<()> {
        // Migrated to WrapUnreachablePreTargetStreamPass
        Ok(())
    }
}


