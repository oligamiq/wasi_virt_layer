use std::{fs, path::Path};

use camino::Utf8PathBuf;
use eyre::{Context as _, ContextCompat};

use crate::{
    args::TargetMemoryType,
    common::{VFSExternalMemoryManager, Wasip1Op, Wasip1OpKind},
    instrs::InstrRewrite,
    util::{CaminoUtilModule as _, ResultUtil as _, WalrusUtilFuncs, WalrusUtilModule as _},
};

pub fn adjust_merged_wasm(
    path: &Utf8PathBuf,
    wasm_paths: &[impl AsRef<Path>],
    threads: bool,
    mem_type: TargetMemoryType,
    debug: bool,
    dwarf: bool,
) -> eyre::Result<Utf8PathBuf> {
    let mut module = walrus::Module::load(path, dwarf)?;

    let mut manager = VFSExternalMemoryManager::new();

    for wasm_path in wasm_paths {
        let wasm_name = wasm_path.as_ref().get_file_main_name().unwrap();

        let mut ops = module
            .imports
            .iter()
            .filter(|import| import.module == "wasip1-vfs")
            .filter(|import| {
                import
                    .name
                    .starts_with(&format!("__wasip1_vfs_{wasm_name}_"))
            })
            .map(|import| {
                let op = Wasip1Op::parse(&module, import, &wasm_name)
                    .wrap_err("Failed to parse import")?;

                Ok(op)
            })
            .collect::<eyre::Result<Vec<_>>>()
            .wrap_err("Failed to collect imports")?;

        ops.into_iter()
            .try_for_each(|op| {
                op.replace(&mut module, memory_id, vfs_memory_id, debug)
                    .wrap_err_with(|| eyre::eyre!("Failed to replace import on {wasm_name}"))?;
                eyre::Ok(())
            })
            .wrap_err_with(|| eyre::eyre!("Failed to replace Wasm memory access on {wasm_name}"))?;

        module
            .exports
            .remove(&format!("__wasip1_vfs_{wasm_name}__start_anchor"))
            .to_eyre()
            .wrap_err_with(|| {
                eyre::eyre!("Failed to remove __start_anchor export on {wasm_name}.")
            })?;
    }

    let new_path = path.with_extension("adjusted.wasm");

    if fs::metadata(&new_path).is_ok() {
        fs::remove_file(&new_path).wrap_err("Failed to remove existing file")?;
    }
    module
        .producers
        .add_processed_by("virtual-layer-cli", env!("CARGO_PKG_VERSION"));

    module
        .emit_wasm_file(new_path.clone())
        .to_eyre()
        .wrap_err("Failed to write temporary wasm file")?;

    Ok(new_path)
}
