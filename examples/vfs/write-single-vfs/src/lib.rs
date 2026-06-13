use std::sync::LazyLock;
use wasi_virt_layer::{file::*, plug_process, prelude::*, process::StandardProcess};

wit_bindgen::generate!({
    world: "init",
});

struct Starter;

import_wasm!(write_single_target);

impl Guest for Starter {
    fn init() {}

    fn start() {
        write_single_target::_start();
    }

    fn main() {
        write_single_target::_start();
        write_single_target::_main();
    }
}

#[cfg(not(test))]
export!(Starter);

plug_thread!(
    { wasi_virt_layer::thread::DirectThreadPool::<ThreadAccessor>::new_const() },
    self,
    write_single_target
);

plug_process!(StandardProcess, write_single_target, self);

mod env {
    use super::*;
    use const_struct::const_struct;

    #[const_struct]
    const VIRTUAL_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
        environ: &["HOME=~/"],
    };

    plug_env!(@embedded, VirtualEnvTy, write_single_target, self);
}

mod fs {
    use super::*;

    type LFS = StandardDynamicLFS<DefaultStdIO>;

    pub static VIRTUAL_FILE_SYSTEM: LazyLock<StandardDynamicFileSystem<LFS>> =
        LazyLock::new(|| {
            let lfs = StandardDynamicLFS::new();
            let root_inode = lfs.add_preopen(".");
            let vfs = StandardDynamicFileSystem::new(lfs);
            vfs.add_fd(root_inode, !0, !0);
            vfs
        });

    plug_fs!(&*VIRTUAL_FILE_SYSTEM, write_single_target, self);
}
