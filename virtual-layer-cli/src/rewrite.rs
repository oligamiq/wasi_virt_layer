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
                .any(|e| e.name == format!("{name}_import_wrap"))
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
            .any(|export| export.name == format!("__wasip1_vfs_{wasm_name}__start_wrap"))
        {
            eyre::bail!(
                "Failed to get __start_wrap export on {wasm_name}. You may forget definition `import_wasm!` macro with wasm name."
            );
        }

        // let memory = module.memories
        println!("memories: {:?}", module.memories);
    }

    for (name, (namespace, root)) in <Wasip1SnapshotPreview1Func as VariantNames>::VARIANTS
        .iter()
        .zip(core::iter::repeat(("wasip1", CORE_MODULE_ROOT)))
        .chain(
            (if threads {
                <Wasip1SnapshotPreview1ThreadsFunc as VariantNames>::VARIANTS
            } else {
                &[]
            })
            .iter()
            .zip(core::iter::repeat(("wasip1-threads", THREADS_MODULE_ROOT))),
        )
    {
        let component_name = format!("[static]{namespace}.{}-import", name.replace("_", "-"));

        module
            .exports
            .remove(format!("{name}_import_wrap"))
            .to_eyre()
            .wrap_err_with(|| eyre::eyre!("{name}_import_wrap not found"))?;

        module
            .imports
            .find_mut(root, &component_name)
            .map(|import| {
                import.module = "archived".to_string();
            })
            .ok_or_else(|| eyre::eyre!("{component_name} import not found"))?;

        module
            .imports
            .find_mut("wasi_snapshot_preview1", name)
            .map(|import| {
                import.module = root.to_string();
                import.name = component_name;
            });
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
