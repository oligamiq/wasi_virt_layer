#!/bin/bash
set -e

# Build the threaded target
echo "Building test_threads..."
cargo +nightly b -r --target wasm32-wasip1-threads -p test_threads

# Build the other targets
echo "Building ls, args, tree..."
cargo b -r --target wasm32-wasip1 -p ls
cargo b -r --target wasm32-wasip1 -p args
cargo b -r --target wasm32-wasip1 -p tree

# Build the VFS and combine them
echo "Combining with wasi_virt_layer-cli..."
cargo run -p wasi_virt_layer-cli -- build \
    --manifest-path examples/vfs/repro_multi_target_table_bug/Cargo.toml \
    target/wasm32-wasip1-threads/release/test_threads.wasm \
    target/wasm32-wasip1/release/ls.wasm \
    target/wasm32-wasip1/release/args.wasm \
    target/wasm32-wasip1/release/tree.wasm \
    -t single \
    --threads true \
    --keep-build-artifacts \
    --out-dir dist

echo "Running with Deno..."
deno run -A dist/test_run.ts
