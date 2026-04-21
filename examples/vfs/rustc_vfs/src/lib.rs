use const_struct::const_struct;
use wasi_virt_layer::{file::*, prelude::*};

wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "main",
});

struct Main;

impl Guest for Main {
    fn main() -> () {}

    fn start() -> () {
        todo!()
    }
}

#[cfg(not(test))]
export!(Main);

import_wasm!(rustc_opt);

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

mod fs {
    use super::*;

    type LFS = StandardEmbeddedNormalLFS<EmbeddedFilesTy, WasiEmbeddedFile<&'static str>, FILE_COUNT, DefaultStdIO>;

    static VIRTUAL_FILE_SYSTEM: StandardEmbeddedFileSystem<LFS, FILE_COUNT> =
        StandardEmbeddedFileSystem::new_embedded(StandardEmbeddedNormalLFS::new_embedded());

    plug_fs!(&VIRTUAL_FILE_SYSTEM, rustc_opt);
}



