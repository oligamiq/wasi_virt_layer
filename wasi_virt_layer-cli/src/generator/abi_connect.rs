use itertools::Itertools;
use strum::VariantNames;

use crate::{
    abi::{Wasip1ABIFunc, Wasip1ThreadsABIExportFunc, Wasip1ThreadsABIFunc},
    generator::Generator,
    unique_name::UniqueName,
    util::{
        WalrusFID, WalrusUtilExport, WalrusUtilImport, WalrusUtilModule, WasmName,
        gen_component_name,
    },
};

#[derive(Debug, strum::AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum Wasip1ABI<'a> {
    #[strum(serialize = "__self")]
    SelfDefault {
        import: &'a str,
    },
    #[strum(serialize = "")]
    TargetTemporal {
        wasm: &'a WasmName,
        import: &'a str,
    },
    WasiThreadStart(&'a WasmName),
    #[strum(serialize = "wasi_thread_start")]
    WasiThreadStartDestination(&'a WasmName),
    WasiThreadSpawn(&'a WasmName),
    WasiThreadStartAnchor(&'a WasmName),
}

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
            let export = UniqueName::Wasip1ABI(&Wasip1ABI::SelfDefault { import })
                .get_fid(&module.exports)
                .ok();

            if let Some(import_id) = (
                // CORE_MODULE_ROOT,
                // &format!("[static]wasip1.{}-import", import.replace("_", "-")),
                UniqueName::WASIP1_ABI_MODULE,
                import,
            )
                .get_fid(&module.imports)
                .ok()
            {
                if let Some(_) = export {
                    module.connect_func_alt_with_remove_export(
                        import_id,
                        &UniqueName::Wasip1ABI(&Wasip1ABI::SelfDefault { import }).to_string(),
                        ctx.unstable_print_debug,
                    )?;
                } else {
                    log::warn!("No plug found for Wasip1 ABI import self: {import}");
                }
            } else {
                if let Some(_) = export {
                    module
                        .exports
                        .remove(
                            &UniqueName::Wasip1ABI(&Wasip1ABI::SelfDefault { import }).to_string(),
                        )
                        .unwrap();
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
                    && import.module == UniqueName::WASIP1_ABI_MODULE
            })
            .for_each(|import| {
                import.name =
                    crate::unique_name::UniqueName::Wasip1ABI(&Wasip1ABI::TargetTemporal {
                        wasm: &external.name,
                        import: import.name.as_str(),
                    })
                    .to_string();
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
                if let Some(import_id) = (UniqueName::WASIP1_ABI_MODULE, import)
                    .get_fid(&module.imports)
                    .ok()
                {
                    unreachable!();
                    // module.connect_func_alt(import_id, &export_name, ctx.unstable_print_debug)?;
                } else {
                    module
                        .exports
                        .erase_with(
                            &UniqueName::Wasip1ABI(&Wasip1ABI::TargetTemporal { wasm, import }),
                            ctx.unstable_print_debug,
                        )
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
                if UniqueName::Wasip1ABI(&Wasip1ABI::WasiThreadStartDestination(wasm))
                    .get_fid(&module.exports)
                    .ok()
                    .is_some()
                {
                    module.connect_func_alt_with_remove_export(
                        (
                            UniqueName::NAMESPACE,
                            &UniqueName::Wasip1ABI(&Wasip1ABI::WasiThreadStart(wasm)),
                        ),
                        &UniqueName::Wasip1ABI(&Wasip1ABI::WasiThreadStartDestination(wasm))
                            .to_string(),
                        ctx.unstable_print_debug,
                    )?;

                    module.exports.erase_with(
                        &UniqueName::Wasip1ABI(&Wasip1ABI::WasiThreadStartAnchor(wasm)),
                        ctx.unstable_print_debug,
                    )?;

                    module.connect_func_alt_with_remove_export(
                        (
                            UniqueName::WASIP1_THREADS_ABI_MODULE,
                            &UniqueName::Wasip1ABI(&&Wasip1ABI::WasiThreadSpawn(wasm)),
                        ),
                        &UniqueName::Wasip1ABI(&Wasip1ABI::WasiThreadSpawn(wasm)).to_string(),
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
            if let Some(fid) = (UniqueName::CORE_NON_RECURSIVE_MODULE_ROOT, import)
                .get_fid(&module.imports)
                .ok()
            {
                // If it already exists, make it possible to call it.
                if let Some(import_id) = (UniqueName::WASIP1_ABI_MODULE, import)
                    .get_fid(&module.imports)
                    .ok()
                {
                    module.renew_call_fn(fid, import_id)?;
                } else {
                    let import_id = module.imports.get_imported_func(fid).unwrap().id();
                    let import = module.imports.get_mut(import_id);
                    import.module = UniqueName::WASIP1_ABI_MODULE.to_string();
                }
            }
        }

        module
            .imports
            .iter()
            .filter(|import| import.module == UniqueName::CORE_NON_RECURSIVE_MODULE_ROOT)
            .map(|import| &import.name)
            .for_each(|name| {
                log::warn!(
                    "Non-recursive Wasip1 ABI import exists: {name}, but this is not verified."
                );
            });

        Ok(())
    }
}

/// Adjust ABI to match wasip1-threads
#[derive(Debug, Clone, Default)]
pub struct AdjustABI;

impl Generator for AdjustABI {
    fn post_components(
        &mut self,
        module: &mut walrus::Module,
        ctx: &super::ComponentCtx,
    ) -> eyre::Result<()> {
        if !ctx.adjust_abi {
            return Ok(());
        }

        for import_name in <Wasip1ABIFunc as strum::VariantNames>::VARIANTS {
            if let Ok(import_id) = (
                UniqueName::CORE_MODULE_ROOT,
                &gen_component_name(UniqueName::WASIP1_ABI_MODULE_ALT, import_name),
            )
                .get_fid(&module.imports)
            {
                if let Ok(import_id) =
                    (UniqueName::WASIP1_ABI_MODULE, import_name).get_fid(&module.imports)
                {
                    module.imports.erase(import_id).unwrap();
                }

                let import_id = module.imports.get_imported_func(import_id).unwrap().id();
                let import = module.imports.get_mut(import_id);
                import.module = UniqueName::WASIP1_ABI_MODULE.to_string();
                import.name = import_name.to_string();
            }
        }

        let import_name = <Wasip1ThreadsABIFunc as strum::VariantNames>::VARIANTS
            .iter()
            .exactly_one()
            .unwrap();

        if let Ok(import_id) = (
            UniqueName::THREADS_MODULE_ROOT,
            &gen_component_name(UniqueName::WASIP1_THREADS_ABI_MODULE_ALT, import_name),
        )
            .get_fid(&module.imports)
        {
            if let Ok(import_id) =
                (UniqueName::WASIP1_THREADS_ABI_MODULE, import_name).get_fid(&module.imports)
            {
                module.imports.erase(import_id).unwrap();
            }

            let import_id = module.imports.get_imported_func(import_id).unwrap().id();
            let import = module.imports.get_mut(import_id);
            import.module = UniqueName::WASIP1_THREADS_ABI_MODULE.to_string();
            import.name = import_name.to_string();

            let export_name = Wasip1ThreadsABIExportFunc::VARIANTS
                .iter()
                .exactly_one()
                .unwrap();

            if let Ok(export_id) = export_name.get_fid(&module.exports) {
                module.exports.erase(export_id)?;
            }

            // adjust export
            let id = UniqueName::THREADS_EXPORT_MODULE_ROOT.get_fid(&module.exports)?;
            let eid = module.exports.get_exported_func(id).unwrap().id();
            let export = module.exports.get_mut(eid);
            export.name = UniqueName::THREADS_EXPORT_MODULE_ROOT.to_string();
        }

        Ok(())
    }
}
