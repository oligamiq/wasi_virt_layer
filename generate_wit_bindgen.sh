#!/usr/bin/env bash
set -euo pipefail

# Generate wit-bindgen Rust bindings from WIT definitions.
# Requires: wit-bindgen CLI (cargo install wit-bindgen-cli@0.41.0)

if ! command -v wit-bindgen >/dev/null 2>&1; then
  echo "wit-bindgen not found. Install with: cargo install wit-bindgen-cli" >&2
  exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
WIT_DIR="${SCRIPT_DIR}/wasi_virt_layer/wit"
OUT_DIR="${SCRIPT_DIR}/wasi_virt_layer/src/wit"

echo "Generating virtual-file-system bindings..."
wit-bindgen rust "${WIT_DIR}" --out-dir "${OUT_DIR}" -w virtual-file-system

echo "Generating virtual-file-system-threads bindings..."
wit-bindgen rust "${WIT_DIR}" --out-dir "${OUT_DIR}" -w virtual-file-system-threads

echo "Formatting generated files..."
rustfmt "${OUT_DIR}/virtual_file_system.rs"
rustfmt "${OUT_DIR}/virtual_file_system_threads.rs"

echo "Done. Generated files in ${OUT_DIR}"
