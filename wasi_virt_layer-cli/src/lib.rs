#![warn(missing_docs)]
//! CLI tool for creating and inspecting WASI virtualized binaries.
//!
//! Provides commands and generator internals `wasi_virt_layer`.

use clap::Parser;

use crate::{
    commands::{build::build, new::new, postbuild::postbuild, prebuild::prebuild},
    fallback_command::CommandLock,
};

/// WASI ABI transformation and generation constants.
pub mod abi;
/// CLI argument parsing structures and definitions.
pub mod args;
/// Handlers for various CLI subcommands.
pub mod commands;
/// Compilation routines for Wasm virtual filesystems.
pub mod compile;
/// Checks and manages features required in Cargo configs.
pub mod config_checker;
/// Signal handler for graceful termination (e.g., Ctrl-C).
pub mod ctrlc_handler;
/// Helper logic to lower text colors and apply them.
pub mod down_color;
/// Contains fallback execution processes (managed via locks).
pub mod fallback_command; // fallback logic guarded by DISABLE_FALLBACK
/// Internal generators for modifying and stitching Wasm structures.
pub mod generator;
/// Instruction scanning and rewriting utilities for Walrus IR.
pub mod instrs;
/// Utilities for running integration tests against Wasm runtimes.
pub mod test_run;
/// Utilities for generating globally unique IDs/names within Wasm modules.
pub mod unique_name;
/// General utility functions for CLI logic and AST operations.
pub mod util;

/// Central execution entrypoint for the CLI logic
pub fn main(args: impl IntoIterator<Item = impl Into<String>>) -> eyre::Result<()> {
    ctrlc_handler::init();

    let command_lock = std::sync::Arc::new(std::sync::Mutex::new(Some(CommandLock::acquire()?)));
    let cl_clone = command_lock.clone();
    ctrlc_handler::register(move || {
        if let Ok(mut lock) = cl_clone.lock() {
            if let Some(l) = lock.take() {
                drop(l);
            }
        }
    });

    struct MainCommandLockGuard(std::sync::Arc<std::sync::Mutex<Option<CommandLock>>>);
    impl Drop for MainCommandLockGuard {
        fn drop(&mut self) {
            if let Ok(mut lock) = self.0.lock() {
                let _ = lock.take();
            }
        }
    }
    let _command_lock_guard = MainCommandLockGuard(command_lock);

    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();
    color_eyre::install()?;

    let args = args
        .into_iter()
        .map(Into::<String>::into)
        .map(Into::<std::ffi::OsString>::into);

    let parsed_args = args::Cli::parse_from(args);

    match parsed_args.command {
        args::Command::Build(build_args) => build(build_args),
        args::Command::Prebuild(prebuild_args) => prebuild(prebuild_args),
        args::Command::Postbuild(postbuild_args) => postbuild(postbuild_args),
        args::Command::New(new_args) => new(new_args),
    }
}
