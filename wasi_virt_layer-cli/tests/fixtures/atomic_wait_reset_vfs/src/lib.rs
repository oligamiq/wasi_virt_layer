use const_struct::const_struct;
use wasi_virt_layer::{file::*, poll::*, prelude::*, process::*, thread::VirtualThreadPool};

struct ComponentABI;

wit_bindgen::generate!({
    world: "component-abi",
});

import_wasm!(atomic_wait_reset_target);

static THREAD_POOL: VirtualThreadPool<ThreadAccessor> = unsafe { VirtualThreadPool::new_const(2) };

impl Guest for ComponentABI {
    fn main() {
        unsafe { THREAD_POOL.init_with_capacity_and_wait(2) };
        println!("Starting atomic wait reset test");
        atomic_wait_reset_target::_start();
        atomic_wait_reset_target::_reset();
        atomic_wait_reset_target::_start();
        atomic_wait_reset_target::_main();
        println!("Atomic wait reset test passed");
    }
}

#[cfg(not(test))]
export!(ComponentABI);

wasi_virt_layer::plug_thread!({ &THREAD_POOL }, atomic_wait_reset_target, self);
wasi_virt_layer::plug_poll!(DefaultWaitPoll, atomic_wait_reset_target);

wasi_virt_layer::plug_clock!(
    wasi_virt_layer::clock::StandardClock,
    atomic_wait_reset_target,
    self
);

mod process {
    use super::*;
    wasi_virt_layer::plug_process!(
        wasi_virt_layer::process::StandardProcess,
        atomic_wait_reset_target,
        self
    );
}

mod env {
    use super::*;
    #[const_struct]
    const HOST_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
        environ: &["HOME=~/"],
    };
    wasi_virt_layer::plug_env!(@embedded, HostEnvTy, atomic_wait_reset_target, self);
}

mod fs {
    use super::*;
    const FILE_COUNT: usize = 2;
    #[const_struct]
    const EMBEDDED_FILES: StandardEmbeddedFiles<WasiEmbeddedFile<&'static str>, { FILE_COUNT }> =
        EmbeddedFiles!([("/", [("dummy.txt", WasiEmbeddedFile::new("dummy"))])]);

    type LFS = StandardEmbeddedNormalLFS<
        EmbeddedFilesTy,
        WasiEmbeddedFile<&'static str>,
        FILE_COUNT,
        DefaultStdIO,
    >;

    static VIRTUAL_FILE_SYSTEM: StandardEmbeddedFileSystem<LFS, FILE_COUNT> =
        StandardEmbeddedFileSystem::new_const(StandardEmbeddedNormalLFS::new_const());

    wasi_virt_layer::plug_fs!(&VIRTUAL_FILE_SYSTEM, atomic_wait_reset_target, self);
}
