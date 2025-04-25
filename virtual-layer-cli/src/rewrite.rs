use std::fs;

use camino::Utf8PathBuf;

use crate::util::{WalrusUtilImport, WalrusUtilModule};

const WASIP1_FUNC: [&str; 4] = ["fd_write", "environ_sizes_get", "environ_get", "proc_exit"];

/// wasip1 import to adjust to wit
/// block vfs-wasm's environ_sizes_get etc
/// embedding __wasip1_vfs_flag_{name}_memory
pub fn adjust_wasm(path: &Utf8PathBuf) -> anyhow::Result<Utf8PathBuf> {
    let mut module = walrus::Module::from_file(path)?;

    for name in WASIP1_FUNC.iter() {
        let component_name = format!("[static]wasip1.{}-import", name.replace("_", "-"));

        module
            .exports
            .remove(format!("{name}_import_wrap"))
            .expect(format!("{name} not found").as_str());

        module
            .imports
            .find_mut("$root", &component_name)
            .map(|import| {
                import.module = "archived".to_string();
            })
            .ok_or_else(|| anyhow::anyhow!("{name} import not found"))?;

        module
            .imports
            .find_mut("wasi_snapshot_preview1", name)
            .map(|import| {
                import.module = "$root".to_string();
                import.name = component_name;
            })
            .ok_or_else(|| anyhow::anyhow!("{name} import not found"))?;
    }

    let check = block_func(&mut module, "environ_get")?;
    let next_check = block_func(&mut module, "environ_sizes_get")?;

    if check != next_check {
        return Err(anyhow::anyhow!(
            "environ_get and environ_sizes_get are not the same"
        ));
    }

    fn block_func(module: &mut walrus::Module, func_name: impl AsRef<str>) -> anyhow::Result<bool> {
        let export_func_name = format!("__wasip1_vfs_{}", func_name.as_ref());

        if matches!(
            module.exports.iter().find(|e| e.name == export_func_name),
            Some(walrus::Export {
                item: walrus::ExportItem::Function(_),
                ..
            })
        ) {
            let import_func_name = format!(
                "[static]wasip1.{}-import",
                func_name.as_ref().replace("_", "-")
            );
            module.connect_func("$root", import_func_name, export_func_name)?;

            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    let new_path = path.with_extension("adjusted.wasm");

    if fs::metadata(&new_path).is_ok() {
        fs::remove_file(&new_path).expect("Failed to remove existing file");
    }
    module.emit_wasm_file(new_path.clone())?;

    Ok(new_path)
}
