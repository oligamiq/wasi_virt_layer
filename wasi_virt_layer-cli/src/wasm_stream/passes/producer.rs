use crate::wasm_stream::pipeline::StreamPass;
use eyre::Result;
use wasm_encoder::{CustomSection, Module, RawSection, Section};
use wasmparser::Parser;

/// Generator that adds `wasi-virt-layer` as a processed producer to the final binary metric.
#[derive(Debug, Default)]
pub struct ProducerStreamPass;

impl ProducerStreamPass {
    pub fn new() -> Self {
        Self
    }
}

impl StreamPass for ProducerStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> Result<Vec<u8>> {
        let mut module = Module::new();
        let mut has_producer_section = false;

        let parser = Parser::new(0);
        for payload in parser.parse_all(input_wasm) {
            let payload = payload?;
            match payload {
                wasmparser::Payload::CustomSection(c) if c.name() == "producers" => {
                    // TODO: We could append to existing producers section, 
                    // but for now, we just pass it through and append our own at the end,
                    // or append to the existing one. Wait, multiple producers sections are allowed?
                    // According to WASM spec, custom sections can appear multiple times.
                    // It's easier to just pass through all existing ones and append ours at the end.
                    module.section(&RawSection {
                        id: wasm_encoder::SectionId::Custom as u8,
                        data: c.data(),
                    });
                    has_producer_section = true;
                }
                _ => {
                    if let Some((id, range)) = payload.as_section() {
                        module.section(&RawSection {
                            id,
                            data: &input_wasm[range.clone()],
                        });
                    }
                }
            }
        }

        // Add our producer section
        let mut producers = wasm_encoder::ProducersSection::new();
        let mut field = wasm_encoder::ProducersField::new();
        field.value("wasi-virt-layer", env!("CARGO_PKG_VERSION"));
        producers.field("processed-by", &field);
        module.section(&producers);

        Ok(module.finish())
    }
}
