use eyre::Context as _;

use crate::{
    generator::{Generator, GeneratorCtx, ModuleExternal},
    util::WalrusUtilModule as _,
};

#[derive(Debug, Default)]
pub struct ResetCondition;

impl Generator for ResetCondition {
    fn pre_vfs(&mut self, module: &mut walrus::Module, _: &GeneratorCtx) -> eyre::Result<()> {
        module
            .create_global_anchor("vfs")
            .wrap_err("Failed to create global anchor")?;

        Ok(())
    }

    fn pre_target(
        &mut self,
        module: &mut walrus::Module,
        _: &GeneratorCtx,
        external: &ModuleExternal,
    ) -> eyre::Result<()> {
        module
            .create_global_anchor(&external.name)
            .wrap_err("Failed to create global anchor")?;

        Ok(())
    }
}
