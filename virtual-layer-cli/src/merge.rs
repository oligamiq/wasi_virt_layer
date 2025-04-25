use std::{fs, path::Path};

use camino::Utf8PathBuf;

use crate::util::CaminoUtilModule;

pub fn merge<T: AsRef<Path>, U: AsRef<Path>>(
    vfs: &Utf8PathBuf,
    wasm: &[T],
    output: &U,
) -> anyhow::Result<()> {
    let custom_section = {
        let mut vfs_module = walrus::Module::from_file(vfs)?;
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

    for wasm in wasm {
        merge_cmd.arg(wasm.as_ref()).arg(format!(
            "wasip1_vfs_{}",
            wasm.as_ref().get_file_main_name().unwrap()
        ));
    }

    merge_cmd
        .arg(vfs)
        .arg("wasi_snapshot_preview1")
        .arg("--output")
        .arg(output.as_ref())
        .arg("--rename-export-conflicts")
        .arg("--enable-multimemory");

    merge_cmd
        .spawn()
        .expect("Failed to spawn wasm-merge command")
        .wait()
        .expect("Failed to wait for wasm-merge command");

    let mut module = walrus::Module::from_file(output)?;
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
