sed -i 's/let err_wasm_names = import/dbg!(\&import); let err_wasm_names = import/' wasi_virt_layer-cli/src/abi.rs
cargo run -p wasi_virt_layer-cli -- build -p ls-vfs c_target.wasm
git restore wasi_virt_layer-cli/src/abi.rs
