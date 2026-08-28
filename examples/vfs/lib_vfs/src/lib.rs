// This is an example of a VFS formatted as a library that does not have a `main` function.
// It directly uses `import_wasm!` and `plug_*!` macros without needing a `Guest` implementation
// or `wit-bindgen::generate!`.

use const_struct::const_struct;
use wasi_virt_layer::{file::*, plug_process, prelude::*, process::StandardProcess};

import_wasm!(test_lib);

plug_process!(StandardProcess, test_lib, self);
plug_random!(StandardRandom, test_lib, self);
plug_sched!(DefaultSched, test_lib, self);

// We define a simple embedded environment
#[const_struct]
const HOST_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
    environ: &["HELLO=World_from_VFS"],
};

plug_env!(@embedded, HostEnvTy, test_lib);

// We define a simple embedded file system (empty is fine if we don't use files)
const FILE_COUNT: usize = 2;

#[const_struct]
const EMBEDDED_FILES: StandardEmbeddedFiles<WasiEmbeddedFile<&'static str>, { FILE_COUNT }> =
    EmbeddedFiles!([("/", [("dummy.txt", WasiEmbeddedFile::new("dummy"))])]);

mod fs {
    use super::*;

    type LFS = StandardEmbeddedNormalLFS<
        EmbeddedFilesTy,
        WasiEmbeddedFile<&'static str>,
        FILE_COUNT,
        DefaultStdIO,
    >;

    static VIRTUAL_FILE_SYSTEM: StandardEmbeddedFileSystem<LFS, FILE_COUNT> =
        StandardEmbeddedFileSystem::new_const(StandardEmbeddedNormalLFS::new_const());

    plug_fs!(&VIRTUAL_FILE_SYSTEM, test_lib);
}
