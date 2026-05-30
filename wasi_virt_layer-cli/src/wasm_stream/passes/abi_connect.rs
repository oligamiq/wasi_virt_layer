use crate::abi::{Wasip1ABIFunc, Wasip1ThreadsABIFunc};
use crate::unique_name::UniqueName;
use crate::wasm_stream::pipeline::StreamPass;
use crate::wasm_stream::translator::DefaultRebinder;
use eyre::Result;
use strum::VariantNames;
use wasm_encoder::{ImportSection, Module};

/// Helper to translate a `wasmparser::TypeRef` into a `wasm_encoder::EntityType`.
fn translate_type_ref(ty: wasmparser::TypeRef) -> wasm_encoder::EntityType {
    match ty {
        wasmparser::TypeRef::Func(f) => wasm_encoder::EntityType::Function(f),
        wasmparser::TypeRef::Table(t) => wasm_encoder::EntityType::Table(
            crate::wasm_stream::translator::translate_table_type(t, &DefaultRebinder),
        ),
        wasmparser::TypeRef::Memory(m) => wasm_encoder::EntityType::Memory(
            crate::wasm_stream::translator::translate_memory_type(m),
        ),
        wasmparser::TypeRef::Global(g) => wasm_encoder::EntityType::Global(
            crate::wasm_stream::translator::translate_global_type(g, &DefaultRebinder),
        ),
        wasmparser::TypeRef::Tag(t) => {
            wasm_encoder::EntityType::Tag(crate::wasm_stream::translator::translate_tag_type(t))
        }
        _ => unimplemented!("TypeRef variant not supported"),
    }
}

/// Helper to re-emit all sections, optionally rewriting imports.
/// `rewrite_import` is called for each import and returns `(module, name)`.
fn rewrite_imports(
    input_wasm: &[u8],
    rewrite_import: impl Fn(&str, &str, &wasmparser::TypeRef) -> (String, String),
) -> Result<Vec<u8>> {
    let mut module = Module::new();
    let parser = wasmparser::Parser::new(0);

    for payload in parser.parse_all(input_wasm) {
        let payload = payload?;
        let section_info = payload.as_section().map(|(id, r)| (id, r.clone()));

        match payload {
            wasmparser::Payload::ImportSection(s) => {
                let mut new_import_section = ImportSection::new();
                for import_group in s {
                    for import_res in import_group.unwrap() {
                        let (_, import) = import_res.unwrap();
                        let entity_type = translate_type_ref(import.ty);
                        let (new_module, new_name) =
                            rewrite_import(import.module, import.name, &import.ty);
                        new_import_section.import(&new_module, &new_name, entity_type);
                    }
                }
                module.section(&new_import_section);
            }
            _ => {
                if let Some((id, range)) = section_info {
                    module.section(&wasm_encoder::RawSection {
                        id,
                        data: &input_wasm[range.clone()],
                    });
                }
            }
        }
    }

    Ok(module.finish())
}

// ---------------------------------------------------------------------------
// ConnectWasip1ABIPreVfsStreamPass
// ---------------------------------------------------------------------------

/// Renames WASI ABI imports in the **VFS module** from `wasi_snapshot_preview1`
/// to `__wasip1_vfs-host`, with names like `__wasip1_vfs___self_{import}`.
///
/// This ensures that after merging, the VFS module's own WASI calls get routed
/// through the host (or resolved against matching exports) instead of recursing
/// back into the VFS.
pub struct ConnectWasip1ABIPreVfsStreamPass;

impl ConnectWasip1ABIPreVfsStreamPass {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self
    }
}

impl StreamPass for ConnectWasip1ABIPreVfsStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> Result<Vec<u8>> {
        rewrite_imports(input_wasm, |imp_module, imp_name, _ty| {
            if <Wasip1ABIFunc as VariantNames>::VARIANTS.contains(&imp_name)
                && imp_module == UniqueName::WASIP1_ABI_MODULE
            {
                let new_name = format!("__wasip1_vfs___self_{imp_name}");
                ("__wasip1_vfs-host".to_string(), new_name)
            } else {
                (imp_module.to_string(), imp_name.to_string())
            }
        })
    }
}

// ---------------------------------------------------------------------------
// ConnectWasip1ABIPreTargetStreamPass
// ---------------------------------------------------------------------------

/// Renames WASI ABI imports in a **target module** from `wasi_snapshot_preview1`
/// to `__wasip1_vfs-host`, with names like `__wasip1_vfs_{target}_{import}`.
pub struct ConnectWasip1ABIPreTargetStreamPass {
    /// The target module name.
    pub target_name: String,
}

impl ConnectWasip1ABIPreTargetStreamPass {
    /// Creates a new instance for the given target name.
    pub fn new(target_name: String) -> Self {
        Self { target_name }
    }
}

impl StreamPass for ConnectWasip1ABIPreTargetStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> Result<Vec<u8>> {
        let target = &self.target_name;
        rewrite_imports(input_wasm, |imp_module, imp_name, _ty| {
            if <Wasip1ABIFunc as VariantNames>::VARIANTS.contains(&imp_name)
                && imp_module == UniqueName::WASIP1_ABI_MODULE
            {
                let new_name = format!("__wasip1_vfs_{target}_{imp_name}");
                ("__wasip1_vfs-host".to_string(), new_name)
            } else {
                (imp_module.to_string(), imp_name.to_string())
            }
        })
    }
}

// ---------------------------------------------------------------------------
// NonRecursiveWasiABIPreVfsStreamPass
// ---------------------------------------------------------------------------

/// Rewrites non-recursive WASI imports (module = `non_recursive_wasi_snapshot_preview1`)
/// to point at the standard `wasi_snapshot_preview1` module so that the VFS can
/// call the *real* host WASI functions without infinite recursion.
pub struct NonRecursiveWasiABIPreVfsStreamPass;

impl NonRecursiveWasiABIPreVfsStreamPass {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self
    }
}

impl StreamPass for NonRecursiveWasiABIPreVfsStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> Result<Vec<u8>> {
        rewrite_imports(input_wasm, |imp_module, imp_name, _ty| {
            if <Wasip1ABIFunc as VariantNames>::VARIANTS.contains(&imp_name)
                && imp_module == UniqueName::CORE_NON_RECURSIVE_MODULE_ROOT
            {
                (
                    UniqueName::WASIP1_ABI_MODULE.to_string(),
                    imp_name.to_string(),
                )
            } else {
                (imp_module.to_string(), imp_name.to_string())
            }
        })
    }
}

// ---------------------------------------------------------------------------
// ConnectWasip1ThreadsABIPreTargetStreamPass
// ---------------------------------------------------------------------------

/// Renames WASI-threads ABI imports in a **target module** from the standard
/// `wasi` module to `__wasip1_vfs-host`, with names like
/// `__wasip1_vfs_wasi_thread_spawn_{target}`.
pub struct ConnectWasip1ThreadsABIPreTargetStreamPass {
    /// The target module name.
    pub target_name: String,
}

impl ConnectWasip1ThreadsABIPreTargetStreamPass {
    /// Creates a new instance for the given target name.
    pub fn new(target_name: String) -> Self {
        Self { target_name }
    }
}

impl StreamPass for ConnectWasip1ThreadsABIPreTargetStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> Result<Vec<u8>> {
        let target = &self.target_name;
        rewrite_imports(input_wasm, |imp_module, imp_name, _ty| {
            if <Wasip1ThreadsABIFunc as VariantNames>::VARIANTS.contains(&imp_name)
                && imp_module == UniqueName::WASIP1_THREADS_ABI_MODULE
            {
                let new_name = format!("__wasip1_vfs_wasi_thread_spawn_{target}");
                ("__wasip1_vfs-host".to_string(), new_name)
            } else {
                (imp_module.to_string(), imp_name.to_string())
            }
        })
    }
}
