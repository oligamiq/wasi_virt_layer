//! If `<anonymous>` is specified,
//! and there is only one target WASM,
//! it will be automatically assigned.

use smallvec::SmallVec;
use strum::VariantNames;

use crate::{
    abi::Wasip1ABIFunc,
    generator::{Generator, GeneratorCtx, memory::MemoryUniqueName},
    unique_name::UniqueName,
};

/// Generator component translating un-named single targets into explicit generic bindings.
#[derive(Debug, Default)]
pub struct Anonymous;

// TODO: Check thourgh with threads feature.
impl Generator for Anonymous {
    fn pre_vfs(&mut self, module: &mut walrus::Module, ctx: &GeneratorCtx) -> eyre::Result<()> {
        // If vfs include anonymous, and there is only one target, assign it to the target.

        // To find the all library which VFS wasm required.
        // Check export function `__wasip1_vfs_<anonymous>__start_anchor`.
        let anonymous_targets = module
            .exports
            .iter()
            .filter_map(|e| {
                e.name
                    .strip_prefix("__wasip1_vfs_")?
                    .strip_suffix("__start_anchor")
                    .map(|s| s.to_string())
            })
            .inspect(|targets| {
                log::info!("Found target which VFS wasm required: {targets}");
            })
            .collect::<Vec<_>>();

        let collected = ctx
            .target_names
            .iter()
            .filter(|t| !anonymous_targets.iter().any(|at| at == t.as_ref()))
            .collect::<SmallVec<[_; 1]>>();

        if collected.len() == 0 {
            return Ok(());
        }
        if collected.len() > 1 {
            eyre::bail!(
                "There are multiple targets which VFS wasm required but not assigned: {:?}. Please specify the target explicitly instead of `<anonymous>`.",
                collected
            );
        }

        // Replace VFS from `<anonymous>` to the only one target.
        let only_target = &collected[0];

        // Rewrite <anonymous> to the only one target.
        const EXPORT_POSTFIXS: &[&str] = &[
            "__start_anchor",
            "_memory_trap_anchor",
            "_wasi_thread_start_anchor",
        ];
        const PREFIX: &str = UniqueName::PREFIX;

        for postfix in EXPORT_POSTFIXS {
            let anonymous_export_name = format!("{PREFIX}anonymous{postfix}");
            let target_export_name = format!("{PREFIX}{only_target}{postfix}");
            if let Some(export) = module
                .exports
                .iter_mut()
                .find(|e| e.name == anonymous_export_name)
            {
                export.name = target_export_name;
            }
        }

        for export in module.exports.iter_mut() {
            if let Some(anonymous_suffix) = export
                .name
                .strip_prefix(PREFIX)
                .and_then(|s| s.strip_prefix("anonymous_"))
            {
                if let Some(f) = Wasip1ABIFunc::VARIANTS
                    .iter()
                    .find(|v| anonymous_suffix == **v)
                {
                    export.name = format!("{PREFIX}{only_target}_{f}");
                }
            }
        }

        if let Some(special_anonymous_export) = module
            .exports
            .iter_mut()
            .find(|e| e.name == format!("{PREFIX}wasi_thread_spawn_anonymous"))
        {
            special_anonymous_export.name = format!("{PREFIX}wasi_thread_spawn_{only_target}");
        }

        const NAMESPACE: &str = UniqueName::NAMESPACE;

        // TODO! Implement in `<>UniqueName` and use it.
        const EXTRA_IMPORTS: &[&str] = &[
            "_start",
            "memory_trap",
            "__main_void",
            "reset",
            "wasi_thread_start",
        ];

        for import in module
            .imports
            .iter_mut()
            .filter(|import| import.module == NAMESPACE)
        {
            if let Some(anonymous_suffix) = import
                .name
                .strip_prefix(PREFIX)
                .and_then(|s| s.strip_prefix("anonymous_"))
            {
                if let Some(f) = MemoryUniqueName::VARIANTS
                    .iter()
                    .chain(EXTRA_IMPORTS)
                    .find(|v| anonymous_suffix == **v)
                {
                    import.name = format!("{PREFIX}{only_target}_{f}");
                }
            }
        }

        // Print all exports
        for export in module.exports.iter() {
            log::info!("Export: {}", export.name);
        }

        // Print all imports
        for import in module.imports.iter() {
            log::info!("Import: {}.{}", import.module, import.name);
        }

        Ok(())
    }
}
