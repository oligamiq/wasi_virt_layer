@echo off
setlocal

echo Building test_threads...
cargo build -r --target wasm32-wasip1-threads -p test_threads
if %ERRORLEVEL% neq 0 exit /b %ERRORLEVEL%

echo Building ls, args...
cargo build -r --target wasm32-wasip1 -p ls
if %ERRORLEVEL% neq 0 exit /b %ERRORLEVEL%
cargo build -r --target wasm32-wasip1 -p args
if %ERRORLEVEL% neq 0 exit /b %ERRORLEVEL%

copy /y target\wasm32-wasip1\release\ls.wasm target\wasm32-wasip1\release\ls2.wasm

echo Combining with wasi_virt_layer-cli...
rem Use package/artifact names so the CLI can apply target-specific ABI handling.
rem Threaded VFS/reactor builds need Rust 1.100+; use the first supported nightly as a fallback.
set "RUSTUP_TOOLCHAIN=nightly-2026-08-27"
cargo run -p wasi_virt_layer-cli -- build ^
    -p repro_multi_target_table_bug ^
    test_threads ls args ls2 ^
    -t single ^
    --threads true ^
    --keep-build-artifacts ^
    --out-dir dist
if %ERRORLEVEL% neq 0 exit /b %ERRORLEVEL%
set "RUSTUP_TOOLCHAIN="

echo Running with Deno...
deno run -A dist/test_run.ts
if %ERRORLEVEL% neq 0 exit /b %ERRORLEVEL%
