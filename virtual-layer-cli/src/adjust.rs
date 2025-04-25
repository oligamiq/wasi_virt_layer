use std::{fs, path::Path};

use camino::Utf8PathBuf;
use eyre::Context;
use walrus::{
    LocalId,
    ir::{MemArg, StoreKind, Value},
};

use crate::{
    common::WASIP1_FUNC,
    util::{CaminoUtilModule, ResultUtil as _, WalrusUtilModule},
};

pub fn adjust_merged_wasm(
    path: &Utf8PathBuf,
    wasm: &[impl AsRef<Path>],
) -> eyre::Result<Utf8PathBuf> {
    let mut module = walrus::Module::from_file(path)
        .to_eyre()
        .wrap_err_with(|| eyre::eyre!("Failed to load module"))?;

    for wasm in wasm {
        let wasm_name = wasm.as_ref().get_file_main_name().unwrap();

        for name in WASIP1_FUNC.iter() {
            let export_name = format!("__wasip1_vfs_{wasm_name}_{name}");

            if module
                .imports
                .find("wasi_snapshot_preview1", name)
                .is_some()
            {
                module
                    .connect_func("wasi_snapshot_preview1", name, &export_name)
                    .wrap_err_with(|| eyre::eyre!("Failed to connect {name}"))?;
            } else {
                if module.exports.get_func(&export_name).is_ok() {
                    module
                        .exports
                        .remove(&export_name)
                        .to_eyre()
                        .wrap_err_with(|| eyre::eyre!("Failed to remove {name} export"))?;
                }
            }
        }

        let memory_id = module
            .get_target_memory_id(&wasm_name)
            .wrap_err_with(|| eyre::eyre!("Failed to get memory id"))?;
    }

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
