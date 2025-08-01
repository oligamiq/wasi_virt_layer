use std::fs;

use camino::Utf8PathBuf;
use eyre::Context as _;

use crate::{
    common::Wasip1SnapshotPreview1Func,
    util::{CaminoUtilModule as _, ResultUtil as _, WalrusUtilModule},
};

pub fn adjust_target_wasm(path: &Utf8PathBuf) -> eyre::Result<Utf8PathBuf> {
    let name = path
        .get_file_main_name()
        .unwrap_or_else(|| panic!("Failed to get file name from {path}"));

    let mut module = walrus::Module::from_file(path)
        .to_eyre()
        .wrap_err_with(|| eyre::eyre!("Failed to load module"))?;

    module.create_memory_anchor(&name)?;

    module
        .create_global_anchor(&name)
        .wrap_err_with(|| eyre::eyre!("Failed to create global anchor"))?;

    let rewrite_exports = ["_start", "__main_void", "memory"];

    module
        .exports
        .iter_mut()
        .filter(|export| rewrite_exports.contains(&export.name.as_str()))
        .for_each(|export| {
            export.name = format!("__wasip1_vfs_{}_{}", &name, export.name);
        });

    module
        .imports
        .iter_mut()
        .filter(|import| {
            <Wasip1SnapshotPreview1Func as strum::VariantNames>::VARIANTS
                .contains(&import.name.as_str())
                && import.module == "wasi_snapshot_preview1"
        })
        .for_each(|import| {
            import.name = format!("__wasip1_vfs_{name}_{}", import.name);
        });

    let new_path = path.with_extension("adjusted.wasm");

    if fs::metadata(&new_path).is_ok() {
        fs::remove_file(&new_path).expect("Failed to remove existing file");
    }
    module
        .emit_wasm_file(new_path.clone())
        .to_eyre()
        .wrap_err_with(|| eyre::eyre!("Failed to emit wasm file"))?;

    Ok(new_path)
}
