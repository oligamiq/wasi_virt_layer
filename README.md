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
4. Use the `plug!` macro series (`plug_process!`, `plug_env!`, `plug_fs!`, etc.) to connect to the wasip1 ABI, and link virtual filesystems or virtual environment variables.
5. Run the build command
```bash
wasi_virt_layer build <wasm_path> -p my_vfs_project
```
   to execute the virtualization process.
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
- Support non-binary Wasm modules
- [x] Enable specifying multiple Wasm modules
- Support self is not passed in plug_thread!
- Support self binary
- support flush sync to file system
- Fake global allocator and center allocator and merge with vfs
- Access Time Trait
- [x] Multiple lfs file system (VFS)
- [x] Static file system
- Feature Access time etc traits
- Separate mode (connect function by javascript)
- threading vfs with non threading wasm
- validator with error on threads
- Unicode support
- Async wit support

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

# メモ
cargo r -- -p threads_vfs test_threads -t multi --threads true
_resetなしだとsingleもmultiも成功

        test_threads::_reset();
        test_threads::_start();
        test_threads::_main();
だとmultiで成功。singleでunreacable

ここら辺もテストに追加
build target dirのキャッシュ(--no-cache)
同時実行対策
超絶長い引数で失敗するかも
自分自身を呼び出す（フォールバック）

## 色んなスレッド作成の順番での失敗か
single_memory failed
multi_memory OK
VFS -> Body

success
Body -> Body

success
VFS -> VFS
