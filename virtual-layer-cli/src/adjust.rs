use std::fs;

use camino::Utf8PathBuf;

pub fn adjust_merged_wasm(path: &Utf8PathBuf) -> anyhow::Result<Utf8PathBuf> {
    let mut module = walrus::Module::from_file(path)?;

    // module
    //     .imports
    //     .remove("$root", "[static]wasip1.fd-write-import")
    //     .expect("fd_write_import not found");

    let import_id = module
        .imports
        .find("$root", "[static]wasip1.fd-write-import")
        .expect("fd_write_import not found");

    let fid = module
        .funcs
        .iter()
        .find(|f| {
            if let walrus::FunctionKind::Import(imported_function) = &f.kind {
                imported_function.import == import_id
            } else {
                false
            }
        })
        .expect("fd_write_import not found")
        .id();

    module
        .replace_imported_func(fid, |(body, _)| {
            body.unreachable();
        })
        .expect("Failed to replace fd_write_import");

    let new_path = path.with_extension("adjusted.wasm");

    if fs::metadata(&new_path).is_ok() {
        fs::remove_file(&new_path).expect("Failed to remove existing file");
    }
    module.emit_wasm_file(new_path.clone())?;

    Ok(new_path)
}
