use std::fs;

use camino::Utf8PathBuf;
use eyre::{Context, ContextCompat};

use crate::{rewrite::TargetMemoryType, util::ResultUtil};

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
    target_memory_type: TargetMemoryType,
) -> eyre::Result<(Utf8PathBuf, Option<(u64, u64)>)> {
    let mut module = walrus::Module::from_file(path)
        .to_eyre()
        .wrap_err("Failed to load module")?;

    module.memories.iter_mut().for_each(|mem| {
        mem.shared = true;
    });

    let mem_size = if matches!(target_memory_type, TargetMemoryType::Single) {
        if module.memories.len() > 1 {
            eyre::bail!("Why are there multiple memories in core wasm? This is unexpected.");
        }

        let mem = module.memories.iter_mut().next().unwrap();

        let id = module.imports.add(
            "env",
            mem.name.as_ref().unwrap_or(&"memory".into()),
            walrus::ImportKind::Memory(mem.id()),
        );

        mem.import = Some(id);

        Some((
            mem.initial,
            mem.maximum.wrap_err("Failed to get memory maximum")?,
        ))
    } else {
        None
    };

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
