use parking_lot::Mutex;
use std::sync::LazyLock;
use wasi_virt_layer::{file::*, plug_process, prelude::*, process::StandardProcess};

wit_bindgen::generate!({
    world: "hello",
});

struct Hello;

import_wasm!(test_args);

impl Guest for Hello {
    fn world() {
        println!("Hello from VFS!");
    }

    fn add_env(env: String) {
        let mut state = VIRTUAL_ENV.lock();
        state.environ.push(env);
    }

    fn get_envs() -> Vec<String> {
        VIRTUAL_ENV.lock().get_environ().to_vec()
    }

    fn main() {
        // We can dynamically change args before starting the wasm
        {
            let mut state = VIRTUAL_ARGS.lock();
            state.args.push("dynamic_arg_1".into());
            state.args.push("dynamic_arg_2".into());
        }

        test_args::_reset();
        test_args::_start();
        test_args::_main();
    }
}

#[cfg(not(test))]
export!(Hello);

plug_process!(StandardProcess, test_args, self);
plug_random!(StandardRandom, test_args, self);
plug_sched!(DefaultSched, test_args, self);

// Environment virtualization
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
    environ.push("VIRTUAL_ENV_VAR=fixed_value".into());
    Mutex::new(VirtualEnvState { environ })
});

plug_env!(@dynamic, &mut VIRTUAL_ENV.lock(), test_args);

// Arguments virtualization
struct VirtualArgsState {
    args: Vec<String>,
}

impl<'a> VirtualArgs<'a> for VirtualArgsState {
    type Str = String;

    fn get_args(&mut self) -> &[Self::Str] {
        &self.args
    }
}

static VIRTUAL_ARGS: LazyLock<Mutex<VirtualArgsState>> = LazyLock::new(|| {
    let mut args = Vec::<String>::new();
    args.push("virtual_executable_name".into());
    Mutex::new(VirtualArgsState { args })
});

plug_args!(@dynamic, &mut VIRTUAL_ARGS.lock(), test_args);

const FILE_COUNT: usize = 2;
const FD_COUNT: usize = 10;

#[const_struct::const_struct]
const EMBEDDED_FILES: StandardEmbeddedFiles<WasiEmbeddedFile<&'static str>, { FILE_COUNT }> =
    EmbeddedFiles!([(".", [("dummy.txt", WasiEmbeddedFile::new("dummy"))])]);

mod fs {
    use super::*;

    type LFS = StandardEmbeddedNormalLFS<
        EmbeddedFilesTy,
        WasiEmbeddedFile<&'static str>,
        { FILE_COUNT },
        DefaultStdIO,
    >;

    static VIRTUAL_FILE_SYSTEM: StandardEmbeddedFileSystem<LFS, { FD_COUNT }> =
        StandardEmbeddedFileSystem::new_const(StandardEmbeddedNormalLFS::new_const());

    plug_fs!(&VIRTUAL_FILE_SYSTEM, test_args);
}
