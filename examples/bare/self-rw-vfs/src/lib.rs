use std::sync::LazyLock;

use wasi_virt_layer::{file::*, plug_process, prelude::*, process::DefaultProcess};

wit_bindgen::generate!({
    world: "component-abi",
});

struct ComponentABI;

import_wasm!(<anonymous>);

impl Guest for ComponentABI {
    fn main() {
        // The VFS itself reads from the virtual filesystem
        let content = std::fs::read_to_string("./readme.txt").unwrap();
        println!("VFS read readme.txt: {content}");

        // The VFS writes a new file into the virtual filesystem
        let output = format!("Processed: {content} (length={})", content.len());
        std::fs::write("./output.txt", &output).unwrap();
        println!("VFS wrote output.txt");

        // The VFS writes another file to show multiple mutations
        std::fs::write("./vfs.log", "VFS was here").unwrap();
        println!("VFS wrote vfs.log");

        // List directory to show both original and new files
        println!("VFS listing directory:");
        for entry in std::fs::read_dir(".").unwrap() {
            let entry = entry.unwrap();
            println!("  {}", entry.path().display());
        }

        // Now run the target executable (ls), which will also see
        // the files the VFS just created
        println!("--- Running target executable ---");
        anonymous::_reset();
        anonymous::_start();
        anonymous::_main();
    }
}

#[cfg(not(test))]
export!(ComponentABI);

plug_process!(DefaultProcess, anonymous, self);

mod env {
    use super::*;
    use const_struct::const_struct;

    #[const_struct]
    const HOST_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
        environ: &["HOME=~/"],
    };

    plug_env!(@embedded, HostEnvTy, anonymous, self);
}

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

    plug_fs!(&*VIRTUAL_FILE_SYSTEM, anonymous, self);
}
