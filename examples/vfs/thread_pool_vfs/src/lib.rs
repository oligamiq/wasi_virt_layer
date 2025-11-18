use const_struct::const_struct;
use wasi_virt_layer::{
    file::{VFSConstNormalFiles, WasiConstFile},
    prelude::*,
};

wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "init",
});

struct Starter;

impl Guest for Starter {
    fn main() {
        test_pool_thread::_reset();
        test_pool_thread::_start();
        test_pool_thread::_main();
    }

    fn init(pool_size: u32) {
        let pool = &raw mut THREAD_POOL;
        let pool = unsafe { &mut *pool };
        pool.init();
        pool.set_capacity(pool_size as usize);
        pool.flush_capacity().wait();
    }
}

export!(Starter);

import_wasm!(test_pool_thread);

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

plug_process!(test_pool_thread);

#[const_struct]
const ENV: VirtualEnvConstState = VirtualEnvConstState {
    environ: &["HOME=~/", "RUST_BACKTRACE=1"],
};

plug_env!(@const, EnvTy, test_pool_thread);

use wasi_virt_layer::thread::VirtualThreadPool;

static mut THREAD_POOL: VirtualThreadPool<ThreadAccessor> =
    unsafe { VirtualThreadPool::const_new(4) };

plug_thread!(
    { unsafe { &mut *(&raw mut THREAD_POOL) } },
    test_pool_thread,
    self
);

mod fs {
    use wasi_virt_layer::file::{DefaultStdIO, VFSConstNormalLFS, Wasip1ConstVFS};

    use super::*;

    type LFS = VFSConstNormalLFS<FilesTy, WasiConstFile<&'static str>, FILE_COUNT, DefaultStdIO>;

    static mut VIRTUAL_FILE_SYSTEM: Wasip1ConstVFS<LFS, FILE_COUNT> =
        Wasip1ConstVFS::new(VFSConstNormalLFS::new());

    plug_fs!(@const, {
        #[allow(static_mut_refs)]
        unsafe { &mut VIRTUAL_FILE_SYSTEM }
    }, test_pool_thread);
}
