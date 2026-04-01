use crate::generator::{Generator, GeneratorCtx};

#[derive(Debug, Default)]
pub struct Producer;

// TODO: Check thourgh with threads feature.
impl Generator for Producer {
    fn pre_vfs(&mut self, module: &mut walrus::Module, _: &GeneratorCtx) -> eyre::Result<()> {
        module.producers.add_processed_by("wasi-virt-layer", env!("CARGO_PKG_VERSION"));

        Ok(())
    }
}
