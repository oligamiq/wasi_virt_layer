use const_struct::const_struct;
use wasi_virt_layer::{
    file::{StandardEmbeddedFiles, WasiEmbeddedFile},
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

#[cfg(not(test))]
export!(Starter);

import_wasm!(test_wasm);

const FILE_COUNT: usize = 10;

#[const_struct]
const EMBEDDED_FILES: StandardEmbeddedFiles<WasiEmbeddedFile<&'static str>, { FILE_COUNT }> = EmbeddedFiles!([
    ("/root", [("root.txt", WasiEmbeddedFile::new("This is root"))]),
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

plug_process!(test_wasm);

#[const_struct]
const ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
    environ: &["HOME=~/", "RUST_BACKTRACE=1"],
};

plug_env!(@embedded, EnvTy, test_wasm);

mod fs {
    use wasi_virt_layer::file::{DefaultStdIO, StandardEmbeddedNormalLFS, StandardEmbeddedFileSystem};

    use super::*;

    type LFS = StandardEmbeddedNormalLFS<EmbeddedFilesTy, WasiEmbeddedFile<&'static str>, FILE_COUNT, DefaultStdIO>;

    static VIRTUAL_FILE_SYSTEM: StandardEmbeddedFileSystem<LFS, FILE_COUNT> =
        StandardEmbeddedFileSystem::new_embedded(StandardEmbeddedNormalLFS::new_embedded());

    plug_fs!(&VIRTUAL_FILE_SYSTEM, test_wasm);
}



