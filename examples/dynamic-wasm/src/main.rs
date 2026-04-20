use wasi_virt_layer::file::*;
use wasi_virt_layer::*;

static WASM_HOLDER: StandardPseudoWasmHolder = StandardPseudoWasmHolder::new_const();

export_pseudo_wasm!(wasm_holder);
import_pseudo_wasm!(wasm_holder);

fn main() {
    let mut vfs = Wasip1MultipleVFS::<BoxedInodeNormal>::new();

    vfs.add_wasm::<wasm_holder>();

    let lfs1 = ChangeableLFS::<DefaultStdIO>::new();
    let root_inode1 = lfs1.add_preopen(".");
    lfs1.add_file(root_inode1, "hello.txt", b"Hello, Dynamic WASM!".to_vec())
        .unwrap();

    vfs.add_lfs(Box::new(lfs1));
    vfs.add_preopen_fd(0, BoxedInodeNormal::from_inode(root_inode1));

    println!("Successfully initialized Wasip1MultipleVFS with export_pseudo_wasm macro!");
}
