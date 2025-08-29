use const_struct::const_struct;
use wasip1_virtual_layer::{self, wasi::file::constant::lfs_raw::*, *};

wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "main",
});

struct Main;

impl Guest for Main {
    fn main() -> () {}

    fn start() -> () {
        println!("Files: {:?}", FILES);

        todo!()
    }
}

export!(Main);

import_wasm!(test_threads);

const FILE_COUNT: usize = 5;

#[const_struct]
const FILES: VFSConstNormalFiles<WasiConstFile<&'static str>, { FILE_COUNT }> = ConstFiles!([(
    ".",
    [
        ("hey", WasiConstFile::new("Hey!")),
        (
            "hello",
            [
                ("world", WasiConstFile::new("Hello, world!")),
                ("everyone", WasiConstFile::new("Hello, everyone!")),
            ],
        ),
    ],
)]);

mod fs {
    use super::*;
    use wasip1_virtual_layer::{
        export_fs,
        wasi::file::{
            constant::{lfs::VFSConstNormalLFS, vfs::Wasip1ConstVFS},
            stdio::DefaultStdIO,
        },
    };

    type LFS = VFSConstNormalLFS<FilesTy, WasiConstFile<&'static str>, FILE_COUNT, DefaultStdIO>;

    static mut VIRTUAL_FILE_SYSTEM: Wasip1ConstVFS<LFS, FILE_COUNT> =
        Wasip1ConstVFS::new(VFSConstNormalLFS::new());

    export_fs!(@const, {
        #[allow(static_mut_refs)]
        unsafe { &mut VIRTUAL_FILE_SYSTEM }
    }, test_threads);
}
