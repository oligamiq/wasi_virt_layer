use const_struct::const_struct;
use wasi_virt_layer::{
    file::*, plug_thread, poll::*, prelude::*, process::*, thread::VirtualThreadPool,
};

struct ComponentABI;

wit_bindgen::generate!({
    world: "component-abi",
});

import_wasm!(spawn_main_target);

static THREAD_POOL: VirtualThreadPool<ThreadAccessor> = unsafe { VirtualThreadPool::new_const(5) };

impl Guest for ComponentABI {
    fn main() {
        // spawn_main_target::_reset();
        unsafe { THREAD_POOL.init_with_capacity_and_wait(5) };
        println!("Custom Pool threads initialized.");

        #[cfg(feature = "spawn_main")]
        {
            println!("Spawning main in a new thread.");
            let handle = std::thread::spawn(|| {
                spawn_main_target::_start();
                spawn_main_target::_main();
            });
            handle.join().unwrap();
        }

        #[cfg(not(feature = "spawn_main"))]
        {
            spawn_main_target::_start();
            spawn_main_target::_main();
        }
    }
}

#[cfg(not(test))]
export!(ComponentABI);

plug_thread!({ &THREAD_POOL }, spawn_main_target, self);
plug_poll!(DefaultWaitPoll, spawn_main_target);

wasi_virt_layer::plug_clock!(
    wasi_virt_layer::clock::StandardClock,
    spawn_main_target,
    self
);

mod process {
    use super::*;
    plug_process!(
        wasi_virt_layer::process::StandardProcess,
        spawn_main_target,
        self
    );
}

mod env {
    use super::*;
    #[const_struct]
    const HOST_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
        environ: &["HOME=~/"],
    };
    plug_env!(@embedded, HostEnvTy, spawn_main_target, self);
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

    plug_fs!(&VIRTUAL_FILE_SYSTEM, spawn_main_target, self);
}
