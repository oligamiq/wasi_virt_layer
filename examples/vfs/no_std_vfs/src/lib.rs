use const_struct::const_struct;
use wasi_virt_layer::{
    file::{VFSConstNormalFiles, WasiConstFile},
    prelude::*,
};

wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "init",
});

struct Starter;

impl Guest for Starter {
    fn main() {
        // test_wasm::_reset();
        // test_wasm::_start();
        test_wasm::_main();
    }
}

export!(Starter);

import_wasm!(test_wasm);

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

plug_process!(test_wasm);

#[const_struct]
const ENV: VirtualEnvConstState = VirtualEnvConstState {
    environ: &["HOME=~/", "RUST_BACKTRACE=1"],
};

plug_env!(@const, EnvTy, test_wasm);

mod fs {
    use wasi_virt_layer::file::{DefaultStdIO, VFSConstNormalLFS, Wasip1ConstVFS};

    use super::*;

    type LFS = VFSConstNormalLFS<FilesTy, WasiConstFile<&'static str>, FILE_COUNT, DefaultStdIO>;

    static mut VIRTUAL_FILE_SYSTEM: Wasip1ConstVFS<LFS, FILE_COUNT> =
        Wasip1ConstVFS::new(VFSConstNormalLFS::new());

    plug_fs!(@const, {
        #[allow(static_mut_refs)]
        unsafe { &mut VIRTUAL_FILE_SYSTEM }
    }, test_wasm);
}
