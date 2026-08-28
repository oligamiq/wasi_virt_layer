@echo off
setlocal

echo Building test_threads...
cargo build -r --target wasm32-wasip1-threads -p test_threads
if %ERRORLEVEL% neq 0 exit /b %ERRORLEVEL%

echo Building ls...
cargo build -r --target wasm32-wasip1 -p ls
if %ERRORLEVEL% neq 0 exit /b %ERRORLEVEL%

echo Combining with wasi_virt_layer-cli...
rem Threaded VFS/reactor builds need Rust 1.100+; use the first supported nightly as a fallback.
set "RUSTUP_TOOLCHAIN=nightly-2026-08-27"
cargo run -p wasi_virt_layer-cli -- build ^
    --manifest-path examples/vfs/minimal_repro/Cargo.toml ^
    target/wasm32-wasip1-threads/release/test_threads.wasm ^
    target/wasm32-wasip1/release/ls.wasm ^
    -t single ^
    --threads true ^
    --keep-build-artifacts ^
    --out-dir dist_minimal
if %ERRORLEVEL% neq 0 exit /b %ERRORLEVEL%
set "RUSTUP_TOOLCHAIN="

echo Running with Deno...
deno run -A dist_minimal/test_run.ts
if %ERRORLEVEL% neq 0 exit /b %ERRORLEVEL%
