//! Shared memory ABI for zero-copy inter-module memory access.
//!
//! This module implements a trait-based system for managing shared memory regions
//! between multiple WASM modules, following the `export_pseudo_wasm` pattern.
//!
//! The key pattern here is:
//! 1. Define a builder struct for function pointers
//! 2. Define a concrete storage struct that holds the built data
//! 3. Create a trait that abstracts the behavior
//! 4. Provide a macro for easy export

#[cfg(not(target_os = "wasi"))]
use crate::wasip1;
#[cfg(target_os = "wasi")]
use wasip1;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};
use parking_lot::RwLock;
use smallvec::SmallVec;

/// Metadata about a single target module's memory region.
/// Stored in the shared VFS memory, passed between modules via pointers.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct TargetMemoryMetadata {
    /// Base pointer of this target's memory region in the shared linear memory
    pub base_ptr: i32,
    /// Limit pointer (end) of this target's memory region
    pub limit_ptr: i32,
    /// Current number of pages allocated
    pub current_pages: u32,
    /// Maximum pages allowed for this target
    pub max_pages: u32,
}

/// Builder for the shared memory manager functions.
/// This struct holds function pointers and metadata that will be transferred
/// from the VFS to target modules via WASM calls.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SharedMemoryManagerBuilder {
    /// Pointer to lock management structure (parking_lot::RwLock)
    pub lock_ptr: *const u8,
    /// Pointer to metadata array in shared memory
    pub metadata_ptr: *const TargetMemoryMetadata,
    /// Number of registered targets
    pub target_count: usize,
    /// Function to register a new target
    pub register_target_ptr: fn(*const TargetMemoryMetadata, i32, i32, u32) -> i32,
    /// Function to grow memory for a target
    pub grow_memory_ptr: fn(i32, u32) -> i32,
    /// Function to get lock pointer for a target
    pub get_lock_ptr: fn(i32) -> *const u8,
}

impl SharedMemoryManagerBuilder {
    pub fn build(self) -> SharedMemoryManager {
        SharedMemoryManager {
            lock_ptr: self.lock_ptr,
            metadata_ptr: self.metadata_ptr,
            target_count: self.target_count,
            register_target_ptr: self.register_target_ptr,
            grow_memory_ptr: self.grow_memory_ptr,
            get_lock_ptr: self.get_lock_ptr,
        }
    }
}

/// Built shared memory manager instance.
/// This holds all the function pointers and data needed for a target module
/// to access shared memory regions managed by the VFS.
#[derive(Clone, Debug)]
pub struct SharedMemoryManager {
    pub lock_ptr: *const u8,
    pub metadata_ptr: *const TargetMemoryMetadata,
    pub target_count: usize,
    pub register_target_ptr: fn(*const TargetMemoryMetadata, i32, i32, u32) -> i32,
    pub grow_memory_ptr: fn(i32, u32) -> i32,
    pub get_lock_ptr: fn(i32) -> *const u8,
}

impl SharedMemoryManager {
    /// Register a new target module's memory region
    pub fn register_target(&self, metadata: *const TargetMemoryMetadata, base: i32, limit: i32, pages: u32) -> i32 {
        (self.register_target_ptr)(metadata, base, limit, pages)
    }

    /// Grow memory for a target module
    pub fn grow_memory(&self, metadata_ptr: i32, pages: u32) -> i32 {
        (self.grow_memory_ptr)(metadata_ptr, pages)
    }

    /// Get lock pointer for thread-safe operations
    pub fn get_lock_ptr(&self, metadata_ptr: i32) -> *const u8 {
        (self.get_lock_ptr)(metadata_ptr)
    }
}

/// Trait for shared memory manager implementations.
/// Defines the interface for receiving and managing shared memory configurations.
pub trait SharedMemoryManagerTrait {
    type Generated;

    /// Restore the manager from generated data
    fn restore(&self, generated: Self::Generated) -> SharedMemoryManager;

    /// Receive the builder and initialize the manager
    fn receive_manager(&self, builder: SharedMemoryManagerBuilder) -> Self::Generated;
}

/// Standard single target memory manager holder.
/// Stores a single shared memory manager that can be initialized once.
#[derive(Debug)]
pub struct StandardSharedMemoryHolder {
    #[cfg(feature = "threads")]
    pub once: AtomicBool,
    pub manager: core::cell::UnsafeCell<Option<SharedMemoryManager>>,
}

unsafe impl Send for StandardSharedMemoryHolder {}
unsafe impl Sync for StandardSharedMemoryHolder {}

impl StandardSharedMemoryHolder {
    pub const fn new() -> Self {
        Self {
            #[cfg(feature = "threads")]
            once: AtomicBool::new(false),
            manager: core::cell::UnsafeCell::new(None),
        }
    }

    pub fn get(&self) -> Option<&SharedMemoryManager> {
        unsafe { (*self.manager.get()).as_ref() }
    }

    pub fn get_or_panic(&self) -> &SharedMemoryManager {
        self.get().expect("SharedMemoryManager is not initialized yet")
    }
}

impl SharedMemoryManagerTrait for StandardSharedMemoryHolder {
    type Generated = ();

    fn restore(&self, _: Self::Generated) -> SharedMemoryManager {
        self.get_or_panic().clone()
    }

    fn receive_manager(&self, builder: SharedMemoryManagerBuilder) -> Self::Generated {
        let manager = builder.build();

        #[cfg(feature = "threads")]
        {
            if self
                .once
                .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                .is_err()
            {
                return ();
            }
            unsafe { *self.manager.get() = Some(manager) };
        }

        #[cfg(not(feature = "threads"))]
        {
            if unsafe { &*self.manager.get() }.is_some() {
                panic!("SharedMemoryManager is already initialized");
            }
            unsafe { *self.manager.get() = Some(manager) };
        }
    }
}

/// Standard multi-target memory manager holder.
/// Stores multiple shared memory managers for different target modules.
#[derive(Debug)]
pub struct StandardSharedMemoryMultiHolder {
    #[cfg(feature = "threads")]
    pub managers: RwLock<SmallVec<[SharedMemoryManager; 4]>>,

    #[cfg(not(feature = "threads"))]
    pub managers: core::cell::UnsafeCell<SmallVec<[SharedMemoryManager; 4]>>,
}

unsafe impl Send for StandardSharedMemoryMultiHolder {}
unsafe impl Sync for StandardSharedMemoryMultiHolder {}

impl StandardSharedMemoryMultiHolder {
    pub const fn new() -> Self {
        Self {
            #[cfg(feature = "threads")]
            managers: RwLock::new(SmallVec::new_const()),

            #[cfg(not(feature = "threads"))]
            managers: core::cell::UnsafeCell::new(SmallVec::new_const()),
        }
    }

    /// Add a new manager for a target module
    pub fn add_manager(&self, manager: SharedMemoryManager) {
        #[cfg(feature = "threads")]
        {
            self.managers.write().push(manager);
        }

        #[cfg(not(feature = "threads"))]
        {
            unsafe { (*self.managers.get()).push(manager) };
        }
    }

    /// Get a manager by ID with a closure
    pub fn get_manager_with<R>(&self, id: usize, f: impl FnOnce(&SharedMemoryManager) -> R) -> R {
        #[cfg(feature = "threads")]
        {
            let managers = self.managers.read();
            let manager = &managers[id];
            f(manager)
        }

        #[cfg(not(feature = "threads"))]
        {
            let managers = unsafe { &*self.managers.get() };
            let manager = &managers[id];
            f(manager)
        }
    }

    /// Get count of registered managers
    pub fn manager_count(&self) -> usize {
        #[cfg(feature = "threads")]
        {
            self.managers.read().len()
        }

        #[cfg(not(feature = "threads"))]
        {
            unsafe { (*self.managers.get()).len() }
        }
    }
}

impl SharedMemoryManagerTrait for StandardSharedMemoryMultiHolder {
    type Generated = usize;

    fn restore(&self, id: Self::Generated) -> SharedMemoryManager {
        self.get_manager_with(id, |m| m.clone())
    }

    fn receive_manager(&self, builder: SharedMemoryManagerBuilder) -> Self::Generated {
        let manager = builder.build();
        let id = self.manager_count();
        self.add_manager(manager);
        id
    }
}

/// Export a shared memory manager implementation.
/// This macro creates an extern "C" function that target modules can call
/// to register their shared memory configuration with the VFS.
///
/// Usage:
/// ```ignore
/// static SHARED_MEMORY_MANAGER: StandardSharedMemoryHolder = StandardSharedMemoryHolder::new();
/// export_shared_memory_manager!(SHARED_MEMORY_MANAGER);
/// ```
#[macro_export]
macro_rules! export_shared_memory_manager {
    ($name:ident) => {
        $crate::__private::paste::paste! {
            $crate::export_shared_memory_manager!($name; &$name);
        }
    };

    ($name:ident; $holder:expr) => {
        const _: () = {
            const fn __asserter(_: &impl $crate::shared_memory::SharedMemoryManagerTrait) {}
            __asserter($holder);
        };

        $crate::__private::paste::paste! {
            #[unsafe(no_mangle)]
            pub extern "C" fn [<__wasi_export_shared_memory_manager_ $name>](
                builder: $crate::shared_memory::SharedMemoryManagerBuilder
            ) {
                let holder = $holder;
                holder.receive_manager(builder);
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_size() {
        // Ensure metadata struct is properly sized for C FFI
        assert_eq!(core::mem::size_of::<TargetMemoryMetadata>(), 16);
    }

    #[test]
    fn test_manager_builder() {
        fn dummy_register(_: *const TargetMemoryMetadata, _: i32, _: i32, _: u32) -> i32 {
            0
        }
        fn dummy_grow(_: i32, _: u32) -> i32 {
            0
        }
        fn dummy_lock(_: i32) -> *const u8 {
            core::ptr::null()
        }

        let builder = SharedMemoryManagerBuilder {
            lock_ptr: core::ptr::null(),
            metadata_ptr: core::ptr::null(),
            target_count: 0,
            register_target_ptr: dummy_register,
            grow_memory_ptr: dummy_grow,
            get_lock_ptr: dummy_lock,
        };

        let _manager = builder.build();
    }

    #[test]
    fn test_single_holder() {
        let holder = StandardSharedMemoryHolder::new();
        assert!(holder.get().is_none());
    }

    #[test]
    fn test_multi_holder() {
        let holder = StandardSharedMemoryMultiHolder::new();
        assert_eq!(holder.manager_count(), 0);
    }
}
