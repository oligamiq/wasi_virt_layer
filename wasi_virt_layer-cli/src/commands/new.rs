use eyre::Context;

use crate::args::NewArgs;

/// Executes the new command, initializing a new WASI Virt Layer project with a template.
pub fn new(args: NewArgs) -> eyre::Result<()> {
    let NewArgs { path, threads } = args;

    // If already exists
    if path.exists() {
        eyre::bail!("Path already exists");
    }

    let name = path
        .file_name()
        .ok_or_else(|| eyre::eyre!("Failed to get file name"))?;

    let dir = match path.parent() {
        Some(parent) if parent != "" => {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
            parent
                .to_path_buf()
                .canonicalize()
                .with_context(|| format!("Failed to canonicalize parent directory `{parent}`"))
        }
        _ => std::env::current_dir()
            .context("Failed to get current directory")
            .and_then(|d| {
                d.canonicalize()
                    .with_context(|| "Failed to canonicalize current directory")
            }),
    }
    .context("Failed to get parent directory")?;

    log::info!("Creating new crate at `{path}` with name `{name}`");

    // run `cargo init` in the new directory
    std::process::Command::new("cargo")
        .arg("new")
        .arg("--lib")
        .arg(&name)
        .current_dir(&dir)
        .status()
        .context("Failed to initialize new crate")?;

    log::info!("Adding dependencies");

    // setting lib
    // [lib]
    // crate-type = ["cdylib"]
    let cargo_toml_path = path.join("Cargo.toml");
    let mut cargo_toml =
        std::fs::read_to_string(&cargo_toml_path).context("Failed to read Cargo.toml")?;
    cargo_toml.push_str("\n[lib]\ncrate-type = [\"cdylib\"]\n");
    std::fs::write(cargo_toml_path, cargo_toml).context("Failed to write Cargo.toml")?;

    let dependencies = if threads {
        vec!["wasi-virt-layer", "const_struct", "parking_lot"]
    } else {
        vec!["wasi-virt-layer", "const_struct"]
    };
    for dependency in dependencies {
        let mut cmd = std::process::Command::new("cargo");
        cmd.arg("add").arg(dependency);
        if threads && dependency == "wasi-virt-layer" {
            cmd.args(["--features", "threads,const-fs"]);
        } else if dependency == "wasi-virt-layer" {
            cmd.args(["--features", "const-fs"]);
        }
        cmd.current_dir(&path)
            .status()
            .context(format!("Failed to add dependency {dependency}"))?;
    }

    std::process::Command::new("cargo")
        .args([
            "add",
            "wit-bindgen",
            "--no-default-features",
            "--features",
            "macros,std",
        ])
        .current_dir(&path)
        .status()
        .context(format!("Failed to add dependency wit-bindgen"))?;

    // Rewrite src/lib.rs
    let lib_rs_path = path.join("src").join("lib.rs");
    std::fs::write(
        lib_rs_path,
        if threads {
            SRC_TEMPLATE_THREADS
        } else {
            SRC_TEMPLATE
        },
    )
    .context("Failed to write src/lib.rs")?;

    // Create @/wit/component-abi.wit
    let wit_dir = path.join("wit");
    std::fs::create_dir_all(&wit_dir).context("Failed to create wit directory")?;

    let wit_path = wit_dir.join("component-abi.wit");
    std::fs::write(
        wit_path,
        if threads {
            WIT_TEMPLATE_THREADS
        } else {
            WIT_TEMPLATE
        },
    )
    .context("Failed to write component-abi.wit")?;

    Ok(())
}

const WIT_TEMPLATE: &str = r#"
package component-abi:host;

world component-abi {
  export main: func();
}
"#
.trim_ascii();

const WIT_TEMPLATE_THREADS: &str = r#"
package component-abi:host;

world component-abi {
  export init: func();
  export start: func();
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

    static VIRTUAL_FILE_SYSTEM: Wasip1ConstVFS<LFS, FILE_COUNT> =
        Wasip1ConstVFS::new_const(VFSConstNormalLFS::new_const());

    plug_fs!(&VIRTUAL_FILE_SYSTEM, anonymous, self);
}
"#
.trim_ascii();

const SRC_TEMPLATE_THREADS: &str = r#"
use const_struct::const_struct;
use wasi_virt_layer::{file::*, plug_thread, prelude::*, process::*};

struct ComponentABI;

wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "component-abi",
});

import_wasm!(<anonymous>);

impl Guest for ComponentABI {
    fn init() {
        // initialization logic if needed
    }

    fn start() {
        anonymous::_start();
    }

    fn main() {
        anonymous::_reset();
        anonymous::_start();
        anonymous::_main();
    }
}

export!(ComponentABI);

plug_thread!(
    { wasi_virt_layer::thread::DirectThreadPool::<ThreadAccessor>::new_const() },
    anonymous,
    self
);

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

    static VIRTUAL_FILE_SYSTEM: Wasip1ConstVFS<LFS, FILE_COUNT> =
        Wasip1ConstVFS::new_const(VFSConstNormalLFS::new_const());

    plug_fs!(&VIRTUAL_FILE_SYSTEM, anonymous, self);
}
"#
.trim_ascii();
