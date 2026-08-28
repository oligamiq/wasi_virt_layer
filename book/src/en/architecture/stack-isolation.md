# Stack Isolation and Handoff

WASI Virtual Layer (WVL) provides robust stack isolation and handoff mechanisms to prevent stack collisions between the Virtual File System (VFS) and the target WebAssembly modules, particularly in multi-threaded and asynchronous environments.

## Stack Handoff & Isolation ABI

The core of the stack isolation is managed in the library side (`wasi_virt_layer`). It defines an Application Binary Interface (ABI) that allows the runtime to dynamically switch between the stack of the target module and a standby or VFS stack.

### Key Components

*   **`StackInfo`**: Represents the configuration and runtime state of a stack, including its size, the current stack base pointer, and its slot index (for multi-memory environments).
*   **`StackHandoffRecord`**: Manages a serialized, standby stack handoff. It uses atomic locks and memory pointers to ensure that stack transitions are safe and prevent data corruption or collisions when multiple instances interact.
*   **Ensure and Release Operations**: 
    *   Functions like `ensure_vfs_stack` and `ensure_wasm_stack` are called when transitioning into a specific execution context to guarantee a valid stack is installed.
    *   Functions like `release_vfs_stack` and `release_wasm_stack` return the stack to a standby state, performing a handoff.

## Wasm Stream Passes (CLI Instrumentation)

The `wasi_virt_layer-cli` automates stack isolation by instrumenting WebAssembly binaries during the build process through a series of "stream passes".

### Single-Memory Targets
*   **`ExportStackPreVfsStreamPass`**: Instruments the VFS module by injecting stackless dynamic-stack handoff wrappers onto exported functions.
*   **`ExportStackPreTargetStreamPass`**: Instruments single-memory target modules to acquire and reclaim a private stack during function entry and exit.

### Multi-Memory Targets
In multi-memory scenarios, the CLI utilizes an "Arena" approach for stack management.

*   **`ExportStackArenaStreamPass`**: Creates a reserved stack slot arena in multi-memory target modules. It offsets all linear memory operations (like loads, stores, `MemorySize`, `MemoryGrow`) to virtualize the remaining memory space.
    *   It generates WebAssembly helpers (`build_slot_acquire`) that atomically claim a stack slot in the arena using an atomic Compare-And-Swap (CAS) operation.
    *   It also generates helpers (`build_slot_release`) to atomically clear a slot in the arena bitmap.
*   **`ExportStackMultiMemoryTargetStreamPass`**: Emits wrapper functions for multi-memory targets, utilizing the arena's slot-acquisition mechanisms to wrap exported functions safely.
