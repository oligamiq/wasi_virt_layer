use crate::args::NewArgs;

pub fn new(args: NewArgs) -> eyre::Result<()> {
    let path = args.path;

    // If already exists
    if path.exists() {
        return Err(eyre::eyre!("Directory already exists"));
    }

    let name = path
        .file_name()
        .ok_or_else(|| eyre::eyre!("Failed to get file name"))?;

    let dir = path
        .parent()
        .ok_or_else(|| eyre::eyre!("Failed to get parent directory"))?;
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }

    // run `cargo init` in the new directory
    std::process::Command::new("cargo")
        .arg("new")
        .arg("--lib")
        .arg(&name)
        .current_dir(&dir)
        .status()?;

    // setting lib
    // [lib]
    // crate-type = ["cdylib"]
    let cargo_toml_path = path.join("Cargo.toml");
    let mut cargo_toml = std::fs::read_to_string(&cargo_toml_path)?;
    cargo_toml.push_str("\n[lib]\ncrate-type = [\"cdylib\"]\n");
    std::fs::write(cargo_toml_path, cargo_toml)?;

    let dependencies = ["wasi-virt-layer", "wit-bindgen", "const_struct"];
    for dependency in dependencies {
        std::process::Command::new("cargo")
            .arg("add")
            .arg(dependency)
            .current_dir(&path)
            .status()?;
    }

    // Rewrite src/lib.rs
    let lib_rs_path = path.join("src").join("lib.rs");
    std::fs::write(lib_rs_path, SRC_TEMPLATE)?;

    // Create @/wit/component-abi.wit
    let wit_dir = path.join("wit");
    std::fs::create_dir_all(&wit_dir)?;

    let wit_path = wit_dir.join("component-abi.wit");
    std::fs::write(wit_path, WIT_TEMPLATE)?;

    Ok(())
}

const WIT_TEMPLATE: &str = r#"
package component-abi:host;

world component-abi {
  export main: func();
}
"#
.trim_ascii();

const SRC_TEMPLATE: &str = r#"
use const_struct::const_struct;
use wasi_virt_layer::{file::*, prelude::*, process::*};

struct ComponentABI;

wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "component-abi",
});

import_wasm!(<anonymous>);

impl Guest for ComponentABI {
    fn main() {
        anonymous::_reset();
        anonymous::_start();
        anonymous::_main();
    }
}

export!(ComponentABI);

mod process {
    use super::*;
    plug_process!(DefaultProcess, anonymous, self);
}

mod env {
    use super::*;

    #[const_struct]
    const HOST_ENV: VirtualEnvConstState = VirtualEnvConstState {
        environ: &["HOME=~/"],
    };

    plug_env!(@const, HostEnvTy, anonymous, self);
}

mod fs {
    use super::*;

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

    type LFS = VFSConstNormalLFS<FilesTy, WasiConstFile<&'static str>, FILE_COUNT, DefaultStdIO>;

    static mut VIRTUAL_FILE_SYSTEM: Wasip1ConstVFS<LFS, FILE_COUNT> =
        Wasip1ConstVFS::new(VFSConstNormalLFS::new());

    plug_fs!(@const, {
        #[allow(static_mut_refs)]
        unsafe { &mut VIRTUAL_FILE_SYSTEM }
    }, anonymous);
}

"#
.trim_ascii();
