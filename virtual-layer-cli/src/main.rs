use rewrite::adjust_wasm;

pub mod args;
pub mod building;
pub mod down_color;
pub mod merge;
pub mod rewrite;

fn main() {
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
        .expect("Failed to build VFS");

    println!("Optimizing Wasm...");
    let ret = building::optimize_wasm(&ret).expect("Failed to optimize Wasm");

    println!("Adjusting Wasm...");
    let ret = adjust_wasm(&ret).expect("Failed to adjust Wasm");

    println!("Optimizing Wasm...");
    let ret = building::optimize_wasm(&ret).expect("Failed to optimize Wasm");

    println!("Generated VFS: {ret}");

    println!("Remove existing output directory...");
    if std::fs::metadata(&parsed_args.out_dir).is_ok() {
        std::fs::remove_dir_all(&parsed_args.out_dir).expect("Failed to remove existing directory");
    }
    std::fs::create_dir_all(&parsed_args.out_dir).expect("Failed to create output directory");

    println!("Merging Wasm...");

    let output = format!("{}/merged.wasm", parsed_args.out_dir);
    if std::fs::metadata(&output).is_ok() {
        std::fs::remove_file(&output).expect("Failed to remove existing file");
    }
    merge::merge(&ret, &parsed_args.wasm, &output).expect("Failed to merge Wasm");

    println!("Optimizing Merged Wasm...");
    let ret =
        building::optimize_wasm(&output.clone().into()).expect("Failed to optimize merged Wasm");

    println!("Adjusting Merged Wasm...");

    println!("Translating Wasm to Component...");
    let component = building::wasm_to_component(&ret).expect("Failed to convert Wasm to Component");

    println!("Translating Component to JS...");
    let binary = std::fs::read(&component).expect("Failed to read Wasm file");
    let transpiled = parsed_args
        .transpile_to_js(&binary, &building_crate.name)
        .expect("Failed to transpile Wasm to Component");

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
}
