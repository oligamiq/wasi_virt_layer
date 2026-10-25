//! Standalone native regression harness; no WASI or Cargo build required.
#![allow(dead_code)]

use std::sync::atomic::{AtomicUsize, Ordering};

#[macro_export]
macro_rules! gen_alt_global {
    ($($tokens:tt)*) => {};
}

#[path = "../src/shared_global.rs"]
mod original_lock;

static NOTIFICATIONS: AtomicUsize = AtomicUsize::new(0);

#[unsafe(no_mangle)]
extern "C" fn __wvl_atomic_wait32_vfs(_: *const u32, _: u32, _: i64) -> i32 {
    panic!("uncontended reader must not wait");
}

#[unsafe(no_mangle)]
extern "C" fn __wvl_atomic_notify_vfs(_: *const u32, _: u32) -> i32 {
    NOTIFICATIONS.fetch_add(1, Ordering::Relaxed);
    0
}

#[unsafe(no_mangle)]
extern "C" fn __wasip1_vfs_memory_grow_alt(_: i32) -> i32 {
    panic!("native protocol test does not grow memory");
}

#[test]
fn uncontended_readers_notify_only_in_baseline() {
    for _ in 0..3 {
        original_lock::__wasip1_vfs_memory_lock_read_acquire();
        original_lock::__wasip1_vfs_memory_lock_read_release();
    }
    let expected = if cfg!(feature = "waiter-aware-memory-lock") {
        0
    } else {
        3
    };
    assert_eq!(NOTIFICATIONS.load(Ordering::Relaxed), expected);
}
