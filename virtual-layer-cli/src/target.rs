use std::fs;

use camino::Utf8PathBuf;
use eyre::Context as _;

use crate::util::{CaminoUtilModule as _, ResultUtil as _, WalrusUtilModule};

pub fn adjust_target_wasm(path: &Utf8PathBuf, dwarf: bool) -> eyre::Result<Utf8PathBuf> {
    let name = path
        .get_file_main_name()
        .unwrap_or_else(|| panic!("Failed to get file name from {path}"));

    let mut module = walrus::Module::load(path, dwarf)?;

    let rewrite_exports = ["__main_void"];

    module
        .exports
        .iter_mut()
        .filter(|export| rewrite_exports.contains(&export.name.as_str()))
        .for_each(|export| {
            export.name = format!("__wasip1_vfs_{}_{}", &name, export.name);
        });

    let new_path = path.with_extension("adjusted.wasm");

    if fs::metadata(&new_path).is_ok() {
        fs::remove_file(&new_path).expect("Failed to remove existing file");
    }
    module
        .emit_wasm_file(new_path.clone())
        .to_eyre()
        .wrap_err("Failed to emit wasm file")?;

    Ok(new_path)
}
