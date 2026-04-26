use const_struct::const_struct;
use std::thread;
use wasi_virt_layer::{
    file::*,
    plug_thread,
    prelude::*,
    process::*,
    thread::VirtualThreadPool,
    poll::WaitPoll,
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
        anonymous::_start();

        // Initialize the pool
        let pool = unsafe { &mut *(&raw mut THREAD_POOL) };
        pool.init();
        pool.set_capacity(2);
        pool.flush_capacity().wait();

        println!("Pool threads initialized.");

        let mut handles = vec![];
        for i in 0..3 {
            let handle = thread::spawn(move || {
                println!("Thread {} running on virtual pool.", i);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        
        println!("All threads finished.");
    }
}

#[cfg(not(test))]
export!(ComponentABI);

plug_thread!(
    { unsafe { &mut *(&raw mut THREAD_POOL) } },
    anonymous,
    self
);

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
        EmbeddedFiles!([
            ("/", [("dummy.txt", WasiEmbeddedFile::new("dummy"))])
        ]);

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
