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

static LOCK: std::sync::RwLock<()> = std::sync::RwLock::new(());

static mut ALT_GLOBAL_VAR: i32 = 0;

// It should already be in the write lock.
#[unsafe(no_mangle)]
pub extern "C" fn __wasip1_vfs_memory_grow_global_alt_set(v: i32) {
    // use crate::debug::*;

    // if !is_pre_init() {
    //     out(b"set global: ");
    //     num_to_str(v, out);
    //     out(b"\n");
    // }
    unsafe { ALT_GLOBAL_VAR = v };
}

#[unsafe(no_mangle)]
pub extern "C" fn __wasip1_vfs_memory_grow_global_alt_init_once(v: i32) {
    static INIT: std::sync::Once = std::sync::Once::new();

    INIT.call_once(|| {
        // use crate::debug::*;
        // out(b"init once global: ");
        // num_to_str(v, out);
        // out(b"\n");
        unsafe { ALT_GLOBAL_VAR = v };
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn __wasip1_vfs_memory_grow_global_alt_pos() -> i32 {
    &raw const ALT_GLOBAL_VAR as *const i32 as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn __wasip1_vfs_memory_grow_global_alt_get_no_wait() -> i32 {
    // use crate::debug::*;

    let i = unsafe { ALT_GLOBAL_VAR };
    // if !is_pre_init() {
    //     out(b"get global no wait: ");
    //     num_to_str(i, out);
    //     out(b"\n");
    // }
    i
}

#[unsafe(no_mangle)]
pub extern "C" fn __wasip1_vfs_memory_grow_global_alt_get() -> i32 {
    // use crate::debug::*;

    // out(b"waiting for read lock...\n");
    let _guard = LOCK.read().unwrap();
    let i = unsafe { ALT_GLOBAL_VAR };
    // if !is_pre_init() {
    //     out(b"get global: ");
    //     num_to_str(i, out);
    //     out(b"\n");
    // }
    core::mem::drop(_guard);
    // out(b"unlocked global read\n");
    i
}

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
