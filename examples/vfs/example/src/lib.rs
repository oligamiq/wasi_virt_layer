use const_struct::const_struct;
use parking_lot::Mutex;
use std::sync::LazyLock;
use wasi_virt_layer::{file::*, plug_process, prelude::*, process::DefaultProcess};

wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "hello",
});

struct Hello;

import_wasm!(test_wasm);

impl Guest for Hello {
    fn world() {
        println!("Hello, world!");

        type F = WasiEmbeddedFile<&'static str>;
        const FILES2: StandardEmbeddedFiles<F, 5> = EmbeddedFiles!([(
            ".",
            [
                ("hey", F::new("Hey!")),
                (
                    "hello",
                    [
                        ("world", F::new("Hello, world!")),
                        ("everyone", F::new("Hello, everyone!")),
                    ],
                ),
            ],
        )]);

        println!("Files: {FILES2:?}");
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
        test_wasm::_reset();
        test_wasm::_start();
        test_wasm::_main();
    }
}

#[cfg(not(test))]
export!(Hello);

plug_process!(DefaultProcess, test_wasm, self);

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
    // environ.push("RUST_MIN_STACK=16777216".into());
    environ.push("HOME=~/".into());
    environ.push("RUST_BACKTRACE=1".into());
    Mutex::new(VirtualEnvState { environ })
});

plug_env!(@dynamic, &mut VIRTUAL_ENV.lock(), test_wasm);

#[const_struct]
const HOST_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
    environ: &[
        // "RUST_MIN_STACK=16777216",
        "HOME=~/",
        "RUST_BACKTRACE=1",
    ],
};

plug_env!(@embedded, HostEnvTy, self);

const FILE_COUNT: usize = 10;

#[const_struct]
const EMBEDDED_FILES: StandardEmbeddedFiles<WasiEmbeddedFile<&'static str>, { FILE_COUNT }> = EmbeddedFiles!([
    ("/root", [("root.txt", WasiEmbeddedFile::new("This is root"))]),
    (
        ".",
        [
            ("hey", WasiEmbeddedFile::new("Hey!")),
            (
                "hello",
                [
                    ("world", WasiEmbeddedFile::new("Hello, world!")),
                    ("everyone", WasiEmbeddedFile::new("Hello, everyone!")),
                ]
            )
        ]
    ),
    (
        "~",
        [
            ("home", WasiEmbeddedFile::new("This is home")),
            ("user", WasiEmbeddedFile::new("This is user")),
        ]
    )
]);

mod fs {
    use super::*;

    type LFS = StandardEmbeddedNormalLFS<EmbeddedFilesTy, WasiEmbeddedFile<&'static str>, FILE_COUNT, DefaultStdIO>;

    static VIRTUAL_FILE_SYSTEM: StandardEmbeddedFileSystem<LFS, FILE_COUNT> =
        StandardEmbeddedFileSystem::new_const(StandardEmbeddedNormalLFS::new_const());

    plug_fs!(&VIRTUAL_FILE_SYSTEM, test_wasm);
}



