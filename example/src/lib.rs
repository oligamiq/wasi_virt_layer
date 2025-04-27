use const_struct::const_struct;
use parking_lot::Mutex;
use std::sync::LazyLock;
use wasip1_virtual_layer::{
    ConstFiles, export_fs,
    prelude::*,
    wasi::file::non_atomic::{
        ConstFileSystemRoot, DefaultStdIO, VirtualFileSystemConstState, WasiConstFile,
    },
};

wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "hello",
});

struct Hello;

impl Guest for Hello {
    fn world() {
        println!("Hello, world!");
    }
}

export!(Hello);

import_wasm!(test_wasm_opt);

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
    Mutex::new(VirtualEnvState { environ })
});

export_env!(@block, @static, &mut VIRTUAL_ENV.lock(), test_wasm_opt);

#[const_struct]
const FILES: ConstFileSystemRoot<WasiConstFile<&'static str>, 3> = ConstFiles!([
    ("/", { WasiConstFile::new("This is root") }),
    (
        ".",
        [
            ("hey", { WasiConstFile::new("Hey!") }),
            (
                "hello",
                [
                    ("world", { WasiConstFile::new("Hello, world!") }),
                    ("everyone", { WasiConstFile::new("Hello, everyone!") }),
                ]
            )
        ]
    ),
    (
        "~",
        [
            ("home", { WasiConstFile::new("This is home") }),
            ("user", { WasiConstFile::new("This is user") }),
        ]
    )
]);

static FS_STATE: std::sync::LazyLock<
    Mutex<VirtualFileSystemConstState<WasiConstFile<&str>, 3, FilesTy, DefaultStdIO>>,
> = std::sync::LazyLock::new(|| Mutex::new(VirtualFileSystemConstState::new(&FILES)));

export_fs!(@const, &mut (*FS_STATE.lock()), test_wasm_opt);
