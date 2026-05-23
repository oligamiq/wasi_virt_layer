use crate::wasm_stream::pipeline::StreamPass;
use eyre::Result;

#[derive(Debug, Default)]
pub struct ThreadsStreamPass {
    pub threads: bool,
    pub unstable_print_debug: bool,
    pub target_names: Vec<String>,
}

impl ThreadsStreamPass {
    pub fn new(threads: bool, unstable_print_debug: bool, target_names: Vec<String>) -> Self {
        Self { threads, unstable_print_debug, target_names }
    }
}

impl StreamPass for ThreadsStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> Result<Vec<u8>> {
        if !self.threads {
            return Ok(input_wasm.to_vec());
        }

        let mut module = walrus::Module::from_buffer(input_wasm).map_err(|e| eyre::eyre!("{}", e))?;
        
        use crate::generator::Generator;
        let mut old_pass = crate::generator::threads::ThreadsSpawn::default();
        
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let compact_strings: Box<[_]> = self.target_names.iter().map(|s| compact_str::CompactString::new(s)).collect();
        let holder = crate::util::WasmNameHolder::new(compact_strings);
        let target_names = holder.iter().collect::<Vec<_>>();
        let mock_ctx = crate::generator::GeneratorCtx {
            vfs_name: crate::util::WasmName::new("mock", &COUNTER),
            target_names: target_names.into_boxed_slice(),
            target_names_with_self: Box::new([]),
            vfs_used_memory_id: None,
            vfs_used_global_id: None,
            target_used_memory_id: None,
            target_used_global_id: None,
            start_func_id: None,
            target_memory_type: crate::args::TargetMemoryType::Single,
            unstable_print_debug: self.unstable_print_debug,
            dwarf: false,
            threads: self.threads,
            adjust_abi: false,
            keep_build_artifacts: false,
            vfs_is_library: false,
            starts: crate::generator::starts::FnInStarts::new(&[]),
        };
        
        old_pass.pre_vfs(&mut module, &mock_ctx)?;
        old_pass.post_combine(&mut module, &mock_ctx)?;
        
        let mut encoder = wasm_encoder::Module::new();
        let wasm_bytes = module.emit_wasm();
        Ok(wasm_bytes)
    }
}
