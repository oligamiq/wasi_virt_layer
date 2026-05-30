//! Command for preparing target Wasm modules for shared memory operation.
//!
//! This command transforms a wasip1 target WASM module to use the shared memory ABI:
//! 1. Imports three ABI functions for shared memory management
//! 2. Adds global variables for metadata_ptr and lock_mgr_ptr
//! 3. Injects initialization code to register with VFS
//! 4. Replaces memory.grow instructions with ABI calls

use crate::util::WalrusUtilFuncs;
use camino::Utf8PathBuf;
use eyre::Context;
use std::fs;
use walrus::{ConstExpr, Module, ModuleConfig, ValType, ir::*};

/// Arguments for the `prepare-target` command.
#[derive(Debug, Clone)]
pub struct PrepareTargetHandler {
    /// Path to the target WASM module (wasip1)
    pub target_wasm: Utf8PathBuf,

    /// Output path for the transformed WASM module
    pub output: Utf8PathBuf,

    /// Whether to keep intermediate artifacts
    pub keep_artifacts: bool,
}

/// Transforms a target WASM module for shared memory operation.
pub fn prepare_target(args: PrepareTargetHandler) -> eyre::Result<()> {
    // Validate input file exists
    if !args.target_wasm.exists() {
        return Err(eyre::eyre!(
            "Target WASM file not found: {}",
            args.target_wasm
        ));
    }

    log::info!("Preparing target WASM: {}", args.target_wasm);

    // Load module with walrus
    let config = ModuleConfig::new();
    let mut module = Module::from_file_with_config(args.target_wasm.as_std_path(), &config)
        .map_err(|e| eyre::eyre!("Failed to parse target WASM: {}", e))?;

    // Transform module for shared memory
    transform_for_shared_memory(&mut module)?;

    // Emit the transformed module
    let output_bytes = module.emit_wasm();
    fs::write(&args.output, &output_bytes)
        .context(format!("Failed to write output WASM: {}", args.output))?;

    log::info!("Successfully prepared target Wasm: {}", args.output);

    Ok(())
}

/// Main transformation pipeline.
fn transform_for_shared_memory(module: &mut Module) -> eyre::Result<()> {
    log::debug!("Transforming module for shared memory operation");

    // Step 1: Import ABI functions
    let register_fn = add_abi_register(module)?;
    let get_lock_ptr_fn = add_abi_get_lock_ptr(module)?;
    let grow_fn = add_abi_grow(module)?;

    // Step 2: Add globals for metadata_ptr and lock_mgr_ptr
    let (metadata_ptr_global, lock_ptr_global) = add_globals(module)?;
    log::debug!(
        "Added globals: metadata_ptr={:?}, lock_ptr={:?}",
        metadata_ptr_global,
        lock_ptr_global
    );

    // Step 3: Inject initialization code to call ABI registration functions
    inject_init_code(
        module,
        register_fn,
        get_lock_ptr_fn,
        metadata_ptr_global,
        lock_ptr_global,
    )?;

    // Step 4: Check if memory.grow exists and replace it
    replace_memory_grow(module, grow_fn, metadata_ptr_global)?;

    log::debug!("Transformation complete");
    Ok(())
}

/// Adds the register ABI function import.
fn add_abi_register(module: &mut Module) -> eyre::Result<walrus::FunctionId> {
    log::debug!("Adding register ABI import");

    // Type: (i32, i32, i32) -> i32
    let register_type = module
        .types
        .add(&[ValType::I32, ValType::I32, ValType::I32], &[ValType::I32]);

    let (register_fn, _import_id) = module.add_import_func(
        "env",
        "wasip1_vfs_register_shared_memory_target",
        register_type,
    );

    log::debug!("Imported register function: {:?}", register_fn);
    Ok(register_fn)
}

/// Adds the grow ABI function import.
fn add_abi_grow(module: &mut Module) -> eyre::Result<walrus::FunctionId> {
    log::debug!("Adding grow ABI import");

    // Type: (i32, i32) -> i32
    let grow_type = module
        .types
        .add(&[ValType::I32, ValType::I32], &[ValType::I32]);

    let (grow_fn, _import_id) =
        module.add_import_func("env", "wasip1_vfs_shared_memory_grow", grow_type);

    log::debug!("Imported grow function: {:?}", grow_fn);
    Ok(grow_fn)
}

/// Adds the get_lock_ptr ABI function import.
fn add_abi_get_lock_ptr(module: &mut Module) -> eyre::Result<walrus::FunctionId> {
    log::debug!("Adding get_lock_ptr ABI import");

    // Type: (i32) -> i32
    let get_lock_ptr_type = module.types.add(&[ValType::I32], &[ValType::I32]);

    let (get_lock_ptr_fn, _import_id) = module.add_import_func(
        "env",
        "wasip1_vfs_shared_memory_get_lock_ptr",
        get_lock_ptr_type,
    );

    log::debug!("Imported get_lock_ptr function: {:?}", get_lock_ptr_fn);
    Ok(get_lock_ptr_fn)
}

/// Adds global variables for metadata_ptr and lock_mgr_ptr.
/// Returns global IDs for later use in initialization code.
fn add_globals(module: &mut Module) -> eyre::Result<(walrus::GlobalId, walrus::GlobalId)> {
    log::debug!("Adding shared memory globals");

    let metadata_ptr = module.globals.add_local(
        walrus::ValType::I32,
        true,
        false,
        ConstExpr::Value(Value::I32(0)),
    );

    let lock_ptr = module.globals.add_local(
        walrus::ValType::I32,
        true,
        false,
        ConstExpr::Value(Value::I32(0)),
    );

    // Give them names for easier debugging
    module.globals.get_mut(metadata_ptr).name =
        Some("__wvl_shared_memory_metadata_ptr".to_string());
    module.globals.get_mut(lock_ptr).name = Some("__wvl_shared_memory_lock_ptr".to_string());

    Ok((metadata_ptr, lock_ptr))
}

/// Injects initialization code to call ABI registration functions.
fn inject_init_code(
    module: &mut Module,
    register_fn: walrus::FunctionId,
    get_lock_ptr_fn: walrus::FunctionId,
    metadata_ptr_global: walrus::GlobalId,
    lock_ptr_global: walrus::GlobalId,
) -> eyre::Result<()> {
    log::debug!("Injecting initialization code");

    // 1. Find original _start
    let original_start_id =
        module
            .exports
            .iter()
            .find(|e| e.name == "_start")
            .and_then(|e| match e.item {
                walrus::ExportItem::Function(f) => Some(f),
                _ => None,
            });

    // 2. Determine initial memory parameters
    let memory = module
        .memories
        .iter()
        .next()
        .ok_or_else(|| eyre::eyre!("No memory found in module"))?;
    let initial_pages = memory.initial as i32;
    let max_pages = memory.maximum.map(|m| m as i32).unwrap_or(0);

    // 3. Create a new initialization function
    let mut builder = walrus::FunctionBuilder::new(&mut module.types, &[], &[]);

    // Call wasip1_vfs_register_shared_memory_target(base=0, pages, max_pages)
    builder
        .func_body()
        .i32_const(0) // base_ptr (assumed 0 for standalone, VFS will adjust if needed)
        .i32_const(initial_pages)
        .i32_const(max_pages)
        .call(register_fn)
        .global_set(metadata_ptr_global)
        // Call wasip1_vfs_shared_memory_get_lock_ptr(metadata_ptr)
        .global_get(metadata_ptr_global)
        .call(get_lock_ptr_fn)
        .global_set(lock_ptr_global);

    // 4. If original _start exists, call it and re-export the new one
    if let Some(orig_id) = original_start_id {
        builder.func_body().call(orig_id);

        let new_start = builder.finish(Vec::new(), &mut module.funcs);

        // Update export to point to the new function
        let export = module
            .exports
            .iter_mut()
            .find(|e| e.name == "_start")
            .unwrap();
        export.item = walrus::ExportItem::Function(new_start);

        log::debug!("Injected initialization into _start");
    } else {
        // Just create an initialization function and export it as __wvl_init
        let init_func = builder.finish(Vec::new(), &mut module.funcs);
        module.exports.add("__wvl_init", init_func);
        log::debug!("Created __wvl_init as no _start was found");
    }

    Ok(())
}

/// Replaces all memory.grow instructions with ABI calls.
fn replace_memory_grow(
    module: &mut Module,
    grow_fn: walrus::FunctionId,
    metadata_ptr_global: walrus::GlobalId,
) -> eyre::Result<()> {
    log::debug!("Replacing memory.grow instructions with helper function");

    // Create a helper function: __wvl_grow_wrapper(pages) -> old_size
    // wasip1_vfs_shared_memory_grow(metadata_ptr, pages) -> old_size
    let mut builder = walrus::FunctionBuilder::new(
        &mut module.types,
        &[walrus::ValType::I32],
        &[walrus::ValType::I32],
    );
    let pages_local = module.locals.add(walrus::ValType::I32);

    builder
        .func_body()
        .global_get(metadata_ptr_global)
        .local_get(pages_local)
        .call(grow_fn);

    let wrapper_fn = builder.finish(vec![pages_local], &mut module.funcs);
    module.funcs.get_mut(wrapper_fn).name = Some("__wvl_grow_wrapper".to_string());

    // Replace all memory.grow with call wrapper_fn
    module.funcs.par_all_rewrite(
        |instr, _| {
            if matches!(instr, walrus::ir::Instr::MemoryGrow(_)) {
                *instr = walrus::ir::Instr::Call(walrus::ir::Call { func: wrapper_fn });
            }
        },
        &[] as &[walrus::FunctionId],
    )?;

    log::debug!("Finished replacing memory.grow instructions");
    Ok(())
}
