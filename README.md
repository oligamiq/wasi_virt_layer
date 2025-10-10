# TODO!
- Support non-binary Wasm modules
- Enable specifying multiple Wasm modules
- Support self binary
- support flush sync to file system
- Fake global allocator and center allocator and merge with vfs
- Access Time Trait
- Multiple lfs file system (VFS)
- Static file system
- Feature Access time etc traits
- Separate mode (connect function by javascript)
- threading vfs with non threading wasm
- valider with error on threads

# run example
```bash
cargo r -r -- -p example_vfs examples/test_wasm/example/test_wasm_opt.wasm
cargo r -r -- -p threads_vfs test_threads -t single --no-tracing --threads true
```

# cannot
- support wasm-bindgen
  because it cannot use wasi

# non goal

# goal
