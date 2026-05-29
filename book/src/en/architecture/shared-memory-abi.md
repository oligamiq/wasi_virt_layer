# Shared Memory ABI Specification

## 1. Overview and Goals

### Purpose
To achieve memory sharing in `pseudo_import_wasm`, completely eliminating memory copies between the VFS and target Wasm modules.

### Prerequisites
- All memory used by the target Wasm module resides within **shared memory**.
- Zero-copy: No memory copying occurs during normal operation.
- **Memory sharing between targets**: Similar to wasm-opt's memory unification approach.
- Memory growth is **requested by the target** and automatically allocated by the VFS.
- **Only available when the `threads` feature is enabled.**

---

## 2. Architecture Overview

### 2.1 VFS Side (Rust)
- Exports ABI functions (`register`, `grow`, `get_lock_ptr`).
- Manages target metadata (base pointer, limit pointer, etc.).
- Uses `parking_lot::RwLock` for thread-safe access to shared memory.

### 2.2 Target Side (WASIP1)
- Wasm modules are patched using the `prepare-target` command.
- Memory instructions are transformed to handle shared memory offsets.
- `memory.grow` is replaced with ABI calls to the VFS.
- Global variables store pointers to metadata and lock managers.

---

## 3. ABI Function Specifications

### 3.1 `wasip1_vfs_register_shared_memory_target`
Called once by the target during initialization.
- **Input**: `base_ptr`, `current_pages`, `max_pages`.
- **Output**: `metadata_ptr` (pointer to metadata assigned by VFS).

### 3.2 `wasip1_vfs_shared_memory_get_lock_ptr`
Called once by the target during initialization.
- **Input**: `metadata_ptr`.
- **Output**: `lock_mgr_ptr` (pointer to lock manager on the VFS side).

### 3.3 `wasip1_vfs_shared_memory_grow`
Called by the target when out-of-bounds access occurs or when explicit growth is needed.
- **Input**: `metadata_ptr`, `required_pages`.
- **Output**: Success indicator (0 for success, -1 for failure).

---

## 4. Initialization Flow

1. Target Wasm starts.
2. `_start()` or `_main()` is called.
3. `__init_shared_memory()` executes:
    - Calls `register_shared_memory_target`.
    - Saves `metadata_ptr` to a global variable.
    - Calls `get_lock_ptr`.
    - Saves `lock_mgr_ptr` to a global variable.
4. Main target code runs.
