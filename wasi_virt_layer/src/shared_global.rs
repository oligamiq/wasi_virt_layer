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
//   N > 0: N active readers (plus the optional sticky writer-waiting bit)
//   -1   : writer active
//
// `waiter-aware-memory-lock` reserves bit 30 of positive states for a waiting
// writer, so uncontended last-reader releases need not notify. The feature is
// off by default; both modes keep the same exports and relocation exclusion.

use core::sync::atomic::AtomicI32;

#[path = "memory_lock.rs"]
mod memory_lock;
use memory_lock::{MemoryAccessLock, WaitNotify};

/// Shared relocation lock; all workers use the same storage.
static MEMORY_LOCK: MemoryAccessLock = MemoryAccessLock::new();

struct WasmWaitNotify;

impl WaitNotify for WasmWaitNotify {
    #[inline(always)]
    fn wait(&self, state: &AtomicI32, expected: i32) {
        unsafe {
            __wvl_atomic_wait32_vfs(state.as_ptr().cast(), expected as u32, -1);
        }
    }

    #[inline(always)]
    fn notify(&self, state: &AtomicI32, count: u32) {
        unsafe {
            __wvl_atomic_notify_vfs(state.as_ptr().cast(), count);
        }
    }
}

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
    MEMORY_LOCK.read_acquire(&WasmWaitNotify);
}

/// Release a read lock.
///
/// If this was the last reader, wakes one waiting writer. With the experimental
/// waiter-aware feature, notification requires a registered writer.
/// # Safety
/// Called from generated wasm code via C-ABI.
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn __wasip1_vfs_memory_lock_read_release() {
    MEMORY_LOCK.read_release(&WasmWaitNotify);
}

/// Acquire a write lock (blocks until all readers and other writers release).
///
/// Only one writer can hold the lock at a time. No readers can be active.
/// # Safety
/// Called from generated wasm code via C-ABI.
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn __wasip1_vfs_memory_lock_write_acquire() {
    MEMORY_LOCK.write_acquire(&WasmWaitNotify);
}

/// Release the write lock and wake all waiting threads.
/// # Safety
/// Called from generated wasm code via C-ABI.
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn __wasip1_vfs_memory_lock_write_release() {
    MEMORY_LOCK.write_release(&WasmWaitNotify);
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
