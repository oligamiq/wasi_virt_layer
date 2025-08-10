use std::io::Write as _;

use eyre::{Context, ContextCompat};
use rewrite::adjust_wasm;
use util::CaminoUtilModule as _;

pub mod adjust;
pub mod args;
pub mod building;
pub mod common;
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

    println!("Compiling {}", building_crate.name);

    let ret = building::build_vfs(
        manifest_path.clone(),
        &parsed_args.package,
        building_crate.clone(),
    )
    .wrap_err_with(|| eyre::eyre!("Failed to build VFS"))?;

    println!("Optimizing VfS Wasm...");
    let ret = building::optimize_wasm(&ret, &[])
        .wrap_err_with(|| eyre::eyre!("Failed to optimize Wasm"))?;

    println!("Adjusting VFS Wasm...");
    let ret = adjust_wasm(&ret).wrap_err_with(|| eyre::eyre!("Failed to adjust Wasm"))?;

    println!("Optimizing VFS Wasm...");
    let ret = building::optimize_wasm(&ret, &[])
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
        .map(|old_wasm| {
            let name = old_wasm.get_file_main_name().unwrap();
            let wasm = format!("{}/{}", parsed_args.out_dir, old_wasm.file_name().unwrap());
            std::fs::copy(old_wasm, &wasm)
                .wrap_err_with(|| eyre::eyre!("Failed to find Wasm file {old_wasm}"))?;
            println!("Optimizing target Wasm [{name}]...");
            tmp_files.push(wasm.to_string());
            let wasm = building::optimize_wasm(&wasm.into(), &[])
                .wrap_err_with(|| eyre::eyre!("Failed to optimize Wasm"))?;
            tmp_files.push(wasm.to_string());
            println!("Adjusting target Wasm [{name}]...");
            let wasm = target::adjust_target_wasm(&wasm)
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
    let ret = building::optimize_wasm(&output.clone().into(), &[])
        .wrap_err_with(|| eyre::eyre!("Failed to optimize merged Wasm"))?;
    tmp_files.push(ret.to_string());

    println!("Adjusting Merged Wasm...");
    let ret = adjust::adjust_merged_wasm(&ret, &wasm_paths)
        .wrap_err_with(|| eyre::eyre!("Failed to adjust merged Wasm"))?;
    tmp_files.push(ret.to_string());

    println!("Optimizing Merged Wasm...");
    let ret = building::optimize_wasm(&ret, &[])
        .wrap_err_with(|| eyre::eyre!("Failed to optimize merged Wasm"))?;
    tmp_files.push(ret.to_string());

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
        std::fs::write(&file_name, &data).expect("Failed to write file");
        if name.ends_with(".core.wasm") {
            core_wasm = Some(file_name);
        }
    }

    let core_wasm = core_wasm
        .as_ref()
        .ok_or_else(|| eyre::eyre!("Failed to find core wasm"))?;

    let core_wasm_opt = building::optimize_wasm(&core_wasm.into(), &[])
        .wrap_err_with(|| eyre::eyre!("Failed to optimize core Wasm"))?;

    std::fs::remove_file(&core_wasm).expect("Failed to remove existing file");
    std::fs::rename(&core_wasm_opt, &core_wasm).expect("Failed to rename file");

    println!("Generating single memory Wasm...");
    let core_single_wasm_opt =
        building::optimize_wasm(&core_wasm.into(), &["--multi-memory-lowering"])?;

    std::fs::rename(
        &core_single_wasm_opt,
        camino::Utf8PathBuf::from(core_wasm.clone()).with_extension("single_memory.wasm"),
    )
    .expect("Failed to rename file");

    for tmp_file in tmp_files {
        std::fs::remove_file(tmp_file).expect("Failed to remove tmp file");
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
