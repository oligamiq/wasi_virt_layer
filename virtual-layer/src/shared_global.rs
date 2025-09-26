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

#[cfg(all(feature = "threads", not(feature = "multi_memory")))]
pub mod single_memory {
    pub static __MEMORY_GROW_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    pub static mut ALT_GLOBAL_VAR: i32 = 0;

    #[unsafe(no_mangle)]
    pub extern "C" fn __wasip1_vfs_memory_grow_global_alt_set(v: i32) {
        unsafe { ALT_GLOBAL_VAR = v };
    }
}

#[cfg(all(feature = "threads", not(feature = "multi_memory")))]
#[macro_export]
macro_rules! __memory_grow_locker {
    ($wasm:tt) => {
        use crate::__private::inner::__MEMORY_GROW_LOCK;

        $crate::__private::paste::paste! {
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "wasip1-vfs_single_memory")]
            unsafe extern "C" {
                fn [<__wasip1_vfs_memory_grow_ $wasm _alt>](_: i32) -> i32;
            }

            #[unsafe(no_mangle)]
            extern "C" fn [<__wasip1_vfs_debug_memory_grow_ $wasm _locker>](page_size: i32) -> i32 {
                let _guard = __MEMORY_GROW_LOCK.lock().unwrap();

                let pre_page_size = unsafe { [<__wasip1_vfs_memory_grow_ $wasm _alt>](page_size) };

                core::mem::drop(_guard);

                pre_page_size
            }
        }
    };
}

#[cfg(not(all(feature = "threads", not(feature = "multi_memory"))))]
#[macro_export]
macro_rules! __memory_grow_locker {
    ($wasm:tt) => {};
}
