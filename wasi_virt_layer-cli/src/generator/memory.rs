use std::{collections::HashMap, str::FromStr};

use eyre::{Context as _, ContextCompat as _};
use walrus::ir;

use crate::{
    args::TargetMemoryType,
    generator::{ComponentCtx, Generator, GeneratorCtx},
    unique_name::UniqueName,
    util::{ResultUtil as _, WalrusFID, WalrusUtilExport, WalrusUtilModule, WasmName},
};

/// Encapsulates naming variants reserved for VFS memory interactions.
#[derive(Debug, strum::AsRefStr, strum::EnumCount, Hash, PartialEq, Eq, strum::VariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum MemoryUniqueName<'a> {
    /// Copies contents from Target Memory to VFS Memory.
    MemoryCopyFrom(&'a WasmName),
    /// Copies contents from VFS Memory to Target Memory.
    MemoryCopyTo(&'a WasmName),
    /// Emplaces a runtime trap to deduce pointer arithmetic limits.
    MemoryTrap(&'a WasmName),
    /// Serves as a linked anchor ensuring `MemoryTrap` preserves its compiled identity.
    MemoryTrapAnchor(&'a WasmName),
    /// Wraps dynamic function dispatch controlling pointers to merged targets.
    MemoryDirector(&'a WasmName),
    /// Serves as a linked anchor preserving the compiled `MemoryDirector` identity.
    MemoryDirectorAnchor(&'a WasmName),
    /// The fundamental reference identifying the exported memory location of the underlying module.
    Memory(&'a WasmName),
}

/// ABI Generator: Manages memory configurations temporarily during transpilation.
/// When exchanging data via Wasip1ABI,
/// there are operations involving writing to
/// and reading from memory.
/// However, as these cannot be accessed during compilation,
/// alternative functions are employed. These shall be replaced.
///
/// **Why this is needed:**
/// Since the VFS layer needs to transfer data between its own memory and the guest's memory
/// when processing ABI calls, it defines `memory_copy_from` and `memory_copy_to` as external imports.
/// Since `walrus` does not allow generating components with unknown raw imports, this generator
/// patches those structural import declarations with native `memory.copy` WebAssembly instructions
/// executing over the correct `multi-memory` tables.
#[derive(Debug, Default)]
pub struct MemoryBridge;

macro_rules! assert_ptr {
    ($ptr:expr) => {
        if { $ptr } != walrus::ValType::I32 {
            let ptr = $ptr;
            eyre::bail!("Invalid pointer type, expected i32. Got {ptr}");
        }
    };
}

macro_rules! assert_len {
    ($len:expr) => {
        if { $len } != walrus::ValType::I32 {
            let len = $len;
            eyre::bail!("Invalid length type, expected i32. Got {len}");
        }
    };
}

macro_rules! check_len {
    ($params:expr, $len:expr) => {
        if { $params.len() } != { $len } {
            let len = $len;
            eyre::bail!(
                "Invalid params length, expected {len}. Got {}",
                { $params }.len()
            );
        }
    };
}

impl Generator for MemoryBridge {
    fn post_combine(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        for wasm in &ctx.target_names {
            let wasm_mem = ctx.target_used_memory_id.as_ref().unwrap()[wasm];
            let vfs_mem = ctx.vfs_used_memory_id.unwrap();

            if let Some(id) = (
                UniqueName::NAMESPACE,
                &UniqueName::Memory(&MemoryUniqueName::MemoryCopyFrom(wasm)),
            )
                .get_fid(&module.imports)
                .ok()
            {
                let func = module.funcs.get(id);
                let ty = module.types.get(func.ty());
                let params = ty.params();

                check_len!(params, 3);
                assert_ptr!(params[0]); // offset
                assert_ptr!(params[1]); // src
                assert_len!(params[2]); // len
                check_len!(ty.results(), 0);

                module
                    .replace_imported_func(id, |(body, args)| {
                        body.local_get(args[0])
                            .local_get(args[1])
                            .local_get(args[2])
                            .memory_copy(vfs_mem, wasm_mem);
                    })
                    .to_eyre()
                    .wrap_err_with(|| eyre::eyre!("Failed to replace memory_copy_from"))?;
            }

            if let Some(id) = (
                UniqueName::NAMESPACE,
                &UniqueName::Memory(&MemoryUniqueName::MemoryCopyTo(wasm)),
            )
                .get_fid(&module.imports)
                .ok()
            {
                let func = module.funcs.get(id);
                let ty = module.types.get(func.ty());
                let params = ty.params();

                check_len!(params, 3);
                assert_ptr!(params[0]); // offset
                assert_ptr!(params[1]); // src
                assert_len!(params[2]); // len
                check_len!(ty.results(), 0);

                module
                    .replace_imported_func(id, |(body, args)| {
                        body.local_get(args[0])
                            .local_get(args[1])
                            .local_get(args[2])
                            .memory_copy(wasm_mem, vfs_mem);
                    })
                    .to_eyre()
                    .wrap_err_with(|| eyre::eyre!("Failed to replace memory_copy_to"))?;
            }
        }

        Ok(())
    }
}

/// The final wasm, due to ABI constraints,
/// only exposes vfs memory.
/// Therefore, when calling the ABI from non-vfs memory,
/// data must be copied. However,
/// when ultimately consolidating memory into a single pool,
/// data can be passed externally by directly passing pointers.
/// To implement this optimization,
/// a function is provided to determine the pointer bias
/// before memory consolidation.
///
/// **Why this is needed:**
/// If running in `single_memory` mode (without the `multi_memory` WASM feature), all modules'
/// memories are merged into one large address space. The VFS needs to know where the guest
/// module's memory segment actually resides within this merged pool. This generator uses a trap
/// mechanism to securely determine the actual pointer bias at runtime so external callers and
/// the VFS can translate addresses correctly.
#[derive(Debug, Default)]
pub struct MemoryTrap;

impl Generator for MemoryTrap {
    fn post_combine(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        if matches!(ctx.target_memory_type, TargetMemoryType::Multi) {
            return Ok(());
        }

        for wasm in &ctx.target_names {
            // NOTE: The import was created by the import_wasm! macro using the
            // Rust identifier name (with underscores), but ctx.target_names has
            // the package name (with dashes). We need to normalize to underscores
            // to match what the macro generated.
            let normalized_wasm = wasm.as_ref().replace('-', "_");
            let import_name = format!("__wasip1_vfs_{normalized_wasm}_memory_trap");

            if let Some(id) = (UniqueName::NAMESPACE, &import_name)
                .get_fid(&module.imports)
                .ok()
            {
                let func = module.funcs.get(id);
                let ty = module.types.get(func.ty());
                let params = ty.params();

                check_len!(params, 1);
                assert_ptr!(params[0]); // offset
                check_len!(ty.results(), 1);
                assert_ptr!(ty.results()[0]); // result

                let wasm_mem = ctx.target_used_memory_id.as_ref().unwrap()[wasm];

                module
                    .replace_imported_func(id, |(body, args)| {
                        body.local_get(args[0])
                            .i32_const(0)
                            .store(
                                wasm_mem,
                                ir::StoreKind::I32_8 { atomic: false },
                                ir::MemArg {
                                    align: 0,
                                    offset: 0,
                                },
                            )
                            .i32_const(0);
                    })
                    .to_eyre()
                    .wrap_err_with(|| eyre::eyre!("Failed to replace memory_trap"))?;
            }

            // --- Merged post_lower_memory logic ---
            let wasm_mem = ctx.target_used_memory_id.as_ref().unwrap()[wasm];
            let trap_export_name = format!("__wasip1_vfs_{normalized_wasm}_memory_trap_anchor");
            let trap_id = trap_export_name
                .get_fid(&module.exports)
                .wrap_err_with(|| {
                    eyre::eyre!("Failed to get {trap_export_name} export on {wasm}.")
                })?;

            module
                .exports
                .erase_with(trap_id, ctx.unstable_print_debug)?;

            let mut current_trap_id = trap_id;
            let mut store_found = None;

            loop {
                let current_body = module.funcs.get(current_trap_id).kind.unwrap_local();
                let block = current_body.block(current_body.entry_block());

                let mut next_call = None;
                for (i, (instr, _)) in block.iter().enumerate() {
                    if let walrus::ir::Instr::Store(walrus::ir::Store {
                        kind: walrus::ir::StoreKind::I32_8 { atomic: false },
                        memory,
                        arg,
                    }) = instr
                    {
                        if *memory != wasm_mem {
                            store_found = Some(Err(eyre::eyre!(
                                "Unexpected memory ID: expected {:?}, got {:?}",
                                wasm_mem,
                                *memory
                            )));
                        } else {
                            store_found = Some(Ok((current_trap_id, i, arg.clone())));
                        }
                        break;
                    } else if let walrus::ir::Instr::Call(call) = instr {
                        next_call = Some(call.func);
                    }
                }

                if store_found.is_some() {
                    break;
                }

                if let Some(callee) = next_call {
                    current_trap_id = callee;
                } else {
                    store_found = Some(Err(eyre::eyre!("Failed to find store instruction")));
                    break;
                }
            }

            let (actual_trap_id, store_index, store_info) = store_found.unwrap()?;

            let actual_trap_body = module.funcs.get_mut(actual_trap_id).kind.unwrap_local_mut();
            let actual_trap_block = actual_trap_body.block_mut(actual_trap_body.entry_block());

            actual_trap_block.remove(store_index + 1);
            actual_trap_block.remove(store_index);
            actual_trap_block.remove(store_index - 1);

            if let Some(id) = (
                UniqueName::NAMESPACE,
                &UniqueName::Memory(&MemoryUniqueName::MemoryDirector(wasm)),
            )
                .get_fid(&module.imports)
                .ok()
            {
                module
                    .replace_imported_func(id, |(builder, args)| {
                        let mut func_body = builder.func_body();
                        func_body
                            .local_get(args[0])
                            .call(trap_id)
                            .i32_const(store_info.offset as i32)
                            .binop(walrus::ir::BinaryOp::I32Add)
                            .return_();
                    })
                    .to_eyre()
                    .wrap_err_with(|| eyre::eyre!("Failed to replace imported function"))?;

                if ctx.unstable_print_debug {
                    module.exports.add(
                        &UniqueName::Memory(&MemoryUniqueName::MemoryDirectorAnchor(wasm))
                            .to_string(),
                        id,
                    );
                }
            }
        }

        Ok(())
    }
}
