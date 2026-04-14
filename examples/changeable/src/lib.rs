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

plug_env!(@static, &mut VIRTUAL_ENV.lock(), ls);

#[allow(dead_code)]
mod fs {
    use std::cell::{LazyCell, OnceCell};

    use super::*;

    type LFS = ChangeableLFS<DefaultStdIO>;

    static VIRTUAL_FILE_SYSTEM: LazyCell<ChangeableVFS<LFS>> = LazyCell::new(|| {
        let mut lfs = ChangeableLFS::new(); // Inode 0 is root "."

        // Create a dynamic directory and file
        let docs_inode = lfs.add_dir(0, "docs").unwrap();
        lfs.add_file(docs_inode, "readme.txt", b"Hello World!".to_vec())
            .unwrap();

        let sub_inode = lfs.add_dir(docs_inode, "sub").unwrap();
        lfs.add_file(sub_inode, "hello.txt", b"Sub directory!".to_vec())
            .unwrap();

        // Add a preopen entry for root "."
        lfs.add_preopen(0, ".");

        // Wrap in VFS
        ChangeableVFS::new(lfs, [0])
    });

    plug_fs!(&*VIRTUAL_FILE_SYSTEM, ls);
}
