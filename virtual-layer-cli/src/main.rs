use std::io::{BufRead, Write as _};

use cargo_metadata::Message;

pub mod args;
pub mod building;
pub mod down_color;

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

    let ret =
        building::build_vfs(manifest_path.clone(), building_crate).expect("Failed to build VFS");

    println!("Generated VFS: {ret}");
}
