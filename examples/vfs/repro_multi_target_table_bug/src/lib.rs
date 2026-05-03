use const_struct::const_struct;
use parking_lot::Mutex;
use std::sync::LazyLock;
use wasi_virt_layer::file::multiple::inode::BoxedInodeNormal;
use wasi_virt_layer::file::multiple::*;
use wasi_virt_layer::{file::*, plug_process, prelude::*, process::StandardProcess};

wit_bindgen::generate!({
    world: "hello",
});

struct Hello;

import_wasm!(test_threads);
import_wasm!(ls);
import_wasm!(args);
import_wasm!(ls2);

impl Guest for Hello {
    fn world() {
        println!("Hello from repro_multi_target_table_bug!");
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
        println!("Running test_threads...");
        test_threads::_reset();
        test_threads::_start();
        test_threads::_main();

        println!("Running ls...");
        ls::_reset();
        ls::_start();
        ls::_main();

        println!("Running args...");
        args::_reset();
        args::_start();
        args::_main();

        println!("Running ls2...");
        ls2::_reset();
        ls2::_start();
        ls2::_main();
    }
}

#[cfg(not(test))]
export!(Hello);

plug_thread!(
    { wasi_virt_layer::thread::DirectThreadPool::<ThreadAccessor>::new_const() },
    self,
    test_threads,
    ls,
    args,
    ls2
);

plug_process!(StandardProcess, test_threads, ls, args, ls2, self);

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

plug_env!(@dynamic, &mut VIRTUAL_ENV.lock(), test_threads, ls, args, ls2);

#[const_struct]
const VIRTUAL_ARGS: VirtualArgsEmbeddedState = VirtualArgsEmbeddedState { args: &["repro"] };
plug_args!(@embedded, VirtualArgsTy, test_threads, ls, args, ls2, self);

plug_clock!(StandardClock, test_threads, ls, args, ls2, self);
plug_random!(StandardRandom, test_threads, ls, args, ls2, self);
plug_poll!(DefaultPoll, test_threads, ls, args, ls2, self);

#[allow(dead_code)]
mod fs {
    use super::*;
    pub static VIRTUAL_FILE_SYSTEM: LazyLock<StandardMultipleFileSystem> = LazyLock::new(|| {
        let mut vfs = StandardMultipleFileSystem::new();

        vfs.add_wasm::<test_threads>();
        vfs.add_wasm::<ls>();
        vfs.add_wasm::<args>();
        vfs.add_wasm::<ls2>();

        let lfs = StandardDynamicLFS::<DefaultStdIO>::new();
        let root_inode = lfs.add_preopen(".");
        vfs.add_lfs(lfs);
        vfs.add_preopen_fd(0, BoxedInodeNormal::from_inode(root_inode));

        vfs
    });

    plug_fs!(&*VIRTUAL_FILE_SYSTEM, test_threads, ls, args, ls2);
}
