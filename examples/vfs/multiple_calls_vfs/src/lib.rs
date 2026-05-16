use const_struct::const_struct;
use wasi_virt_layer::{file::*, plug_process, prelude::*};

wit_bindgen::generate!({
    world: "init",
});

struct Starter;

impl Guest for Starter {
    fn start() {
        println!("Starting multiple calls of the target Wasm...");

        for i in 1..=3 {
            println!("\n--- Target Call {} ---", i);
            // Reset the target Wasm memory state to its initial state
            test_wasm::_reset();
            // Call the standard WASI _start function
            test_wasm::_start();
        }

        println!("\nFinished multiple calls successfully.");
    }
}

#[cfg(not(test))]
export!(Starter);

// Import the target Wasm module named `test_wasm`
import_wasm!(test_wasm);



mod fs {
    use super::*;
    use std::sync::LazyLock;
    use wasi_virt_layer::file::multiple::*;
    pub static VIRTUAL_FILE_SYSTEM: LazyLock<StandardMultipleFileSystem> = LazyLock::new(|| {
        let mut vfs = StandardMultipleFileSystem::new();
        vfs.add_wasm::<test_wasm>();
        let lfs = StandardDynamicLFS::<DefaultStdIO>::new();
        vfs.add_lfs(lfs);
        vfs
    });
    plug_fs!(&*VIRTUAL_FILE_SYSTEM, test_wasm, self);
}

#[const_struct]
const VIRTUAL_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
    environ: &["TEST_VAR=multiple_calls"],
};

plug_env!(@embedded, VirtualEnvTy, test_wasm, self);

plug_process!(
    wasi_virt_layer::process::StandardProcess,
    test_wasm,
    self
);
