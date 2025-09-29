use std::{fs, path::Path};

use camino::Utf8PathBuf;
use eyre::Context as _;
use strum::VariantNames;

use crate::{
    args::TargetMemoryType,
    common::{Wasip1SnapshotPreview1Func, Wasip1SnapshotPreview1ThreadsFunc},
    threads,
    util::{
        CORE_MODULE_ROOT, CaminoUtilModule as _, ResultUtil as _, THREADS_MODULE_ROOT, WalrusFID,
        WalrusUtilFuncs as _, WalrusUtilImport, WalrusUtilModule,
    },
};

/// wasip1 import to adjust to wit
/// block vfs-wasm's environ_sizes_get etc
/// embedding __wasip1_vfs_flag_{name}_memory
pub fn adjust_wasm(
    path: &Utf8PathBuf,
    wasm: &[impl AsRef<Path>],
    threads: bool,
    debug: bool,
    dwarf: bool,
) -> eyre::Result<(Utf8PathBuf, TargetMemoryType, bool)> {
    // let mut module = walrus::Module::load(path, dwarf)?;
    let mut module = walrus::Module::load(path, dwarf)?;

    if !<Wasip1SnapshotPreview1Func as VariantNames>::VARIANTS
        .iter()
        .chain(<Wasip1SnapshotPreview1ThreadsFunc as VariantNames>::VARIANTS)
        .any(|name| {
            module
                .exports
                .iter()
                .any(|e| e.name == format!("{name}_import_anchor"))
        })
    {
        eyre::bail!(
            r#"This wasm file is not use "wasip1-virtual-layer" crate, you need to add it to your dependencies and use wasip1_virtual_layer;"#
        );
    }

    if threads {
        threads::remove_unused_threads_function(&mut module)?;
    }

    // check use import_wasm!
    for wasm in wasm {
        let wasm_name = wasm.as_ref().get_file_main_name().unwrap();

        if !module
            .exports
            .iter()
            .any(|export| export.name == format!("__wasip1_vfs_{wasm_name}__start_anchor"))
        {
            eyre::bail!(
                "Failed to get __start_anchor export on {wasm_name}. You may forget definition `import_wasm!` macro with wasm name."
            );
        }
    }

    fn gen_component_name(namespace: &str, name: &str) -> String {
        format!("[static]{namespace}.{}-import", name.replace("_", "-"))
    }

    for (name, (namespace, root)) in <Wasip1SnapshotPreview1Func as VariantNames>::VARIANTS
        .iter()
        .zip(core::iter::repeat(("wasip1", CORE_MODULE_ROOT)))
    {
        let component_name = gen_component_name(namespace, name);

        module
            .exports
            .remove(format!("{name}_import_anchor"))
            .to_eyre()
            .wrap_err_with(|| eyre::eyre!("{name}_import_anchor not found"))?;

        module
            .imports
            .may_swap_import((root, &component_name), ("wasi_snapshot_preview1", name))?;
    }

    // Relocate thread creation from root spawn to the outer layer
    if threads {
        for (name, (namespace, root)) in
            <Wasip1SnapshotPreview1ThreadsFunc as VariantNames>::VARIANTS
                .iter()
                .zip(core::iter::repeat(("wasip1-threads", THREADS_MODULE_ROOT)))
        {
            let component_name = gen_component_name(namespace, name);

            module
                .exports
                .remove(format!("{name}_import_anchor"))
                .to_eyre()
                .wrap_err_with(|| eyre::eyre!("{name}_import_anchor not found"))?;

            let real_thread_spawn_fn_id = (root, &component_name).get_fid(&module.imports)?;

            let branch_fid = "__wasip1_vfs_is_root_spawn".get_fid(&module.exports)?;

            let normal_thread_spawn_fn_id = ("wasi", "thread-spawn").get_fid(&module.imports)?;

            let self_thread_spawn_fn_id = "__wasip1_vfs_wasi_thread_spawn_self".get_fid(&module)?;

            module
                .exports
                .remove("__wasip1_vfs_wasi_thread_spawn_self")
                .unwrap();

            let debug_something_id = "debug_something".get_fid(&module.exports).ok();

            use walrus::ValType::I32;
            let real_thread_spawn_fn_id = module
                .add_func(&[I32], &[I32], |builder, args| {
                    let mut body = builder.func_body();
                    if let Some(debug_something_id) = debug_something_id {
                        body.call(debug_something_id);
                    }
                    body.call(branch_fid)
                        .if_else(
                            I32,
                            |then| {
                                if let Some(debug_something_id) = debug_something_id {
                                    then.call(debug_something_id);
                                }
                                then.local_get(args[0]) // pass the argument to thread-spawn
                                    .call(real_thread_spawn_fn_id);
                            },
                            |else_| {
                                if let Some(debug_something_id) = debug_something_id {
                                    else_.call(debug_something_id);
                                }
                                else_
                                    .local_get(args[0]) // pass the argument to thread-spawn
                                    .call(self_thread_spawn_fn_id); // call thread-spawn
                            },
                        )
                        .return_();

                    Ok(())
                })
                .wrap_err("Failed to add real thread spawn function")?;

            module
                .renew_call_fn(normal_thread_spawn_fn_id, real_thread_spawn_fn_id)
                .wrap_err("Failed to rewrite thread-spawn call")?;

            let exporting_thread_starter_id = "wasi_thread_start".get_fid(&module.exports)?;

            module
                .connect_func_alt(
                    ("wasip1-vfs", "__wasip1_vfs_self_wasi_thread_start"),
                    exporting_thread_starter_id,
                    debug,
                )
                .wrap_err("Failed to rewrite self_wasi_thread_start call in root spawn")?;

            if !debug {
                module
                    .exports
                    .remove("__wasip1_vfs_self_wasi_thread_start_anchor")
                    .to_eyre()
                    .wrap_err(
                        "Failed to remove __wasip1_vfs_self_wasi_thread_start_anchor export",
                    )?;
            }

            if debug {
                module
                    .exports
                    .add("real_thread_spawn_fn", real_thread_spawn_fn_id);
            }

            // __wasip1_vfs_self_wasi_thread_start
            module
                .renew_call_fn(
                    ("wasip1-vfs", "__wasip1_vfs_wasi_thread_start_entry"),
                    exporting_thread_starter_id,
                )
                .wrap_err("Failed to connect wasip1-vfs.wasi_thread_start")?;

            if !debug {
                module.exports.remove("__wasip1_vfs_is_root_spawn").unwrap();
            }
        }
    }

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

    module.create_global_anchor("vfs")?;

    let (target_memory_type, eid) = module
        .exports
        .iter()
        .find(|e| e.name == "__wasip1_vfs_flag_vfs_multi_memory")
        .map(|e| Ok((TargetMemoryType::Multi, e.id())))
        .unwrap_or(
            module
                .exports
                .iter()
                .find(|e| e.name == "__wasip1_vfs_flag_vfs_single_memory")
                .map(|e| Ok((TargetMemoryType::Single, e.id())))
                .unwrap_or(Err(eyre::eyre!("No target memory type found"))),
        )?;

    module.exports.delete(eid);

    if debug {
        for wasm in wasm {
            let wasm_name = wasm.as_ref().get_file_main_name().unwrap();

            module
                .exports
                .iter()
                .filter(|export| {
                    export
                        .name
                        .starts_with(&format!("__wasip1_vfs_{wasm_name}_"))
                })
                .filter(|export| {
                    Wasip1SnapshotPreview1Func::VARIANTS.contains(
                        &export
                            .name
                            .as_str()
                            .trim_start_matches(&format!("__wasip1_vfs_{wasm_name}_")),
                    )
                })
                .filter_map(|export| match export.item {
                    walrus::ExportItem::Function(fid) => Some((export.name.clone(), fid)),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .into_iter()
                .for_each(|(name, old_fid)| {
                    module.exports.add(&format!("debug_{name}"), old_fid);
                });
        }
    }

    let mut has_debug_call_memory_grow = false;

    if threads {
        if debug {
            // module.memories.iter_mut().for_each(|memory| {
            //     memory.initial = (memory.initial + memory.maximum.unwrap_or(memory.initial)) / 2;
            // });

            if let Some((id, id2)) = "debug_call_memory_grow"
                .get_fid(&module.exports)
                .ok()
                .and_then(|id| {
                    "debug_call_memory_grow_pre"
                        .get_fid(&module.exports)
                        .ok()
                        .map(|id2| (id, id2))
                })
            {
                has_debug_call_memory_grow = true;

                module
                    .gen_inspect_with_finalize(
                        Some(id),
                        Some(id2),
                        &[walrus::ValType::I32],
                        &[walrus::ValType::I32],
                        &module.funcs.find_children_with(id, false).unwrap(),
                        |instr| {
                            if let walrus::ir::Instr::MemoryGrow(walrus::ir::MemoryGrow {
                                memory: _,
                                ..
                            }) = instr
                            {
                                static mut Z: i32 = 17;
                                unsafe { Z += 1 };
                                Some([0, unsafe { Z }])
                            } else {
                                None
                            }
                        },
                    )
                    .unwrap();
            }
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

    Ok((new_path, target_memory_type, has_debug_call_memory_grow))
}
