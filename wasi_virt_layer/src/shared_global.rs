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
// The initial values should be loaded into memory the instant
// the vfs start section, which was processed earlier, concludes.
// Ordinary global variables are sometimes used as stacks,
// so they must not be shared between threads.

use crate::utils::InitOnce;

static LOCK: std::sync::RwLock<()> = std::sync::RwLock::new(());

pub fn lock_read() -> std::sync::RwLockReadGuard<'static, ()> {
    LOCK.read().unwrap()
}

crate::gen_alt_global!(vfs_external_memory_manager);

#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "wasip1-vfs_single_memory")]
unsafe extern "C" {
    fn __wasip1_vfs_memory_grow_alt(_: i32) -> i32;
}

#[unsafe(no_mangle)]
extern "C" fn __wasip1_vfs_memory_grow_locker(page_size: i32) -> i32 {
    // use crate::debug::*;

    // out(b"locking memory.grow...\n");
    let _guard = LOCK.write().unwrap();

    // out(b"memory.grow requested: ");
    // crate::debug::num_to_str(page_size, crate::debug::out);
    // out(b"\n");

    let pre_page_size = unsafe { __wasip1_vfs_memory_grow_alt(page_size) };

    // out(b"memory.grow pre: ");
    // crate::debug::num_to_str(pre_page_size, crate::debug::out);
    // out(b"\n");

    core::mem::drop(_guard);

    // out(b"unlocked memory.grow\n");

    pre_page_size
}
