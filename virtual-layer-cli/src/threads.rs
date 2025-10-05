use std::fs;

use camino::Utf8PathBuf;
use eyre::Context;
use itertools::Itertools;
use strum::VariantNames;

use crate::{
    common::Wasip1SnapshotPreview1ThreadsFunc,
    generator::{Generator, GeneratorCtx, ModuleExternal},
    util::{
        ResultUtil, THREADS_MODULE_ROOT, WalrusFID as _, WalrusUtilExport as _,
        WalrusUtilModule as _,
    },
};

pub fn adjust_core_wasm(
    path: &Utf8PathBuf,
    threads: bool,
    dwarf: bool,
) -> eyre::Result<(Utf8PathBuf, Option<Vec<(u64, u64)>>)> {
    let mut module = walrus::Module::load(path, dwarf)?;

    let mem_size = if threads {
        module.memories.iter_mut().for_each(|mem| {
            mem.shared = true;
        });

        let mem_size = {
            module
                .memories
                .iter_mut()
                .enumerate()
                .map(|(count, mem)| {
                    let id = module.imports.add(
                        "env",
                        &mem.name.clone().unwrap_or_else(|| match count {
                            0 => "memory".to_string(),
                            n => format!("memory{n}"),
                        }),
                        walrus::ImportKind::Memory(mem.id()),
                    );

                    mem.import = Some(id);

                    (mem.initial, mem.maximum.unwrap_or(mem.initial))
                })
                .collect::<Vec<_>>()
        };
        Some(mem_size)
    } else {
        None
    };

    // 0: Failed to load Wasm file: ./dist\threads_vfs.core.opt.adjusted.wasm
    // 1: failed to parse global section
    // 2: malformed mutability -- or shared globals require the shared-everything-threads proposal (at offset 0x49f)
    //
    // The Globals causing errors during memory expansion are those generated
    // by wasm-opt --multi-memory-lowering,
    // so for now we will only address these.
    // When a newly created thread is executed,
    // it will use the always-executable VFS code and memory,
    // which are based on an address that never changes,
    // and perform operations on them atomically.
    // Operations on Global variables are replaced,
    // and before memory unification,
    // memory.grow is modified to be an atomic operation.
    // Since this Global variable should only be modified internally,
    // this approach should be sufficient.
    // module
    //     .globals
    //     .iter()
    //     .map(|g| g.id())
    //     .collect::<Vec<_>>()
    //     .iter()
    //     .for_each(|g| {
    //         let g = module.globals.get_mut(*g);
    //         if let walrus::GlobalKind::Local(_) = g.kind {
    //             if g.mutable {
    //                 g.shared = true;
    //             }
    //         }
    //     });

    let new_path = path.with_extension("adjusted.wasm");

    if fs::metadata(&new_path).is_ok() {
        fs::remove_file(&new_path).wrap_err("Failed to remove existing file")?;
    }

    module
        .emit_wasm_file(new_path.clone())
        .to_eyre()
        .wrap_err("Failed to write temporary wasm file")?;

    Ok((new_path, mem_size))
}

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
        let name = <Wasip1SnapshotPreview1ThreadsFunc as VariantNames>::VARIANTS
            .iter()
            .exactly_one()
            .wrap_err("Expected exactly one variant for Wasip1SnapshotPreview1ThreadsFunc")?; // thread-spawn

        let component_name = gen_component_name(namespace, name);

        module
            .exports
            .erase_with(&format!("{name}_import_anchor"), ctx.unstable_print_debug)?;

        let real_thread_spawn_fn_id = (root, &component_name).get_fid(&module.imports)?;

        let branch_fid = "__wasip1_vfs_is_root_spawn".get_fid(&module.exports)?;

        let normal_thread_spawn_fn_id = ("wasi", "thread-spawn").get_fid(&module.imports)?;

        let self_thread_spawn_fn_id =
            "__wasip1_vfs_wasi_thread_spawn_self".get_fid(&module.exports)?;

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
                ("wasip1-vfs", "__wasip1_vfs_self_wasi_thread_start"),
                exporting_thread_starter_id,
                ctx.unstable_print_debug,
            )
            .wrap_err("Failed to rewrite self_wasi_thread_start call in root spawn")?;

        module.exports.erase_with(
            "__wasip1_vfs_self_wasi_thread_start_anchor",
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
                ("wasip1-vfs", "__wasip1_vfs_wasi_thread_start_entry"),
                exporting_thread_starter_id,
            )
            .wrap_err("Failed to connect wasip1-vfs.wasi_thread_start")?;

        module
            .exports
            .erase_with(branch_fid, ctx.unstable_print_debug)?;

        Ok(())
    }
}
