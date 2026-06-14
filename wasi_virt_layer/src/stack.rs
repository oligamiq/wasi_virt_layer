//! Shared ABI state used by generated export-stack isolation wrappers.
//!
//! Stack switching itself is emitted directly as WebAssembly instructions by
//! `wasi_virt_layer-cli`. Ordinary Rust code must not run before the generated
//! wrapper has installed a private stack.

use core::sync::atomic::AtomicU32;

/// Shared dynamic-stack handoff record.
///
/// The generated wrapper serializes access through `bootstrap_lock`, consumes
/// `next_stack_*`, switches its instance-local stack pointer, then allocates
/// and publishes the replacement standby stack.
#[repr(C, align(16))]
pub struct StackHandoffRecord {
    /// Zero while unlocked and one while a wrapper owns the bootstrap path.
    pub bootstrap_lock: AtomicU32,
    /// Physical base address of the prepared standby stack.
    pub next_stack_base: AtomicU32,
    /// Physical end address of the prepared standby stack.
    pub next_stack_end: AtomicU32,
    /// Generation incremented whenever a standby stack is published.
    pub generation: AtomicU32,
}

impl StackHandoffRecord {
    /// Creates an empty handoff record.
    pub const fn new() -> Self {
        Self {
            bootstrap_lock: AtomicU32::new(0),
            next_stack_base: AtomicU32::new(0),
            next_stack_end: AtomicU32::new(0),
            generation: AtomicU32::new(0),
        }
    }
}

impl Default for StackHandoffRecord {
    fn default() -> Self {
        Self::new()
    }
}

/// Shared VFS stack handoff state.
///
/// On wasm32 this data symbol is exported as an immutable global containing
/// the record's linear-memory address. Generated code accesses the record
/// directly and does not call Rust before switching stacks.
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub static __wasip1_vfs_stack_handoff_vfs: StackHandoffRecord = StackHandoffRecord::new();

/// Shared dynamic-stack handoff records for single-memory target modules.
///
/// The CLI assigns each target a stable build-order index and addresses its
/// record as `base + index * size_of::<StackHandoffRecord>()`.
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub static __wasip1_vfs_stack_handoff_targets: [StackHandoffRecord; 64] =
    [const { StackHandoffRecord::new() }; 64];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handoff_record_layout_is_stable() {
        assert_eq!(core::mem::size_of::<StackHandoffRecord>(), 16);
        assert_eq!(core::mem::align_of::<StackHandoffRecord>(), 16);
    }
}
