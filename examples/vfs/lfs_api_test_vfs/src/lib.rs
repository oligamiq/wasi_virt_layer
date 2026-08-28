use const_struct::const_struct;
use std::sync::LazyLock;
use wasi_virt_layer::{file::*, plug_process, prelude::*, process::StandardProcess};

wit_bindgen::generate!({
    world: "hello",
});

struct Hello;

import_wasm!(test_wasm);

impl Guest for Hello {
    fn world() {
        test_wasm::_reset();
        test_wasm::_start();
        test_wasm::_main();
    }
    fn add_env(_: String) {}
    fn get_envs() -> Vec<String> {
        vec![]
    }
    fn main() {
        test_wasm::_reset();
        test_wasm::_start();
        test_wasm::_main();
    }
}

#[cfg(not(test))]
export!(Hello);

plug_process!(StandardProcess, test_wasm, self);
plug_clock!(StandardClock, test_wasm, self);
plug_random!(StandardRandom, test_wasm, self);
plug_poll!(DefaultPoll, test_wasm, self);
plug_sched!(DefaultSched, test_wasm, self);

#[const_struct]
const HOST_ARGS: VirtualArgsEmbeddedState = VirtualArgsEmbeddedState { args: &["test"] };
plug_args!(@embedded, HostArgsTy, test_wasm, self);

#[const_struct]
const HOST_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
    environ: &["HOME=~/"],
};

plug_env!(@embedded, HostEnvTy, test_wasm, self);

mod fs {
    use super::*;

    type LFS = StandardDynamicLFS<DefaultStdIO>;

    pub static VIRTUAL_FILE_SYSTEM: LazyLock<StandardDynamicFileSystem<LFS>> =
        LazyLock::new(|| {
            let lfs = StandardDynamicLFS::new();
            let root = lfs.add_preopen(".");

            // 1. add_file & write_file & read_file
            let f1 = lfs.add_file(root, "init.txt", b"Initial".to_vec()).unwrap();
            lfs.write_file(f1, b"Updated content".to_vec()).unwrap();
            let content = lfs.read_file(f1).unwrap();
            assert_eq!(content, b"Updated content");

            // 2. add_dir & read_dir
            let test_dir = lfs.add_dir(root, "test_dir").unwrap();
            let dir_entries = lfs.read_dir(root).unwrap();
            assert!(dir_entries.iter().any(|(name, _)| name == "test_dir"));

            // 3. rename (moving file into dir)
            let f2 = lfs
                .add_file(root, "to_move.txt", b"Move me".to_vec())
                .unwrap();
            lfs.rename(root, "to_move.txt", test_dir, "moved.txt")
                .unwrap();

            // 4. get_inode_by_path_str
            let resolved = lfs
                .get_inode_by_path_str(root, "test_dir/moved.txt")
                .unwrap();
            assert_eq!(resolved, f2);

            // 5. add_symlink
            lfs.add_symlink(root, "link.txt", "test_dir/moved.txt")
                .unwrap();

            // 6. remove_file
            let f3 = lfs
                .add_file(root, "remove_me.txt", b"temp".to_vec())
                .unwrap();
            lfs.remove_file(root, "remove_me.txt").unwrap();
            assert!(lfs.read_file(f3).is_err());

            // 7. metadata & set_metadata
            let meta = lfs.metadata(test_dir).unwrap();
            lfs.set_metadata(test_dir, meta).unwrap();

            // 8. remove_dir
            let _empty_dir = lfs.add_dir(root, "empty_dir").unwrap();
            lfs.remove_dir(root, "empty_dir").unwrap();

            let vfs = StandardDynamicFileSystem::new(lfs);
            vfs.add_fd(root, !0, !0);
            vfs
        });

    plug_fs!(&*VIRTUAL_FILE_SYSTEM, test_wasm, self);
}
