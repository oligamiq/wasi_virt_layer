use wasi_virt_layer::file::*;
use wasi_virt_layer::*;
use wasi_virt_layer::file::multiple::dynamic_wasm::StandardPseudoWasmMultipleHolder;

static WASM_MULTIPLE_HOLDER: StandardPseudoWasmMultipleHolder = StandardPseudoWasmMultipleHolder::new();

export_pseudo_wasm!(wasm_multiple_holder; &WASM_MULTIPLE_HOLDER);

// Note: import_pseudo_wasm! with a holder argument is currently a WIP in the library
// and produces a compile_error!. We leave it commented out for now.
// import_pseudo_wasm!(wasm_multiple_holder; &WASM_MULTIPLE_HOLDER);

fn main() {
    let mut vfs = Wasip1MultipleVFS::<BoxedInodeNormal>::new();

    // In a real scenario, you would compile this code into WASM, and the WASM
    // module would call `__wasi_export_pseudo_wasm_wasm_multiple_holder` during initialization.
    // The `receive_pseudo_wasm` method would then assign an ID and store the pointers.
    // 
    // Example of manually adding a WASM access if we had an ID (e.g., 0):
    // let pseudo_access = WASM_MULTIPLE_HOLDER.restore(0);
    // vfs.add_wasm_access(
    //     "wasm_multiple_holder".into(),
    //     WasmAccessDynCompatibleWrapper::new(pseudo_access)
    // );

    let lfs1 = ChangeableLFS::<DefaultStdIO>::new();
    let root_inode1 = lfs1.add_preopen(".");
    lfs1.add_file(root_inode1, "hello.txt", b"Hello, Multiple Dynamic WASM!".to_vec())
        .unwrap();

    vfs.add_lfs(Box::new(lfs1));
    vfs.add_preopen_fd(0, BoxedInodeNormal::from_inode(root_inode1));

    println!("Successfully initialized Wasip1MultipleVFS with export_pseudo_wasm and StandardPseudoWasmMultipleHolder!");
}
