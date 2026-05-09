use const_struct::const_struct;
use wasi_virt_layer::file::*;
use wasi_virt_layer::poll::DefaultWaitPoll;
use wasi_virt_layer::prelude::*;
use wasi_virt_layer::{plug_clock, plug_poll, plug_process};

wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "init",
});

struct Starter;

impl Guest for Starter {
    fn init() {
        println!("`init` function done (atomic_wait_vfs).");
    }

    fn start() {
        test_atomic_wait::_start();
    }

    fn main() {
        test_atomic_wait::_reset();
        test_atomic_wait::_start();
        test_atomic_wait::_main();
    }
}

#[cfg(not(test))]
export!(Starter);

import_wasm!(test_atomic_wait);

const FILE_COUNT: usize = 2;

type F = WasiEmbeddedFile<&'static str>;
type NormalFILES = StandardEmbeddedFiles<F, { FILE_COUNT }>;

#[const_struct]
const EMBEDDED_FILES: NormalFILES = EmbeddedFiles!([(".", [("test", F::new("test"))])]);

plug_thread!(
    { wasi_virt_layer::thread::DirectThreadPool::<ThreadAccessor>::new_const() },
    self,
    test_atomic_wait
);

plug_process!(
    wasi_virt_layer::process::StandardProcess,
    test_atomic_wait,
    self
);

plug_poll!(DefaultWaitPoll, test_atomic_wait, self);
plug_clock!(wasi_virt_layer::clock::StandardClock, test_atomic_wait, self);

#[const_struct]
const VIRTUAL_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
    environ: &["HOME=~/"],
};
plug_env!(@embedded, VirtualEnvTy, test_atomic_wait, self);

mod fs {
    use super::*;

    type LFS = StandardEmbeddedNormalLFS<EmbeddedFilesTy, F, FILE_COUNT, DefaultStdIO>;

    static VIRTUAL_FILE_SYSTEM: StandardEmbeddedFileSystem<LFS, FILE_COUNT> =
        StandardEmbeddedFileSystem::new_const(StandardEmbeddedNormalLFS::new_const());

    plug_fs!(&VIRTUAL_FILE_SYSTEM, test_atomic_wait, self);
}
