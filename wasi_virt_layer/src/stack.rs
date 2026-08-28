//! Shared ABI state used by generated export-stack isolation wrappers.
//!
//! Stack switching itself is emitted directly as WebAssembly instructions by
//! `wasi_virt_layer-cli`. Ordinary Rust code must not run before the generated
//! wrapper has installed a private stack.

use core::sync::atomic::AtomicU32;

/// Represents the module for which stack operations are requested.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackModule<'a> {
    /// The VFS module.
    Vfs,
    /// A specific target module by name.
    Target(&'a str),
}

/// The current state of the stack for a given module instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum StackState {
    /// No stack has been acquired yet.
    Uninitialized = 0,
    /// Stack acquisition is in progress.
    Initializing = 1,
    /// A valid stack is currently installed.
    Ready = 2,
    /// The stack is managed by `wasi_thread_start` and should not be dynamically replaced.
    ThreadManaged = 3,
    /// The stack is in the process of being released.
    Releasing = 4,
}

impl From<u8> for StackState {
    fn from(val: u8) -> Self {
        match val {
            0 => StackState::Uninitialized,
            1 => StackState::Initializing,
            2 => StackState::Ready,
            3 => StackState::ThreadManaged,
            4 => StackState::Releasing,
            _ => StackState::Uninitialized,
        }
    }
}

/// Information about the current stack configuration and state.
///
/// Layout is ABI-stable (repr(C)) so generated Wasm code can write into it.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct StackInfo {
    /// The current state of the stack.
    pub state: StackState,
    /// The configured size of the stack in bytes.
    pub stack_size: u32,
    /// The number of active protected export calls on this stack.
    pub active_export_depth: u32,
    /// The current generation of the stack assignment.
    pub generation: u32,
    /// The base address of the currently installed stack.
    pub current_stack_base: u32,
    /// The end address of the currently installed stack.
    pub current_stack_end: u32,
    /// Whether a standby stack is currently available.
    pub next_stack_ready: u32,
    /// The slot index if a fixed arena slot is used (`u32::MAX` = none, multi-memory only).
    pub slot_index: u32,
}

/// Errors that can occur during stack management operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackError {
    /// The specified module is not known or not configured for stack management.
    UnknownModule,
    /// No `__stack_pointer` global was found in the module.
    StackPointerNotFound,
    /// Dynamic stack allocation failed.
    AllocationFailed,
    /// Standby stack allocation failed; the current instance remains valid.
    StandbyAllocationFailed,
    /// No free slot available in the fixed arena (multi-memory only).
    NoFreeSlot,
    /// The stack is currently in use and cannot be released.
    InUse,
    /// The stack is managed by `wasi_thread_start` and cannot be released.
    ThreadManaged,
    /// The generation number does not match the expected value.
    GenerationMismatch,
    /// A memory layout move was required during an active protected call.
    LayoutMoveRequired,
    /// The stack configuration is invalid (e.g., conflicting options).
    InvalidConfiguration,
}

impl core::fmt::Display for StackError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            StackError::UnknownModule => write!(f, "Unknown module"),
            StackError::StackPointerNotFound => write!(f, "Stack pointer not found"),
            StackError::AllocationFailed => write!(f, "Stack allocation failed"),
            StackError::StandbyAllocationFailed => write!(f, "Standby stack allocation failed"),
            StackError::NoFreeSlot => write!(f, "No free stack slot available"),
            StackError::InUse => write!(f, "Stack is currently in use"),
            StackError::ThreadManaged => write!(f, "Stack is thread-managed"),
            StackError::GenerationMismatch => write!(f, "Generation mismatch"),
            StackError::LayoutMoveRequired => write!(f, "Layout move required during active call"),
            StackError::InvalidConfiguration => write!(f, "Invalid stack configuration"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for StackError {}

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

// External generated functions for stack management.
#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn __wasip1_vfs_stack_ensure_vfs() -> i32;
    fn __wasip1_vfs_stack_ensure(module_id: u32) -> i32;
    fn __wasip1_vfs_stack_release_vfs(generation: u32) -> i32;
    fn __wasip1_vfs_stack_release(module_id: u32, generation: u32) -> i32;
    fn __wasip1_vfs_stack_info_vfs(result_ptr: *mut StackInfo) -> i32;
    fn __wasip1_vfs_stack_info(module_id: u32, result_ptr: *mut StackInfo) -> i32;
    fn __wasip1_vfs_stack_force_release_vfs() -> i32;
    fn __wasip1_vfs_stack_force_release(module_id: u32) -> i32;
}

/// Maps a generated errno to a `StackError`.
#[cfg(target_arch = "wasm32")]
fn map_errno(errno: i32) -> Result<(), StackError> {
    match errno {
        0 => Ok(()),
        1 => Err(StackError::AllocationFailed),
        2 => Err(StackError::StandbyAllocationFailed),
        3 => Err(StackError::LayoutMoveRequired),
        4 => Err(StackError::NoFreeSlot),
        5 => Err(StackError::InUse),
        6 => Err(StackError::ThreadManaged),
        7 => Err(StackError::GenerationMismatch),
        _ => Err(StackError::UnknownModule),
    }
}

/// Ensures a valid VFS stack is installed for the current instance.
#[cfg(target_arch = "wasm32")]
pub fn ensure_vfs_stack() -> Result<(), StackError> {
    unsafe { map_errno(__wasip1_vfs_stack_ensure_vfs()) }
}

/// Ensures a valid stack is installed for the specified module.
#[cfg(target_arch = "wasm32")]
pub fn ensure_wasm_stack(module: StackModule<'_>) -> Result<(), StackError> {
    let module_id = match module {
        StackModule::Vfs => 0,
        StackModule::Target(_) => {
            // In a full implementation, this would resolve the target name to an ID.
            // For now, we return an error as name resolution requires CLI-generated metadata.
            return Err(StackError::UnknownModule);
        }
    };
    unsafe { map_errno(__wasip1_vfs_stack_ensure(module_id)) }
}

/// Releases the VFS stack, performing a handoff to a new stack before freeing the old one.
#[cfg(target_arch = "wasm32")]
pub fn release_vfs_stack(generation: u32) -> Result<(), StackError> {
    unsafe { map_errno(__wasip1_vfs_stack_release_vfs(generation)) }
}

/// Releases the stack for the specified module.
#[cfg(target_arch = "wasm32")]
pub fn release_wasm_stack(module: StackModule<'_>, generation: u32) -> Result<(), StackError> {
    let module_id = match module {
        StackModule::Vfs => 0,
        StackModule::Target(_) => return Err(StackError::UnknownModule),
    };
    unsafe { map_errno(__wasip1_vfs_stack_release(module_id, generation)) }
}

/// Retrieves information about the current VFS stack.
#[cfg(target_arch = "wasm32")]
pub fn vfs_stack_info() -> Result<StackInfo, StackError> {
    let mut info = StackInfo {
        state: StackState::Uninitialized,
        stack_size: 0,
        active_export_depth: 0,
        generation: 0,
        current_stack_base: 0,
        current_stack_end: 0,
        next_stack_ready: 0,
        slot_index: u32::MAX,
    };
    let errno = unsafe { __wasip1_vfs_stack_info_vfs(&mut info) };
    map_errno(errno)?;
    Ok(info)
}

/// Retrieves information about the stack for the specified module.
#[cfg(target_arch = "wasm32")]
pub fn wasm_stack_info(module: StackModule<'_>) -> Result<StackInfo, StackError> {
    let module_id = match module {
        StackModule::Vfs => 0,
        StackModule::Target(_) => return Err(StackError::UnknownModule),
    };
    let mut info = StackInfo {
        state: StackState::Uninitialized,
        stack_size: 0,
        active_export_depth: 0,
        generation: 0,
        current_stack_base: 0,
        current_stack_end: 0,
        next_stack_ready: 0,
        slot_index: u32::MAX,
    };
    let errno = unsafe { __wasip1_vfs_stack_info(module_id, &mut info) };
    map_errno(errno)?;
    Ok(info)
}

/// Force releases the VFS stack without performing a handoff.
///
/// # Safety
/// The caller must guarantee that no execution can still access this stack.
#[cfg(target_arch = "wasm32")]
pub unsafe fn force_release_vfs_stack() -> Result<(), StackError> {
    map_errno(unsafe { __wasip1_vfs_stack_force_release_vfs() })
}

/// Force releases the stack for the specified module without performing a handoff.
///
/// # Safety
/// The caller must guarantee that no execution can still access this stack.
#[cfg(target_arch = "wasm32")]
pub unsafe fn force_release_wasm_stack(module: StackModule<'_>) -> Result<(), StackError> {
    let module_id = match module {
        StackModule::Vfs => 0,
        StackModule::Target(_) => return Err(StackError::UnknownModule),
    };
    map_errno(unsafe { __wasip1_vfs_stack_force_release(module_id) })
}

/// Configures export-stack isolation for the current module.
///
/// Emits well-known global exports that the `wasi_virt_layer-cli` reads during
/// build to configure stack size, slots, and release permissions.
///
/// # Example
///
/// ```ignore
/// use wasi_virt_layer::configure_wasm_stack;
///
/// configure_wasm_stack!(size: 1 * 1024 * 1024);
/// configure_wasm_stack!(size: 2 * 1024 * 1024, slots: 32, allow_release: true);
/// ```
#[macro_export]
macro_rules! configure_wasm_stack {
    (size: $size:expr $(, slots: $slots:expr )? $(, allow_release: $allow:expr )? $(,)?) => {
        #[cfg(target_arch = "wasm32")]
        mod __wasi_virt_layer_stack_config {
            #[unsafe(export_name = "__wasi_virt_layer_stack_cfg_size")]
            pub static __SIZE: u32 = $size;
            $(
                #[unsafe(export_name = "__wasi_virt_layer_stack_cfg_slots")]
                pub static __SLOTS: u32 = $slots;
            )?
            $(
                #[unsafe(export_name = "__wasi_virt_layer_stack_cfg_allow_release")]
                pub static __ALLOW_RELEASE: u32 = if $allow { 1 } else { 0 };
            )?
        }
    };
}

/// Marks specific exports for stack protection.
///
/// Emits well-known global exports that the `wasi_virt_layer-cli` reads to
/// determine which function exports should receive stack isolation wrappers.
///
/// # Example
///
/// ```ignore
/// use wasi_virt_layer::protect_wasm_exports;
///
/// protect_wasm_exports!(run, reset, _main);
/// ```
#[macro_export]
macro_rules! protect_wasm_exports {
    ( $( $export:ident ),* $(,)? ) => {
        #[cfg(target_arch = "wasm32")]
        mod __wasi_virt_layer_protected_exports {
            $(
                #[unsafe(
                    export_name = concat!(
                        "__wasi_virt_layer_protect_",
                        stringify!($export)
                    )
                )]
                pub static __PROTECT__ $export : u32 = 0;
            )*
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handoff_record_layout_is_stable() {
        assert_eq!(core::mem::size_of::<StackHandoffRecord>(), 16);
        assert_eq!(core::mem::align_of::<StackHandoffRecord>(), 16);
    }

    #[test]
    fn stack_state_conversion() {
        assert_eq!(StackState::from(0), StackState::Uninitialized);
        assert_eq!(StackState::from(1), StackState::Initializing);
        assert_eq!(StackState::from(2), StackState::Ready);
        assert_eq!(StackState::from(3), StackState::ThreadManaged);
        assert_eq!(StackState::from(4), StackState::Releasing);
        assert_eq!(StackState::from(255), StackState::Uninitialized);
    }
}
