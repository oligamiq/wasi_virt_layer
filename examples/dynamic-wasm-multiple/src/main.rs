use wasi_virt_layer::file::*;
use wasi_virt_layer::*;
use wasi_virt_layer::file::multiple::dynamic_wasm::StandardPseudoWasmMultipleHolder;

static WASM_MULTIPLE_HOLDER: StandardPseudoWasmMultipleHolder = StandardPseudoWasmMultipleHolder::new();

export_pseudo_wasm!(wasm_multiple_holder; &WASM_MULTIPLE_HOLDER);
import_pseudo_wasm!(wasm_multiple_holder; &WASM_MULTIPLE_HOLDER);

fn main() {
    let mut vfs = Wasip1MultipleVFS::<BoxedInodeNormal>::new();

    // Use the generated struct to add the pseudo Wasm module
    vfs.add_wasm::<wasm_multiple_holder>();

    let lfs1 = ChangeableLFS::<DefaultStdIO>::new();
    let root_inode1 = lfs1.add_preopen(".");
    lfs1.add_file(root_inode1, "hello.txt", b"Hello, Multiple Dynamic WASM!".to_vec())
        .unwrap();

    vfs.add_lfs(Box::new(lfs1));
    vfs.add_preopen_fd(0, BoxedInodeNormal::from_inode(root_inode1));

    println!("Successfully initialized Wasip1MultipleVFS with export_pseudo_wasm and StandardPseudoWasmMultipleHolder!");
}
