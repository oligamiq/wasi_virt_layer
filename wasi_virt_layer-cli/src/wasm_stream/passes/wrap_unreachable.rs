use crate::wasm_stream::pipeline::StreamPass;
use eyre::Result;

#[derive(Debug, Default)]
pub struct WrapUnreachableStreamPass {
    pub adjust_abi: bool,
}

impl WrapUnreachableStreamPass {
    pub fn new(adjust_abi: bool) -> Self {
        Self { adjust_abi }
    }
}

impl StreamPass for WrapUnreachableStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> Result<Vec<u8>> {
        if !self.adjust_abi {
            return Ok(input_wasm.to_vec());
        }

        let mut module = walrus::Module::from_buffer(input_wasm).map_err(|e| eyre::eyre!("{}", e))?;
        
        use crate::generator::Generator;
        let mut old_pass = crate::generator::wrap_unreachable::WrapUnreachableGenerator::default();
        
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let mock_ctx = crate::generator::GeneratorCtx {
            vfs_name: crate::util::WasmName::new("mock", &COUNTER),
            target_names: Box::new([]),
            target_names_with_self: Box::new([]),
            vfs_used_memory_id: None,
            vfs_used_global_id: None,
            target_used_memory_id: None,
            target_used_global_id: None,
            start_func_id: None,
            target_memory_type: crate::args::TargetMemoryType::Single,
            unstable_print_debug: false,
            dwarf: false,
            threads: false,
            adjust_abi: self.adjust_abi,
            keep_build_artifacts: false,
            vfs_is_library: false,
            starts: crate::generator::starts::FnInStarts::new(&[]),
        };
        
        old_pass.pre_vfs(&mut module, &mock_ctx)?;
        
        let wasm_bytes = module.emit_wasm();
        Ok(wasm_bytes)
    }
}
