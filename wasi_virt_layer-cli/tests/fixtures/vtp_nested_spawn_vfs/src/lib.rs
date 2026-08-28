use const_struct::const_struct;
use wasi_virt_layer::{
    file::*, plug_thread, poll::*, prelude::*, process::*, thread::VirtualThreadPool,
};

struct ComponentABI;

wit_bindgen::generate!({
    world: "component-abi",
});

import_wasm!(vtp_nested_spawn_target);

static THREAD_POOL: VirtualThreadPool<ThreadAccessor> = unsafe { VirtualThreadPool::new_const(4) };

impl Guest for ComponentABI {
    fn main() {
        unsafe { THREAD_POOL.init_with_capacity_and_wait(4) };
        println!("Nested spawn pool initialized.");
        vtp_nested_spawn_target::_start();
        vtp_nested_spawn_target::_main();
        vtp_nested_spawn_target::_main();
    }
}

#[cfg(not(test))]
export!(ComponentABI);

plug_thread!({ &THREAD_POOL }, vtp_nested_spawn_target, self);
plug_poll!(DefaultWaitPoll, vtp_nested_spawn_target);

wasi_virt_layer::plug_clock!(
    wasi_virt_layer::clock::StandardClock,
    vtp_nested_spawn_target,
    self
);

mod process {
    use super::*;
    plug_process!(
        wasi_virt_layer::process::StandardProcess,
        vtp_nested_spawn_target,
        self
    );
}

mod env {
    use super::*;
    #[const_struct]
    const HOST_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
        environ: &["HOME=~/"],
    };
    plug_env!(@embedded, HostEnvTy, vtp_nested_spawn_target, self);
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

    plug_fs!(&VIRTUAL_FILE_SYSTEM, vtp_nested_spawn_target, self);
}
