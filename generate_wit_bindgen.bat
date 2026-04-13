@echo off
REM Generate wit-bindgen Rust bindings from WIT definitions.
REM Requires: wit-bindgen CLI (cargo install wit-bindgen-cli)

where wit-bindgen >nul 2>&1
if errorlevel 1 (
  echo wit-bindgen not found. Install with: cargo install wit-bindgen-cli
  exit /b 1
)

set "SCRIPT_DIR=%~dp0"
set "WIT_DIR=%SCRIPT_DIR%wasi_virt_layer\wit"
set "OUT_DIR=%SCRIPT_DIR%wasi_virt_layer\src\wit"

echo Generating virtual-file-system bindings...
wit-bindgen rust "%WIT_DIR%" --out-dir "%OUT_DIR%" -w virtual-file-system
if errorlevel 1 exit /b 1

echo Generating virtual-file-system-threads bindings...
wit-bindgen rust "%WIT_DIR%" --out-dir "%OUT_DIR%" -w virtual-file-system-threads
if errorlevel 1 exit /b 1

echo Formatting generated files...
rustfmt "%OUT_DIR%\virtual_file_system.rs"
if errorlevel 1 exit /b 1
rustfmt "%OUT_DIR%\virtual_file_system_threads.rs"
if errorlevel 1 exit /b 1

echo Done. Generated files in %OUT_DIR%
