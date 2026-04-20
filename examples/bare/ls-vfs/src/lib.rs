use const_struct::const_struct;
use wasi_virt_layer::{file::*, prelude::*, process::*};

struct ComponentABI;

wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "component-abi",
});

import_wasm!(<anonymous>);

impl Guest for ComponentABI {
    fn main() {
        anonymous::_reset();
        anonymous::_start();
        anonymous::_main();
    }
}

#[cfg(not(test))]
export!(ComponentABI);

mod process {
    use super::*;
    plug_process!(DefaultProcess, anonymous, self);
}

mod env {
    use super::*;

    #[const_struct]
    const HOST_ENV: VirtualEnvConstState = VirtualEnvConstState {
        environ: &["HOME=~/"],
    };

    plug_env!(@const, HostEnvTy, anonymous, self);
}

mod fs {
    use super::*;

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

    type LFS = VFSConstNormalLFS<FilesTy, WasiConstFile<&'static str>, FILE_COUNT, DefaultStdIO>;

    static VIRTUAL_FILE_SYSTEM: Wasip1ConstVFS<LFS, FILE_COUNT> =
        Wasip1ConstVFS::const_new(VFSConstNormalLFS::const_new());

    plug_fs!(&VIRTUAL_FILE_SYSTEM, anonymous);
}
