use crate::args::NewArgs;

pub fn new(args: NewArgs) -> eyre::Result<()> {
    let path = args.path;

    // If already exists
    if path.exists() {
        return Err(eyre::eyre!("Directory already exists"));
    }

    let name = path.file_name().ok_or_else(|| eyre::eyre!("Failed to get file name"))?;

    let dir = path.parent().ok_or_else(|| eyre::eyre!("Failed to get parent directory"))?;
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }

    // run `cargo init` in the new directory
    std::process::Command::new("cargo")
        .arg("new")
        .arg("--lib")
        .arg(&name)
        .current_dir(&dir)
        .status()?;

    // setting lib
    // [lib]
    // crate-type = ["cdylib"]
    let cargo_toml_path = path.join("Cargo.toml");
    let mut cargo_toml = std::fs::read_to_string(&cargo_toml_path)?;
    cargo_toml.push_str("\n[lib]\ncrate-type = [\"cdylib\"]\n");
    std::fs::write(cargo_toml_path, cargo_toml)?;

    let dependencies = ["wasi-virt-layer", "wit-bindgen", "const_struct"];
    for dependency in dependencies {
        std::process::Command::new("cargo")
            .arg("add")
            .arg(dependency)
            .current_dir(&path)
            .status()?;
    }

    Ok(())
}
