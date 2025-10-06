use eyre::ContextCompat as _;

use crate::generator::{Generator, GeneratorCtx};

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
