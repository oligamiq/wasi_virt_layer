use const_struct::const_struct;
use parking_lot::Mutex;
use std::sync::LazyLock;
use wasi_virt_layer::file::multiple::*;
use wasi_virt_layer::thread::*;
use wasi_virt_layer::{file::*, plug_process, prelude::*, process::StandardProcess};

wit_bindgen::generate!({
    world: "hello",
});

struct Hello;

import_wasm!(test_threads);
import_wasm!(ls);

wrap_unreachable!(
    wasi_virt_layer::wasi::wrap_unreachable::StandardWrapUnreachable,
    test_threads,
    ls
);

impl Guest for Hello {
    fn world() {
        // println!("--- Starting ls in world() ---");
        // ls::_start();
        // ls::_main();
    }
    fn add_env(_: String) {}
    fn get_envs() -> Vec<String> {
        vec![]
    }
    fn main() {
        use wasi_virt_layer::__private::utils::InitOnce;
        static INIT: InitOnce = InitOnce::new_const();
        INIT.call_once(|| {
            unsafe { THREAD_POOL.init_with_capacity_and_wait(4) };
        });

        println!("--- Starting test_threads ---");
        test_threads::_start();
        test_threads::_main();
    }
}

#[cfg(not(test))]
export!(Hello);

static THREAD_POOL: VirtualThreadPool<ThreadAccessor> = unsafe { VirtualThreadPool::new_const(4) };

plug_thread!({ &THREAD_POOL }, self, test_threads);
plug_sched!(DefaultSched, ls, self);

plug_process!(StandardProcess, test_threads, ls, self);

struct VirtualEnvState {
    environ: Vec<String>,
}

impl<'a> VirtualEnv<'a> for VirtualEnvState {
    type Str = String;
    fn get_environ(&mut self) -> &[Self::Str] {
        &self.environ
    }
}

static VIRTUAL_ENV: LazyLock<Mutex<VirtualEnvState>> =
    LazyLock::new(|| Mutex::new(VirtualEnvState { environ: vec![] }));
plug_env!(@dynamic, &mut VIRTUAL_ENV.lock(), test_threads, ls);

#[const_struct]
const VIRTUAL_ARGS: VirtualArgsEmbeddedState = VirtualArgsEmbeddedState { args: &["repro"] };
plug_args!(@embedded, VirtualArgsTy, test_threads, ls, self);

plug_clock!(StandardClock, test_threads, ls, self);
plug_random!(StandardRandom, test_threads, ls, self);
plug_poll!(DefaultPoll, test_threads, ls, self);

mod fs {
    use super::*;
    pub static VIRTUAL_FILE_SYSTEM: LazyLock<StandardMultipleFileSystem> = LazyLock::new(|| {
        let mut vfs = StandardMultipleFileSystem::new();
        vfs.add_wasm::<test_threads>();
        vfs.add_wasm::<ls>();
        let lfs = StandardDynamicLFS::<DefaultStdIO>::new();
        vfs.add_lfs(lfs);
        vfs
    });
    plug_fs!(&*VIRTUAL_FILE_SYSTEM, test_threads, ls);
}
