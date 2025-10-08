use eyre::ContextCompat as _;

use crate::{
    generator::{Generator, GeneratorCtx},
    util::{LString, WalrusFID},
};

#[derive(Debug, Default)]
pub struct TemporaryRefugeMemory;

impl Generator for TemporaryRefugeMemory {
    fn post_combine(
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

/// When exchanging data via Wasip1ABI,
/// there are operations involving writing to
/// and reading from memory.
/// However, as these cannot be accessed during compilation,
/// alternative functions are employed. These shall be replaced.
#[derive(Debug, Default)]
pub struct MemoryBridge;

impl MemoryBridge {
    const NAMESPACE: &str = "wasip1-vfs";
    fn with_name(wasm: &LString, name: &str) -> String {
        format!("__wasip1_vfs_{wasm}_{name}")
    }
}

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

            if let Some(id) = (Self::NAMESPACE, &Self::with_name(wasm, "memory_copy_from"))
                .get_fid(&module.imports)
                .ok()
            {
                let func = module.funcs.get_mut(id);
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

            if let Some(id) = (Self::NAMESPACE, &Self::with_name(wasm, "memory_copy_to"))
                .get_fid(&module.imports)
                .ok()
            {
                let func = module.funcs.get_mut(id);
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
