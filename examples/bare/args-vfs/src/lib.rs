use const_struct::const_struct;
use wasi_virt_layer::{prelude::*, process::*};

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
    plug_process!(StandardProcess, anonymous, self);
}

mod env {
    use super::*;

    #[const_struct]
    const HOST_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
        environ: &["HOME=~/"],
    };

    plug_env!(@embedded, HostEnvTy, anonymous, self);
}

mod args {
    use super::*;

    #[const_struct]
    const HOST_ARGS: VirtualArgsEmbeddedState = VirtualArgsEmbeddedState {
        args: &["my_args_program", "hello", "world"],
    };

    plug_args!(@embedded, HostArgsTy, anonymous, self);
}

mod fs {
    use super::*;
    use wasi_virt_layer::file::*;

    #[const_struct]
    const EMBEDDED_FILES: StandardEmbeddedFiles<WasiEmbeddedFile<&'static str>, 2> =
        EmbeddedFiles!([(".", [("dummy.txt", WasiEmbeddedFile::new("dummy"))])]);

    type LFS =
        StandardEmbeddedNormalLFS<EmbeddedFilesTy, WasiEmbeddedFile<&'static str>, 2, DefaultStdIO>;

    static VIRTUAL_FILE_SYSTEM: StandardEmbeddedFileSystem<LFS, 2> =
        StandardEmbeddedFileSystem::new_const(StandardEmbeddedNormalLFS::new_const());

    plug_fs!(&VIRTUAL_FILE_SYSTEM, anonymous, self);
}
