use const_struct::const_struct;
use wasi_virt_layer::{
    self, ConstFiles,
    file::{VFSConstNormalFiles, WasiConstFile},
    import_wasm, plug_env, plug_fs, plug_process, plug_thread,
    prelude::VirtualEnvConstState,
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
        test_pool_thread::_main();
    }
}

export!(Starter);

import_wasm!(test_pool_thread);

const FILE_COUNT: usize = 10;

#[const_struct]
const FILES: VFSConstNormalFiles<WasiConstFile<&'static str>, { FILE_COUNT }> = ConstFiles!([
    ("/root", [("root.txt", WasiConstFile::new("This is root"))]),
    (
        ".",
        [
            ("hey", WasiConstFile::new("Hey!")),
            (
                "hello",
                [
                    ("world", WasiConstFile::new("Hello, world!")),
                    ("everyone", WasiConstFile::new("Hello, everyone!")),
                ]
            )
        ]
    ),
    (
        "~",
        [
            ("home", WasiConstFile::new("This is home")),
            ("user", WasiConstFile::new("This is user")),
        ]
    )
]);

plug_process!(test_pool_thread);

#[const_struct]
const ENV: VirtualEnvConstState = VirtualEnvConstState {
    environ: &["HOME=~/", "RUST_BACKTRACE=1"],
};

plug_env!(@const, EnvTy, test_pool_thread);

struct ThreadAlt;
impl wasi_virt_layer::thread::VirtualThread for ThreadAlt {
    fn new_thread(
        &mut self,
        accessor: impl wasi_virt_layer::thread::ThreadAccess,
        runner: wasi_virt_layer::thread::ThreadRunner,
    ) -> Option<std::num::NonZero<u32>> {
        unreachable!();
    }
}

plug_thread!(@sched_yield, ThreadAlt, test_pool_thread);

mod fs {
    use wasi_virt_layer::file::{DefaultStdIO, VFSConstNormalLFS, Wasip1ConstVFS};

    use super::*;

    type LFS = VFSConstNormalLFS<FilesTy, WasiConstFile<&'static str>, FILE_COUNT, DefaultStdIO>;

    static mut VIRTUAL_FILE_SYSTEM: Wasip1ConstVFS<LFS, FILE_COUNT> =
        Wasip1ConstVFS::new(VFSConstNormalLFS::new());

    plug_fs!(@const, {
        #[allow(static_mut_refs)]
        unsafe { &mut VIRTUAL_FILE_SYSTEM }
    }, test_pool_thread);
}
