# TODO!
- Support non-binary Wasm modules
- Enable specifying multiple Wasm modules
- Support self binary
- Allow paths to Cargo.toml instead of paths to wasm
- support flush sync to file system
- adjust test_run.ts to use VFS
- Fake global allocator and center allocator and merge with vfs
- Access Time Trait
- Multiple lfs file system (VFS)
- Static file system
- Resolve threads export conflict
- Feature Access time etc traits

# run example
```bash
cargo r -r -- -p example_vfs examples/test_wasm/example/test_wasm_opt.wasm
```

# cannot
- support wasm-bindgen
  because it cannot use wasi

# non goal

# goal
