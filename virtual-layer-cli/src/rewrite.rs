use std::fs;

use camino::Utf8PathBuf;
use cargo_metadata::{Metadata, Package};
use eyre::Context as _;
use strum::VariantNames;
use toml_edit::{DocumentMut, Item};

use crate::{
    common::Wasip1SnapshotPreview1Func,
    util::{ResultUtil as _, WalrusUtilImport, WalrusUtilModule},
};

/// wasip1 import to adjust to wit
/// block vfs-wasm's environ_sizes_get etc
/// embedding __wasip1_vfs_flag_{name}_memory
pub fn adjust_wasm(path: &Utf8PathBuf) -> eyre::Result<(Utf8PathBuf, TargetMemoryType)> {
    let mut module = walrus::Module::from_file(path)
        .to_eyre()
        .wrap_err_with(|| eyre::eyre!("Failed to load module"))?;

    for name in <Wasip1SnapshotPreview1Func as VariantNames>::VARIANTS.iter() {
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
            .ok_or_else(|| eyre::eyre!("{name} import not found"))?;

        module
            .imports
            .find_mut("wasi_snapshot_preview1", name)
            .map(|import| {
                import.module = "$root".to_string();
                import.name = component_name;
            });
    }

    // todo!(); separate block system from environ
    let check = block_func(&mut module, "environ_get")?;
    let next_check = block_func(&mut module, "environ_sizes_get")?;

    if check != next_check {
        return Err(eyre::eyre!(
            "environ_get and environ_sizes_get are not the same"
        ));
    }

    fn block_func(module: &mut walrus::Module, func_name: impl AsRef<str>) -> eyre::Result<bool> {
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
        .wrap_err_with(|| eyre::eyre!("Failed to emit wasm file"))?;

    Ok((new_path, target_memory_type))
}

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumString, strum::Display)]
pub enum TargetMemoryType {
    #[strum(ascii_case_insensitive)]
    Single,
    #[strum(ascii_case_insensitive)]
    Multi,
}

pub fn change_target_memory_type(
    metadata: &Metadata,
    building_crate: &Package,
    target_memory_type: TargetMemoryType,
) -> eyre::Result<()> {
    let manifest_path = building_crate.manifest_path.clone();

    let file_data = fs::read_to_string(&manifest_path)
        .wrap_err_with(|| eyre::eyre!("Failed to read manifest file"))?;
    let mut doc = file_data.parse::<DocumentMut>().expect("invalid doc");

    const CRATE: &'static str = "wasip1-virtual-layer";

    let crate_setting = &mut doc["dependencies"][CRATE];

    let set_table = |table: &mut Item| -> eyre::Result<()> {
        if target_memory_type == TargetMemoryType::Multi {
            if matches!(table.get("features"), None) {
                table["features"] = toml_edit::value(toml_edit::Array::new());
            }
            if table["features"]
                .as_array()
                .unwrap()
                .iter()
                .filter_map(|s| s.as_str())
                .any(|s| s == "multi_memory")
            {
                return Ok(());
            }
            table["features"]
                .as_array_mut()
                .unwrap()
                .push("multi_memory");
        }

        if target_memory_type == TargetMemoryType::Single {
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
                    .find(|(_, s)| *s == "multi_memory")
                    .map(|(i, _)| i)
            } {
                table["features"].as_array_mut().unwrap().remove(i);
                if table["features"].as_array().unwrap().is_empty() {
                    table["features"] = Item::None;
                }
            }
        }

        Ok(())
    };

    enum FeatureMultiMemory {
        Disabled,
        EnabledOnNormal,
        EnabledOnWorkspace,
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

    // check normal crate setting
    let has_feature_multi_memory = if has_feature(crate_setting, "multi_memory") {
        FeatureMultiMemory::EnabledOnNormal
    } else {
        FeatureMultiMemory::Disabled
    };
    // check workspace
    let has_feature_multi_memory =
        if matches!(has_feature_multi_memory, FeatureMultiMemory::Disabled) {
            match &crate_setting["workspace"] {
                v if v.as_bool().unwrap_or(false) => {
                    let manifest_path = metadata.workspace_root.join("Cargo.toml");

                    let file_data = fs::read_to_string(&manifest_path)
                        .wrap_err_with(|| eyre::eyre!("Failed to read workspace manifest file"))?;
                    let mut doc = file_data.parse::<DocumentMut>().expect("invalid doc");

                    let crate_setting = &mut doc["workspace"]["dependencies"][CRATE];

                    if has_feature(crate_setting, "multi_memory") {
                        FeatureMultiMemory::EnabledOnWorkspace
                    } else {
                        FeatureMultiMemory::Disabled
                    }
                }
                _ => FeatureMultiMemory::Disabled,
            }
        } else {
            has_feature_multi_memory
        };

    match (has_feature_multi_memory, target_memory_type) {
        (FeatureMultiMemory::Disabled, TargetMemoryType::Multi) => {
            set_table(crate_setting)?;
            std::fs::write(&manifest_path, doc.to_string())
                .wrap_err_with(|| eyre::eyre!("Failed to write manifest file"))?;
            Ok(())
        }
        (FeatureMultiMemory::EnabledOnNormal, TargetMemoryType::Single) => {
            set_table(crate_setting)?;
            std::fs::write(&manifest_path, doc.to_string())
                .wrap_err_with(|| eyre::eyre!("Failed to write manifest file"))?;
            Ok(())
        }
        (FeatureMultiMemory::EnabledOnWorkspace, TargetMemoryType::Single) => {
            let manifest_path = metadata.workspace_root.join("Cargo.toml");

            let file_data = fs::read_to_string(&manifest_path)
                .wrap_err_with(|| eyre::eyre!("Failed to read workspace manifest file"))?;
            let mut doc = file_data.parse::<DocumentMut>().expect("invalid doc");

            let crate_setting = &mut doc["workspace"]["dependencies"][CRATE];

            set_table(crate_setting)?;

            std::fs::write(&manifest_path, doc.to_string())
                .wrap_err_with(|| eyre::eyre!("Failed to write workspace manifest file"))?;

            Ok(())
        }
        (FeatureMultiMemory::EnabledOnWorkspace, TargetMemoryType::Multi) => Ok(()),
        (FeatureMultiMemory::EnabledOnNormal, TargetMemoryType::Multi) => Ok(()),
        (FeatureMultiMemory::Disabled, TargetMemoryType::Single) => Ok(()),
    }

    // match crate_setting {
    //     Item::Table(table) => match &mut table["workspace"] {
    //         v if v.as_bool().unwrap_or(false) => {
    //             let manifest_path = metadata.workspace_root.join("Cargo.toml");

    //             let file_data = fs::read_to_string(&manifest_path)
    //                 .wrap_err_with(|| eyre::eyre!("Failed to read workspace manifest file"))?;
    //             let mut doc = file_data.parse::<DocumentMut>().expect("invalid doc");

    //             let crate_setting = &mut doc["workspace"]["dependencies"][CRATE];

    //             let table = match crate_setting {
    //                 Item::Table(_) | Item::Value(_) => crate_setting,
    //                 _ => {
    //                     eyre::bail!(
    //                         "Cannot find crate {CRATE} on root Cargo.toml so cannot change target memory type"
    //                     );
    //                 }
    //             };

    //             set_table(table)?;

    //             std::fs::write(&manifest_path, doc.to_string())
    //                 .wrap_err_with(|| eyre::eyre!("Failed to write workspace manifest file"))?;

    //             Ok(())
    //         }
    //         Item::Value(_) => {
    //             set_table(crate_setting)?;

    //             std::fs::write(&manifest_path, doc.to_string())
    //                 .wrap_err_with(|| eyre::eyre!("Failed to write manifest file"))?;

    //             Ok(())
    //         }
    //         _ => {
    //             eyre::bail!("Cannot find crate {CRATE} so cannot change target memory type");
    //         }
    //     },
    //     _ => {
    //         eyre::bail!(r#"Failed to find crate "{CRATE}" so cannot change target memory type"#);
    //     }
    // }
}
