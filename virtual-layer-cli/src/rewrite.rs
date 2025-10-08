use std::fs;

use camino::Utf8PathBuf;
use eyre::Context as _;
use strum::VariantNames;

use crate::{
    args::TargetMemoryType,
    common::{Wasip1ABIFunc, Wasip1ThreadsABIFunc},
    threads,
    util::{
        CORE_MODULE_ROOT, ResultUtil as _, THREADS_MODULE_ROOT, WalrusFID, WalrusUtilFuncs as _,
        WalrusUtilImport, WalrusUtilModule,
    },
};

/// wasip1 import to adjust to wit
/// block vfs-wasm's environ_sizes_get etc
/// embedding __wasip1_vfs_flag_{name}_memory
pub fn adjust_wasm(
    path: &Utf8PathBuf,
    wasm_names: &[impl AsRef<str>],
    threads: bool,
    debug: bool,
    dwarf: bool,
) -> eyre::Result<Utf8PathBuf> {
    // let mut module = walrus::Module::load(path, dwarf)?;
    let mut module = walrus::Module::load(path, dwarf)?;

    // todo!(); separate block system from environ
    let check = block_func(&mut module, "environ_get", debug)?;
    let next_check = block_func(&mut module, "environ_sizes_get", debug)?;

    if check != next_check {
        eyre::bail!("environ_get and environ_sizes_get are not the same");
    }

    fn block_func(
        module: &mut walrus::Module,
        func_name: impl AsRef<str>,
        debug: bool,
    ) -> eyre::Result<bool> {
        let func_name = func_name.as_ref();
        let export_func_name = format!("__wasip1_vfs_self_{func_name}");
        let func_name = func_name.replace("_", "-");

        if matches!(
            module.exports.iter().find(|e| e.name == export_func_name),
            Some(walrus::Export {
                item: walrus::ExportItem::Function(_),
                ..
            })
        ) {
            module.connect_func_alt(
                (
                    CORE_MODULE_ROOT,
                    &format!("[static]wasip1.{func_name}-import"),
                ),
                &export_func_name,
                debug,
            )?;

            return Ok(true);
        } else {
            return Ok(false);
        }
    }

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
