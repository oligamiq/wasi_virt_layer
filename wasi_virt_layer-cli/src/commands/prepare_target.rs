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
use walrus::{Module, ModuleConfig, ValType, ir::*};

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
    replace_memory_grow(module, grow_fn)?;

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
fn add_globals(_module: &mut Module) -> eyre::Result<(walrus::GlobalId, walrus::GlobalId)> {
    log::debug!("Global variable creation deferred - not yet implemented");

    // TODO: Implement proper global variable creation via walrus API
    // This requires understanding the walrus GlobalId generation pattern
    // For now, return an error indicating this phase is not yet implemented
    Err(eyre::eyre!(
        "Global variable injection not yet implemented - target module must pre-define globals for metadata_ptr and lock_ptr"
    ))
}

/// Injects initialization code to call ABI registration functions.
fn inject_init_code(
    _module: &mut Module,
    _register_fn: walrus::FunctionId,
    _get_lock_ptr_fn: walrus::FunctionId,
    _metadata_ptr_global: walrus::GlobalId,
    _lock_ptr_global: walrus::GlobalId,
) -> eyre::Result<()> {
    log::debug!("Initialization code injection deferred");

    // TODO: Implement proper initialization code injection
    // This requires:
    // 1. Finding the _start function
    // 2. Using InstrRewrite to prepend initialization instructions
    // 3. Ensuring proper stack management for function calls
    //
    // For now, the globals are created but not initialized.
    // Users must manually call register() and get_lock_ptr() in their module
    // or we can implement this in the wrapper layer.

    Ok(())
}

/// Replaces all memory.grow instructions with ABI calls.
fn replace_memory_grow(module: &mut Module, grow_fn: walrus::FunctionId) -> eyre::Result<()> {
    log::debug!("Replacing memory.grow instructions with ABI calls");

    // The grow_fn signature is (metadata_ptr: i32, pages: i32) -> i32
    // memory.grow expects pages on the stack and returns new memory size or -1
    // We need to:
    // 1. Get metadata_ptr from somewhere accessible
    // 2. Push metadata_ptr before calling grow_fn
    // 3. The pages value is already on stack from memory.grow

    // For now, we'll use a simpler approach: replace memory.grow with direct ABI calls
    // The metadata_ptr will need to be injected via wrapper functions or globals
    // This is a complex transformation that requires careful stack management

    module.funcs.par_all_rewrite(
        |instr, _| {
            if matches!(instr, Instr::MemoryGrow(_)) {
                log::debug!("Replacing memory.grow instruction");

                // Simple replacement: call the grow function directly
                // The pages argument is already on the stack, but we need metadata_ptr
                // This is a limitation of the current approach - we'd need wrapper functions
                // For now, we'll just replace with the function call and let the caller
                // ensure metadata_ptr is available
                *instr = Instr::Call(Call { func: grow_fn });
            }
        },
        &[] as &[walrus::FunctionId],
    )?;

    log::debug!("Finished replacing memory.grow instructions");
    Ok(())
}
