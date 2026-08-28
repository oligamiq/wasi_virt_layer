use const_struct::const_struct;
use wasi_virt_layer::{file::*, plug_process, prelude::*};

wit_bindgen::generate!({
    world: "init",
});

struct Starter;
impl Guest for Starter {
    fn start() {
        example::_start();
    }
}
export!(Starter);
import_wasm!(example);

mod fs {
    use super::*;
    use std::sync::LazyLock;
    use wasi_virt_layer::file::multiple::*;
    pub static VIRTUAL_FILE_SYSTEM: LazyLock<StandardMultipleFileSystem> = LazyLock::new(|| {
        let mut vfs = StandardMultipleFileSystem::new();
        vfs.add_wasm::<example>();
        let lfs = StandardDynamicLFS::<DefaultStdIO>::new();
        vfs.add_lfs(lfs);
        vfs
    });
    plug_fs!(&*VIRTUAL_FILE_SYSTEM, example, self);
}

#[const_struct]
const VIRTUAL_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState { environ: &[] };

plug_env!(@embedded, VirtualEnvTy, example, self);
plug_process!(wasi_virt_layer::process::StandardProcess, example, self);
