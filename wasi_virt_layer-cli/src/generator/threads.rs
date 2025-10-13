use eyre::Context;
use itertools::Itertools;
use strum::VariantNames;

use crate::{
    abi::Wasip1ThreadsABIFunc,
    generator::{Generator, GeneratorCtx},
    util::{
        NAMESPACE, THREADS_MODULE_ROOT, WalrusFID as _, WalrusUtilExport as _,
        WalrusUtilImport as _, WalrusUtilModule as _,
    },
};

fn gen_component_name(namespace: &str, name: &str) -> String {
    format!("[static]{namespace}.{}-import", name.replace("_", "-"))
}

/// The thread spawn process itself within the VFS is also caught,
/// but processing is performed to exclude only the root spawn from this.
/// Relocate thread creation from root spawn to the outer layer
#[derive(Debug, Default)]
pub struct ThreadsSpawn;

impl Generator for ThreadsSpawn {
    fn pre_vfs(&mut self, module: &mut walrus::Module, ctx: &GeneratorCtx) -> eyre::Result<()> {
        if !ctx.threads {
            return Ok(());
        }

        let namespace = "wasip1-threads";
        let root = THREADS_MODULE_ROOT;
        let name = <Wasip1ThreadsABIFunc as VariantNames>::VARIANTS
            .iter()
            .exactly_one()
            .wrap_err("Expected exactly one variant for Wasip1ThreadsABIFunc")?; // thread-spawn

        let component_name = gen_component_name(namespace, name);

        module
            .exports
            .erase_with(&format!("{name}_import_anchor"), ctx.unstable_print_debug)?;

        let real_thread_spawn_fn_id = (root, &component_name).get_fid(&module.imports)?;

        let branch_fid = "__wasip1_vfs_is_root_spawn".get_fid(&module.exports)?;

        let normal_thread_spawn_fn_id = ("wasi", "thread-spawn").get_fid(&module.imports)?;

        let self_thread_spawn_fn_id =
            "__wasip1_vfs_wasi_thread_spawn___self".get_fid(&module.exports)?;

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

        let exporting_thread_starter_id = "wasi_thread_start".get_fid(&module.exports)?;

        module
            .connect_func_alt(
                (NAMESPACE, "__wasip1_vfs___self_wasi_thread_start"),
                exporting_thread_starter_id,
                ctx.unstable_print_debug,
            )
            .wrap_err("Failed to rewrite self_wasi_thread_start call in root spawn")?;

        module.exports.erase_with(
            "__wasip1_vfs___self_wasi_thread_start_anchor",
            ctx.unstable_print_debug,
        )?;

        if ctx.unstable_print_debug {
            module
                .exports
                .add("real_thread_spawn_fn", real_thread_spawn_fn_id);
        }

        // __wasip1_vfs_self_wasi_thread_start
        module
            .renew_call_fn(
                (NAMESPACE, "__wasip1_vfs_wasi_thread_start_entry"),
                exporting_thread_starter_id,
            )
            .wrap_err("Failed to connect wasip1-vfs.wasi_thread_start")?;

        module
            .exports
            .erase_with(branch_fid, ctx.unstable_print_debug)?;

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

        let name = &external.name;

        module
            .imports
            .find_mut(("wasi", "thread-spawn"))
            .ok()
            .map(|import| {
                import.name = format!("__wasip1_vfs_wasi_thread_spawn_{name}");
            });

        module
            .exports
            .iter_mut()
            .find(|export| export.name == "wasi_thread_start")
            .map(|export| {
                export.name = format!("__wasip1_vfs_wasi_thread_start_{name}");
            });

        Ok(())
    }
}

/// https://github.com/rust-lang/rust/issues/146843
/// thread spawn is broken on wasm32-wasip1-threads for building library
#[derive(Debug, Default)]
pub struct ThreadsSpawnPatch;

impl Generator for ThreadsSpawnPatch {
    fn pre_vfs(&mut self, module: &mut walrus::Module, ctx: &GeneratorCtx) -> eyre::Result<()> {
        let initializer = "__wasip1_vfs_thread_initializer"
            .get_fid(&module.exports)
            .ok();

        if let Some(_) = initializer {
            module
                .exports
                .erase_with(initializer.unwrap(), ctx.unstable_print_debug)?;
        }

        let old_start = module.start;
        let new_start = module.add_func(&[], &[], |builder, _| {
            let mut body = builder.func_body();
            if let Some(old_start) = old_start {
                body.call(old_start);
            }
            if let Some(initializer) = initializer {
                body.call(initializer);
            }
            Ok(())
        })?;

        module.start = Some(new_start);

        if ctx.unstable_print_debug {
            if let Some(old_start) = old_start {
                module.exports.add("__vfs_old_start", old_start);
            }
        }

        Ok(())
    }
}
