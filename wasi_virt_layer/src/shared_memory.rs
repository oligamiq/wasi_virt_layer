//! Shared memory management for zero-copy memory sharing between VFS and target Wasm modules.
//!
//! This module provides ABI functions for registering target Wasm modules,
//! managing shared memory, and handling memory growth across multiple targets.

use alloc::vec::Vec;

/// Metadata for a single target Wasm module.
///
/// Each target holds only a pointer to its `TargetMetadata` element in the VFS-managed Vec.
/// This 16-byte structure is stored in VFS memory, not in target memory.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TargetMetadata {
    /// Start address of this target's memory region
    pub base_ptr: u32,
    
    /// Current upper limit of the allocated memory region
    pub limit_ptr: u32,
    
    /// Current number of pages allocated
    pub current_pages: u32,
    
    /// Maximum pages allowed (0 = unlimited)
    pub max_pages: u32,
}

/// VFS-side shared memory manager.
///
/// This struct holds all target metadata and the shared linear memory.
/// Protected by `parking_lot::RwLock` for thread-safe access.
pub struct SharedMemoryManager {
    /// Metadata for all registered targets
    pub targets: Vec<TargetMetadata>,
    
    /// Shared linear memory (all targets share this)
    /// Option allows lazy initialization
    pub memory: Option<alloc::boxed::Box<[u8]>>,
}

impl SharedMemoryManager {
    /// Creates a new shared memory manager with initial capacity.
    pub fn new() -> Self {
        Self {
            targets: Vec::new(),
            memory: Some(alloc::vec![0u8; 65536].into_boxed_slice()), // Start with 1 page
        }
    }

    /// Const constructor (stub for zero-sized memory)
    const fn new_const() -> Self {
        Self {
            targets: Vec::new(),
            memory: None,
        }
    }
}

impl Default for SharedMemoryManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "threads")]
/// Static shared memory manager protected by RwLock.
/// Manages all registered target metadata and the shared linear memory.
pub static SHARED_MEMORY: parking_lot::RwLock<SharedMemoryManager> =
    parking_lot::const_rwlock(SharedMemoryManager::new_const());

/// Registers a new target Wasm module for shared memory access.
///
/// # Arguments
/// - `base_ptr`: Starting address of this target's memory region
/// - `current_pages`: Initial number of pages allocated
/// - `max_pages`: Maximum pages allowed (0 = unlimited)
///
/// # Returns
/// Pointer to the registered `TargetMetadata` entry (0 = failure)
#[unsafe(export_name = "wasip1_vfs_register_shared_memory_target")]
#[cfg(feature = "threads")]
extern "C" fn register_shared_memory_target(
    base_ptr: u32,
    current_pages: u32,
    max_pages: u32,
) -> u32 {
    let mut mgr = SHARED_MEMORY.write();
    
    // Create new target metadata
    let metadata = TargetMetadata {
        base_ptr,
        limit_ptr: base_ptr + (current_pages * 65536),
        current_pages,
        max_pages,
    };
    
    // Add to targets Vec
    mgr.targets.push(metadata);
    
    // Return pointer to the newly added metadata element
    let ptr = &mgr.targets[mgr.targets.len() - 1] as *const TargetMetadata;
    ptr as u32
}

/// Returns the pointer to the lock manager (parking_lot::RwLock).
///
/// # Arguments
/// - `metadata_ptr`: Pointer to this target's TargetMetadata (unused, for validation)
///
/// # Returns
/// Pointer to the shared lock manager for direct lock operations (0 = failure)
#[unsafe(export_name = "wasip1_vfs_shared_memory_get_lock_ptr")]
#[cfg(feature = "threads")]
extern "C" fn get_lock_ptr(_metadata_ptr: u32) -> u32 {
    // Return the pointer to the RwLock itself
    // This allows targets to perform direct memory operations for lock management
    let lock_ptr = &SHARED_MEMORY as *const parking_lot::RwLock<SharedMemoryManager>;
    lock_ptr as u32
}

/// Grows the shared memory when a target needs more space.
///
/// # Arguments
/// - `metadata_ptr`: Pointer to this target's TargetMetadata
/// - `required_pages`: Number of additional pages needed
///
/// # Returns
/// 0 = success, -1 = failure
#[unsafe(export_name = "wasip1_vfs_shared_memory_grow")]
#[cfg(feature = "threads")]
extern "C" fn grow_shared_memory(
    metadata_ptr: u32,
    required_pages: u32,
) -> i32 {
    let mut mgr = SHARED_MEMORY.write();
    
    // Get the target metadata
    let target_ref = unsafe {
        &mut *(metadata_ptr as *mut TargetMetadata)
    };
    
    // Initialize memory if needed
    if mgr.memory.is_none() {
        mgr.memory = Some(alloc::vec![0u8; 65536].into_boxed_slice());
    }
    
    let memory = match &mut mgr.memory {
        Some(mem) => mem,
        None => return -1,
    };
    
    // Calculate new memory size needed
    let current_pages = memory.len() / 65536;
    let new_total_pages = current_pages + required_pages as usize;
    let new_size = new_total_pages * 65536;
    
    // Check max_pages limit for this target
    if target_ref.max_pages > 0 && target_ref.current_pages + required_pages > target_ref.max_pages {
        return -1; // Exceeded max pages
    }
    
    // Expand memory if needed
    if memory.len() < new_size {
        let mut new_memory = alloc::vec![0u8; new_size];
        new_memory[..memory.len()].copy_from_slice(memory);
        *memory = new_memory.into_boxed_slice();
    }
    
    // Update this target's metadata
    target_ref.limit_ptr = target_ref.base_ptr + (new_total_pages as u32 * 65536);
    target_ref.current_pages += required_pages;
    
    0 // Success
}

#[cfg(not(feature = "threads"))]
mod no_threads {
    /// Stub: Not available without threads feature
    #[unsafe(export_name = "wasip1_vfs_register_shared_memory_target")]
    extern "C" fn register_shared_memory_target(_: u32, _: u32, _: u32) -> u32 {
        0 // Failure
    }

    /// Stub: Not available without threads feature
    #[unsafe(export_name = "wasip1_vfs_shared_memory_get_lock_ptr")]
    extern "C" fn get_lock_ptr(_: u32) -> u32 {
        0 // Failure
    }

    /// Stub: Not available without threads feature
    #[unsafe(export_name = "wasip1_vfs_shared_memory_grow")]
    extern "C" fn grow_shared_memory(_: u32, _: u32) -> i32 {
        -1 // Failure
    }
}

