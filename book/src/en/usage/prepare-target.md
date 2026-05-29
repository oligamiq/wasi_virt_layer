# Preparing Targets

The `prepare-target` command is used to transform a standard WASIP1 Wasm module into one that supports the Zero-Copy Shared Memory ABI.

## Usage

```bash
wasi_virt_layer prepare-target <input_wasm> -o <output_wasm>
```

## What it does

The command performs a series of transformations on the Wasm module:

1.  **Import ABI Functions**: Injects imports for the shared memory ABI functions (`register`, `grow`, `get_lock_ptr`).
2.  **Global Variable Injection**: Adds global variables to store the metadata pointer and lock manager pointer.
3.  **Initialization Code**: Injects a call to the shared memory initialization routine into the module's entry point.
4.  **Instruction Replacement**: Replaces all `memory.grow` instructions with calls to the VFS-side ABI.

## Why it is needed

Standard WASIP1 modules expect their own private linear memory. When running in a multi-threaded virtualized environment, we need to share a single linear memory across multiple modules to avoid expensive copying. The `prepare-target` command patches the module to cooperate with this shared memory model.
