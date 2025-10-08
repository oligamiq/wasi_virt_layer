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
                let op = Wasip1Op::parse(
                    &module,
                    import,
                    &wasm_name,
                    &mut manager,
                    memory_id,
                    globals.clone(),
                )
                .wrap_err("Failed to parse import")?;

                Ok(op)
            })
            .collect::<eyre::Result<Vec<_>>>()
            .wrap_err("Failed to collect imports")?;

        let reset_op = ops
            .iter()
            .enumerate()
            .find(|(_, op)| matches!(op.kind, Wasip1OpKind::Reset { .. }))
            .map(|(reset_op_i, _)| reset_op_i)
            .map(|reset_op_i| ops.remove(reset_op_i));

        ops.into_iter()
            .try_for_each(|op| {
                op.replace(
                    &mut module,
                    memory_id,
                    vfs_memory_id,
                    reset_op.as_ref(),
                    debug,
                )
                .wrap_err_with(|| eyre::eyre!("Failed to replace import on {wasm_name}"))?;
                eyre::Ok(())
            })
            .wrap_err_with(|| eyre::eyre!("Failed to replace Wasm memory access on {wasm_name}"))?;

        reset_op
            .map(|op| {
                op.replace(&mut module, memory_id, vfs_memory_id, None, debug)
                    .wrap_err_with(|| eyre::eyre!("Failed to replace import on {wasm_name}"))
            })
            .transpose()
            .wrap_err("Failed to implement reset wasm memory etc before call main function")?;

        module
            .exports
            .remove(&format!("__wasip1_vfs_{wasm_name}__start_anchor"))
            .to_eyre()
            .wrap_err_with(|| {
                eyre::eyre!("Failed to remove __start_anchor export on {wasm_name}.")
            })?;

        // module
        //     .exports
        //     .iter_mut()
        //     .find(|export| export.name == format!("__wasip1_vfs_{wasm_name}__start_anchor"))
        //     .map(|export| {
        //         export.name = format!("_{wasm_name}_start").into();
        //     })
        //     .ok_or_else(|| eyre::eyre!("Failed to get __start_anchor export on {wasm_name}."))?;
    }

    // memory_init(memory, data)
    // fn(&mut self, Id<Memory>, Id<Data>)
    // data_drop(&mut self, data: DataId)
    // so we remove all data_drop sections.
    module
        .funcs
        .iter_mut()
        .map(|func| {
            match &mut func.kind {
                walrus::FunctionKind::Local(l) => {
                    l.builder_mut()
                        .func_body()
                        .retain(|instr, _| !instr.is_data_drop());
                }
                _ => {}
            }
            Ok(())
        })
        .collect::<eyre::Result<Vec<_>>>()?;

    let mem_id = manager
        .flush(&mut module)
        .wrap_err("Failed to flush memory")?;

    // If there are any leftover exports created to connect your own imports and ABI, delete them.
    module
        .exports
        .iter()
        .filter_map(|export| match export.item {
            walrus::ExportItem::Function(fid) if export.name.starts_with("__wasip1_vfs_self_") => {
                Some((export.id(), fid))
            }
            _ => None,
        })
        .collect::<Vec<_>>()
        .iter()
        .copied()
        .for_each(|(id, fid)| {
            if !debug {
                module.funcs.delete(fid);
                module.exports.delete(id);
            }
        });

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
