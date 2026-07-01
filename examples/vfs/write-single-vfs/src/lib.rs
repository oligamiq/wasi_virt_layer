use std::sync::LazyLock;
use wasi_virt_layer::{file::*, plug_process, prelude::*, process::StandardProcess};

wit_bindgen::generate!({
    world: "init",
});

struct Starter;

import_wasm!(test_write_single);

impl Guest for Starter {
    fn init() {}

    fn start() {
        test_write_single::_start();
    }

    fn main() {
        test_write_single::_start();
        test_write_single::_main();
    }
}

#[cfg(not(test))]
export!(Starter);

plug_thread!(
    { wasi_virt_layer::thread::DirectThreadPool::<ThreadAccessor>::new_const() },
    self,
    test_write_single
);

plug_clock!(StandardClock, test_write_single, self);
plug_process!(StandardProcess, test_write_single, self);

mod env {
    use super::*;
    use const_struct::const_struct;

    #[const_struct]
    const VIRTUAL_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
        environ: &["HOME=~/"],
    };

    plug_env!(@embedded, VirtualEnvTy, test_write_single, self);
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

    plug_fs!(&*VIRTUAL_FILE_SYSTEM, test_write_single, self);
}
