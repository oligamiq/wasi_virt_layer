#![warn(missing_docs)]
//! CLI tool for creating and inspecting WASI virtualized binaries.
//!
//! Provides commands and generator internals `wasi_virt_layer`.

use clap::Parser;

use crate::{
    commands::{
        build::build, new::new, postbuild::postbuild, prebuild::prebuild,
        prepare_target::prepare_target,
    },
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
/// Generate TypeScript helper code for VFS modules.
pub mod gen_ts_helper;
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
    let args_vec: Vec<String> = args.into_iter().map(Into::into).collect();

    if let Some(bin) = std::env::var(fallback_command::COMMAND_ALTERNATE_ENV_VAR).ok() {
        match bin.as_str() {
            "wasm-merge" => {
                return match fallback_command::wasm_merge(&args_vec) {
                    0 => Ok(()),
                    code => Err(eyre::eyre!("wasm-merge failed with exit code {code}")),
                };
            }
            "wasm-opt" => {
                return match fallback_command::wasm_opt(&args_vec) {
                    0 => Ok(()),
                    code => Err(eyre::eyre!("wasm-opt failed with exit code {code}")),
                };
            }
            _ => {
                // For custom fallbacks (like in tests), we might need to handle them.
                // However, the self-call fallback doesn't support closures.
                // If this is reached, it means an unknown fallback was requested.
                // We'll just exit with an error to avoid deadlock/unexpected behavior.
                return Err(eyre::eyre!("Unsupported fallback command specified: {bin}"));
            }
        }
    }

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

    let args = args_vec.into_iter().map(Into::<std::ffi::OsString>::into);

    let parsed_args = args::Cli::parse_from(args);

    match parsed_args.command {
        args::Command::Build(build_args) => build(build_args),
        args::Command::Prebuild(prebuild_args) => prebuild(prebuild_args),
        args::Command::Postbuild(postbuild_args) => postbuild(postbuild_args),
        args::Command::New(new_args) => new(new_args),
        args::Command::PrepareTarget(prepare_target_args) => {
            let output = prepare_target_args.output.unwrap_or_else(|| {
                prepare_target_args
                    .target_wasm
                    .with_extension("prepared.wasm")
            });
            let args = commands::prepare_target::PrepareTargetHandler {
                target_wasm: prepare_target_args.target_wasm,
                output,
                keep_artifacts: prepare_target_args.keep_artifacts,
            };
            prepare_target(args)
        }
    }
}
