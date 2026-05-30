use crate::generator::{ComponentCtx, Generator};
use eyre::Result;
use walrus::{ImportKind, Module};

#[derive(Debug, Default)]
pub struct PostComponentsMemoryFix {}

impl Generator for PostComponentsMemoryFix {
    fn post_components(&mut self, module: &mut Module, ctx: &ComponentCtx) -> Result<()> {
        if !ctx.threads.unwrap_or(false) {
            return Ok(());
        }

        for mem in module.memories.iter_mut() {
            mem.shared = true;
            if mem.maximum.is_none() {
                mem.maximum = Some(mem.initial.max(65536));
            }

            // In the original TemporaryRefugeMemory, ALL shared memories were
            // imported from `env.memory` after components were built.
            let import_id = module
                .imports
                .add("env", "memory", ImportKind::Memory(mem.id()));
            mem.import = Some(import_id);
        }

        Ok(())
    }
}
