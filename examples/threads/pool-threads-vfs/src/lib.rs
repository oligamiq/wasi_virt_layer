use const_struct::const_struct;
use wasi_virt_layer::{
    file::*, plug_thread, poll::WaitPoll, prelude::*, process::*, thread::VirtualThreadPool,
};

struct ComponentABI;

wit_bindgen::generate!({
    world: "component-abi",
});

import_wasm!(<anonymous>);

static mut THREAD_POOL: VirtualThreadPool<ThreadAccessor> =
    unsafe { VirtualThreadPool::new_const(2) };

impl Guest for ComponentABI {
    fn main() {
        anonymous::_reset();

        // Initialize the pool before _start(), because _start() triggers
        // the guest's main() which immediately spawns threads via the pool.
        let pool = unsafe { &mut *(&raw mut THREAD_POOL) };
        pool.init();
        pool.set_capacity(2);
        pool.flush_capacity().wait();

        println!("Pool threads initialized.");

        anonymous::_start();
    }
}

#[cfg(not(test))]
export!(ComponentABI);

plug_thread!({ unsafe { &mut *(&raw mut THREAD_POOL) } }, anonymous, self);

plug_poll!(WaitPoll, anonymous);

mod process {
    use super::*;
    plug_process!(wasi_virt_layer::process::StandardProcess, anonymous, self);
}

mod env {
    use super::*;

    #[const_struct]
    const HOST_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
        environ: &["HOME=~/"],
    };

    plug_env!(@embedded, HostEnvTy, anonymous, self);
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

    plug_fs!(&VIRTUAL_FILE_SYSTEM, anonymous, self);
}
