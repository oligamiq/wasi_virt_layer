use parking_lot::Mutex;
use std::sync::LazyLock;
use wasi_virt_layer::{file::*, plug_process, prelude::*, process::StandardProcess};

wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "hello",
});

struct Hello;

// Import the target WASM. We'll name it test_mkdir.
import_wasm!(test_mkdir);

impl Guest for Hello {
    fn world() {
        println!("Mkdir VFS world initialized.");
    }
    fn add_env(env: String) {
        let mut state = VIRTUAL_ENV.lock();
        state.environ.push(env.clone());
    }
    fn get_envs() -> Vec<String> {
        VIRTUAL_ENV.lock().get_environ().to_vec()
    }
    fn main() {
        test_mkdir::_reset();
        test_mkdir::_start();
        test_mkdir::_main();
    }
}

#[cfg(not(test))]
export!(Hello);

plug_process!(StandardProcess, test_mkdir, self);
plug_random!(StandardRandom, test_mkdir, self);
plug_sched!(DefaultSched, test_mkdir, self);

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

plug_env!(@dynamic, &mut VIRTUAL_ENV.lock(), test_mkdir);

mod fs {
    use super::*;

    type LFS = StandardDynamicLFS<DefaultStdIO>;

    pub static VIRTUAL_FILE_SYSTEM: LazyLock<StandardDynamicFileSystem<LFS>> =
        LazyLock::new(|| {
            let lfs = StandardDynamicLFS::new(); // Inode 0 is root "."

            // Add a preopen entry for root "."
            let root_inode = lfs.add_preopen(".");

            let vfs = StandardDynamicFileSystem::new(lfs);

            // Add preopen fd for root
            vfs.add_fd(root_inode, !0, !0);

            // Wrap in VFS
            vfs
        });

    plug_fs!(&*VIRTUAL_FILE_SYSTEM, test_mkdir);
}
