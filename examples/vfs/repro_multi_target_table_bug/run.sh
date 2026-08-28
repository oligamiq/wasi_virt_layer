#!/bin/bash
set -e

# Build the threaded target
echo "Building test_threads..."
cargo b -r --target wasm32-wasip1-threads -p test_threads

# Build the other targets
echo "Building ls, args..."
cargo b -r --target wasm32-wasip1 -p ls
cargo b -r --target wasm32-wasip1 -p args
cp target/wasm32-wasip1/release/ls.wasm target/wasm32-wasip1/release/ls2.wasm

# Build the VFS and combine them
# Use package/artifact names so the CLI can apply target-specific ABI handling.
echo "Combining with wasi_virt_layer-cli..."
# Threaded VFS/reactor builds need Rust 1.100+; use the first supported nightly as a fallback.
RUSTUP_TOOLCHAIN=nightly-2026-08-27 cargo run -p wasi_virt_layer-cli -- build \
    -p repro_multi_target_table_bug \
    test_threads ls args ls2 \
    -t single \
    --threads true \
    --keep-build-artifacts \
    --out-dir dist

echo "Running with Deno..."
deno run -A dist/test_run.ts
