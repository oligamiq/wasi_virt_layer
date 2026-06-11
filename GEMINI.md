# WASI Virtual Layer (WVL) - Project Context

## Project Overview
WASI Virtual Layer (WVL) provides a virtualization layer for WebAssembly System Interface (WASI) modules. It allows wrapping existing WASI modules with a virtual file system (VFS), environment variables, and custom process/thread handling without significantly modifying the source code of the target modules.

### Core Technologies
- **Rust**: Primary development language.
- **WASI (wasip1 & wasip1-threads)**: Target ABI for virtualization.
- **WebAssembly Component Model**: Uses `wit-bindgen` and `wit-component` for module interaction.
- **Walrus**: Used by the CLI for Wasm IR manipulation and patching.
- **Deno**: Often used as a runtime for testing the generated JS/Wasm bundles.

### Architecture
1.  **`wasi_virt_layer`**: The core library providing macros (`plug_fs!`, `plug_env!`, `plug_process!`, `plug_poll!`, `plug_thread!`) and traits (`Wasip1FileSystem`, `VirtualEnv`, etc.) to implement virtualized behaviors.
2.  **`wasi_virt_layer-cli`**: A CLI tool (`wasi_virt_layer`) that automates the process of combining a VFS module with a target WASM module, patching imports/exports, and optionally transpiling to JavaScript.
3.  **Examples**: Located in `examples/`, showing various use cases like bare WASI, VFS, and multi-threading.

---

## Building and Running

### Prerequisites
- **Rust**: Version 1.89.0 or later (specified in `Cargo.toml`).
- **Cargo-nextest**: Recommended for running tests (`cargo binstall cargo-nextest`).
- **Deno**: Required for running generated JS tests.
- **mdbook**: For building the documentation.

### Key Commands
- **Build CLI**:
  ```bash
  cargo build -p wasi_virt_layer-cli
  ```
- **Run CLI (Example Build)**:
  ```bash
  cargo run -p wasi_virt_layer-cli -- build <wasm_path> -p <vfs_package>
  ```
- **Run Tests**:
  ```bash
  cargo nextest run -r
  ```
- **Build Documentation**:
  ```bash
  ./build_book.sh
  ```

---

## Development Conventions

### Coding Style & Macros
- **ABI Plugging**: The project heavily relies on macros to "plug" virtual implementations into the WASI ABI. These macros generate `extern "C"` functions that override standard WASI imports.
- **WIT Bindings**: Uses WebAssembly Interface Type (WIT) files (found in `wit/` directories) for defining component interfaces.
- **Shared Memory/Multi-threading**: When threading is enabled, the project manages shared memory and atomic operations to synchronize state between the VFS and the main module.

### Testing Practices
- **Integration Tests**: Located in `wasi_virt_layer-cli/tests/`, these often involve building a virtualized module and running it through a runtime like Deno to verify behavior.
- **Examples as Tests**: Many examples in `examples/` serve as both documentation and test cases for specific features (e.g., `threads_vfs`).

### Versioning
- Managed via workspace metadata in the root `Cargo.toml`.
- Version: `0.5.4` (as of current state).

---

## Key Files & Directories
- `wasi_virt_layer/src/lib.rs`: Entry point for the core library and macro exports.
- `wasi_virt_layer-cli/src/main.rs`: Entry point for the CLI tool.
- `wasi_virt_layer-cli/src/generator/`: Logic for patching and stitching Wasm modules together.
- `wasi_virt_layer/src/wasi/`: Implementations of various WASI subsystems (file, env, poll, etc.).
- `wit/`: WIT definitions for the virtualized interfaces.
- `book/`: Source for the `mdbook` documentation.
