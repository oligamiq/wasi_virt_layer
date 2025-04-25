use eyre::{Context, ContextCompat};
use rewrite::adjust_wasm;
use util::CaminoUtilModule as _;

pub mod adjust;
pub mod args;
pub mod building;
pub mod common;
pub mod down_color;
pub mod merge;
pub mod rewrite;
pub mod target;
pub mod util;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let parsed_args = args::Args::new();

    let manifest_path = parsed_args.get_manifest_path();
    let cargo_metadata = {
        let mut metadata_command = cargo_metadata::MetadataCommand::new();
        if let Some(manifest_path) = manifest_path {
            metadata_command.manifest_path(manifest_path);
        }
        metadata_command.exec().unwrap()
    };
    let building_crate = building::get_building_crate(&cargo_metadata);

    println!("Compiling {}", building_crate.name);

    let ret = building::build_vfs(manifest_path.clone(), building_crate.clone())
        .wrap_err_with(|| eyre::eyre!("Failed to build VFS"))?;

    println!("Optimizing VfS Wasm...");
    let ret =
        building::optimize_wasm(&ret).with_context(|| eyre::eyre!("Failed to optimize Wasm"))?;

    println!("Adjusting VFS Wasm...");
    let ret = adjust_wasm(&ret).with_context(|| eyre::eyre!("Failed to adjust Wasm"))?;

    println!("Optimizing VFS Wasm...");
    let ret =
        building::optimize_wasm(&ret).with_context(|| eyre::eyre!("Failed to optimize Wasm"))?;

    println!("Generated VFS: {ret}");

    println!("Remove existing output directory...");
    if std::fs::metadata(&parsed_args.out_dir).is_ok() {
        std::fs::remove_dir_all(&parsed_args.out_dir).expect("Failed to remove existing directory");
    }
    std::fs::create_dir_all(&parsed_args.out_dir).expect("Failed to create output directory");

    println!("Preparing target Wasm...");
    let wasm = parsed_args
        .wasm
        .iter()
        .map(|old_wasm| {
            let name = old_wasm.get_file_main_name().unwrap();
            let wasm = format!("{}/{}", parsed_args.out_dir, old_wasm.file_name().unwrap());
            std::fs::copy(old_wasm, &wasm).expect("Failed to copy file");
            println!("Optimizing target Wasm [{name}]...");
            let wasm = building::optimize_wasm(&wasm.into())
                .with_context(|| eyre::eyre!("Failed to optimize Wasm"))?;
            println!("Adjusting target Wasm [{name}]...");
            let wasm = target::adjust_target_wasm(&wasm)
                .with_context(|| eyre::eyre!("Failed to adjust Wasm"))?;
            Ok(wasm)
        })
        .collect::<eyre::Result<Vec<_>>>()?;

    println!("Merging Wasm...");

    let output = format!("{}/merged.wasm", parsed_args.out_dir);
    if std::fs::metadata(&output).is_ok() {
        std::fs::remove_file(&output).expect("Failed to remove existing file");
    }
    merge::merge(&ret, &wasm, &output).with_context(|| eyre::eyre!("Failed to merge Wasm"))?;

    println!("Optimizing Merged Wasm...");
    let ret = building::optimize_wasm(&output.clone().into())
        .wrap_err_with(|| eyre::eyre!("Failed to optimize merged Wasm"))?;

    println!("Adjusting Merged Wasm...");
    let ret = adjust::adjust_merged_wasm(&ret, &wasm)
        .with_context(|| eyre::eyre!("Failed to adjust merged Wasm"))?;

    println!("Translating Wasm to Component...");
    let component = building::wasm_to_component(&ret)
        .with_context(|| eyre::eyre!("Failed to translate Wasm to Component"))?;

    println!("Translating Component to JS...");
    let binary =
        std::fs::read(&component).with_context(|| eyre::eyre!("Failed to read component"))?;
    let transpiled = parsed_args
        .transpile_to_js(&binary, &building_crate.name)
        .with_context(|| eyre::eyre!("Failed to transpile to JS"))?;

    for (name, data) in transpiled.files.iter() {
        let file_name = format!("{}/{name}", parsed_args.out_dir);
        if std::fs::metadata(&file_name).is_ok() {
            std::fs::remove_file(&file_name).expect("Failed to remove existing file");
        }
        std::fs::write(file_name, &data).expect("Failed to write file");
    }

    std::fs::remove_file(&output).expect("Failed to remove tmp file");
    std::fs::remove_file(&ret).expect("Failed to remove tmp file");
    std::fs::remove_file(&component).expect("Failed to remove tmp file");

    Ok(())
}
