use rewrite::adjust_wasm;

pub mod args;
pub mod building;
pub mod down_color;
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

    let cmd = std::process::Command::new("wasm-merge")
        .spawn()
        .expect("Failed to spawn wasm-merge command");

    println!("Generated VFS: {ret}");

    println!("Translating Wasm to Component...");
    let component = building::wasm_to_component(&ret).expect("Failed to convert Wasm to Component");

    println!("Translating Component to JS...");
    let binary = std::fs::read(&component).expect("Failed to read Wasm file");
    let transpiled = parsed_args
        .transpile_to_js(&binary, &building_crate.name)
        .expect("Failed to transpile Wasm to Component");

    println!("Transpiled to JS: {:#?}", transpiled.exports);
    println!("Transpiled to JS: {:#?}", transpiled.imports);
    println!("Transpiled to JS: {:#?}", transpiled.files);
}
