@echo off
setlocal

echo Building test_threads...
cargo +nightly build -r --target wasm32-wasip1-threads -p test_threads
if %ERRORLEVEL% neq 0 exit /b %ERRORLEVEL%

echo Building ls, args...
cargo build -r --target wasm32-wasip1 -p ls
if %ERRORLEVEL% neq 0 exit /b %ERRORLEVEL%
cargo build -r --target wasm32-wasip1 -p args
if %ERRORLEVEL% neq 0 exit /b %ERRORLEVEL%

copy /y target\wasm32-wasip1\release\ls.wasm target\wasm32-wasip1\release\ls2.wasm
copy /y target\wasm32-wasip1\release\args.wasm target\wasm32-wasip1\release\args_target.wasm

echo Combining with wasi_virt_layer-cli...
cargo run -p wasi_virt_layer-cli -- build ^
    --manifest-path examples/vfs/repro_multi_target_table_bug/Cargo.toml ^
    target/wasm32-wasip1-threads/release/test_threads.wasm ^
    target/wasm32-wasip1/release/ls.wasm ^
    target/wasm32-wasip1/release/args_target.wasm ^
    target/wasm32-wasip1/release/ls2.wasm ^
    -t single ^
    --threads true ^
    --keep-build-artifacts ^
    --out-dir dist
if %ERRORLEVEL% neq 0 exit /b %ERRORLEVEL%

echo Running with Deno...
deno run -A dist/test_run.ts
if %ERRORLEVEL% neq 0 exit /b %ERRORLEVEL%
