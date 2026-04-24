use std::sync::LazyLock;

use wasi_virt_layer::{file::*, plug_process, prelude::*, process::DefaultProcess};

wit_bindgen::generate!({
    world: "init",
});

struct Starter;

import_wasm!(self_rw_example);

impl Guest for Starter {
    fn init() {
        // VFS writes initial log file from a spawned thread
        let handle = std::thread::spawn(|| {
            std::fs::write("./init.log", "VFS initialized from thread").unwrap();
            println!("VFS init thread: wrote init.log");
        });

        // VFS reads initial files from the main thread
        let content = std::fs::read_to_string("./readme.txt").unwrap();
        println!("VFS init main: readme.txt = {content}");

        handle.join().unwrap();
        println!("VFS init complete.");
    }

    fn start() {
        self_rw_example::_start();
    }

    fn main() {
        self_rw_example::_reset();
        self_rw_example::_start();

        // VFS writes a file before the target runs
        std::fs::write("./pre-run.txt", "Written by VFS before target").unwrap();
        println!("VFS main: wrote pre-run.txt");

        // Run the target executable
        println!("--- Running target executable ---");
        self_rw_example::_main();
        println!("--- Target executable finished ---");

        // VFS reads back what the target wrote, from a new thread
        let handle = std::thread::spawn(|| {
            match std::fs::read_to_string("./target-was-here.txt") {
                Ok(content) => println!("VFS thread: target wrote: {content}"),
                Err(e) => println!("VFS thread: could not read target file: {e}"),
            }

            // List all files to show the combined result
            println!("VFS thread: final directory listing:");
            for entry in std::fs::read_dir(".").unwrap() {
                let entry = entry.unwrap();
                println!("  {}", entry.path().display());
            }
        });

        handle.join().unwrap();
    }
}

#[cfg(not(test))]
export!(Starter);

mod env {
    use super::*;
    use const_struct::const_struct;

    #[const_struct]
    const VIRTUAL_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
        environ: &["HOME=~/"],
    };

    plug_env!(@embedded, VirtualEnvTy, self_rw_example, self);
}

plug_thread!(
    { wasi_virt_layer::thread::DirectThreadPool::<ThreadAccessor>::new_const() },
    self,
    self_rw_example
);

plug_process!(DefaultProcess, self_rw_example, self);

mod fs {
    use super::*;

    type LFS = StandardDynamicLFS<DefaultStdIO>;

    pub static VIRTUAL_FILE_SYSTEM: LazyLock<StandardDynamicFileSystem<LFS>> =
        LazyLock::new(|| {
            let lfs = StandardDynamicLFS::new();

            let root_inode = lfs.add_preopen(".");

            lfs.add_file(
                root_inode,
                "readme.txt",
                b"Hello from the virtual filesystem!".to_vec(),
            )
            .unwrap();

            lfs.add_file(root_inode, "data.txt", b"Some initial data".to_vec())
                .unwrap();

            let vfs = StandardDynamicFileSystem::new(lfs);
            vfs.add_fd(root_inode, !0, !0);
            vfs
        });

    plug_fs!(&*VIRTUAL_FILE_SYSTEM, self_rw_example, self);
}
