// The Globals causing errors during memory expansion are those generated
// by wasm-opt --multi-memory-lowering,
// so for now we will only address these.
// When a newly created thread is executed,
// it will use the always-executable VFS code and memory,
// which are based on an address that never changes,
// and perform operations on them atomically.
// Operations on Global variables are replaced,
// and before memory unification,
// memory.grow is modified to be an atomic operation.
// Since this Global variable should only be modified internally,
// this approach should be sufficient.

static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

static mut ALT_GLOBAL_VAR: i32 = 0;

#[unsafe(no_mangle)]
pub extern "C" fn __wasip1_vfs_memory_grow_global_alt_set(v: i32) {
    unsafe { ALT_GLOBAL_VAR = v };
}

#[unsafe(no_mangle)]
pub extern "C" fn __wasip1_vfs_memory_grow_global_alt_get() -> i32 {
    unsafe { ALT_GLOBAL_VAR }
}

#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "wasip1-vfs_single_memory")]
unsafe extern "C" {
    fn __wasip1_vfs_memory_grow_alt(_: i32) -> i32;
}

#[unsafe(no_mangle)]
extern "C" fn __wasip1_vfs_memory_grow_locker(page_size: i32) -> i32 {
    let _guard = LOCK.lock().unwrap();

    let pre_page_size = unsafe { __wasip1_vfs_memory_grow_alt(page_size) };

    core::mem::drop(_guard);

    pre_page_size
}
