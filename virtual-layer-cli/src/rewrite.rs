use std::{fs, path::Path};

use camino::Utf8PathBuf;
use cargo_metadata::{Metadata, Package};
use eyre::Context as _;
use itertools::Itertools;
use strum::VariantNames;
use toml_edit::{Document, DocumentMut, Item};
use walrus::ir::BinaryOp;

use crate::{
    common::{Wasip1SnapshotPreview1Func, Wasip1SnapshotPreview1ThreadsFunc},
    instrs::InstrRewrite,
    threads,
    util::{
        CORE_MODULE_ROOT, CaminoUtilModule as _, ResultUtil as _, THREADS_MODULE_ROOT,
        WalrusUtilImport, WalrusUtilModule,
    },
};

/// wasip1 import to adjust to wit
/// block vfs-wasm's environ_sizes_get etc
/// embedding __wasip1_vfs_flag_{name}_memory
pub fn adjust_wasm(
    path: &Utf8PathBuf,
    wasm: &[impl AsRef<Path>],
    threads: bool,
) -> eyre::Result<(Utf8PathBuf, TargetMemoryType)> {
    let mut module = walrus::Module::from_file(path)
        .to_eyre()
        .wrap_err("Failed to load module")?;

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

        // module
        // .imports
        // .find_mut(root, &component_name)
        // .map(|import| {
        //     import.module = "archived".to_string();
        // })
        // .ok_or_else(|| eyre::eyre!("{component_name} import not found"))?;

        // module
        //     .imports
        //     .find_mut("wasi_snapshot_preview1", name)
        //     .map(|import| {
        //         import.module = root.to_string();
        //         import.name = component_name;
        //     });

        module
            .imports
            .swap_import(root, &component_name, "wasi_snapshot_preview1", name)
            .wrap_err("thread-spawn import not found")?;
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

            // module
            //     .imports
            //     .swap_import(root, &component_name, "wasi", "thread-spawn")
            //     .wrap_err("thread-spawn import not found")?;

            let import_root_thread_spawn_fn_id = module
                .imports
                .get_func(root, &component_name)
                .to_eyre()
                .wrap_err_with(|| eyre::eyre!("{component_name} import not found"))?;

            // rewrite call id in export.__wasip1_vfs_root_spawn_anchor
            let anchor_fid = module
                .exports
                .get_func("__wasip1_vfs_root_spawn_anchor")
                .to_eyre()
                .wrap_err("__wasip1_vfs_root_spawn_anchor not found")?;

            let fid = module
                .imports
                .get_func("wasi", "thread-spawn")
                .to_eyre()
                .wrap_err("wasi.thread-spawn import not found")?;

            for child in module
                .find_children(anchor_fid)?
                .into_iter()
                .chain(core::iter::once(anchor_fid))
            {
                let keys = {
                    let child = module.funcs.get_mut(child);
                    let func = match &mut child.kind {
                        walrus::FunctionKind::Local(imported_function) => imported_function,
                        _ => continue,
                    };
                    func.builder_mut()
                        .func_body()
                        .rewrite(|instr, (pos, key)| {
                            if let Some(call) = instr.call_mut() {
                                if call.func == fid {
                                    call.func = import_root_thread_spawn_fn_id;
                                }
                            }
                            if let Some(call) = instr.call_indirect_mut() {
                                return Some((pos, key, (call.table, call.ty)));
                            }
                            None
                        })
                        .wrap_err("Failed to rewrite thread-spawn call in root spawn")?
                        .into_iter()
                        .filter_map(|key| key)
                        .collect::<Vec<_>>()
                };

                let fid_pos = module.fid_pos_on_table(fid)?;

                for (table, keys) in keys
                    .into_iter()
                    .into_group_map_by(|(_, _, (table, _))| *table)
                    .into_iter()
                {
                    let fid = fid_pos
                        .iter()
                        .filter(|(tid, _)| *tid == table)
                        .map(|(_, pos)| *pos as i32)
                        .collect::<Vec<_>>();

                    if fid.is_empty() {
                        continue;
                    }

                    if fid.len() > 1 {
                        log::warn!("Multiple fid pos found on table, why? using the first one");
                    }

                    let fid = fid[0];

                    let new_func = {
                        use walrus::*;

                        let ty_id = keys
                            .iter()
                            .find(|(_, _, (table, _))| *table == *table)
                            .unwrap()
                            .2
                            .1;

                        let ty = module.types.get(ty_id);
                        let params_ty = core::iter::once(ValType::I32)
                            .chain(ty.params().iter().copied())
                            .collect::<Vec<_>>();
                        let results_ty = ty.results().to_vec();

                        let args = params_ty
                            .iter()
                            .map(|ty| module.locals.add(*ty))
                            .collect::<Vec<_>>();

                        let mut func =
                            FunctionBuilder::new(&mut module.types, &params_ty, &results_ty);
                        func.func_body()
                            .local_get(args[0])
                            .i32_const(fid)
                            .binop(BinaryOp::I32Eq)
                            .if_else(
                                ValType::I32,
                                |cons| {
                                    for arg in args.iter().skip(1) {
                                        cons.local_get(*arg);
                                    }
                                    cons.call(import_root_thread_spawn_fn_id).return_();
                                },
                                |els| {
                                    for arg in args.iter().skip(1) {
                                        els.local_get(*arg);
                                    }
                                    els.call_indirect(ty_id, table);
                                },
                            );
                        func.finish(args, &mut module.funcs)
                    };

                    let child = module.funcs.get_mut(child);
                    let func = match &mut child.kind {
                        walrus::FunctionKind::Local(imported_function) => imported_function,
                        _ => continue,
                    };
                    for (pos, key, _) in keys
                        .into_iter()
                        .into_group_map_by(|(_, key, _)| *key)
                        .into_values()
                        .flat_map(|keys| {
                            keys.into_iter()
                                .sorted_by(|(a, _, _), (b, _, _)| a.cmp(&b))
                                .rev()
                        })
                    {
                        let mut seq = func.builder_mut().instr_seq(key);
                        seq.instrs_mut().remove(pos);
                        seq.call_at(pos, new_func);
                    }
                }
            }

            module.connect_func(
                "wasi",
                "thread-spawn",
                "__wasip1_vfs_wasi_thread_spawn_self",
            )?;

            let dup_id = module
                .imports
                .get_func("wasip1-vfs", "__wasip1_vfs_self_wasi_thread_start")
                .to_eyre()
                .wrap_err("Failed to get wasip1-vfs.__wasip1_vfs_self_wasi_thread_start")?;

            for (id, _, _) in module
                .get_using_func(dup_id)
                .wrap_err("Failed to get using func")?
            {
                module
                    .rewrite(
                        |instr, _| {
                            if let walrus::ir::Instr::Call(call) = instr {
                                if call.func == dup_id {
                                    call.func = fid;
                                }
                            }
                        },
                        id,
                    )
                    .wrap_err("Failed to rewrite self_wasi_thread_start call in root spawn")?;
            }

            module.renew_id_on_table(dup_id, fid)?;

            // __wasip1_vfs_self_wasi_thread_start
            module
                .connect_func(
                    "wasip1-vfs",
                    "__wasip1_vfs_wasi_thread_start_entry",
                    "wasi_thread_start",
                )
                .wrap_err("Failed to connect wasip1-vfs.wasi_thread_start")?;

            module
                .exports
                .remove("__wasip1_vfs_root_spawn_anchor")
                .unwrap();
        }
    }

    // todo!(); separate block system from environ
    let check = block_func(&mut module, "environ_get")?;
    let next_check = block_func(&mut module, "environ_sizes_get")?;

    if check != next_check {
        eyre::bail!("environ_get and environ_sizes_get are not the same");
    }

    fn block_func(module: &mut walrus::Module, func_name: impl AsRef<str>) -> eyre::Result<bool> {
        let func_name = func_name.as_ref();
        let export_func_name = format!("__wasip1_vfs_{func_name}");
        let func_name = func_name.replace("_", "-");

        if matches!(
            module.exports.iter().find(|e| e.name == export_func_name),
            Some(walrus::Export {
                item: walrus::ExportItem::Function(_),
                ..
            })
        ) {
            let import_func_name = format!("[static]wasip1.{func_name}-import");
            module.connect_func(CORE_MODULE_ROOT, import_func_name, export_func_name)?;

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

    let new_path = path.with_extension("adjusted.wasm");

    if fs::metadata(&new_path).is_ok() {
        fs::remove_file(&new_path).expect("Failed to remove existing file");
    }
    module
        .emit_wasm_file(new_path.clone())
        .to_eyre()
        .wrap_err("Failed to emit wasm file")?;

    Ok((new_path, target_memory_type))
}

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumString, strum::Display)]
pub enum TargetMemoryType {
    #[strum(ascii_case_insensitive)]
    Single,
    #[strum(ascii_case_insensitive)]
    Multi,
}

const CRATE: &'static str = "wasip1-virtual-layer";

pub fn get_target_feature(
    building_crate: &Package,
    feature: impl AsRef<str>,
) -> eyre::Result<bool> {
    let manifest_path = building_crate.manifest_path.clone();

    let file_data = fs::read_to_string(&manifest_path).wrap_err("Failed to read manifest file")?;
    let doc = Document::parse(&file_data).expect("invalid doc");

    let crate_setting = &doc["dependencies"][CRATE];

    if matches!(crate_setting, Item::None) {
        eyre::bail!("Crate `{CRATE}` not found in dependencies");
    }

    Ok(has_feature(crate_setting, feature.as_ref()))
}

pub fn adjust_target_feature(
    metadata: &Metadata,
    building_crate: &Package,
    on: bool,
    feature: impl AsRef<str>,
) -> eyre::Result<()> {
    let feature = feature.as_ref();

    let manifest_path = building_crate.manifest_path.clone();

    let file_data = fs::read_to_string(&manifest_path).wrap_err("Failed to read manifest file")?;
    let mut doc = file_data.parse::<DocumentMut>().expect("invalid doc");

    let crate_setting = &mut doc["dependencies"][CRATE];

    if matches!(crate_setting, Item::None) {
        eyre::bail!("Crate `{CRATE}` not found in dependencies");
    }

    enum HasFeature {
        Disabled,
        EnabledOnNormal,
        EnabledOnWorkspace,
    }

    // check normal crate setting
    let crate_has_feature = if has_feature(crate_setting, feature) {
        HasFeature::EnabledOnNormal
    } else {
        HasFeature::Disabled
    };
    // check workspace
    let crate_has_feature = if matches!(crate_has_feature, HasFeature::Disabled) {
        match &crate_setting["workspace"] {
            v if v.as_bool().unwrap_or(false) => {
                let manifest_path = metadata.workspace_root.join("Cargo.toml");

                let file_data = fs::read_to_string(&manifest_path)
                    .wrap_err("Failed to read workspace manifest file")?;
                let mut doc = file_data.parse::<DocumentMut>().expect("invalid doc");

                let crate_setting = &mut doc["workspace"]["dependencies"][CRATE];

                if has_feature(crate_setting, feature) {
                    HasFeature::EnabledOnWorkspace
                } else {
                    HasFeature::Disabled
                }
            }
            _ => HasFeature::Disabled,
        }
    } else {
        crate_has_feature
    };

    match (crate_has_feature, on) {
        (HasFeature::Disabled, true) => {
            set_table(crate_setting, feature, on)?;
            std::fs::write(&manifest_path, doc.to_string())
                .wrap_err("Failed to write manifest file")?;
            Ok(())
        }
        (HasFeature::EnabledOnNormal, false) => {
            set_table(crate_setting, feature, on)?;
            std::fs::write(&manifest_path, doc.to_string())
                .wrap_err("Failed to write manifest file")?;
            Ok(())
        }
        (HasFeature::EnabledOnWorkspace, false) => {
            log::warn!(
                "Feature `{feature}` is enabled on workspace, so changing it may affect other crates."
            );

            let manifest_path = metadata.workspace_root.join("Cargo.toml");

            let file_data = fs::read_to_string(&manifest_path)
                .wrap_err("Failed to read workspace manifest file")?;
            let mut doc = file_data.parse::<DocumentMut>().expect("invalid doc");

            let crate_setting = &mut doc["workspace"]["dependencies"][CRATE];

            set_table(crate_setting, feature, on)?;

            std::fs::write(&manifest_path, doc.to_string())
                .wrap_err("Failed to write workspace manifest file")?;

            Ok(())
        }
        _ => Ok(()),
    }
}

fn has_feature(item: &Item, feature: &str) -> bool {
    match item {
        Item::Table(table) => table["features"]
            .as_array()
            .map(|arr| arr.iter().any(|s| s.as_str() == Some(feature)))
            .unwrap_or(false),
        _ => false,
    }
}

fn set_table(table: &mut Item, feature: &str, on: bool) -> eyre::Result<()> {
    if on {
        if matches!(table.get("features"), None) {
            table["features"] = toml_edit::value(toml_edit::Array::new());
        }
        if table["features"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|s| s.as_str())
            .any(|s| s == feature)
        {
            return Ok(());
        }
        table["features"].as_array_mut().unwrap().push(feature);
    } else {
        if matches!(table.get("features"), None) {
            return Ok(());
        }
        if let Some(i) = {
            table["features"]
                .as_array()
                .unwrap()
                .iter()
                .enumerate()
                .filter_map(|(i, s)| s.as_str().map(|s| (i, s)))
                .find(|(_, s)| *s == feature)
                .map(|(i, _)| i)
        } {
            table["features"].as_array_mut().unwrap().remove(i);
            if table["features"].as_array().unwrap().is_empty() {
                table["features"] = Item::None;
            }
        }
    }

    Ok(())
}
