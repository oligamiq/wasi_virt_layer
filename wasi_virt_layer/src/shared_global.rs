// Multi-memory lowering offset global protection.
//
// When multiple memories are condensed into a single combined memory,
// offset globals track where each original memory begins. These offsets
// are updated during `memory.grow` (data shift). In a multi-threaded
// environment, concurrent reads of offset globals during a grow would
// produce inconsistent addresses. This module provides an atomic
// counter-based RwLock so that:
//
//   - Target memory accesses hold a **read lock** while computing
//     the effective address from the offset global and performing the
//     memory instruction.
//   - `memory.grow` helpers hold a **write lock** while growing the
//     combined memory, shifting data, and updating offset globals.
//
// The lock state lives in a single `AtomicI32`:
//   0    : free
//   N > 0: N active readers
//   -1   : writer active

use core::sync::atomic::{AtomicI32, Ordering};

/// Atomic RwLock state variable.
///
/// Layout:
/// -  `0`   → unlocked
/// -  `N>0` → N readers active
/// -  `-1`  → writer active
static RWLOCK_STATE: AtomicI32 = AtomicI32::new(0);

// ---------------------------------------------------------------------------
// wvl_atomic primitives (resolved by CLI generator to VFS memory instructions)
// ---------------------------------------------------------------------------

#[link(wasm_import_module = "wvl_atomic")]
unsafe extern "C" {
    fn __wvl_atomic_wait32_vfs(addr: *const u32, expected: u32, timeout: i64) -> i32;
    fn __wvl_atomic_notify_vfs(addr: *const u32, count: u32) -> i32;
}

// ---------------------------------------------------------------------------
// Exported lock functions — called from generated code
// ---------------------------------------------------------------------------

/// Acquire a read lock (blocks if a writer is active).
///
/// Multiple readers can hold the lock concurrently.
/// # Safety
/// Called from generated wasm code via C-ABI.
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn __wasip1_vfs_memory_lock_read_acquire() {
    loop {
        let old = RWLOCK_STATE.load(Ordering::Relaxed);
        if old >= 0 {
            // Attempt to increment reader count
            if RWLOCK_STATE
                .compare_exchange_weak(old, old + 1, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                return;
            }
            // CAS failed — retry immediately (no wait needed, state just changed)
            continue;
        }
        // Writer is active (state < 0) — wait until state changes
        unsafe {
            __wvl_atomic_wait32_vfs(
                RWLOCK_STATE.as_ptr() as *const u32,
                old as u32, // wait while state == old (i.e. writer still active)
                -1,         // no timeout
            );
        }
    }
}

/// Release a read lock.
///
/// If this was the last reader, wakes one waiting writer.
/// # Safety
/// Called from generated wasm code via C-ABI.
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn __wasip1_vfs_memory_lock_read_release() {
    let old = RWLOCK_STATE.fetch_sub(1, Ordering::Release);
    if old == 1 {
        // Last reader released — wake a waiting writer
        unsafe {
            __wvl_atomic_notify_vfs(RWLOCK_STATE.as_ptr() as *const u32, 1);
        }
    }
}

/// Acquire a write lock (blocks until all readers and other writers release).
///
/// Only one writer can hold the lock at a time. No readers can be active.
/// # Safety
/// Called from generated wasm code via C-ABI.
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn __wasip1_vfs_memory_lock_write_acquire() {
    loop {
        match RWLOCK_STATE.compare_exchange_weak(0, -1, Ordering::Acquire, Ordering::Relaxed) {
            Ok(_) => return, // Successfully acquired write lock
            Err(current) => {
                // Someone else holds the lock — wait until state changes
                unsafe {
                    __wvl_atomic_wait32_vfs(
                        RWLOCK_STATE.as_ptr() as *const u32,
                        current as u32,
                        -1,
                    );
                }
            }
        }
    }
}

/// Release the write lock and wake all waiting threads.
/// # Safety
/// Called from generated wasm code via C-ABI.
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn __wasip1_vfs_memory_lock_write_release() {
    RWLOCK_STATE.store(0, Ordering::Release);
    unsafe {
        __wvl_atomic_notify_vfs(RWLOCK_STATE.as_ptr() as *const u32, u32::MAX);
    }
}

// ---------------------------------------------------------------------------
// Legacy API — used by gen_alt_global! macro (kept for backward compatibility)
// ---------------------------------------------------------------------------

/// Acquire a read lock and return a guard-like token.
///
/// The caller must call [`lock_read_release`] when done.
/// This is used by the `gen_alt_global!` macro's `_global_alt_get` functions.
pub fn lock_read() -> ReadGuard {
    __wasip1_vfs_memory_lock_read_acquire();
    ReadGuard(())
}

/// Acquire a write lock and return a guard-like token.
///
/// The caller must call [`lock_write_release`] when done.
/// This is used by the `gen_alt_global!` macro's `_global_alt_set_with_lock` functions.
pub fn lock_write() -> WriteGuard {
    __wasip1_vfs_memory_lock_write_acquire();
    WriteGuard(())
}

/// RAII guard for read lock. Releases the lock on drop.
pub struct ReadGuard(());

impl Drop for ReadGuard {
    fn drop(&mut self) {
        __wasip1_vfs_memory_lock_read_release();
    }
}

/// RAII guard for write lock. Releases the lock on drop.
pub struct WriteGuard(());

impl Drop for WriteGuard {
    fn drop(&mut self) {
        __wasip1_vfs_memory_lock_write_release();
    }
}

// ---------------------------------------------------------------------------
// VFS external memory manager offset global proxy
// ---------------------------------------------------------------------------

crate::gen_alt_global!(vfs_external_memory_manager);

// ---------------------------------------------------------------------------
// Memory grow locker (legacy — wraps memory.grow with write lock)
// ---------------------------------------------------------------------------

#[link(wasm_import_module = "wasip1-vfs_single_memory")]
unsafe extern "C" {
    fn __wasip1_vfs_memory_grow_alt(_: i32) -> i32;
}

/// Wraps `memory.grow` with a write lock to prevent concurrent access
/// to the combined memory while it is being resized and data is shifted.
#[unsafe(no_mangle)]
extern "C" fn __wasip1_vfs_memory_grow_locker(page_size: i32) -> i32 {
    let _guard = lock_write();
    unsafe { __wasip1_vfs_memory_grow_alt(page_size) }
}
