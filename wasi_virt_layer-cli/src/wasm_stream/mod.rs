pub mod mem_info;
pub mod passes;
pub mod pipeline;
pub mod tracker;
pub mod translator;

use wasm_encoder::Module;
use wasmparser::Parser;

/// Context for streaming transformations over a Wasm module.
pub struct StreamContext {
    pub parser: Parser,
    pub encoder: Module,
    pub func_tracker: tracker::IndexTracker,
    pub global_tracker: tracker::IndexTracker,
    pub memory_tracker: tracker::IndexTracker,
    pub type_tracker: tracker::IndexTracker,
}

impl StreamContext {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(0),
            encoder: Module::new(),
            func_tracker: tracker::IndexTracker::new(),
            global_tracker: tracker::IndexTracker::new(),
            memory_tracker: tracker::IndexTracker::new(),
            type_tracker: tracker::IndexTracker::new(),
        }
    }
}

impl Default for StreamContext {
    fn default() -> Self {
        Self::new()
    }
}
