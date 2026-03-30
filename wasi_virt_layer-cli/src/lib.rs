use clap::Parser;
use eyre::Context;

use crate::{
    args::{BuildArgs, TargetMemoryType},
    commands::{build::build, new::new},
    config_checker::{FeatureChecker, HasFeature, TomlRestorers},
    generator::WasmPath,
};

pub mod abi;
pub mod args;
pub mod commands;
pub mod compile;
pub mod config_checker;
pub mod down_color;
pub mod fallback_command;
pub mod generator;
pub mod instrs;
pub mod test_run;
pub mod util;

pub fn main(args: impl IntoIterator<Item = impl Into<String>>) -> eyre::Result<()> {
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
        args::Command::New(new_args) => new(new_args),
    }
}

// deno run dist/example_vfs.js
