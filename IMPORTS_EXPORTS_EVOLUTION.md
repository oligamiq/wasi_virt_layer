# Wasm Import/Export Evolution in `wasi_virt_layer`

This document outlines the ideal transformation of imports and exports as a virtual file system (VFS) and a target Wasm module are processed by `wasi_virt_layer`. Understanding this evolution is key to debugging and comprehending the virtualization process.

We will follow two modules as an example:
-   **VFS Module (`no_std_vfs.wasm`)**: A simple, no-std compatible virtual file system.
-   **Target Module (`test_wasm.wasm`)**: A standard WASI application that reads files and prints to stdout.

## Stage 1: The Original Modules

At the beginning, we have two separate core Wasm modules.

### VFS Module (`no_std_vfs.wasm`)

The VFS module is special. It's written to *provide* WASI file system APIs to another module, but it also needs to call "real" WASI APIs on the host to perform its own operations (like printing debug logs).

-   **Imports:**
    -   `wasi_snapshot_preview1`: Imports standard WASI functions it needs for its own logic (e.g., `proc_exit`, `fd_write`).
    -   `wasip1-vfs:host/virtual-file-system-wasip1-core`: Imports the full suite of WASI functions defined by the project's WIT interface. This is how it accesses the underlying host functionality.
    -   `wasip1-vfs`: Imports functions from the *target* module it will be linked with (e.g., `__wasip1_vfs_test_wasm_memory_director`, `__wasip1_vfs_test_wasm__start`). The `wasi_virt_layer` build tool expects these imports to be present.

-   **Exports:**
    -   `memory`: Its own linear memory.
    -   `__wasip1_vfs_test_wasm_fd_write`, etc.: It exports its own implementation of the WASI functions, which are named to correspond to the target module (`test_wasm`). These functions contain the VFS logic that intercepts the target's file calls.
    -   `__wasip1_vfs_flag_*`: Special flag exports used by the build tool to identify it as a VFS module.
    -   `main`: The entry point for the VFS logic itself.

### Target Module (`test_wasm.wasm`)

This is a standard WASI application.

-   **Imports:**
    -   `wasi_snapshot_preview1`: Imports all the WASI functions it needs to operate (e.g., `fd_read`, `path_open`, `proc_exit`).

-   **Exports:**
    -   `memory`: Its own linear memory.
    -   `_start` and `__main_void`: The standard entry points for a WASI executable.

## Stage 2: The Merged Module (`merged.wasm`)

The `wasi-merge` tool is used to combine the VFS and Target modules into a single core Wasm module. This stage is a critical transformation.

-   **Multi-Memory:** The module now contains two linear memories, one from the VFS (`memory 0`) and one from the target (`memory 1`).
-   **Name Mangling:** All of the target module's original exports are renamed (mangled) by `wasm-merge` to have a `__wasip1_vfs_test_wasm_` prefix. For example, `_start` becomes `__wasip1_vfs_test_wasm__start`.
-   **Import Resolution (Internal):** The VFS module's imports from the `wasip1-vfs` module (e.g., `__wasip1_vfs_test_wasm__start`) are now satisfied, as they link to the newly renamed exports from the target module.
-   **WASI Import Consolidation:** The final merged module no longer contains the target's original imports from `wasi_snapshot_preview1`. All WASI calls are now routed through the VFS. The only remaining WASI-related imports are those required by the VFS itself from the `wasip1-vfs:host/...` interface.
-   **New Exports:** The merged module exports the VFS's `memory` as the primary memory, and the target's memory as `__wasip1_vfs_test_wasm_memory`. It also exports the VFS's `main` function.

At this point, we have a single, self-contained module where all file system calls from the original target logic are intercepted by the VFS logic.

## Stage 3: The Component (`no_std_vfs.component.wasm`)

The `wit-component` tool wraps the merged core module in the Component Model format. This changes the interface of the module from low-level function imports to high-level interface imports.

-   **Component Imports:** The component now has a single, high-level import:
    -   `(import "wasip1-vfs:host/virtual-file-system-wasip1-core" (instance ...))`
    This declares that the component requires a host that implements the entire `virtual-file-system-wasip1-core` interface. The low-level WASI function imports of the core module are lifted to this component-level dependency.
-   **Core Module within Component:** The core module is embedded inside the component. Its imports are largely unchanged from the `merged.wasm` stage, still pointing to the `wasip1-vfs:host/...` functions. The component's instantiation logic will wire the component's high-level interface import to these low-level core module imports.
-   **Component Exports:** The component does not export any functions at this stage. It is an encapsulated unit whose behavior is triggered via its `main` function.

## Stage 4: The Final Core Module (`no_std_vfs.core.wasm`)

After the component is generated, the `jco` transpile process extracts a final, optimized core Wasm module. This is the module that is ultimately shipped and executed by the JavaScript runtime.

-   **Single Memory:** The multi-memory layout has been "lowered" back into a single, unified linear memory space.
-   **Consolidated Imports:** The imports are now minimal and clean. They are the fundamental system calls that the combined VFS and application logic needs from the host, as defined by the WIT interface.
    -   `(import "wasip1-vfs:host/virtual-file-system-wasip1-core" "[static]wasip1.fd-read-import" ...)`
    -   `(import "wasip1-vfs:host/virtual-file-system-wasip1-core" "[static]wasip1.proc-exit-import" ...)`
-   **Consolidated Exports:** The only significant exports are:
    -   `memory`: The single linear memory for the module.
    -   `main`: The single entry point to start the application.
    -   `cabi_realloc`: The canonical ABI function for memory management.

This final module is a self-contained executable where the original application's logic runs transparently on top of the virtual file system, and the entire system communicates with the host via a well-defined WIT interface.
