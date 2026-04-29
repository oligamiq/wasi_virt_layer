//! Test SharedMemory VFS with export_shared_memory_manager! macro

use wasi_virt_layer::shared_memory::{
    SharedMemoryManagerBuilder, SharedMemoryManagerTrait, StandardSharedMemoryHolder,
};

wit_bindgen::generate!({
    world: "test-helper-shared-memory",
});

struct TestHelperSharedMemoryVfs;

impl Guest for TestHelperSharedMemoryVfs {
    fn hello() {
        println!("Hello from test_helper_shared_memory_vfs!");
    }
}

#[cfg(not(test))]
export!(TestHelperSharedMemoryVfs);

/// SharedMemory manager holder exported for registration
pub static SHARED_MEMORY_VFS: StandardSharedMemoryHolder = StandardSharedMemoryHolder::new();

/// Export the SharedMemory manager interface for TypeScript helper generation
#[unsafe(no_mangle)]
pub extern "C" fn __wasi_export_shared_memory_manager_SHARED_MEMORY_VFS(
    builder: SharedMemoryManagerBuilder,
) {
    SHARED_MEMORY_VFS.receive_manager(builder);
}
