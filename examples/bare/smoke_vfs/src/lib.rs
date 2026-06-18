use const_struct::const_struct;
use wasi_virt_layer::prelude::*;

wit_bindgen::generate!({
    world: "component-abi",
});

struct ComponentABI;

import_wasm!(smoke_target);

wasi_virt_layer::own_memory!(smoke_target);

impl Guest for ComponentABI {
    fn main() {
        // Expand memory for smoke_target to handle allocations and thread stacks
        println!("Expanding memory by 3200 pages (approx 200MB) for smoke_target...");
        // Use the generated memory_grow function
        let res = memory_grow::<smoke_target>(3200);
        println!("Memory grow result: {:?}", res);

        smoke_target::_reset();
        smoke_target::_start();
        smoke_target::_main();
    }
}

export!(ComponentABI);

// Plug standard WASI imports for smoke_target
mod plug {
    use super::*;
    use wasi_virt_layer::file::*;
    use wasi_virt_layer::process::*;
    use wasi_virt_layer::thread::*;

    // Threading support
    static THREAD_POOL: wasi_virt_layer::thread::DirectThreadPool<ThreadAccessor> =
        wasi_virt_layer::thread::DirectThreadPool::new_const();
    plug_thread!({ &THREAD_POOL }, smoke_target, self);

    plug_process!(StandardProcess, smoke_target, self);
    plug_random!(StandardRandom, smoke_target, self);
    wasi_virt_layer::plug_clock!(wasi_virt_layer::clock::StandardClock, smoke_target, self);

    // Use StandardEmbeddedFiles with FLAT_LEN=2
    #[const_struct]
    const EMBEDDED_FILES: StandardEmbeddedFiles<WasiEmbeddedFile<&'static str>, 2> =
        EmbeddedFiles!([(".", [("dummy.txt", WasiEmbeddedFile::new("dummy"))])]);

    type LFS =
        StandardEmbeddedNormalLFS<EmbeddedFilesTy, WasiEmbeddedFile<&'static str>, 2, DefaultStdIO>;

    static VIRTUAL_FILE_SYSTEM: StandardEmbeddedFileSystem<LFS, 2> =
        StandardEmbeddedFileSystem::new_const(StandardEmbeddedNormalLFS::new_const());

    plug_fs!(&VIRTUAL_FILE_SYSTEM, smoke_target, self);

    // Minimal env
    #[const_struct]
    const DEFAULT_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
        environ: &["HOME=/", "RUST_BACKTRACE=1"],
    };
    plug_env!(@embedded, DefaultEnvTy, smoke_target, self);
}

// Enable shared memory management for threads
use wasi_virt_layer::shared_memory::SharedMemoryManagerTrait;
static SHARED_MEMORY_HOLDER: wasi_virt_layer::shared_memory::StandardSharedMemoryHolder =
    wasi_virt_layer::shared_memory::StandardSharedMemoryHolder::new();
wasi_virt_layer::export_shared_memory_manager!(SHARED_MEMORY_HOLDER);
