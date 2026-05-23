use super::StreamContext;
use eyre::Context as _;

pub trait StreamPass: Send + Sync {
    /// Runs the transformation pass on the provided WebAssembly binary.
    /// Returns the modified WebAssembly binary.
    fn run(&mut self, input_wasm: &[u8]) -> eyre::Result<Vec<u8>>;
}

/// A pipeline of sequential WebAssembly transformations.
pub struct Pipeline {
    passes: Vec<Box<dyn StreamPass>>,
}

/// Helper function to process all functions in a CodeSection in parallel.
/// Takes the raw bytes of the CodeSection, and a transformation function.
pub fn par_process_code_section<F>(
    code_section: wasmparser::CodeSectionReader<'_>,
    transform: F,
) -> eyre::Result<wasm_encoder::CodeSection>
where
    F: Fn(usize, wasmparser::FunctionBody<'_>) -> eyre::Result<wasm_encoder::Function> + Sync + Send,
{
    use rayon::prelude::*;
    use eyre::Context;

    // Collect all function bodies
    let bodies = code_section
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .wrap_err("Failed to read function bodies")?;

    // Process them in parallel
    let encoded_functions = bodies
        .into_par_iter()
        .enumerate()
        .map(|(i, body)| transform(i, body))
        .collect::<eyre::Result<Vec<_>>>()?;

    // Stitch back together
    let mut new_code_section = wasm_encoder::CodeSection::new();
    for encoded_func in encoded_functions {
        new_code_section.function(&encoded_func);
    }

    Ok(new_code_section)
}

impl Pipeline {
    pub fn new() -> Self {
        Self { passes: Vec::new() }
    }

    pub fn add_pass(&mut self, pass: Box<dyn StreamPass>) {
        self.passes.push(pass);
    }

    pub fn run(&mut self, input_wasm: &[u8]) -> eyre::Result<Vec<u8>> {
        let mut current_wasm = input_wasm.to_vec();
        
        for (idx, pass) in self.passes.iter_mut().enumerate() {
            current_wasm = pass.run(&current_wasm)
                .wrap_err_with(|| format!("Failed in pass #{idx}"))?;
        }
        
        Ok(current_wasm)
    }
}
