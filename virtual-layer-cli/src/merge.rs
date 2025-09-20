use std::{fs, path::Path};

use camino::Utf8PathBuf;

use crate::util::{CaminoUtilModule, WalrusUtilModule};

pub fn merge(
    vfs: &Utf8PathBuf,
    wasm: &[impl AsRef<Path>],
    output: impl AsRef<Path>,
    dwarf: bool,
) -> eyre::Result<()> {
    let custom_section = {
        let mut vfs_module = walrus::Module::load(vfs, dwarf)?;
        let custom_section_names = vfs_module
            .customs
            .iter()
            .map(|(_, section)| section.name().to_string())
            .filter(|name| name.starts_with("component-type:"))
            .collect::<Vec<_>>();
        // let custom_section = vfs_module
        //     .customs.delete(custom_section_names)
        let custom_section = custom_section_names
            .iter()
            .map(|id| {
                let section = vfs_module.customs.remove_raw(id);
                section.unwrap()
            })
            .collect::<Vec<_>>();

        custom_section
    };

    let mut merge_cmd = std::process::Command::new("wasm-merge");

    if dwarf {
        merge_cmd.arg("--debuginfo");
    }

    merge_cmd.arg(vfs).arg("wasi_snapshot_preview1");

    for wasm in wasm {
        merge_cmd.arg(wasm.as_ref()).arg(format!(
            "wasip1_vfs_{}",
            wasm.as_ref().get_file_main_name().unwrap()
        ));
    }

    merge_cmd
        .arg("--output")
        .arg(output.as_ref())
        // .arg("--rename-export-conflicts")
        .arg("--enable-multimemory")
        .arg("--enable-threads");

    let result = merge_cmd
        .spawn()
        .map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => eyre::eyre!(
                "wasm-merge command not found. Please install wasm-merge from https://github.com/WebAssembly/binaryen/releases/latest"
            ),
            _ => e.into(),
        })?
        .wait()
        .expect("Failed to wait for wasm-merge command");

    if !result.success() {
        return Err(eyre::eyre!("wasm-merge command failed"));
    }

    let mut module = walrus::Module::load(output.as_ref(), dwarf)?;
    for section in custom_section {
        module.customs.add(section);
    }

    // to output
    fs::remove_file(output.as_ref()).expect("Failed to remove existing file");

    module
        .emit_wasm_file(output.as_ref())
        .expect("Failed to emit wasm file");

    Ok(())
}
