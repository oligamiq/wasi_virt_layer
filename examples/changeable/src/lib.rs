use parking_lot::Mutex;
use std::sync::LazyLock;
use wasi_virt_layer::{file::*, plug_process, prelude::*, process::DefaultProcess};

wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "hello",
});

struct Hello;

import_wasm!(ls);

impl Guest for Hello {
    fn world() {
        println!("Hello, changeable world!");
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
        println!("Listing root directory:");
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
    use std::cell::{LazyCell, OnceCell};

    use super::*;

    type LFS = ChangeableLFS<DefaultStdIO>;

    pub static VIRTUAL_FILE_SYSTEM: LazyLock<ChangeableVFS<LFS>> = LazyLock::new(|| {
        let lfs = ChangeableLFS::new(); // Inode 0 is root "."

        // Add a preopen entry for root "."
        let root_inode = lfs.add_preopen(".");

        let vfs = ChangeableVFS::new(lfs);

        // Create a dynamic directory and file
        let docs_inode = vfs.add_dir(root_inode, "docs").unwrap();
        vfs.add_file(docs_inode, "readme.txt", b"Hello World!".to_vec())
            .unwrap();

        let sub_inode = vfs.add_dir(docs_inode, "sub").unwrap();
        vfs.add_file(sub_inode, "hello.txt", b"Sub directory!".to_vec())
            .unwrap();

        // Wrap in VFS
        vfs
    });

    plug_fs!(&*VIRTUAL_FILE_SYSTEM, ls);
}
