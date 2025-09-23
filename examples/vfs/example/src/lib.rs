use const_struct::const_struct;
use parking_lot::Mutex;
use std::sync::LazyLock;
use wasip1_virtual_layer::{export_process, file::*, prelude::*};

wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "hello",
});

struct Hello;

import_wasm!(test_wasm_opt);

impl Guest for Hello {
    fn world() {
        println!("Hello, world!");

        type F = WasiConstFile<&'static str>;
        const FILES2: VFSConstNormalFiles<F, 5> = ConstFiles!([(
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
    fn start() {
        test_wasm_opt::_start();
    }
    fn main() {
        test_wasm_opt::reset();
        test_wasm_opt::_start();
        test_wasm_opt::_main();
    }
}

export!(Hello);

export_process!(test_wasm_opt);

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
    environ.push("RUST_MIN_STACK=16777216".into());
    environ.push("HOME=~/".into());
    environ.push("RUST_BACKTRACE=1".into());
    Mutex::new(VirtualEnvState { environ })
});

export_env!(@static, &mut VIRTUAL_ENV.lock(), test_wasm_opt, self);

const FILE_COUNT: usize = 10;

#[const_struct]
const FILES: VFSConstNormalFiles<WasiConstFile<&'static str>, { FILE_COUNT }> = ConstFiles!([
    ("/root", [("root.txt", WasiConstFile::new("This is root"))]),
    (
        ".",
        [
            ("hey", WasiConstFile::new("Hey!")),
            (
                "hello",
                [
                    ("world", WasiConstFile::new("Hello, world!")),
                    ("everyone", WasiConstFile::new("Hello, everyone!")),
                ]
            )
        ]
    ),
    (
        "~",
        [
            ("home", WasiConstFile::new("This is home")),
            ("user", WasiConstFile::new("This is user")),
        ]
    )
]);

mod fs {
    use super::*;

    type LFS = VFSConstNormalLFS<FilesTy, WasiConstFile<&'static str>, FILE_COUNT, DefaultStdIO>;

    static mut VIRTUAL_FILE_SYSTEM: Wasip1ConstVFS<LFS, FILE_COUNT> =
        Wasip1ConstVFS::new(VFSConstNormalLFS::new());

    export_fs!(@const, {
        #[allow(static_mut_refs)]
        unsafe { &mut VIRTUAL_FILE_SYSTEM }
    }, test_wasm_opt);
}
