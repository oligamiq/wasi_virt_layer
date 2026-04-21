use eyre::Context as _;
use strum::VariantNames;

use crate::{
    abi::{Wasip1ABIFunc, Wasip1ThreadsABIFunc},
    generator::{Generator, GeneratorCtx, ModuleExternal, threads::ThreadsSpawnName},
    unique_name::UniqueName,
    util::{ResultUtil as _, WalrusFID as _, WalrusUtilModule},
};

fn has_library_import_anchor_names(export_names: &[&str]) -> bool {
    let has_legacy_thread_anchor = <Wasip1ThreadsABIFunc as VariantNames>::VARIANTS
        .iter()
        .map(|name| format!("{name}_import_anchor"))
        .any(|legacy| export_names.iter().any(|name| *name == legacy));

    if has_legacy_thread_anchor {
        return false;
    }

    let has_wasip1_anchor = <Wasip1ABIFunc as VariantNames>::VARIANTS
        .iter()
        .map(|name| format!("{name}_import_anchor"))
        .any(|required| export_names.iter().any(|name| *name == required));

    let has_prefixed_thread_anchor = <Wasip1ThreadsABIFunc as VariantNames>::VARIANTS
        .iter()
        .map(
            |name| UniqueName::ThreadsSpawn(&ThreadsSpawnName::ImportAnchor(name)).to_string(),
        )
        .any(|required| export_names.iter().any(|name| *name == required));

    has_wasip1_anchor || has_prefixed_thread_anchor
}

/// Checks if the provided Wasm securely linked `wasi_virt_layer` anchors correctly.
#[derive(Debug, Default)]
pub struct CheckUseLibrary;

impl Generator for CheckUseLibrary {
    fn pre_vfs(&mut self, module: &mut walrus::Module, ctx: &GeneratorCtx) -> eyre::Result<()> {
        // If you're using the library, anchors should be generated automatically.
        let export_names = module
            .exports
            .iter()
            .map(|e| e.name.as_str())
            .collect::<Vec<_>>();

        if !has_library_import_anchor_names(&export_names)
        {
            eyre::bail!(
                r#"This wasm file is not use "wasi_virt_layer" crate, you need to add it to your dependencies and use wasi_virt_layer; or, it does not import a crate."#
            );
        }

        // check use import_wasm!
        for wasm_name in &ctx.target_names {
            if !module
                .exports
                .iter()
                .any(|export| export.name == format!("__wasip1_vfs_{wasm_name}__start_anchor"))
            {
                let suggests = module
                    .exports
                    .iter()
                    .filter_map(|e| {
                        e.name
                            .strip_prefix("__wasip1_vfs_")?
                            .strip_suffix("__start_anchor")
                            .map(|s| s.to_string())
                    })
                    .collect::<Vec<_>>();

                let best_suggest = suggests
                    .iter()
                    .map(|s| (strsim::jaro_winkler(&s, wasm_name.as_ref()), s))
                    .min_by(|(a, _), (b, _)| b.partial_cmp(a).unwrap());

                let msg = if let Some((score, suggest)) = best_suggest
                    && score > 0.8
                {
                    format!(
                        "\nDid you mean `{wasm_name}`? You used `{suggest}` in `import_wasm!` macro."
                    )
                } else {
                    String::new()
                };

                eyre::bail!(
                    "Failed to get __start_anchor export on {wasm_name}. You may forget definition `import_wasm!` macro with wasm name.{msg}",
                );
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strict_accepts_prefixed_thread_anchor() {
        let exports = vec!["__wasip1_vfs_thread-spawn_import_anchor"];
        assert!(has_library_import_anchor_names(&exports));
    }

    #[test]
    fn strict_rejects_legacy_thread_anchor() {
        let exports = vec!["thread-spawn_import_anchor"];
        assert!(!has_library_import_anchor_names(&exports));
    }

    #[test]
    fn strict_rejects_legacy_thread_anchor_with_wasip1_anchor() {
        let exports = vec!["thread-spawn_import_anchor", "fd_write_import_anchor"];
        assert!(!has_library_import_anchor_names(&exports));
    }

    #[test]
    fn still_accepts_wasip1_import_anchor() {
        let exports = vec!["fd_write_import_anchor"];
        assert!(has_library_import_anchor_names(&exports));
    }

    #[test]
    fn strict_accepts_prefixed_thread_anchor_with_wasip1_anchor() {
        let exports = vec![
            "__wasip1_vfs_thread-spawn_import_anchor",
            "fd_write_import_anchor",
        ];
        assert!(has_library_import_anchor_names(&exports));
    }
}

/// Strict check validating that compiled WASM matches intended thread-safe / single-threaded memory paradigms.
#[derive(Debug, Default)]
pub struct CheckVFSMemoryType;

impl Generator for CheckVFSMemoryType {
    fn pre_vfs(&mut self, module: &mut walrus::Module, ctx: &GeneratorCtx) -> eyre::Result<()> {
        let target_memory_type = module.get_memory_type(true)?;

        if ctx.target_memory_type != target_memory_type {
            eyre::bail!(
                "Target memory type mismatch: expected {:?}, found {:?}. Why?",
                ctx.target_memory_type,
                target_memory_type
            );
        }

        Ok(())
    }
}

/// Detects unused thread imports and efficiently cleans thread-related exports statically.
#[derive(Debug, Default)]
pub struct CheckUnusedThreads;

impl CheckUnusedThreads {
    fn remove_unused_threads_function(wasm: &mut walrus::Module) -> eyre::Result<()> {
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
}

impl Generator for CheckUnusedThreads {
    fn pre_vfs(&mut self, module: &mut walrus::Module, ctx: &GeneratorCtx) -> eyre::Result<()> {
        if ctx.threads {
            Self::remove_unused_threads_function(module)
                .wrap_err("Failed to remove unused threads function")?;
        }
        Ok(())
    }

    fn pre_target(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
        _: &ModuleExternal,
    ) -> eyre::Result<()> {
        if ctx.threads {
            Self::remove_unused_threads_function(module)
                .wrap_err("Failed to remove unused threads function")?;
        }
        Ok(())
    }
}

/// Asserts the file comes from the Rust `rustc` compiler via `bindgen` standard structures.
#[derive(Debug, Default)]
pub struct IsRustWasm;

impl Generator for IsRustWasm {
    fn pre_target(
        &mut self,
        module: &mut walrus::Module,
        _: &GeneratorCtx,
        external: &ModuleExternal,
    ) -> eyre::Result<()> {
        let producers = format!("{:?}", module.producers);

        if !["_start", "__main_void"]
            .iter()
            .all(|name| module.exports.iter().any(|e| e.name == *name))
            || !["Rust", "rustc"]
                .iter()
                .all(|name| producers.contains(format!(r#""{name}""#).as_str()))
        {
            log::error!(
                "This file: {} is not built by rust toolchain, or you forget to export _start or main_void function. If you use `cdylib` or `rlib`, please change to `bin` or `lib`.\nIf you use other language, create an issue.",
                external.name
            );
        }

        Ok(())
    }
}

/// Quick invariant check asserting if Virtual Memory hooks exist inside Target Wasm representation.
#[derive(Debug, Default)]
pub struct CheckUseWasiVirtLayer;

impl Generator for CheckUseWasiVirtLayer {
    fn pre_vfs(&mut self, module: &mut walrus::Module, _: &GeneratorCtx) -> eyre::Result<()> {
        if "__wasip1_vfs_flag_vfs_memory"
            .get_fid(&module.exports)
            .ok()
            .is_none()
        {
            eyre::bail!(
                r#"This wasm file is not use "wasi_virt_layer" crate, you need to add it to your dependencies and use wasi_virt_layer;"#
            );
        }

        Ok(())
    }
}
