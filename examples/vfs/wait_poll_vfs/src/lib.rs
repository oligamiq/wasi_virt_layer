use const_struct::const_struct;
use wasi_virt_layer::file::*;
use wasi_virt_layer::poll::DefaultWaitPoll;
use wasi_virt_layer::prelude::*;
use wasi_virt_layer::{
    EmbeddedFiles, import_wasm, plug_clock, plug_env, plug_fs, plug_poll, plug_process, plug_sched,
};

import_wasm!(test_poll);

// Plug WaitPoll for the test_poll target
plug_poll!(DefaultWaitPoll, test_poll, self);

// Minimal environment setup
#[const_struct]
const ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
    environ: &["HOME=/"],
};
plug_env!(@embedded, EnvTy, test_poll, self);

// Minimal clock setup
plug_clock!(wasi_virt_layer::clock::StandardClock, test_poll, self);

// Minimal sched setup (needed for sleep/yield)
plug_sched!(wasi_virt_layer::sched::DefaultSched, test_poll, self);

// Minimal process setup
plug_process!(wasi_virt_layer::process::StandardProcess, test_poll, self);

const FILE_COUNT: usize = 2; // "." directory + "placeholder.txt"

#[const_struct]
const EMBEDDED_FILES: StandardEmbeddedFiles<WasiEmbeddedFile<&'static str>, { FILE_COUNT }> =
    EmbeddedFiles!([(
        ".",
        [("placeholder.txt", WasiEmbeddedFile::new("placeholder"))]
    )]);

// Minimal filesystem setup
mod fs {
    use super::*;

    type LFS = StandardEmbeddedNormalLFS<
        EmbeddedFilesTy,
        WasiEmbeddedFile<&'static str>,
        FILE_COUNT,
        DefaultStdIO,
    >;

    static VIRTUAL_FILE_SYSTEM: StandardEmbeddedFileSystem<LFS, FILE_COUNT> =
        StandardEmbeddedFileSystem::new_const(StandardEmbeddedNormalLFS::new_const());

    plug_fs!(&VIRTUAL_FILE_SYSTEM, test_poll, self);
}

#[cfg(target_os = "wasi")]
#[unsafe(no_mangle)]
pub fn main() {
    println!("### Starting WaitPoll VFS example...");
    test_poll::_start();
    test_poll::_main();
}
