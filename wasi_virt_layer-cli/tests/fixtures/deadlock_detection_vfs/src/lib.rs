use const_struct::const_struct;
use wasi_virt_layer::{file::*, poll::*, prelude::*, process::*, thread::VirtualThreadPool};

struct ComponentABI;

wit_bindgen::generate!({
    world: "component-abi",
});

import_wasm!(deadlock_detection_target);

static THREAD_POOL: VirtualThreadPool<ThreadAccessor> = unsafe { VirtualThreadPool::new_const(2) };

impl Guest for ComponentABI {
    fn main() {
        unsafe { THREAD_POOL.init_with_capacity_and_wait(2) };
        println!("Starting deadlock detection test");
        deadlock_detection_target::_start();
        println!("Deadlock detection false-positive test passed");
    }
}

#[cfg(not(test))]
export!(ComponentABI);

wasi_virt_layer::plug_thread!({ &THREAD_POOL }, deadlock_detection_target, self);
wasi_virt_layer::plug_poll!(DefaultWaitPoll, deadlock_detection_target);

wasi_virt_layer::plug_clock!(
    wasi_virt_layer::clock::StandardClock,
    deadlock_detection_target,
    self
);

mod process {
    use super::*;
    wasi_virt_layer::plug_process!(
        wasi_virt_layer::process::StandardProcess,
        deadlock_detection_target,
        self
    );
}

mod env {
    use super::*;
    #[const_struct]
    const HOST_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
        environ: &["HOME=~/"],
    };
    wasi_virt_layer::plug_env!(@embedded, HostEnvTy, deadlock_detection_target, self);
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

    wasi_virt_layer::plug_fs!(&VIRTUAL_FILE_SYSTEM, deadlock_detection_target, self);
}
