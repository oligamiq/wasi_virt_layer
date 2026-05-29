# Core Macros

WASI Virtual Layer provides several macros to "plug" virtualized implementations into the WASI ABI.

## `import_wasm!`

Imports a WebAssembly module for use with the virtualization layer.

```rust
import_wasm!(module_name);
```

## `plug_fs!`

Connects a virtual file system implementation to one or more Wasm modules.

### Usage
```rust
plug_fs!(vfs_implementation, target_wasm, self);
```

- `vfs_implementation`: An object implementing the `Wasip1FileSystem` trait.
- `target_wasm`: The identifier of the Wasm module (imported via `import_wasm!`).
- `self`: Optional. If included, file operations within the VFS itself will also be virtualized.

## `plug_env!`

Connects virtual environment variables.

### Usage
```rust
plug_env!(env_implementation, target_wasm, self);
```

## `plug_args!`

Connects virtual command-line arguments.

## `plug_process!`

Connects virtual process management (e.g., exit handlers).

## `plug_thread!`

Connects virtual threading support. (Requires the `threads` feature).
