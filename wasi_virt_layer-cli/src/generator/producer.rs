use crate::generator::{Generator, GeneratorCtx};

/// Generator that adds `wasi-virt-layer` as a processed producer to the final binary metric.
#[derive(Debug, Default)]
pub struct Producer;

impl Generator for Producer {
    fn pre_vfs(&mut self, module: &mut walrus::Module, _: &GeneratorCtx) -> eyre::Result<()> {
        module
            .producers
            .add_processed_by("wasi-virt-layer", env!("CARGO_PKG_VERSION"));

        Ok(())
    }
}
