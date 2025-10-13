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

export!(Main);

import_wasm!(rustc_opt);

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

mod fs {
    use super::*;

    type LFS = VFSConstNormalLFS<FilesTy, WasiConstFile<&'static str>, FILE_COUNT, DefaultStdIO>;

    static mut VIRTUAL_FILE_SYSTEM: Wasip1ConstVFS<LFS, FILE_COUNT> =
        Wasip1ConstVFS::new(VFSConstNormalLFS::new());

    plug_fs!(@const, {
        #[allow(static_mut_refs)]
        unsafe { &mut VIRTUAL_FILE_SYSTEM }
    }, rustc_opt);
}
