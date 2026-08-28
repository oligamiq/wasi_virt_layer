use const_struct::const_struct;
use parking_lot::Mutex;
use std::sync::LazyLock;
use wasi_virt_layer::{file::*, plug_process, prelude::*, process::StandardProcess};

wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "hello",
});

struct Hello;

import_wasm!(test_unwind_target);

impl Guest for Hello {
    fn world() {
        println!("Hello, world!");
    }
    fn add_env(env: String) {}
    fn get_envs() -> Vec<String> {
        Vec::new()
    }
    fn main() {
        // There is no _reset or _start or _main in std lib unless we are wrapping a specific ABI.
        // Actually we just run the default command from WASI.
        // wait, usually an executable target has `_start`
        test_unwind_target::_start();
    }
}

#[cfg(not(test))]
export!(Hello);

plug_process!(StandardProcess, test_unwind_target, self);
plug_random!(StandardRandom, test_unwind_target, self);
plug_sched!(DefaultSched, test_unwind_target, self);

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
    Mutex::new(VirtualEnvState { environ })
});

plug_env!(@dynamic, &mut VIRTUAL_ENV.lock(), test_unwind_target);

const FILE_COUNT: usize = 2;

#[const_struct]
const EMBEDDED_FILES: StandardEmbeddedFiles<WasiEmbeddedFile<&'static str>, { FILE_COUNT }> =
    EmbeddedFiles!([(".", [("dummy", WasiEmbeddedFile::new("dummy"))])]);

mod fs {
    use super::*;

    type LFS = StandardEmbeddedNormalLFS<
        EmbeddedFilesTy,
        WasiEmbeddedFile<&'static str>,
        FILE_COUNT,
        DefaultStdIO,
    >;

    static VIRTUAL_FILE_SYSTEM: StandardEmbeddedFileSystem<LFS, FILE_COUNT> =
        StandardEmbeddedFileSystem::new_const(StandardEmbeddedNormalLFS::new_const());

    plug_fs!(&VIRTUAL_FILE_SYSTEM, test_unwind_target);
}
