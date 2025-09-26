use std::fs;

use camino::Utf8PathBuf;
use eyre::Context;

use crate::util::{ResultUtil, WalrusUtilModule as _};

pub fn remove_unused_threads_function(wasm: &mut walrus::Module) -> eyre::Result<()> {
    // if wasm doesn't have wasi.thread-spawn on import,
    // wasm's export `wasi_thread_start` should be removed

    if !wasm
        .imports
        .iter()
        .any(|i| i.module == "wasi" && i.name == "thread-spawn")
    {
        if wasm.exports.iter().any(|e| e.name == "wasi_thread_start") {
            wasm.exports
                .remove("wasi_thread_start")
                .to_eyre()
                .wrap_err("Failed to remove wasi_thread_start export")?;
        } else {
            log::warn!(
                "wasi.thread-spawn is not imported, and wasi_thread_start is not exported. This expect multi-threaded Wasm. Is this non rust-lang wasm?"
            );
        }

        // todo!() check memory id used on thread-spawn function
        if let Some((mem, id)) = {
            wasm.imports
                .iter()
                .filter_map(|e| match e.kind {
                    walrus::ImportKind::Memory(mem) => Some((mem, e.id())),
                    _ => None,
                })
                .find(|_| true)
        } {
            wasm.imports.delete(id);
            wasm.memories
                .iter_mut()
                .find(|m| m.id() == mem)
                .unwrap()
                .import = None;
        } else {
            log::warn!(
                "wasi.thread-spawn is not imported, and shared memory is not exported. This expect multi-threaded Wasm. Is this non rust-lang wasm?"
            );
        }
    }

    Ok(())
}

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

                    (mem.initial, mem.maximum.unwrap_or(0))
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
