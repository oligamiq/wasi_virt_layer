use std::{fs, path::Path};

use camino::Utf8PathBuf;
use eyre::Context as _;

use crate::{
    common::{VFSExternalMemoryManager, Wasip1Op, Wasip1OpKind, Wasip1SnapshotPreview1Func},
    util::{CaminoUtilModule as _, ResultUtil as _, WalrusUtilModule as _},
};

pub fn director(
    path: &Utf8PathBuf,
    wasm: &[impl AsRef<Path>],
    is_single_memory: bool,
) -> eyre::Result<Utf8PathBuf> {
    let mut module = walrus::Module::from_file(path)
        .to_eyre()
        .wrap_err_with(|| eyre::eyre!("Failed to load module"))?;

    let wasm = wasm.iter().map(|p| {
        camino::Utf8PathBuf::from_path_buf(p.as_ref().to_owned())
            .map_err(|_| eyre::eyre!("Invalid path: {}", p.as_ref().display()))
    });
    for wasm in wasm {
        let wasm = wasm?;

        print!("Wasm file: {wasm}");
        let wasm_name = wasm.get_file_main_name().unwrap();

        print!("Wasm name: {wasm_name}");
    }

    // __wasip1_vfs_test_wasm_opt_memory_directer
    // let id = module.imports.get_func("wasip1-vfs", "__wasip1_vfs_test_wasm_opt_memory_directer")
    //     .wrap_err_with(|| eyre::eyre!("Failed to get import function"))?;
    // module.replace_imported_func(

    // )

    Ok(path.clone())
}
