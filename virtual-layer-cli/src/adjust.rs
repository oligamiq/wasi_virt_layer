use std::{fs, path::Path};

use camino::Utf8PathBuf;
use eyre::{Context as _, ContextCompat};

use crate::{
    common::{VFSExternalMemoryManager, Wasip1Op, Wasip1OpKind, Wasip1SnapshotPreview1Func},
    instrs::InstrRewrite,
    util::{CaminoUtilModule as _, ResultUtil as _, WalrusUtilModule as _},
};

pub fn adjust_merged_wasm(
    path: &Utf8PathBuf,
    wasm_paths: &[impl AsRef<Path>],
    threads: bool,
    debug: bool,
    dwarf: bool,
) -> eyre::Result<Utf8PathBuf> {
    let mut module = walrus::Module::load(path, dwarf)?;

    let vfs_memory_id = module
        .get_target_memory_id("vfs")
        .wrap_err("Failed to get memory id")?;

    #[allow(unused)]
    let vfs_globals = module
        .get_global_anchor("vfs")
        .wrap_err("Failed to get global anchor")?;

    let mut manager = VFSExternalMemoryManager::new(vfs_memory_id, &module);

    for wasm_path in wasm_paths {
        let wasm_name = wasm_path.as_ref().get_file_main_name().unwrap();

        for name in <Wasip1SnapshotPreview1Func as strum::VariantNames>::VARIANTS.iter() {
            let export_name = format!("__wasip1_vfs_{wasm_name}_{name}");

            if module
                .imports
                .find("wasi_snapshot_preview1", name)
                .is_some()
            {
                module
                    .connect_func_without_remove(("wasi_snapshot_preview1", name), &export_name)
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
            .wrap_err("Failed to get memory id")?;

        let globals = module
            .get_global_anchor(&wasm_name)
            .wrap_err("Failed to get global anchor")?;

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
                op.replace(&mut module, memory_id, vfs_memory_id, reset_op.as_ref())
                    .wrap_err_with(|| eyre::eyre!("Failed to replace import on {wasm_name}"))?;
                eyre::Ok(())
            })
            .wrap_err_with(|| eyre::eyre!("Failed to replace Wasm memory access on {wasm_name}"))?;

        reset_op
            .map(|op| {
                op.replace(&mut module, memory_id, vfs_memory_id, None)
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

        // rm memory export
        module
            .exports
            .delete(module.exports.get_exported_memory(memory_id).unwrap().id());

        // threads
        if threads {
            module
                .connect_func_alt(
                    (
                        "wasip1-vfs",
                        &format!("__wasip1_vfs_{wasm_name}_wasi_thread_start"),
                    ),
                    &format!("__wasip1_vfs_wasi_thread_start_{wasm_name}"),
                )
                .wrap_err_with(|| {
                    eyre::eyre!("Failed to connect __wasip1_vfs_wasi_thread_start_{wasm_name}")
                })?;

            module
                .exports
                .remove(&format!(
                    "__wasip1_vfs_{wasm_name}_wasi_thread_start_anchor"
                ))
                .to_eyre()
                .wrap_err_with(|| {
                    eyre::eyre!(
                        "Failed to remove __wasip1_vfs_{wasm_name}_wasi_thread_start_anchor export"
                    )
                })?;

            module
                .connect_func_alt(
                    (
                        "wasi",
                        &format!("__wasip1_vfs_wasi_thread_spawn_{wasm_name}"),
                    ),
                    &format!("__wasip1_vfs_wasi_thread_spawn_{wasm_name}"),
                )
                .wrap_err_with(|| {
                    eyre::eyre!("Failed to connect __wasip1_vfs_wasi_thread_spawn_{wasm_name}")
                })?;
        }
    }

    if threads {
        module
            .memories
            .iter_mut()
            // .skip(1)
            .map(|mem| {
                let id = mem.id();
                let mem_id = module
                    .imports
                    .iter()
                    .find_map(|import| match import.kind {
                        walrus::ImportKind::Memory(mid) if mid == id => Some(import.id()),
                        _ => None,
                    })
                    .wrap_err("Failed to find memory import id")?;

                module.imports.delete(mem_id);
                mem.import = None;

                // Translating component requires WasmFeatures::Threads
                // but we cannot enable it because it in other crates.
                // So, we set shared to false here temporarily.
                mem.shared = false;

                Ok(())
            })
            .collect::<eyre::Result<Vec<_>>>()?;
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

    manager
        .flush(&mut module)
        .wrap_err("Failed to flush memory")?;

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
