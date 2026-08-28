use eyre::Context as _;

pub trait StreamPass: Send + Sync {
    /// Runs the transformation pass on the provided WebAssembly binary.
    /// Returns the modified WebAssembly binary.
    fn run(&mut self, input_wasm: &[u8]) -> eyre::Result<Vec<u8>>;
}

pub trait StreamChecker: Send + Sync {
    fn check(&mut self, payload: &wasmparser::Payload) -> eyre::Result<()>;
}

pub struct ParallelCheckStreamPass {
    checkers: Vec<Box<dyn StreamChecker>>,
}

impl ParallelCheckStreamPass {
    pub fn new(checkers: Vec<Box<dyn StreamChecker>>) -> Self {
        Self { checkers }
    }
}

impl StreamPass for ParallelCheckStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> eyre::Result<Vec<u8>> {
        for payload in wasmparser::Parser::new(0).parse_all(input_wasm) {
            let payload = payload?;
            // Process checkers sequentially for now (parsing itself is fast,
            // and parallelizing across lightweight checkers might not outweigh thread overhead,
            // but the architecture allows replacing this loop with rayon later).
            for checker in &mut self.checkers {
                checker.check(&payload)?;
            }
        }
        Ok(input_wasm.to_vec())
    }
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
    F: Fn(usize, wasmparser::FunctionBody<'_>) -> eyre::Result<wasm_encoder::Function>
        + Sync
        + Send,
{
    use eyre::Context;
    use rayon::prelude::*;

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
    /// Create a new empty streaming pipeline.
    pub fn new() -> Self {
        Self { passes: Vec::new() }
    }

    /// Add a streaming pass to the pipeline.
    pub fn add_pass(&mut self, pass: Box<dyn StreamPass>) {
        self.passes.push(pass);
    }

    /// Run all passes in the pipeline sequentially on the given WebAssembly module.
    pub fn run(&mut self, input_wasm: &[u8]) -> eyre::Result<Vec<u8>> {
        let mut current_wasm = input_wasm.to_vec();
        log::debug!("PIPELINE RUN IS EXECUTING!");

        let mut i = 0;
        for pass in &mut self.passes {
            log::debug!("RUNNING STREAM PASS #{}", i);
            current_wasm = pass
                .run(&current_wasm)
                .wrap_err_with(|| format!("Failed in stream pass #{i}"))?;
            i += 1;
        }

        Ok(current_wasm)
    }
}
