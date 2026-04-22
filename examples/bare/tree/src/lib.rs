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

    plug_fs!(&VIRTUAL_FILE_SYSTEM, anonymous, self);
}



