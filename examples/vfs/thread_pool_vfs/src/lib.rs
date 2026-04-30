use const_struct::const_struct;
use wasi_virt_layer::{
    file::{StandardEmbeddedFiles, WasiEmbeddedFile},
    prelude::*,
    thread::VirtualThreadPool,
};

wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "init",
});

struct Starter;

impl Guest for Starter {
    fn main() {
        test_pool_thread::_reset();
        test_pool_thread::_start();

        Self::init(4);

        test_pool_thread::_main();
    }

    fn init(pool_size: u32) {
        println!("%%% Initializing thread pool with size {}", pool_size);

        let pool = &raw mut THREAD_POOL;
        let pool = unsafe { &mut *pool };
        pool.init();

        println!("%%% Setting thread pool capacity to {}", pool_size);

        pool.set_capacity(pool_size as usize);

        println!("%%% Flushing thread pool capacity...");
        let waiter = pool.flush_capacity();

        println!("%%% Waiting for thread pool to initialize...");

        waiter.wait();

        println!("%%% Thread pool initialized.");
    }
}

#[cfg(not(test))]
export!(Starter);

import_wasm!(test_pool_thread);

const FILE_COUNT: usize = 10;

type F = WasiEmbeddedFile<&'static str>;
type NormalFILES = StandardEmbeddedFiles<F, { FILE_COUNT }>;

#[const_struct]
const EMBEDDED_FILES: NormalFILES = EmbeddedFiles!([
    (
        "/root",
        [("root.txt", WasiEmbeddedFile::new("This is root"))]
    ),
    (
        ".",
        [
            ("hey", WasiEmbeddedFile::new("Hey!")),
            (
                "hello",
                [
                    ("world", WasiEmbeddedFile::new("Hello, world!")),
                    ("everyone", WasiEmbeddedFile::new("Hello, everyone!")),
                ]
            )
        ]
    ),
    (
        "~",
        [
            ("home", WasiEmbeddedFile::new("This is home")),
            ("user", WasiEmbeddedFile::new("This is user")),
        ]
    )
]);

static mut THREAD_POOL: VirtualThreadPool<ThreadAccessor> =
    unsafe { VirtualThreadPool::new_const(4) };

plug_thread!(
    { unsafe { &mut *(&raw mut THREAD_POOL) } },
    // { wasi_virt_layer::thread::DirectThreadPool::<ThreadAccessor>::new_const() },
    self,
    test_pool_thread
);
plug_process!(
    wasi_virt_layer::process::StandardProcess,
    test_pool_thread,
    self
);
#[const_struct]
const ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
    environ: &["HOME=~/"],
};
plug_env!(@embedded, EnvTy, test_pool_thread, self);

use wasi_virt_layer::poll::DefaultWaitPoll;
plug_poll!(DefaultWaitPoll, test_pool_thread);

mod fs {
    use wasi_virt_layer::file::{
        DefaultStdIO, StandardEmbeddedFileSystem, StandardEmbeddedNormalLFS,
    };

    use super::*;

    type LFS = StandardEmbeddedNormalLFS<EmbeddedFilesTy, F, FILE_COUNT, DefaultStdIO>;

    static VIRTUAL_FILE_SYSTEM: StandardEmbeddedFileSystem<LFS, FILE_COUNT> =
        StandardEmbeddedFileSystem::new_const(StandardEmbeddedNormalLFS::new_const());

    plug_fs!(&VIRTUAL_FILE_SYSTEM, test_pool_thread, self);
}
