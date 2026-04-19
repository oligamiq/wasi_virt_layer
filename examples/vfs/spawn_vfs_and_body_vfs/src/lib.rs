use const_struct::const_struct;

use wasi_virt_layer::{file::*, plug_process, prelude::*};

wit_bindgen::generate!({
    world: "init",
});

struct Starter;

impl Guest for Starter {
    fn init() {
        fn print_loop() {
            for i in 0..10 {
                println!("Hello from VFS init thread! {i}");
            }
        }

        let handle = std::thread::spawn(|| {
            print_loop();
        });

        print_loop();

        handle.join().unwrap();

        println!("`init` done in VFS.");
    }

    fn start() {
        spawn_vfs_and_body_example::_start();
    }

    fn main() {
        spawn_vfs_and_body_example::_reset();
        spawn_vfs_and_body_example::_start();
        spawn_vfs_and_body_example::_main();

        std::thread::spawn(|| {
            for i in 0..10 {
                println!("Hello from body main thread! {i}");
            }
        })
        .join()
        .unwrap();
    }
}

export!(Starter);

import_wasm!(spawn_vfs_and_body_example);

const FILE_COUNT: usize = 2;

type F = WasiConstFile<&'static str>;
type NormalFILES = VFSConstNormalFiles<F, { FILE_COUNT }>;

#[const_struct]
const FILES: NormalFILES = ConstFiles!([(".", [("hey", F::new("Hey!"))])]);

mod fs {
    use super::*;

    type LFS = VFSConstNormalLFS<FilesTy, F, FILE_COUNT, DefaultStdIO>;

    static VIRTUAL_FILE_SYSTEM: Wasip1ConstVFS<LFS, FILE_COUNT> =
        Wasip1ConstVFS::const_new(VFSConstNormalLFS::const_new());

    plug_fs!(&VIRTUAL_FILE_SYSTEM, spawn_vfs_and_body_example, self);
}

#[const_struct]
const VIRTUAL_ENV: VirtualEnvConstState = VirtualEnvConstState {
    environ: &["HOME=~/"],
};

plug_env!(@const, VirtualEnvTy, spawn_vfs_and_body_example, self);

plug_thread!(
    { wasi_virt_layer::thread::DirectThreadPool::<ThreadAccessor>::const_new() },
    self,
    spawn_vfs_and_body_example
);

plug_process!(
    wasi_virt_layer::process::DefaultProcess,
    spawn_vfs_and_body_example,
    self
);
