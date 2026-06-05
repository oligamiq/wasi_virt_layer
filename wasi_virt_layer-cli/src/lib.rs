#![warn(missing_docs)]
//! CLI tool for creating and inspecting WASI virtualized binaries.
//!
//! Provides commands and generator internals `wasi_virt_layer`.

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
/// Extracts and associates features with their target Wasm modules based on command line order.
pub mod feature_extractor;
/// Generate TypeScript helper code for VFS modules.
pub mod gen_ts_helper;
/// Internal generators for modifying and stitching Wasm structures.
#[allow(missing_docs)]
pub mod generator;
/// Instruction scanning and rewriting utilities for Walrus IR.

/// Utilities for running integration tests against Wasm runtimes.
pub mod test_run;
/// Utilities for generating globally unique IDs/names within Wasm modules.
pub mod unique_name;
/// General utility functions for CLI logic and AST operations.
pub mod util;
/// Streaming Wasm module modification engine.
#[allow(missing_docs)]
pub mod wasm_stream;

/// Central execution entrypoint for the CLI logic
pub fn main(args: impl IntoIterator<Item = impl Into<String>>) -> eyre::Result<()> {
    let args_vec: Vec<String> = args.into_iter().map(Into::into).collect();

    if let Some(bin) = std::env::var(fallback_command::COMMAND_ALTERNATE_ENV_VAR).ok() {
        match bin.as_str() {
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

    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();
    color_eyre::install()?;

    use clap::{CommandFactory, FromArgMatches};
    let cmd = args::Cli::command();
    let matches = cmd.get_matches_from(&args_vec);
    let mut parsed_args = args::Cli::from_arg_matches(&matches)?;

    if let Some((subcmd, sub_matches)) = matches.subcommand() {
        match subcmd {
            "build" => {
                if let args::Command::Build(ref mut build_args) = parsed_args.command {
                    let (vfs_opts, target_opts) = crate::feature_extractor::extract_features(
                        sub_matches,
                        build_args.wasm.len(),
                    );
                    build_args.vfs_build_opts = vfs_opts;
                    build_args.target_vfs_build_opts = Some(target_opts);
                }
            }
            "prebuild" => {
                if let args::Command::Prebuild(ref mut prebuild_args) = parsed_args.command {
                    let (vfs_opts, target_opts) = crate::feature_extractor::extract_features(
                        sub_matches,
                        prebuild_args.wasm.len(),
                    );
                    prebuild_args.vfs_build_opts = vfs_opts;
                    prebuild_args.target_vfs_build_opts = Some(target_opts);
                }
            }
            _ => {}
        }
    }

    let lock_ids = get_command_lock_identifiers(&parsed_args.command);

    let command_lock = if !lock_ids.is_empty() {
        let lock = std::sync::Arc::new(std::sync::Mutex::new(Some(CommandLock::acquire(
            &lock_ids,
        )?)));
        let cl_clone = lock.clone();
        ctrlc_handler::register(move || {
            if let Ok(mut lock) = cl_clone.lock() {
                if let Some(l) = lock.take() {
                    drop(l);
                }
            }
        });
        Some(lock)
    } else {
        None
    };

    struct MainCommandLockGuard(Option<std::sync::Arc<std::sync::Mutex<Option<CommandLock>>>>);
    impl Drop for MainCommandLockGuard {
        fn drop(&mut self) {
            if let Some(ref lock_arc) = self.0 {
                if let Ok(mut lock) = lock_arc.lock() {
                    let _ = lock.take();
                }
            }
        }
    }
    let _command_lock_guard = MainCommandLockGuard(command_lock);

    match parsed_args.command {
        args::Command::Build(build_args) => build(build_args),
        args::Command::Prebuild(prebuild_args) => prebuild(prebuild_args),
        args::Command::Postbuild(postbuild_args) => postbuild(postbuild_args),
        args::Command::New(new_args) => new(new_args),
    }
}

fn get_command_lock_identifiers(command: &args::Command) -> Vec<String> {
    let mut ids = Vec::new();

    fn get_workspace_root(manifest_path: Option<&camino::Utf8PathBuf>) -> String {
        let mut cmd = cargo_metadata::MetadataCommand::new();
        cmd.no_deps(); // Faster, only need workspace root
        if let Some(path) = manifest_path {
            cmd.manifest_path(path);
        }
        if let Ok(metadata) = cmd.exec() {
            return metadata.workspace_root.to_string();
        }

        // Fallback: look for Cargo.toml upwards
        let mut curr = if let Some(path) = manifest_path {
            path.parent().map(|p| p.to_path_buf().into_std_path_buf())
        } else {
            std::env::current_dir().ok()
        };

        while let Some(path) = curr {
            if path.join("Cargo.toml").exists() {
                return path.to_string_lossy().to_string();
            }
            curr = path.parent().map(|p| p.to_path_buf());
        }
        std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default()
    }

    match command {
        args::Command::Build(args) => {
            ids.push(get_workspace_root(args.manifest_path.as_ref()));
            ids.push(args.out_dir.to_string());
        }
        args::Command::Prebuild(args) => {
            ids.push(get_workspace_root(args.manifest_path.as_ref()));
            ids.push(args.out_dir.to_string());
        }
        args::Command::Postbuild(args) => {
            ids.push(args.out_dir.to_string());
        }
        args::Command::New(_) => {}
    }

    // Canonicalize all IDs if they are paths
    ids.into_iter()
        .map(|id| {
            camino::Utf8PathBuf::from(&id)
                .canonicalize_utf8()
                .map(|p| p.to_string())
                .unwrap_or(id)
        })
        .collect()
}
