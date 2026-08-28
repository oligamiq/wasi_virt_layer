# Handover: Own-Memory Mode Thread Synchronization Refactor

## Current Status
We are in the middle of refactoring the `--own-memory` mode to support multi-threading. The previous implementation using Wasm globals (`logical_size_gid`) failed because these globals are not shared across threads, leading to memory corruption.

The new approach moves the logical memory size state (`own_memory_size`) into the VFS's Rust code as `AtomicI32` variables, ensuring they are synchronized across all threads.

## Changes Made So Far
1.  **VFS Side (`wasi_virt_layer/src/memory.rs`)**:
    *   Updated `import_wasm!` macro to include a static `AtomicI32` and export `get`, `set`, and `init` functions for each target.
    *   Updated `own_memory!` macro to include similar static state and exported functions for the host (VFS) itself.
2.  **CLI Side (`multi_memory_lowering.rs`)**:
    *   Removed the dynamic generation of `logical_size_gid` globals.
    *   Reverted incorrect attempts to inject Wasm imports (as these functions are already available as exports in the merged Wasm).
    *   Reset `ImportRebinder` and `new_func_count` logic to their stable state.

## Remaining Implementation Steps
1.  **Capture Export Indices**: In `multi_memory_lowering.rs`, during the first pass (where `Payload::ExportSection` is parsed), find and store the function indices for:
    *   `__wasip1_vfs_host_own_memory_size_{get,set,init}`
    *   `__wasip1_vfs_{target}_own_memory_size_{get,set,init}` (for each target)
2.  **Update Wrappers**: In the generated `memory.size` and logical `memory.grow` wrappers:
    *   Replace the now-removed `GlobalGet`/`GlobalSet` with `Call` instructions targeting the exported functions identified in Step 1.
    *   Remember to apply `rebinder.function(vfs_exported_idx)` to get the correct remapped index.
3.  **Inject Initialization**: In `multi_memory_lowering.rs`, inside the `CodeSection` processing for the `__init_offset_global` function:
    *   Inject calls to the `_init` functions for the host and all targets using their initial memory sizes (found in `memory_initials`).

## Key Discoveries
*   `wit-component` is very strict about `wasip1-vfs` imports. By using exports from the VFS module (which is merged into the same Wasm), we completely bypass the need for external component imports for internal state tracking.
*   The function index calculation in `multi_memory_lowering.rs` is sensitive to added imports. By using internal exports, we keep the index space stable.
