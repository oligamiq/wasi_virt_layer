use std::{fs, path::Path};

use camino::Utf8PathBuf;
use eyre::Context as _;

use crate::{
    common::{VFSExternalMemoryManager, WASIP1_FUNC, Wasip1Op, Wasip1OpKind},
    util::{CaminoUtilModule as _, ResultUtil as _, WalrusUtilModule as _},
};

pub fn adjust_merged_wasm(
    path: &Utf8PathBuf,
    wasm: &[impl AsRef<Path>],
) -> eyre::Result<Utf8PathBuf> {
    let mut module = walrus::Module::from_file(path)
        .to_eyre()
        .wrap_err_with(|| eyre::eyre!("Failed to load module"))?;

    let vfs_memory_id = module
        .get_target_memory_id("vfs")
        .wrap_err_with(|| eyre::eyre!("Failed to get memory id"))?;

    let mut manager = VFSExternalMemoryManager::new(vfs_memory_id, &module);

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
                let op = Wasip1Op::parse(&module, import, &wasm_name, &mut manager, memory_id)
                    .wrap_err_with(|| eyre::eyre!("Failed to parse import"))?;

                Ok(op)
            })
            .collect::<eyre::Result<Vec<_>>>()
            .wrap_err_with(|| eyre::eyre!("Failed to collect imports"))?;

        let reset_op = ops
            .iter()
            .enumerate()
            .find(|(_, op)| matches!(op.kind, Wasip1OpKind::Reset { .. }))
            .map(|(reset_op_i, _)| reset_op_i)
            .map(|reset_op_i| ops.remove(reset_op_i));

        ops.into_iter()
            .map(|op| {
                op.replace(&mut module, memory_id, vfs_memory_id, reset_op.as_ref())
                    .wrap_err_with(|| eyre::eyre!("Failed to replace import"))?;
                Ok(())
            })
            .collect::<eyre::Result<Vec<_>>>()
            .wrap_err_with(|| eyre::eyre!("Failed to replace imports"))?;

        reset_op
            .map(|op| {
                op.replace(&mut module, memory_id, vfs_memory_id, None)
                    .wrap_err_with(|| eyre::eyre!("Failed to replace import"))
            })
            .transpose()
            .wrap_err_with(|| eyre::eyre!("Failed to replace imports"))?;

        module
            .exports
            .iter_mut()
            .find(|export| export.name == format!("__wasip1_vfs_{wasm_name}__start_wrap"))
            .map(|export| {
                export.name = format!("_{wasm_name}_start").into();
            })
            .ok_or_else(|| eyre::eyre!("Failed to get export"))?;

        // rm memory export
        module
            .exports
            .delete(module.exports.get_exported_memory(memory_id).unwrap().id());
    }

    manager
        .flush(&mut module)
        .wrap_err_with(|| eyre::eyre!("Failed to flush memory"))?;

    // rename vfs memory to "memory"
    // because this memory is used by wit-bindgen
    // and the name is hardcoded in the generated code
    module
        .exports
        .iter_mut()
        .find(|export| match export.item {
            walrus::ExportItem::Memory(memory) => memory == vfs_memory_id,
            _ => false,
        })
        .map(|export| {
            export.name = "memory".into();
        })
        .unwrap();

    let new_path = path.with_extension("adjusted.wasm");

    if fs::metadata(&new_path).is_ok() {
        fs::remove_file(&new_path).expect("Failed to remove existing file");
    }
    module
        .producers
        .add_processed_by("virtual-layer-cli", env!("CARGO_PKG_VERSION"));

    module
        .emit_wasm_file(new_path.clone())
        .to_eyre()
        .wrap_err_with(|| eyre::eyre!("Failed to emit wasm file"))?;

    Ok(new_path)
}
