use const_struct::const_struct;
use wasi_virt_layer::{
    file::*, plug_thread, poll::*, prelude::*, thread::VirtualThreadPool,
};

struct ComponentABI;

wit_bindgen::generate!({
    world: "component-abi",
});

import_wasm!(pool_reused_direct_export_target);

static THREAD_POOL: VirtualThreadPool<ThreadAccessor> = unsafe { VirtualThreadPool::new_const(2) };

impl Guest for ComponentABI {
    fn main() {
        unsafe { THREAD_POOL.init_with_capacity_and_wait(2) };
        println!("Starting non-main direct-export test");

        let handle = std::thread::spawn(|| {
            pool_reused_direct_export_target::_start();
            pool_reused_direct_export_target::_main();
        });
        handle.join().unwrap();

        println!("Non-main direct-export test passed");
    }
}

#[cfg(not(test))]
export!(ComponentABI);

plug_thread!({ &THREAD_POOL }, pool_reused_direct_export_target, self);
plug_poll!(DefaultWaitPoll, pool_reused_direct_export_target);

wasi_virt_layer::plug_clock!(
    wasi_virt_layer::clock::StandardClock,
    pool_reused_direct_export_target,
    self
);

mod process {
    use super::*;
    plug_process!(
        wasi_virt_layer::process::StandardProcess,
        pool_reused_direct_export_target,
        self
    );
}

mod env {
    use super::*;
    #[const_struct]
    const HOST_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
        environ: &["HOME=~/"],
    };
    plug_env!(@embedded, HostEnvTy, pool_reused_direct_export_target, self);
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

    plug_fs!(&VIRTUAL_FILE_SYSTEM, pool_reused_direct_export_target, self);
}
