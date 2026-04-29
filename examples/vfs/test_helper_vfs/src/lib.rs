//! Test VFS with TypeScript helper generation
//! Minimal VFS to test export_pseudo_wasm! macro

use wasi_virt_layer::file::{
    PseudoWasmTrait,
    multiple::dynamic_wasm::{PseudoWasmSimpleBuilder, StandardPseudoWasmHolder},
};

wit_bindgen::generate!({
    world: "test-helper",
});

struct TestHelperVfs;

impl Guest for TestHelperVfs {
    fn hello() {
        println!("Hello from test_helper_vfs!");
    }
}

#[cfg(not(test))]
export!(TestHelperVfs);

/// VFS holder exported for registration
pub static MY_VFS: StandardPseudoWasmHolder = StandardPseudoWasmHolder::new_const();

/// Export the VFS interface for TypeScript helper generation
#[unsafe(no_mangle)]
pub extern "C" fn __wasi_export_pseudo_wasm_MY_VFS(ptrs: PseudoWasmSimpleBuilder) {
    MY_VFS.receive_pseudo_wasm(ptrs);
}
