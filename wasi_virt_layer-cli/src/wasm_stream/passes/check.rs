use crate::abi::{Wasip1ABIFunc, Wasip1ThreadsABIFunc};
use crate::generator::GeneratorCtx;
use crate::unique_name::UniqueName;
use crate::wasm_stream::pipeline::StreamChecker;
use eyre::Result;
use strum::VariantNames;
use wasmparser::Payload;

/// Checks whether the file comes from the Rust `rustc` compiler.
#[derive(Debug, Default)]
pub struct IsRustWasmChecker {
    is_rust: bool,
}

impl IsRustWasmChecker {
    pub fn new() -> Self {
        Self { is_rust: false }
    }
}

impl StreamChecker for IsRustWasmChecker {
    fn check(&mut self, payload: &Payload) -> Result<()> {
        if let Payload::CustomSection(c) = payload {
            if c.name() == ".rustc" || c.name() == ".cargo" {
                self.is_rust = true;
            }
        } else if let Payload::End(_) = payload {
            if !self.is_rust {
                log::warn!("Target is not a Rust module. Missing .rustc or .cargo custom section.");
            }
        }
        Ok(())
    }
}

/// Checks if the provided Wasm securely linked `wasi_virt_layer` anchors correctly.
#[derive(Debug)]
pub struct CheckUseLibraryChecker {
    ctx: GeneratorCtx,
    export_names: Vec<String>,
}

impl CheckUseLibraryChecker {
    pub fn new(ctx: GeneratorCtx) -> Self {
        Self {
            ctx,
            export_names: Vec::new(),
        }
    }
}

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
        .map(|name| {
            UniqueName::ThreadsSpawn(&crate::generator::threads::ThreadsSpawnName::ImportAnchor(
                name,
            ))
            .to_string()
        })
        .any(|required| export_names.iter().any(|name| *name == required));

    has_wasip1_anchor || has_prefixed_thread_anchor
}

fn normalize_name(s: &str) -> String {
    s.replace('-', "_")
}

impl StreamChecker for CheckUseLibraryChecker {
    fn check(&mut self, payload: &Payload) -> Result<()> {
        if let Payload::ExportSection(s) = payload {
            for export in s.clone() {
                let export = export.unwrap();
                self.export_names.push(export.name.to_string());
            }
        } else if let Payload::End(_) = payload {
            let names_ref: Vec<&str> = self.export_names.iter().map(|s| s.as_str()).collect();

            if !has_library_import_anchor_names(&names_ref) {
                eyre::bail!(
                    r#"This wasm file is not use "wasi_virt_layer" crate, you need to add it to your dependencies and use wasi_virt_layer; or, it does not import a crate."#
                );
            }

            // check use import_wasm!
            for wasm_name in self.ctx.target_names.iter() {
                let normalized_name = normalize_name(wasm_name.as_ref());
                if !self
                    .export_names
                    .iter()
                    .any(|name| name == &format!("__wasip1_vfs_{normalized_name}__start_anchor"))
                {
                    let suggests = self
                        .export_names
                        .iter()
                        .filter_map(|name| {
                            if name.starts_with("__wasip1_vfs_") && name.ends_with("__start_anchor")
                            {
                                Some(
                                    name.replace("__wasip1_vfs_", "")
                                        .replace("__start_anchor", ""),
                                )
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                    eyre::bail!(
                        "WASM module `{wasm_name}` is provided as a target argument, but not declared with `import_wasm!` macro in VFS. Found: {suggests:?}"
                    );
                }
            }

            // Check that all import_wasm! declarations have corresponding target WASM arguments
            let declared_wasm_names = self
                .export_names
                .iter()
                .filter_map(|name| {
                    name.strip_prefix("__wasip1_vfs_")?
                        .strip_suffix("__start_anchor")
                        .map(|s| s.to_string())
                })
                .collect::<Vec<_>>();

            for declared_name in declared_wasm_names {
                if declared_name == "anonymous"
                    || normalize_name(&declared_name) == normalize_name(self.ctx.vfs_name.as_ref())
                {
                    continue;
                }

                if !self
                    .ctx
                    .target_names
                    .iter()
                    .any(|target| normalize_name(target.as_ref()) == normalize_name(&declared_name))
                {
                    eyre::bail!(
                        "WASM module `{declared_name}` is declared with `import_wasm!` macro in VFS, but not provided as a target argument. \
                         Did you forget to specify this WASM file in the CLI arguments?"
                    );
                }
            }
        }
        Ok(())
    }
}

/// Strict check validating that compiled WASM matches intended thread-safe / single-threaded memory paradigms.
#[derive(Debug)]
pub struct CheckVFSMemoryTypeChecker {
    ctx: GeneratorCtx,
    target_memory_type: Option<crate::args::TargetMemoryType>,
}

impl CheckVFSMemoryTypeChecker {
    pub fn new(ctx: GeneratorCtx) -> Self {
        Self {
            ctx,
            target_memory_type: None,
        }
    }
}

impl StreamChecker for CheckVFSMemoryTypeChecker {
    fn check(&mut self, payload: &Payload) -> Result<()> {
        if let Payload::MemorySection(s) = payload {
            for memory in s.clone().into_iter() {
                let memory = memory.unwrap();
                // Determine memory type from the first memory definition
                if memory.shared {
                    self.target_memory_type = Some(crate::args::TargetMemoryType::Multi);
                } else {
                    self.target_memory_type = Some(crate::args::TargetMemoryType::Single);
                }
                break;
            }
        } else if let Payload::End(_) = payload {
            if let Some(target_memory_type) = self.target_memory_type {
                if self.ctx.target_memory_type != target_memory_type {
                    eyre::bail!(
                        "Target memory type mismatch: expected {:?}, found {:?}. Why?",
                        self.ctx.target_memory_type,
                        target_memory_type
                    );
                }
            } else {
                log::warn!("No memory section found to check memory type.");
            }
        }
        Ok(())
    }
}

/// Quick invariant check asserting if Virtual Memory hooks exist inside Target Wasm representation.
#[derive(Debug, Default)]
pub struct CheckUseWasiVirtLayerChecker {
    found: bool,
}

impl CheckUseWasiVirtLayerChecker {
    pub fn new() -> Self {
        Self { found: false }
    }
}

impl StreamChecker for CheckUseWasiVirtLayerChecker {
    fn check(&mut self, payload: &Payload) -> Result<()> {
        match payload {
            Payload::ExportSection(s) => {
                for export in s.clone().into_iter() {
                    let export = export?;
                    if export.name == "__wasip1_vfs_flag_vfs_memory" {
                        self.found = true;
                    }
                }
            }
            Payload::End(_) => {
                if !self.found {
                    eyre::bail!(
                        r#"This wasm file is not use "wasi_virt_layer" crate, you need to add it to your dependencies and use wasi_virt_layer;"#
                    );
                }
            }
            _ => {}
        }
        Ok(())
    }
}
