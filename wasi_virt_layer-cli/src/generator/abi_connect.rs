use itertools::Itertools;
use strum::VariantNames;

use crate::{
    abi::{Wasip1ABIFunc, Wasip1ThreadsABIExportFunc, Wasip1ThreadsABIFunc},
    generator::{Generator, threads::ThreadsSpawnName},
    unique_name::UniqueName,
    util::{
        ResultUtil as _, WalrusFID, WalrusFIDAssister as _, WalrusUtilExport, WalrusUtilImport,
        WalrusUtilModule, WasmName, gen_component_name,
    },
};
use eyre::WrapErr as _;

/// Abstraction over internal representation name generations for hooking WASI implementations.
#[derive(Debug, strum::AsRefStr, strum::EnumCount, Hash, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum Wasip1ABIName<'a> {
    #[strum(serialize = "__self")]
    /// Reference to a self-contained import.
    SelfDefault { 
        /// Identifier string representation of the import.
        import: &'a str 
    },
    #[strum(serialize = "")]
    /// Temporarily constructed linking to a specified module during intermediate compilation.
    TargetTemporal { 
        /// Specific wasm module binding origin.
        wasm: &'a WasmName, 
        /// Specific imported signature target.
        import: &'a str 
    },
}

/// Connect Wasip1 ABI
/// If an import exists, add the corresponding export.
/// If it does not exist, remove that export if it exists.
/// Require before PatchComponent
///
/// **Why this is needed:**
/// WASI functions (like `fd_read`, `environ_get`) imported by the target module must be
/// fulfilled by the VFS (Virtual File System) layer. This generator automatically scans
/// for all potential `wasi_snapshot_preview1` imports and maps them to the corresponding
/// exports (`__wasip1_vfs_...`) exposed by the virtualization core. Missing imports or
/// unused exports are pruned to keep the ABI clean.
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
            let export = UniqueName::Wasip1ABI(&Wasip1ABIName::SelfDefault { import })
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
                        &UniqueName::Wasip1ABI(&Wasip1ABIName::SelfDefault { import }).to_string(),
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
                            &UniqueName::Wasip1ABI(&Wasip1ABIName::SelfDefault { import })
                                .to_string(),
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
                    crate::unique_name::UniqueName::Wasip1ABI(&Wasip1ABIName::TargetTemporal {
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
                            &UniqueName::Wasip1ABI(&Wasip1ABIName::TargetTemporal { wasm, import }),
                            ctx.unstable_print_debug,
                        )
                        .ok();
                }
            }
        }
        Ok(())
    }
}

/// ABI Generator: Connects WASI-threads ABI imports across modules.
///
/// **Why this is needed:**
/// Similar to `ConnectWasip1ABI`, thread-specific WASI functions (e.g., `wasi_thread_spawn`)
/// and the Thread ID assignment logic need to be properly intercepted. This generator aligns
/// the standard thread imports with our instrumented logic, allowing the host context to
/// initialize and direct threads properly according to our virtualized shared memory architecture.
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
                let start_import = (
                    UniqueName::NAMESPACE,
                    &UniqueName::ThreadsSpawn(&ThreadsSpawnName::WasiThreadStart(wasm)),
                );

                let dest_name =
                    UniqueName::ThreadsSpawn(&ThreadsSpawnName::WasiThreadStartDestination(wasm))
                        .to_string();
                let dest_fid = module.exports.get_fid_by_name(&dest_name).ok();

                if let Some(_) = dest_fid {
                    module.connect_func_alt_with_remove_export(
                        start_import,
                        &dest_name,
                        ctx.unstable_print_debug,
                    )?;
                } else {
                    // Destination not found. We should still resolve the import to avoid component encoding failure.
                    if let Ok(import_id) = start_import.get_fid(&module.imports) {
                        log::warn!(
                            "Thread start destination not found for {}, connecting to trap.",
                            wasm
                        );
                        // Connect to trap
                        module
                            .replace_imported_func(import_id, |(body, _)| {
                                body.unreachable();
                            })
                            .to_eyre()
                            .wrap_err_with(|| {
                                eyre::eyre!("Failed to replace missing thread start import")
                            })?;
                    }
                }

                module
                    .exports
                    .erase_with(
                        &UniqueName::ThreadsSpawn(&ThreadsSpawnName::WasiThreadStartAnchor(wasm)),
                        ctx.unstable_print_debug,
                    )
                    .ok();

                let spawn_import = (
                    UniqueName::WASIP1_THREADS_ABI_MODULE,
                    &UniqueName::ThreadsSpawn(&ThreadsSpawnName::WasiThreadSpawn(wasm)),
                );
                let spawn_dest_name =
                    UniqueName::ThreadsSpawn(&ThreadsSpawnName::WasiThreadSpawn(wasm)).to_string();

                if let Ok(import_id) = spawn_import.get_fid(&module.imports) {
                    if let Ok(_) = module.exports.get_fid_by_name(&spawn_dest_name) {
                        module.connect_func_alt_with_remove_export(
                            spawn_import,
                            &spawn_dest_name,
                            ctx.unstable_print_debug,
                        )?;
                    } else {
                        module
                            .replace_imported_func(import_id, |(body, _)| {
                                body.unreachable();
                            })
                            .to_eyre()
                            .wrap_err_with(|| {
                                eyre::eyre!("Failed to replace missing thread spawn import")
                            })?;
                    }
                }
            }
        }
        Ok(())
    }
}

/// Require before PatchComponent
/// Require after ConnectWasip1ABI
///
/// **Why this is needed:**
/// Sometimes the VFS implementation itself needs to call the host's real WASI functions
/// (e.g., falling back to the real filesystem). If it just called the normal WASI imports,
/// it could cause an infinite recursion because they are patched to point back to the VFS.
/// This generator rewires special `non_recursive_...` invocations directly to the original
/// host environment imports.
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
///
/// **Why this is needed:**
/// Target runners (like standard environments or `wasmtime`) expect exactly
/// `wasi_snapshot_preview1` and standard threads module names. After all our
/// patching using intermediate component names, this pass rename the finalized
/// module imports back to standard names so that the result is syntactically a
/// correct standard WASM matching the underlying runtime requirements.
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
