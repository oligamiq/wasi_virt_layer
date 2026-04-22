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

mod process {
    use super::*;
    plug_process!(DefaultProcess, anonymous, self);
}

mod env {
    use super::*;

    #[const_struct]
    const HOST_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
        environ: &["HOME=~/"],
    };

    plug_env!(@embedded, HostEnvTy, anonymous, self);
}

mod fs {
    use super::*;

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

    type LFS = StandardEmbeddedNormalLFS<EmbeddedFilesTy, WasiEmbeddedFile<&'static str>, FILE_COUNT, DefaultStdIO>;

    static VIRTUAL_FILE_SYSTEM: StandardEmbeddedFileSystem<LFS, FILE_COUNT> =
        StandardEmbeddedFileSystem::new_const(StandardEmbeddedNormalLFS::new_const());

    plug_fs!(&VIRTUAL_FILE_SYSTEM, anonymous);
}

mod fs {
    use super::*;

    const FILE_COUNT: usize = 10;

    #[const_struct]
    const EMBEDDED_FILES: VFSConstNormalFiles<WasiConstFile<&'static str>, { FILE_COUNT }> = ConstFiles!([
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

    type LFS = VFSConstNormalLFS<EmbeddedFilesTy, WasiConstFile<&'static str>, FILE_COUNT, DefaultStdIO>;

    static VIRTUAL_FILE_SYSTEM: Wasip1ConstVFS<LFS, FILE_COUNT> =
        Wasip1ConstVFS::new_const(VFSConstNormalLFS::new_const());

    plug_fs!(&VIRTUAL_FILE_SYSTEM, anonymous);
}

#[cfg(not(test))]
export!(ComponentABI);
