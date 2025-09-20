use std::{fs, path::Path};

use camino::Utf8PathBuf;
use cargo_metadata::{Metadata, Package};
use eyre::Context as _;
use strum::VariantNames;
use toml_edit::{Document, DocumentMut, Item};

use crate::{
    common::{Wasip1SnapshotPreview1Func, Wasip1SnapshotPreview1ThreadsFunc},
    threads,
    util::{
        CORE_MODULE_ROOT, CaminoUtilModule as _, ResultUtil as _, THREADS_MODULE_ROOT, WalrusFID,
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
    debug: bool,
    dwarf: bool,
) -> eyre::Result<(Utf8PathBuf, TargetMemoryType)> {
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

    // if threads {
    //     threads::remove_unused_threads_function(&mut module)?;
    // }

    // // check use import_wasm!
    // for wasm in wasm {
    //     let wasm_name = wasm.as_ref().get_file_main_name().unwrap();

    //     if !module
    //         .exports
    //         .iter()
    //         .any(|export| export.name == format!("__wasip1_vfs_{wasm_name}__start_anchor"))
    //     {
    //         eyre::bail!(
    //             "Failed to get __start_anchor export on {wasm_name}. You may forget definition `import_wasm!` macro with wasm name."
    //         );
    //     }
    // }

    // fn gen_component_name(namespace: &str, name: &str) -> String {
    //     format!("[static]{namespace}.{}-import", name.replace("_", "-"))
    // }

    // for (name, (namespace, root)) in <Wasip1SnapshotPreview1Func as VariantNames>::VARIANTS
    //     .iter()
    //     .zip(core::iter::repeat(("wasip1", CORE_MODULE_ROOT)))
    // {
    //     let component_name = gen_component_name(namespace, name);

    //     module
    //         .exports
    //         .remove(format!("{name}_import_anchor"))
    //         .to_eyre()
    //         .wrap_err_with(|| eyre::eyre!("{name}_import_anchor not found"))?;

    //     module
    //         .imports
    //         .may_swap_import((root, &component_name), ("wasi_snapshot_preview1", name))?;
    // }

    // // Relocate thread creation from root spawn to the outer layer
    // if threads {
    //     for (name, (namespace, root)) in
    //         <Wasip1SnapshotPreview1ThreadsFunc as VariantNames>::VARIANTS
    //             .iter()
    //             .zip(core::iter::repeat(("wasip1-threads", THREADS_MODULE_ROOT)))
    //     {
    //         let component_name = gen_component_name(namespace, name);

    //         module
    //             .exports
    //             .remove(format!("{name}_import_anchor"))
    //             .to_eyre()
    //             .wrap_err_with(|| eyre::eyre!("{name}_import_anchor not found"))?;

    //         let real_thread_spawn_fn_id = (root, &component_name).get_fid(&module.imports)?;

    //         let branch_fid = "__wasip1_vfs_is_root_spawn".get_fid(&module.exports)?;

    //         let normal_thread_spawn_fn_id = ("wasi", "thread-spawn").get_fid(&module.imports)?;

    //         let self_thread_spawn_fn_id = "__wasip1_vfs_wasi_thread_spawn_self".get_fid(&module)?;

    //         let debug_something_id = if debug {
    //             Some("debug_something".get_fid(&module.exports)?)
    //         } else {
    //             None
    //         };

    //         use walrus::ValType::I32;
    //         let real_thread_spawn_fn_id = module
    //             .add_func(&[I32], &[I32], |builder, args| {
    //                 let mut body = builder.func_body();
    //                 if let Some(debug_something_id) = debug_something_id {
    //                     body.call(debug_something_id);
    //                 }
    //                 body.call(branch_fid)
    //                     .if_else(
    //                         I32,
    //                         |then| {
    //                             if let Some(debug_something_id) = debug_something_id {
    //                                 then.call(debug_something_id);
    //                             }
    //                             then.local_get(args[0]) // pass the argument to thread-spawn
    //                                 .call(real_thread_spawn_fn_id);
    //                         },
    //                         |else_| {
    //                             if let Some(debug_something_id) = debug_something_id {
    //                                 else_.call(debug_something_id);
    //                             }
    //                             else_
    //                                 .local_get(args[0]) // pass the argument to thread-spawn
    //                                 .call(self_thread_spawn_fn_id); // call thread-spawn
    //                         },
    //                     )
    //                     .return_();

    //                 Ok(())
    //             })
    //             .wrap_err("Failed to add real thread spawn function")?;

    //         module
    //             .renew_call_fn(normal_thread_spawn_fn_id, real_thread_spawn_fn_id)
    //             .wrap_err("Failed to rewrite thread-spawn call")?;

    //         let exporting_thread_starter_id = "wasi_thread_start".get_fid(&module.exports)?;

    //         module
    //             .renew_call_fn(
    //                 ("wasip1-vfs", "__wasip1_vfs_self_wasi_thread_start"),
    //                 exporting_thread_starter_id,
    //             )
    //             .wrap_err("Failed to rewrite self_wasi_thread_start call in root spawn")?;

    //         if !debug {
    //             module
    //                 .exports
    //                 .remove("__wasip1_vfs_self_wasi_thread_start_anchor")
    //                 .to_eyre()
    //                 .wrap_err(
    //                     "Failed to remove __wasip1_vfs_self_wasi_thread_start_anchor export",
    //                 )?;
    //         }

    //         if debug {
    //             module
    //                 .exports
    //                 .add("real_thread_spawn_fn", real_thread_spawn_fn_id);
    //         }

    //         // __wasip1_vfs_self_wasi_thread_start
    //         module
    //             .connect_func_without_remove(
    //                 ("wasip1-vfs", "__wasip1_vfs_wasi_thread_start_entry"),
    //                 exporting_thread_starter_id,
    //             )
    //             .wrap_err("Failed to connect wasip1-vfs.wasi_thread_start")?;

    //         if !debug {
    //             module.exports.remove("__wasip1_vfs_is_root_spawn").unwrap();
    //         }
    //     }
    // }

    // // todo!(); separate block system from environ
    // let check = block_func(&mut module, "environ_get")?;
    // let next_check = block_func(&mut module, "environ_sizes_get")?;

    // if check != next_check {
    //     eyre::bail!("environ_get and environ_sizes_get are not the same");
    // }

    // fn block_func(module: &mut walrus::Module, func_name: impl AsRef<str>) -> eyre::Result<bool> {
    //     let func_name = func_name.as_ref();
    //     let export_func_name = format!("__wasip1_vfs_{func_name}");
    //     let func_name = func_name.replace("_", "-");

    //     if matches!(
    //         module.exports.iter().find(|e| e.name == export_func_name),
    //         Some(walrus::Export {
    //             item: walrus::ExportItem::Function(_),
    //             ..
    //         })
    //     ) {
    //         module.connect_func_without_remove(
    //             (
    //                 CORE_MODULE_ROOT,
    //                 &format!("[static]wasip1.{func_name}-import"),
    //             ),
    //             &export_func_name,
    //         )?;

    //         return Ok(true);
    //     } else {
    //         return Ok(false);
    //     }
    // }

    // module.create_global_anchor("vfs")?;

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

    // module.exports.delete(eid);

    // if debug {
    //     for wasm in wasm {
    //         let wasm_name = wasm.as_ref().get_file_main_name().unwrap();

    //         module
    //             .exports
    //             .iter()
    //             .filter(|export| {
    //                 export
    //                     .name
    //                     .starts_with(&format!("__wasip1_vfs_{wasm_name}_"))
    //             })
    //             .filter(|export| {
    //                 Wasip1SnapshotPreview1Func::VARIANTS.contains(
    //                     &export
    //                         .name
    //                         .as_str()
    //                         .trim_start_matches(&format!("__wasip1_vfs_{wasm_name}_")),
    //                 )
    //             })
    //             .filter_map(|export| match export.item {
    //                 walrus::ExportItem::Function(fid) => Some((export.name.clone(), fid)),
    //                 _ => None,
    //             })
    //             .collect::<Vec<_>>()
    //             .into_iter()
    //             .for_each(|(name, old_fid)| {
    //                 module.exports.add(&format!("debug_{name}"), old_fid);
    //             });
    //     }
    // }

    let new_path = path.with_extension("adjusted.wasm");

    // println!("module.debug: {:?}", module.debug);

    if fs::metadata(&new_path).is_ok() {
        fs::remove_file(&new_path).expect("Failed to remove existing file");
    }
    module
        .emit_wasm_file(new_path.clone())
        .to_eyre()
        .wrap_err("Failed to emit wasm file")?;

    // todo!();

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

#[derive(Debug, Clone)]
pub struct TomlRestorer {
    path: Utf8PathBuf,
    original: String,
    changed: String,
}

#[derive(Debug, Clone)]
pub struct TomlRestorers(Vec<TomlRestorer>);

impl TomlRestorers {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn with(restorer: TomlRestorer) -> Self {
        Self(vec![restorer])
    }

    pub fn extend(&mut self, restorers: Vec<TomlRestorer>) {
        for restorer in restorers {
            self.push(restorer);
        }
    }

    pub fn push(&mut self, restorer: TomlRestorer) {
        self.0.push(restorer);
    }

    pub fn restore(self) -> eyre::Result<()> {
        for restorer in self.0 {
            restorer.restore()?;
        }
        Ok(())
    }
}

impl TomlRestorer {
    pub fn new(path: &Utf8PathBuf, original: String, changed: String) -> Self {
        Self {
            path: path.clone(),
            original,
            changed,
        }
    }

    pub fn push(self, mut restorers: Vec<TomlRestorer>) -> Vec<TomlRestorer> {
        loop {
            let mut merged = vec![];
            let len = restorers.len();
            for restorer in restorers {
                merged.extend(self.merge(restorer));
            }
            if merged.len() == len {
                break merged;
            }
            restorers = merged;
        }
    }

    pub fn merge(&self, other: TomlRestorer) -> Vec<TomlRestorer> {
        if self.path != other.path {
            return vec![self.clone(), other];
        }

        if other.original == self.changed {
            vec![TomlRestorer::new(
                &self.path,
                self.original.clone(),
                other.changed,
            )]
        } else if self.original == other.changed {
            vec![TomlRestorer::new(
                &self.path,
                other.original,
                self.changed.clone(),
            )]
        } else if self.original == other.original {
            panic!("Merging two same original toml restorer");
        } else {
            vec![self.clone(), other]
        }
    }

    pub fn restore(self) -> eyre::Result<()> {
        fs::write(&self.path, self.original).wrap_err("Failed to write manifest file")?;
        Ok(())
    }
}

/// [profile.release]
/// debug = true
pub fn set_dwarf(
    metadata: &Metadata,
    building_crate: &Package,
    on: bool,
) -> eyre::Result<TomlRestorer> {
    // if workspace, we set workspace
    // else we set normal toml

    fn set(doc: &mut DocumentMut, on: bool) -> eyre::Result<()> {
        if let Some(debug) = doc.get_mut("profile.release") {
            if debug.as_bool().unwrap_or(false) == on {
                return Ok(());
            }
            debug["debug"] = toml_edit::value(on);
        }

        if let Some(profile) = doc.get_mut("profile") {
            if let Some(release) = profile.get_mut("release") {
                release["debug"] = toml_edit::value(on);
            } else {
                profile["release"] = toml_edit::table();
                profile["release"]["debug"] = toml_edit::value(on);
            }
        } else {
            // inline
            let mut profile = toml_edit::Table::new();
            profile.set_implicit(true);
            profile["release"] = toml_edit::table();
            profile["release"]["debug"] = toml_edit::value(on);
            doc["profile"] = toml_edit::Item::Table(profile);
        }

        Ok(())
    }

    if let Some(_) = &metadata.workspace_members.first() {
        let path = metadata.workspace_root.join("Cargo.toml");
        if !path.exists() {
            eyre::bail!("Failed to find workspace Cargo.toml");
        }
        let file_data =
            fs::read_to_string(&path).wrap_err("Failed to read workspace manifest file")?;
        let mut doc = file_data.parse::<DocumentMut>().expect("invalid doc");

        set(&mut doc, on)?;

        let doc = doc.to_string();

        std::fs::write(&path, &doc).wrap_err("Failed to write workspace manifest file")?;
        return Ok(TomlRestorer::new(&path.into(), file_data, doc));
    }

    let manifest_path = building_crate.manifest_path.clone();
    let file_data = fs::read_to_string(&manifest_path).wrap_err("Failed to read manifest file")?;
    let mut doc = file_data.parse::<DocumentMut>().expect("invalid doc");
    set(&mut doc, on)?;
    let doc = doc.to_string();
    std::fs::write(&manifest_path, &doc).wrap_err("Failed to write manifest file")?;
    Ok(TomlRestorer::new(&manifest_path.into(), file_data, doc))
}

pub fn adjust_target_feature(
    metadata: &Metadata,
    building_crate: &Package,
    on: bool,
    feature: impl AsRef<str>,
) -> eyre::Result<TomlRestorer> {
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
            let doc = doc.to_string();
            std::fs::write(&manifest_path, &doc).wrap_err("Failed to write manifest file")?;
            Ok(TomlRestorer::new(&manifest_path.into(), file_data, doc))
        }
        (HasFeature::EnabledOnNormal, false) => {
            set_table(crate_setting, feature, on)?;
            let doc = doc.to_string();
            std::fs::write(&manifest_path, &doc).wrap_err("Failed to write manifest file")?;
            Ok(TomlRestorer::new(&manifest_path.into(), file_data, doc))
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

            let doc = doc.to_string();
            std::fs::write(&manifest_path, &doc)
                .wrap_err("Failed to write workspace manifest file")?;

            Ok(TomlRestorer::new(&manifest_path.into(), file_data, doc))
        }
        _ => Ok(TomlRestorer::new(
            &manifest_path.into(),
            file_data.clone(),
            file_data,
        )),
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
