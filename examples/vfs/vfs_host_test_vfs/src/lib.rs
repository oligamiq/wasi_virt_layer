use const_struct::const_struct;
use wasi_virt_layer::prelude::*;
use wasi_virt_layer::file::*;
use std::sync::LazyLock;
use parking_lot::Mutex;

// We import the target wasm
import_wasm!(vfs_host_test_target);

#[link(wasm_import_module = "wasip1_vfs_vfs_host_test_target")]
unsafe extern "C" {
    fn test_user_func();
}

#[unsafe(no_mangle)]
pub extern "C" fn importing_test_vfs_func() {
    println!("importing_test_vfs_func called (host side)");
}

#[const_struct]
const EMBEDDED_FILES: StandardEmbeddedFiles<WasiEmbeddedFile<&'static str>, 2> =
    EmbeddedFiles!([(".", [("dummy.txt", WasiEmbeddedFile::new("dummy"))])]);

type LFS =
    StandardEmbeddedNormalLFS<EmbeddedFilesTy, WasiEmbeddedFile<&'static str>, 2, DefaultStdIO>;

static VIRTUAL_FILE_SYSTEM: StandardEmbeddedFileSystem<LFS, 2> =
    StandardEmbeddedFileSystem::new_const(StandardEmbeddedNormalLFS::new_const());

plug_fs!(&VIRTUAL_FILE_SYSTEM, vfs_host_test_target, self);

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

plug_env!(@dynamic, &mut VIRTUAL_ENV.lock(), vfs_host_test_target);

plug_process!(vfs_host_test_target);

#[unsafe(no_mangle)]
fn main() {
    println!("Hello from the host!");

    // We call the user function that should be redirected to the target
    unsafe { test_user_func() };

    vfs_host_test_target::_reset();
    vfs_host_test_target::_start();
    vfs_host_test_target::_main();
}
