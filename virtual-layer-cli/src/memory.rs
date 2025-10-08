use eyre::{Context as _, ContextCompat as _};
use walrus::ir;

use crate::{
    args::TargetMemoryType,
    generator::{Generator, GeneratorCtx},
    util::{LString, NAMESPACE, ResultUtil as _, WalrusFID, WalrusUtilExport},
};

#[derive(Debug, Default)]
pub struct TemporaryRefugeMemory;

impl Generator for TemporaryRefugeMemory {
    fn post_combine(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        // Remove memory exports outside of VFS.
        // Connect all non-VFS ABIs to VFS and hide them.
        let mem_export_id = module
            .exports
            .iter()
            .filter(|export|
                matches!(export.item, walrus::ExportItem::Memory(mem) if ctx.vfs_used_memory_id.as_ref().unwrap() != &mem)
            )
            .map(|export| export.id())
            .collect::<Vec<_>>();

        for id in mem_export_id {
            module.exports.delete(id);
        }

        if !ctx.threads {
            return Ok(());
        }

        module
            .memories
            .iter_mut()
            .map(|mem| {
                let id = mem.id();
                let mem_id = module
                    .imports
                    .iter()
                    .find_map(|import| match import.kind {
                        walrus::ImportKind::Memory(mid) if mid == id => Some(import.id()),
                        _ => None,
                    })
                    .wrap_err("Failed to find memory import id")?;

                module.imports.delete(mem_id);
                mem.import = None;

                // Translating component requires WasmFeatures::Threads
                // but we cannot enable it because it in other crates.
                // So, we set shared to false here temporarily.
                mem.shared = false;

                Ok(())
            })
            .collect::<eyre::Result<Vec<_>>>()?;

        Ok(())
    }

    fn post_components(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        if !ctx.threads {
            return Ok(());
        }

        module
            .memories
            .iter_mut()
            .enumerate()
            .for_each(|(count, mem)| {
                mem.shared = true;
                let import_id = module.imports.add(
                    "env",
                    &mem.name.clone().unwrap_or_else(|| match count {
                        0 => "memory".to_string(),
                        n => format!("memory{n}"),
                    }),
                    walrus::ImportKind::Memory(mem.id()),
                );

                mem.import = Some(import_id);
            });

        Ok(())
    }
}

fn with_name(wasm: &LString, name: &str) -> String {
    format!("__wasip1_vfs_{wasm}_{name}")
}

/// When exchanging data via Wasip1ABI,
/// there are operations involving writing to
/// and reading from memory.
/// However, as these cannot be accessed during compilation,
/// alternative functions are employed. These shall be replaced.
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

            if let Some(id) = (NAMESPACE, &with_name(wasm, "memory_copy_from"))
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

                module.replace_imported_func(id, |(body, args)| {
                    body.local_get(args[0])
                        .local_get(args[1])
                        .local_get(args[2])
                        .memory_copy(vfs_mem, wasm_mem);
                });
            }

            if let Some(id) = (NAMESPACE, &with_name(wasm, "memory_copy_to"))
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

                module.replace_imported_func(id, |(body, args)| {
                    body.local_get(args[0])
                        .local_get(args[1])
                        .local_get(args[2])
                        .memory_copy(wasm_mem, vfs_mem);
                });
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
            if let Some(id) = (NAMESPACE, &with_name(wasm, "memory_trap"))
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

                module.replace_imported_func(id, |(body, args)| {
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
                });
            }
        }

        Ok(())
    }

    fn post_lower_memory(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        if matches!(ctx.target_memory_type, TargetMemoryType::Multi) {
            return Ok(());
        }

        for wasm in &ctx.target_names {
            let trap_export_name = format!("__wasip1_vfs_{wasm}_memory_trap_anchor");
            let trap_id = trap_export_name
                .get_fid(&module.exports)
                .wrap_err_with(|| {
                    eyre::eyre!("Failed to get {trap_export_name} export on {wasm}.")
                })?;

            module
                .exports
                .erase_with(trap_id, ctx.unstable_print_debug)?;

            let trap_body = module.funcs.get_mut(trap_id).kind.unwrap_local_mut();
            let trap_body = trap_body.block_mut(trap_body.entry_block());

            // Remove the fake value instruction
            // Optimization may have significantly altered the content,
            // but I'll put it off for now.
            let (store_index, store_info) = trap_body
                .iter()
                .enumerate()
                .find_map(|(i, (instr, _))| {
                    if let walrus::ir::Instr::Store(walrus::ir::Store {
                        kind: walrus::ir::StoreKind::I32_8 { atomic: false },
                        memory,
                        arg,
                    }) = instr
                    {
                        if *memory != ctx.vfs_used_memory_id.unwrap() {
                            return Some(Err(eyre::eyre!(
                                "Unexpected memory ID: expected {:?}, got {:?}",
                                ctx.vfs_used_memory_id.unwrap(),
                                *memory
                            )));
                        }
                        Some(Ok((i, arg.to_owned())))
                    } else {
                        None
                    }
                })
                .wrap_err_with(|| eyre::eyre!("Failed to find store instruction"))??;
            trap_body.remove(store_index + 1);
            trap_body.remove(store_index);
            trap_body.remove(store_index - 1);

            if let Some(id) = (NAMESPACE, &format!("__wasip1_vfs_{wasm}_memory_director"))
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
                    module
                        .exports
                        .add(&format!("__wasip1_vfs_{wasm}_memory_director_anchor"), id);
                }
            }
        }

        Ok(())
    }
}
