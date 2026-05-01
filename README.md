# wasi-virt-layer
WASI Virtual Layer is a virtual file system layer for WebAssembly System Interface (WASI) modules. It allows you to run WASI modules with a virtual file system that can be customized and extended.

# example usage
0. Install the CLI tool with
```bash
cargo binstall wasi_virt_layer-cli
```
1. Prepare a WebAssembly module built for wasip1 (e.g. wasm32-wasip1 or wasm32-wasip1-threads).
2. Create a new virtual filesystem (VFS) project with
```bash
wasi_virt_layer new my_vfs_project
```
   This will automatically set up `Cargo.toml` with `crate-type = ["cdylib"]` and the necessary dependencies (`wasi_virt_layer`, `wit-bindgen`), as well as a basic `wit` directory and template source code.
3. Edit your VFS implementation. Use the `import_wasm!` macro to prepare for using the target wasm module.
4. Use the `plug!` macro series (`plug_process!`, `plug_env!`, `plug_fs!`, `plug_random!`, `plug_sched!`, etc.) to connect to the wasip1 ABI, and link virtual filesystems or virtual environment variables.
5. Run the build command
```bash
wasi_virt_layer build <wasm_path>
```
   to execute the virtualization process. Note: use `-p <package_name>` if your VFS project is part of a workspace.
6. The built files (including the transpiled JavaScript if applicable) will be generated in the `dist` directory.
7. Run it with Deno or Node.js:
```bash
deno run dist/test_run.ts
```
or start a static server and open `test_run.html` in your browser.

## As a component
By using plug! to block all WASIp1 ABIs and creating the ABI solely with wit-bindgen, it is entirely possible to treat it as a component. However, wasip1-threads is not supported as per the specification.
For further details, please refer to the example.

# TODO!
- [ ] Support non-binary Wasm modules (e.g., .wat files)
- [x] Enable specifying multiple Wasm modules
- [ ] Support `self` is not passed in `plug_thread!` (make it optional)
- [ ] Support self binary (improved `import_wasm!(self)` and CLI integration)
- [ ] Support `flush`/`sync` to virtual file system
- [ ] Fake global allocator and center allocator and merge with VFS
- [ ] Access Time Trait (expose to WASI ABI)
- [x] Multiple LFS file system (VFS)
- [x] Static file system (Embedded VFS)
- [ ] Feature: Access time and other stat traits
- [ ] Separate mode (connecting functions via JavaScript/Host)
- [ ] Threading VFS with non-threading WASM target
- [ ] Validator with detailed error reporting on thread mismatches
- [ ] Full Unicode support for paths (currently assumes Latin-1 bytes)
- [ ] Async WIT support

# run example
```bash
cargo r -r -- -p example_vfs examples/test_wasm/example/test_wasm_opt.wasm
cargo r -r -- -p threads_vfs test_threads -t single --threads true
```

# cannot
- support wasm-bindgen
  because it cannot use wasi

# non goal

# goal

# development
install `cargo binstall cargo-nextest -y`
install deno
- `cargo nextest run -r --fail-fast`

# Notes and Caveats
- **Thread & Memory Modes**: `single_memory` mode can sometimes fail in scenarios where `multi_memory` succeeds. A sequence like `_reset(); _start(); _main();` in multi-threaded environments has been observed to trigger issues in single-memory mode.
- **Build Cache**: The build process may benefit from a `--no-cache` option (currently a TODO) if the target directory caching causes stale builds.
- **Concurrency**: The CLI implements file-based locking to prevent collisions during parallel builds.
- **CLI Arguments**: Very long argument lists might still cause issues in some environments.
- **Self-Calling Fallback**: The tool supports a fallback mechanism for tools like `wasm-merge` and `wasm-opt` by calling itself if the binaries are not found in the PATH.

## Multi-threading execution order
- single_memory: known failures in some configurations.
- multi_memory: generally stable.
- VFS -> Target Module: Success
- Target Module -> Target Module: Success
- VFS -> VFS: Success
