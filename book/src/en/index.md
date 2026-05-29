# wasi-virt-layer
WASI Virtual Layer is a virtual file system layer for WebAssembly System Interface (WASI) modules. It allows you to run WASI modules with a virtual file system that can be customized and extended.

## Example Usage
0. Install the CLI tool with
```bash
cargo binstall wasi_virt_layer-cli
```
1. Prepare a WebAssembly module built for wasip1 (e.g. wasm32-wasip1 or wasm32-wasip1-threads).
2. Create a new virtual filesystem (VFS) project with
```bash
wasi_virt_layer new my_vfs_project
```
3. Edit your VFS implementation. Use the `import_wasm!` macro to prepare for using the target wasm module.
4. Use the `plug!` macro series to connect to the wasip1 ABI.
5. Run the build command
```bash
wasi_virt_layer build <wasm_path>
```
6. The built files will be generated in the `dist` directory.
7. Run it with Deno or Node.js.
