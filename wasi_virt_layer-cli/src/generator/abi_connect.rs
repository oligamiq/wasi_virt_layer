use crate::{
    abi::Wasip1ABIFunc,
    generator::Generator,
    util::{WalrusFID, WalrusUtilExport, WalrusUtilModule},
};

/// Connect Wasip1 ABI
/// If an import exists, add the corresponding export.
/// If it does not exist, remove that export if it exists.
/// Require before PatchComponent
#[derive(Debug, Default)]
pub struct ConnectWasip1ABI;

impl Generator for ConnectWasip1ABI {
    /// todo!();
    /// It must be placed before the patch_component.
    fn pre_vfs(
        &mut self,
        module: &mut walrus::Module,
        ctx: &super::GeneratorCtx,
    ) -> eyre::Result<()> {
        for import in <Wasip1ABIFunc as strum::VariantNames>::VARIANTS {
            let export = format!("__wasip1_vfs___self_{import}")
                .get_fid(&module.exports)
                .ok();

            if let Some(import_id) = (
                // CORE_MODULE_ROOT,
                // &format!("[static]wasip1.{}-import", import.replace("_", "-")),
                "wasi_snapshot_preview1",
                import,
            )
                .get_fid(&module.imports)
                .ok()
            {
                if let Some(export) = export {
                    module.connect_func_alt(import_id, export, ctx.unstable_print_debug)?;
                } else {
                    log::warn!("No export found for Wasip1 ABI import self: {import}");
                }
            } else {
                if let Some(export) = export {
                    module
                        .exports
                        .erase_with(export, ctx.unstable_print_debug)
                        .ok();
                }
            }
        }

        Ok(())
    }

    fn pre_target(
        &mut self,
        module: &mut walrus::Module,
        _: &crate::generator::GeneratorCtx,
        external: &crate::generator::ModuleExternal,
    ) -> eyre::Result<()> {
        module
            .imports
            .iter_mut()
            .filter(|import| {
                <Wasip1ABIFunc as strum::VariantNames>::VARIANTS.contains(&import.name.as_str())
                    && import.module == "wasi_snapshot_preview1"
            })
            .for_each(|import| {
                import.name = format!("__wasip1_vfs_{}_{}", external.name, import.name);
            });

        Ok(())
    }

    fn post_combine(
        &mut self,
        module: &mut walrus::Module,
        ctx: &crate::generator::GeneratorCtx,
    ) -> eyre::Result<()> {
        for wasm in &ctx.target_names {
            for import in <Wasip1ABIFunc as strum::VariantNames>::VARIANTS {
                let export_name = format!("__wasip1_vfs_{wasm}_{import}");
                if let Some(import_id) = ("wasi_snapshot_preview1", import)
                    .get_fid(&module.imports)
                    .ok()
                {
                    module.connect_func_alt(import_id, &export_name, ctx.unstable_print_debug)?;
                } else {
                    module
                        .exports
                        .erase_with(&export_name, ctx.unstable_print_debug)
                        .ok();
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct ConnectWasip1ThreadsABI;

impl Generator for ConnectWasip1ThreadsABI {
    fn post_combine(
        &mut self,
        module: &mut walrus::Module,
        ctx: &crate::generator::GeneratorCtx,
    ) -> eyre::Result<()> {
        if ctx.threads {
            for wasm in &ctx.target_names {
                if format!("__wasip1_vfs_wasi_thread_start_{wasm}")
                    .get_fid(&module.exports)
                    .ok()
                    .is_some()
                {
                    module.connect_func_alt(
                        (
                            "wasip1-vfs",
                            &format!("__wasip1_vfs_{wasm}_wasi_thread_start"),
                        ),
                        &format!("__wasip1_vfs_wasi_thread_start_{wasm}"),
                        ctx.unstable_print_debug,
                    )?;

                    module.exports.erase_with(
                        &format!("__wasip1_vfs_{wasm}_wasi_thread_start_anchor"),
                        ctx.unstable_print_debug,
                    )?;

                    module.connect_func_alt(
                        ("wasi", &format!("__wasip1_vfs_wasi_thread_spawn_{wasm}")),
                        &format!("__wasip1_vfs_wasi_thread_spawn_{wasm}"),
                        ctx.unstable_print_debug,
                    )?;
                }
            }
        }
        Ok(())
    }
}

/// Require before PatchComponent
/// Require after ConnectWasip1ABI
#[derive(Debug, Default)]
pub struct NonRecursiveWasiABI;

impl Generator for NonRecursiveWasiABI {
    fn pre_vfs(
        &mut self,
        module: &mut walrus::Module,
        _: &super::GeneratorCtx,
    ) -> eyre::Result<()> {
        for import in <Wasip1ABIFunc as strum::VariantNames>::VARIANTS {
            if let Some(fid) = ("non_recursive_wasi_snapshot_preview1", import)
                .get_fid(&module.imports)
                .ok()
            {
                // If it already exists, make it possible to call it.
                if let Some(import_id) = ("wasi_snapshot_preview1", import)
                    .get_fid(&module.imports)
                    .ok()
                {
                    module.renew_call_fn(fid, import_id)?;
                } else {
                    let import_id = module.imports.get_imported_func(fid).unwrap().id();
                    let import = module.imports.get_mut(import_id);
                    import.module = "wasi_snapshot_preview1".to_string();
                }
            }
        }

        module
            .imports
            .iter()
            .filter(|import| import.module == "non_recursive_wasi_snapshot_preview1")
            .map(|import| &import.name)
            .for_each(|name| {
                log::warn!(
                    "Non-recursive Wasip1 ABI import exists: {name}, but this is not verified."
                );
            });

        Ok(())
    }
}
