use std::{collections::HashMap, str::FromStr};

use eyre::{Context as _, ContextCompat as _};
use walrus::ir;

use crate::{
    args::TargetMemoryType,
    generator::{ComponentCtx, Generator, GeneratorCtx},
    util::{LString, NAMESPACE, ResultUtil as _, WalrusFID, WalrusUtilExport, WalrusUtilModule},
};

#[derive(Debug, Default)]
pub struct TemporaryRefugeMemory {
    pub memory_count: usize,
}

impl TemporaryRefugeMemory {
    pub fn ready_component_and_transpile(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        let mut is_first = false;

        let mut imported_memories = Vec::new();
        let mut shared_memories = Vec::new();

        let mut mem_names = HashMap::new();

        module
            .memories
            .iter_mut()
            .map(|mem| {
                let id = mem.id();

                if let Some(mem_id) = mem.import {
                    imported_memories.push(id);
                    let import = module.imports.get(mem_id);
                    let name = import.name.clone();
                    module.imports.delete(mem_id);
                    mem.import = None;
                    if mem_names.values().any(|s| *s == name) {
                        let n = (1..)
                            .find(|n| !mem_names.values().any(|s| *s == format!("{name}_{n}")))
                            .unwrap();
                        mem_names.insert(id, format!("{name}_{n}"));
                    } else {
                        mem_names.insert(id, name);
                    }
                }

                // Translating component requires WasmFeatures::Threads
                // but we cannot enable it because it in other crates.
                // So, we set shared to false here temporarily.
                if mem.shared {
                    if ctx.no_transpile && !is_first {
                        is_first = true;
                        log::warn!(
                            r"Transpiling with threads is not supported yet. so this wasm off memory shared flag and can't be used as it is. {mem:?}"
                        );
                    }

                    shared_memories.push(id);
                    mem.shared = false;
                }

                Ok(())
            })
            .collect::<eyre::Result<Vec<_>>>()?;

        for (n, mem_id) in module
            .memories
            .iter()
            .map(|mem| mem.id())
            .enumerate()
            .collect::<Box<_>>()
        {
            module.create_memory_anchor_with_info(
                format!("__wasip1_vfs_memory_anchor_{n}"),
                mem_id,
                Some(HadSharedAndImported {
                    had_shared: shared_memories.contains(&mem_id),
                    had_imported: imported_memories.contains(&mem_id),
                    name: mem_names.get(&mem_id).cloned(),
                }),
            )?;
        }

        self.memory_count = module.memories.iter().count();

        module
            .save_info("memory_count", self.memory_count)
            .wrap_err("Failed to save memory_count")?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct HadSharedAndImported {
    pub had_shared: bool,
    pub had_imported: bool,
    pub name: Option<String>,
}

impl ToString for HadSharedAndImported {
    fn to_string(&self) -> String {
        let n = self.had_shared as u8
            | (self.had_imported as u8) << 1
            | (self.name.is_some() as u8) << 2;
        format!("{n}{}", self.name.as_ref().unwrap_or(&"".into()))
    }
}

impl FromStr for HadSharedAndImported {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = s[0..1]
            .parse::<u8>()
            .wrap_err_with(|| eyre::eyre!("Failed to parse HadSharedAndImported from {s}"))?;
        let name = if n & 4 != 0 {
            Some(s[1..].to_string())
        } else {
            None
        };
        Ok(Self {
            had_shared: (n & 1) != 0,
            had_imported: (n & 2) != 0,
            name,
        })
    }
}

impl Generator for TemporaryRefugeMemory {
    fn pre_target(
        &mut self,
        module: &mut walrus::Module,
        _: &GeneratorCtx,
        external: &crate::generator::ModuleExternal,
    ) -> eyre::Result<()> {
        module
            .exports
            .iter_mut()
            .find(|export| export.name == "memory")
            .unwrap()
            .name = format!("__wasip1_vfs_{}_memory", external.name);

        Ok(())
    }

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

        // rename vfs memory export to memory
        module
            .memories
            .get_mut(ctx.vfs_used_memory_id.unwrap())
            .name = Some("memory".to_string());
        module
            .exports
            .get_mut(
                module
                    .exports
                    .get_exported_memory(ctx.vfs_used_memory_id.unwrap())
                    .unwrap()
                    .id(),
            )
            .name = "memory".to_string();

        if !ctx.threads {
            return Ok(());
        }

        if ctx.target_memory_type == TargetMemoryType::Multi {
            self.ready_component_and_transpile(module, ctx)?;
        } else {
            module
                .memories
                .iter_mut()
                .filter(|mem| mem.id() != *ctx.vfs_used_memory_id.as_ref().unwrap())
                .map(|mem| {
                    let id = mem.id();
                    let mem_id = module.imports.iter().find_map(|import| match import.kind {
                        walrus::ImportKind::Memory(mid) if mid == id => Some(import.id()),
                        _ => None,
                    });

                    if let Some(mem_id) = mem_id {
                        module.imports.delete(mem_id);
                        mem.import = None;
                    }

                    Ok(())
                })
                .collect::<eyre::Result<Vec<_>>>()?;
        }

        Ok(())
    }

    fn post_lower_memory(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        if !ctx.threads {
            return Ok(());
        }

        self.ready_component_and_transpile(module, ctx)?;

        Ok(())
    }

    fn post_components(
        &mut self,
        module: &mut walrus::Module,
        ctx: &ComponentCtx,
    ) -> eyre::Result<()> {
        if !ctx.threads() {
            return Ok(());
        }

        self.memory_count = module
            .load_info::<usize>("memory_count")
            .wrap_err("Failed to load memory_count")?;

        for count in 0..self.memory_count {
            let (id, info) = module
                .get_memory_anchor_with_info::<HadSharedAndImported>(
                    &format!("__wasip1_vfs_memory_anchor_{count}"),
                    true,
                )
                .wrap_err("cannot find info, you may change anchor name")?;
            let info = info.unwrap();
            let mem = module.memories.get_mut(id);

            mem.name = info.name.clone();

            if info.had_imported {
                let import_id = module.imports.add(
                    "env",
                    &info.name.unwrap(),
                    walrus::ImportKind::Memory(mem.id()),
                );

                mem.import = Some(import_id);
            }
            if info.had_shared {
                mem.shared = true;
            }
        }

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
