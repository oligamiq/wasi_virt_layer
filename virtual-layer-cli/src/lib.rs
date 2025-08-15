use std::io::Write as _;

use eyre::{Context, ContextCompat};
use rewrite::adjust_wasm;
use util::CaminoUtilModule as _;

use crate::rewrite::{TargetMemoryType, change_target_memory_type};

pub mod adjust;
pub mod args;
pub mod building;
pub mod common;
pub mod director;
pub mod down_color;
pub mod instrs;
pub mod is_valid;
pub mod merge;
pub mod rewrite;
pub mod target;
pub mod test_run;
pub mod util;

pub fn main(args: impl IntoIterator<Item = impl Into<String>>) -> eyre::Result<()> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();
    color_eyre::install()?;

    let mut tmp_files = Vec::new();

    let parsed_args = args::Args::new(args);

    let manifest_path = parsed_args.get_manifest_path();
    let cargo_metadata = {
        let mut metadata_command = cargo_metadata::MetadataCommand::new();
        if let Some(manifest_path) = manifest_path {
            metadata_command.manifest_path(manifest_path);
        }
        metadata_command.exec().unwrap()
    };
    let building_crate = building::get_building_crate(&cargo_metadata, &parsed_args.package);

    if let Some(target_memory_type) = parsed_args.target_memory_type {
        change_target_memory_type(&cargo_metadata, &building_crate, target_memory_type)?;
    }

    println!("Compiling {}", building_crate.name);

    let ret = building::build_vfs(
        manifest_path.clone(),
        &parsed_args.package,
        building_crate.clone(),
    )
    .wrap_err_with(|| eyre::eyre!("Failed to build VFS"))?;

    println!("Optimizing VfS Wasm...");
    let ret = building::optimize_wasm(&ret, &[], false)
        .wrap_err_with(|| eyre::eyre!("Failed to optimize Wasm"))?;

    println!("Adjusting VFS Wasm...");
    let (ret, target_memory_type) =
        adjust_wasm(&ret).wrap_err_with(|| eyre::eyre!("Failed to adjust Wasm"))?;

    println!("Optimizing VFS Wasm...");
    let ret = building::optimize_wasm(&ret, &[], false)
        .wrap_err_with(|| eyre::eyre!("Failed to optimize Wasm"))?;

    println!("Generated VFS: {ret}");

    println!("Remove existing output directory...");
    if std::fs::metadata(&parsed_args.out_dir).is_ok() {
        std::fs::remove_dir_all(&parsed_args.out_dir).expect("Failed to remove existing directory");
    }
    std::fs::create_dir_all(&parsed_args.out_dir).expect("Failed to create output directory");

    println!("Preparing target Wasm...");
    let (wasm_paths, wasm_names) = parsed_args
        .wasm
        .iter()
        .zip(
            parsed_args
                .wasm_memory_hint
                .iter()
                .map(|h| Some(*h))
                .chain(std::iter::repeat(None)),
        )
        .map(|(old_wasm, memory_hint)| {
            let name = old_wasm.get_file_main_name().unwrap();
            let wasm = format!("{}/{}", parsed_args.out_dir, old_wasm.file_name().unwrap());
            std::fs::copy(old_wasm, &wasm)
                .wrap_err_with(|| eyre::eyre!("Failed to find Wasm file {old_wasm}"))?;
            println!("Optimizing target Wasm [{name}]...");
            tmp_files.push(wasm.to_string());
            let wasm = building::optimize_wasm(&wasm.into(), &[], false)
                .wrap_err_with(|| eyre::eyre!("Failed to optimize Wasm"))?;
            tmp_files.push(wasm.to_string());
            println!("Adjusting target Wasm [{name}]...");
            let wasm = target::adjust_target_wasm(&wasm, memory_hint)
                .wrap_err_with(|| eyre::eyre!("Failed to adjust Wasm"))?;
            tmp_files.push(wasm.to_string());
            Ok((wasm, name))
        })
        .collect::<eyre::Result<(Vec<_>, Vec<_>)>>()?;

    println!("Merging Wasm...");

    let output = format!("{}/merged.wasm", parsed_args.out_dir);
    if std::fs::metadata(&output).is_ok() {
        std::fs::remove_file(&output).expect("Failed to remove existing file");
    }
    merge::merge(&ret, &wasm_paths, &output)
        .wrap_err_with(|| eyre::eyre!("Failed to merge Wasm"))?;
    tmp_files.push(output.clone());

    println!("Optimizing Merged Wasm...");
    let ret = building::optimize_wasm(&output.clone().into(), &[], false)
        .wrap_err_with(|| eyre::eyre!("Failed to optimize merged Wasm"))?;
    tmp_files.push(ret.to_string());

    println!("Adjusting Merged Wasm...");
    let ret = adjust::adjust_merged_wasm(&ret, &wasm_paths)
        .wrap_err_with(|| eyre::eyre!("Failed to adjust merged Wasm"))?;
    tmp_files.push(ret.to_string());

    let ret = if matches!(target_memory_type, TargetMemoryType::Single) {
        println!("Generating single memory Merged Wasm...");
        let ret = building::optimize_wasm(&ret, &["--multi-memory-lowering"], true)?;
        tmp_files.push(ret.to_string());
        ret
    } else {
        ret
    };

    println!("Optimizing Merged Wasm...");
    let ret = building::optimize_wasm(&ret, &[], false)
        .wrap_err_with(|| eyre::eyre!("Failed to optimize merged Wasm"))?;
    tmp_files.push(ret.to_string());

    let ret = if matches!(target_memory_type, TargetMemoryType::Single) {
        println!("Directing process {target_memory_type} memory Merged Wasm...");
        let ret = director::director(&ret, &wasm_paths)?;
        tmp_files.push(ret.to_string());
        ret
    } else {
        ret
    };

    println!("Translating Wasm to Component...");
    let component = building::wasm_to_component(&ret, &wasm_names)
        .wrap_err_with(|| eyre::eyre!("Failed to translate Wasm to Component"))?;
    tmp_files.push(component.to_string());

    println!("Translating Component to JS...");
    let binary =
        std::fs::read(&component).wrap_err_with(|| eyre::eyre!("Failed to read component"))?;
    let transpiled = parsed_args
        .transpile_to_js(&binary, &building_crate.name)
        .wrap_err_with(|| eyre::eyre!("Failed to transpile to JS"))?;

    let mut core_wasm = None;
    for (name, data) in transpiled.files.iter() {
        let file_name = format!("{}/{name}", parsed_args.out_dir);
        if std::fs::metadata(&file_name).is_ok() {
            std::fs::remove_file(&file_name).expect("Failed to remove existing file");
        }
        if name.ends_with(".core.wasm") {
            let file_name = camino::Utf8PathBuf::from(file_name);
            std::fs::write(&file_name, &data).expect("Failed to write file");
            core_wasm = Some(file_name);
        } else {
            std::fs::write(&file_name, &data).expect("Failed to write file");
        }
    }

    let core_wasm = core_wasm
        .as_ref()
        .ok_or_else(|| eyre::eyre!("Failed to find core wasm"))?;

    let core_wasm_opt = building::optimize_wasm(&core_wasm.into(), &[], false)
        .wrap_err_with(|| eyre::eyre!("Failed to optimize core Wasm"))?;

    std::fs::remove_file(&core_wasm).expect("Failed to remove existing file");
    std::fs::rename(&core_wasm_opt, &core_wasm).expect("Failed to rename file");

    for tmp_file in tmp_files {
        std::fs::remove_file(&tmp_file)
            .wrap_err_with(|| eyre::eyre!("Failed to remove tmp file: {tmp_file}"))?;
    }

    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(format!("{}/test_run.ts", parsed_args.out_dir))
        .expect("Failed to create file")
        .write_all(test_run::TEST_RUN.trim_start().as_bytes())
        .expect("Failed to write file");

    Ok(())
}

// deno run dist/example_vfs.js
