use eyre::Context;
use itertools::Itertools;
use strum::VariantNames;

use crate::{
    abi::{Wasip1ThreadsABIExportFunc, Wasip1ThreadsABIFunc},
    generator::{Generator, GeneratorCtx},
    unique_name::UniqueName,
    util::{
        WalrusFID as _, WalrusUtilExport as _, WalrusUtilImport as _, WalrusUtilModule as _,
        WasmName, gen_component_name,
    },
};

/// Constants and format builders for renaming Thread spawn imports and exports during patching.
#[derive(Debug, strum::AsRefStr, strum::EnumCount, Hash, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum ThreadsSpawnName<'a> {
    /// Custom spawn anchor injected for bridging definitions.
    ImportAnchor(&'a str),
    /// Flag indicating whether the current instance is the root spawner thread.
    IsRootSpawn,
    #[strum(serialize = "wasi_thread_spawn___self")]
    /// Function identifier for native WASI spawn capability.
    WasiThreadSpawnSelf,
    #[strum(serialize = "__self_wasi_thread_start")]
    /// The starting routine mapping on thread creation execution.
    SelfWasiThreadStart,
    #[strum(serialize = "__self_wasi_thread_start_anchor")]
    /// The underlying static bridge to anchor thread origins.
    SelfWasiThreadStartAnchor,
    /// Low level function signature to actually request system spawn.
    RealThreadSpawnFn,
    /// Entry stub function used when entering newly spawned routines.
    WasiThreadStartEntry,
    /// Specialized start function per WASM module target.
    WasiThreadStart(&'a WasmName),
    #[strum(serialize = "wasi_thread_start")]
    /// Target resolving memory start capabilities per component.
    WasiThreadStartDestination(&'a WasmName),
    /// Wraps dynamic spawn allocations over components.
    WasiThreadSpawn(&'a WasmName),
    /// Cross-linking anchor marking a spawn location.
    WasiThreadStartAnchor(&'a WasmName),
    /// Initializer function for thread memory limits scaling.
    ThreadInitializer,
    /// Reference to the existing deprecated `_start` handling.
    OldStart,
}

/// The thread spawn process itself within the VFS is also caught,
/// but processing is performed to exclude only the root spawn from this.
/// Relocate thread creation from root spawn to the outer layer
///
/// **Why this is needed:**
/// Per `IMPORTS_EXPORTS_EVOLUTION_DETAILED.md`, `wasi-threads:spawn` requires special handling
/// because thread creation from the "root spawn" (the original process invoking `_start`)
/// differs from spawning within explicitly created WASI threads.
///
/// This generator ensures that only the root spawn initiates the true thread start sequence
/// on the host by intercepting `wasi_thread_spawn`. Real execution is forwarded via
/// `__wasip1_vfs_real_thread_spawn_fn`, avoiding recursive or improper initializations.
#[derive(Debug, Default)]
pub struct ThreadsSpawn;

impl Generator for ThreadsSpawn {
    fn pre_vfs(&mut self, module: &mut walrus::Module, ctx: &GeneratorCtx) -> eyre::Result<()> {
        if !ctx.threads {
            return Ok(());
        }

        let name = <Wasip1ThreadsABIFunc as VariantNames>::VARIANTS
            .iter()
            .exactly_one()
            .wrap_err("Expected exactly one variant for Wasip1ThreadsABIFunc")?; // thread-spawn

        let component_name = gen_component_name(UniqueName::WASIP1_THREADS_ABI_MODULE_ALT, name);

        module.exports.erase_with(
            &UniqueName::ThreadsSpawn(&ThreadsSpawnName::ImportAnchor(name)),
            ctx.unstable_print_debug,
        )?;

        let real_thread_spawn_fn_id =
            (UniqueName::THREADS_MODULE_ROOT, &component_name).get_fid(&module.imports)?;

        let branch_fid =
            UniqueName::ThreadsSpawn(&ThreadsSpawnName::IsRootSpawn).get_fid(&module.exports)?;

        if let Some(normal_thread_spawn_fn_id) = (UniqueName::WASIP1_THREADS_ABI_MODULE, name)
            .get_fid(&module.imports)
            .ok()
        {
            let self_thread_spawn_fn_id =
                UniqueName::ThreadsSpawn(&ThreadsSpawnName::WasiThreadSpawnSelf)
                    .get_fid(&module.exports)?;

            module
                .exports
                .erase_with(self_thread_spawn_fn_id, ctx.unstable_print_debug)?;

            use walrus::ValType::I32;
            let real_thread_spawn_fn_id = module
                .add_func(&[I32], &[I32], |builder, args| {
                    let mut body = builder.func_body();
                    body.call(branch_fid)
                        .if_else(
                            I32,
                            |then| {
                                then.local_get(args[0]) // pass the argument to thread-spawn
                                    .call(real_thread_spawn_fn_id);
                            },
                            |else_| {
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

            let start_name = Wasip1ThreadsABIExportFunc::VARIANTS
                .iter()
                .exactly_one()
                .wrap_err("Expected exactly one variant for Wasip1ThreadsABIExportFunc")?; // wasi-thread-start

            let exporting_thread_starter_id = start_name.get_fid(&module.exports)?;

            module
                .connect_func_alt_with_remove_export(
                    (
                        UniqueName::NAMESPACE,
                        &UniqueName::ThreadsSpawn(&ThreadsSpawnName::SelfWasiThreadStart),
                    ),
                    start_name,
                    ctx.unstable_print_debug,
                )
                .wrap_err("Failed to rewrite self_wasi_thread_start call in root spawn")?;

            module.exports.erase_with(
                &UniqueName::ThreadsSpawn(&ThreadsSpawnName::SelfWasiThreadStartAnchor),
                ctx.unstable_print_debug,
            )?;

            if ctx.unstable_print_debug {
                module.exports.add(
                    &UniqueName::ThreadsSpawn(&ThreadsSpawnName::RealThreadSpawnFn).to_string(),
                    real_thread_spawn_fn_id,
                );
            }

            // __wasip1_vfs_self_wasi_thread_start
            module
                .renew_call_fn(
                    (
                        UniqueName::NAMESPACE,
                        &UniqueName::ThreadsSpawn(&ThreadsSpawnName::WasiThreadStartEntry),
                    ),
                    exporting_thread_starter_id,
                )
                .wrap_err("Failed to connect wasip1-vfs.wasi_thread_start")?;

            module
                .exports
                .erase_with(branch_fid, ctx.unstable_print_debug)?;
        } else {
            log::warn!("No normal thread-spawn found, why are you not using threads?");

            let component_name =
                gen_component_name(UniqueName::WASIP1_THREADS_ABI_MODULE_ALT, name);

            let _real_thread_spawn_fn_id =
                (UniqueName::THREADS_MODULE_ROOT, &component_name).get_fid(&module.imports)?;

            module.exports.erase_with(
                &UniqueName::ThreadsSpawn(&ThreadsSpawnName::IsRootSpawn),
                ctx.unstable_print_debug,
            )?;

            let fake_start = module
                .add_func(
                    &[walrus::ValType::I32, walrus::ValType::I32],
                    &[],
                    |_, _| Ok(()),
                )
                .wrap_err("Failed to add fake thread spawn function")?;

            module
                .renew_call_fn(
                    (
                        UniqueName::NAMESPACE,
                        &UniqueName::ThreadsSpawn(&ThreadsSpawnName::WasiThreadStartEntry),
                    ),
                    fake_start,
                )
                .wrap_err("Failed to connect wasip1-vfs.wasi_thread_start")?;

            // println!("unstable_print_debug: {}", ctx.unstable_print_debug);
        }

        Ok(())
    }

    fn pre_target(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
        external: &crate::generator::ModuleExternal,
    ) -> eyre::Result<()> {
        if !ctx.threads {
            return Ok(());
        }

        let wasm = &external.name;

        let start_name = Wasip1ThreadsABIFunc::VARIANTS
            .iter()
            .exactly_one()
            .wrap_err("Expected exactly one variant for Wasip1ThreadsABIExportFunc")?; // thread-spawn

        module
            .imports
            .find_mut((UniqueName::WASIP1_THREADS_ABI_MODULE, start_name))
            .ok()
            .map(|import| {
                // import.name = format!("__wasip1_vfs_wasi_thread_spawn_{name}");
                import.name =
                    UniqueName::ThreadsSpawn(&ThreadsSpawnName::WasiThreadSpawn(wasm)).to_string();
            });

        let export_name = Wasip1ThreadsABIExportFunc::VARIANTS
            .iter()
            .exactly_one()
            .wrap_err("Expected exactly one variant for Wasip1ThreadsABIExportFunc")?; // wasi_thread_start

        module
            .exports
            .iter_mut()
            .find(|export| export.name == *export_name)
            .map(|export| {
                export.name =
                    UniqueName::ThreadsSpawn(&ThreadsSpawnName::WasiThreadStartDestination(wasm))
                        .to_string();
            });

        Ok(())
    }
}

/// https://github.com/rust-lang/rust/issues/146843
/// thread spawn is broken on wasm32-wasip1-threads for building library
///
/// **Why this is needed:**
/// Currently, thread initialization routines (e.g. `__wasm_init_tls`) can be improperly tied
/// or omitted during WASM generation (see rust-lang/rust#146843). This patching strategy
/// wraps the original `_start` with an injected `ThreadInitializer` call if one is exported.
#[derive(Debug, Default)]
pub struct ThreadsSpawnPatch;

impl Generator for ThreadsSpawnPatch {
    fn pre_vfs(&mut self, module: &mut walrus::Module, ctx: &GeneratorCtx) -> eyre::Result<()> {
        let initializer = UniqueName::ThreadsSpawn(&ThreadsSpawnName::ThreadInitializer)
            .get_fid(&module.exports)
            .ok();

        if let Some(init_id) = initializer {
            module
                .exports
                .erase_with(init_id, ctx.unstable_print_debug)?;

            let init = ctx.starts.thread_patch.get_fid(&module.exports)?;
            module
                .funcs
                .get_mut(init)
                .kind
                .unwrap_local_mut()
                .builder_mut()
                .func_body()
                .call(init_id);
        }

        Ok(())
    }
}
