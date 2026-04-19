use parking_lot::Mutex;
use std::sync::LazyLock;
use wasi_virt_layer::{file::*, plug_process, prelude::*, process::DefaultProcess};
use wasi_virt_layer::file::multiple::*;
use wasi_virt_layer::file::multiple::inode::BoxedInodeNormal;

wit_bindgen::generate!({
    world: "hello",
});

struct Hello;

import_wasm!(ls);

impl Guest for Hello {
    fn world() {
        println!("Hello, multiple VFS world!");
    }
    fn add_env(env: String) {
        let mut state = VIRTUAL_ENV.lock();
        state.environ.push(env.clone());
        println!("Adding env: {}", env);
    }
    fn get_envs() -> Vec<String> {
        VIRTUAL_ENV.lock().get_environ().to_vec()
    }
    fn main() {
        ls::_reset();
        ls::_start();

        // print filesystem entries
        println!("Listing root directory (multiple VFS):");
        println!("{:?}", &*fs::VIRTUAL_FILE_SYSTEM);

        ls::_main();
    }
}

export!(Hello);

plug_process!(DefaultProcess, ls, self);

struct VirtualEnvState {
    environ: Vec<String>,
}

impl<'a> VirtualEnv<'a> for VirtualEnvState {
    type Str = String;

    fn get_environ(&mut self) -> &[Self::Str] {
        &self.environ
    }
}

static VIRTUAL_ENV: LazyLock<Mutex<VirtualEnvState>> = LazyLock::new(|| {
    let mut environ = Vec::<String>::new();
    environ.push("HOME=~/".into());
    environ.push("RUST_BACKTRACE=1".into());
    Mutex::new(VirtualEnvState { environ })
});

plug_env!(@dynamic, &mut VIRTUAL_ENV.lock(), ls);

#[allow(dead_code)]
mod fs {
    use super::*;
    pub static VIRTUAL_FILE_SYSTEM: LazyLock<Wasip1MultipleVFS> = LazyLock::new(|| {
        let mut vfs = Wasip1MultipleVFS::new();

        vfs.add_wasm::<ls>();

        // Create first LFS
        let lfs1 = ChangeableLFS::<DefaultStdIO>::new();
        let root_inode1 = lfs1.add_preopen(".");
        lfs1.add_file(root_inode1, "lfs1.txt", b"Content from LFS 1".to_vec()).unwrap();

        vfs.add_lfs(Box::new(lfs1));
        vfs.add_preopen_fd(0, BoxedInodeNormal::from_inode(root_inode1));

        // Create second LFS
        let lfs2 = ChangeableLFS::<DefaultStdIO>::new();
        let root_inode2 = lfs2.add_preopen("/data");
        lfs2.add_file(root_inode2, "lfs2.txt", b"Content from LFS 2".to_vec()).unwrap();

        vfs.add_lfs(Box::new(lfs2));
        vfs.add_preopen_fd(1, BoxedInodeNormal::from_inode(root_inode2));

        vfs
    });

    plug_fs!(&*VIRTUAL_FILE_SYSTEM, ls);
}
