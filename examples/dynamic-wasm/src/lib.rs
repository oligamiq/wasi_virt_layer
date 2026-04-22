use parking_lot::Mutex;
use std::sync::LazyLock;
use wasi_virt_layer::export_pseudo_wasm;
use wasi_virt_layer::file::multiple::inode::BoxedInodeNormal;
use wasi_virt_layer::file::multiple::*;
use wasi_virt_layer::{file::*, plug_process, prelude::*, process::DefaultProcess};

wit_bindgen::generate!({
    world: "hello",
});

struct Hello;

import_wasm!(ls);

static WASM_HOLDER: StandardPseudoWasmHolder = StandardPseudoWasmHolder::new_const();

export_pseudo_wasm!(wasm_holder);

impl Guest for Hello {
    fn world() {
        println!("Hello, dynamic wasm world!");
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

#[cfg(not(test))]
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
    pub static VIRTUAL_FILE_SYSTEM: LazyLock<StandardMultipleFileSystem<BoxedInodeNormal>> = LazyLock::new(|| {
        let mut vfs = StandardMultipleFileSystem::<BoxedInodeNormal>::new();

        let wasm = WASM_HOLDER.restore(());
        vfs.add_wasm_access(wasm);

        let lfs1 = StandardDynamicLFS::<DefaultStdIO>::new();
        let root_inode1 = lfs1.add_preopen(".");
        lfs1.add_file(root_inode1, "hello.txt", b"Hello, Dynamic WASM!".to_vec())
            .unwrap();

        vfs.add_lfs(lfs1);
        vfs.add_preopen_fd(0, BoxedInodeNormal::from_inode(root_inode1));

        vfs
    });

    plug_fs!(&*VIRTUAL_FILE_SYSTEM, ls);
}


