use std::fs;

use camino::Utf8PathBuf;

const WASIP1_FUNC: [&str; 4] = ["fd_write", "environ_sizes_get", "environ_get", "proc_exit"];

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
            .iter_mut()
            .filter(|e| matches!(e.kind, walrus::ImportKind::Function(_)))
            .find(|e| e.module == "$root" && e.name == component_name)
            .map(|e| {
                e.module = "archived".to_string();
            })
            .ok_or_else(|| anyhow::anyhow!("{name} import not found"))?;

        module
            .imports
            .iter_mut()
            .filter(|e| matches!(e.kind, walrus::ImportKind::Function(_)))
            .find(|e| e.module == "wasi_snapshot_preview1" && e.name == *name)
            .map(|e| {
                e.module = "$root".to_string();
                e.name = component_name;
            })
            .ok_or_else(|| anyhow::anyhow!("{name} import not found"))?;
    }

    let new_path = path.with_extension("adjusted.wasm");

    if fs::metadata(&new_path).is_ok() {
        fs::remove_file(&new_path).expect("Failed to remove existing file");
    }
    module.emit_wasm_file(new_path.clone())?;

    Ok(new_path)
}
