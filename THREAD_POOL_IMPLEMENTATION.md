# Virtual Thread Pool Implementation

## Overview
This document describes the implementation of `VirtualThreadPool` for the wasi_virt_layer project.

The `VirtualThreadPool` provides a true thread pool implementation that manages a fixed number of worker threads. It limits concurrent threads, tracks active workers, and automatically cleans up finished threads.

## Implementation Details

### VirtualThreadPool Structure
Located in `wasi_virt_layer/src/wasi/thread.rs`

```rust
struct PoolWorker {
    thread_handle: Option<std::thread::JoinHandle<()>>,
    busy: AtomicU32,
}

pub struct VirtualThreadPool {
    max_threads: AtomicU32,
    workers: Mutex<Vec<PoolWorker>>,
    current: AtomicU32,
}
```

The pool maintains:
- `max_threads`: Maximum number of threads in the pool (can be changed at runtime)
- `workers`: Vector of worker threads with their handles and busy status
- `current`: An atomic counter for round-robin thread assignment

### Key Features

1. **True Thread Pool**: Manages actual worker threads with handles and status tracking
2. **Dynamic Size**: Pool size can be changed at runtime via `set_size()`
3. **Automatic Cleanup**: Finished threads are automatically removed from the pool
4. **Thread Limit Enforcement**: Blocks new thread creation when pool is full
5. **Thread Safety**: Uses `Mutex` for worker management and `AtomicU32` for counters
6. **Const Constructor**: Pool can be initialized at compile time with `const fn new(size)`
7. **Send + Sync**: Implements `Send` and `Sync` for safe sharing across threads

### API

```rust
impl VirtualThreadPool {
    pub const fn new(max_threads: u32) -> Self
    
    pub fn size(&self) -> u32
    
    pub fn set_size(&self, new_size: u32)
    
    pub fn active_threads(&self) -> usize
}

impl VirtualThread for VirtualThreadPool {
    fn new_thread(
        &mut self,
        accessor: impl ThreadAccess,
        runner: ThreadRunner,
    ) -> Option<NonZero<u32>>
}
```

**Methods**:
- `new(max_threads)`: Create a new thread pool with specified maximum size
- `size()`: Get the maximum number of threads allowed in the pool
- `set_size(new_size)`: Change the maximum pool size at runtime
- `active_threads()`: Get the current number of active worker threads
- `new_thread()`: Spawn a new thread in the pool (returns `None` if pool is full)

## Usage Example

See `examples/vfs/thread_pool_vfs/src/lib.rs`:

```rust
use wasi_virt_layer::thread::VirtualThreadPool;

static mut THREAD_POOL: VirtualThreadPool = VirtualThreadPool::new(4);

plug_thread!(unsafe { &mut THREAD_POOL }, test_pool_thread);
```

This creates a pool with initial size 4 for the `test_pool_thread` WASM module. The pool size can be changed later:

```rust
unsafe { THREAD_POOL.set_size(8); }
```

## Test Case

The test case in `examples/test_wasm/test_pool_thread/src/main.rs` spawns 10 threads that:
1. Increment a shared atomic counter
2. Sleep for 10ms
3. Print status messages

The test verifies that all 10 threads complete successfully by checking the final counter value.

With a pool size of 4, the threads will be queued and executed in batches as slots become available in the pool.

## Building and Testing

### Build the test WASM module:
```bash
cargo build --target wasm32-wasip1-threads -p test_pool_thread --release
```

### Build the VFS wrapper:
```bash
cargo build -p thread_pool_vfs --release
```

### Run with the CLI tool:
```bash
cargo run -r -- -p thread_pool_vfs test_pool_thread -t multi --threads true
```

## Technical Notes

- The pool uses `root_spawn()` to create actual OS threads that won't be intercepted by the WASI virtualization layer
- Each thread receives a unique thread ID from a global atomic counter
- The pool index is calculated as `current % max_threads` for round-robin distribution
- The `ThreadAccessor` is provided by the `plug_thread!` macro and passed to `new_thread()` on each spawn request
- Worker threads are stored with their `JoinHandle` for proper lifecycle management
- Finished threads are automatically cleaned up before spawning new ones
- If the pool is full (all threads active), new spawn requests return `None`
- Pool size can be changed at runtime via `set_size()` using atomic operations
- Uses `parking_lot::Mutex` for efficient worker vector management
- Thread names are formatted as `wasi-pool-worker-{index}` for debugging
- The implementation is compatible with both single and multi-threaded WASM execution modes
- Implements `Send` and `Sync` for safe sharing across threads
- Size changes are thread-safe and take effect immediately for new thread spawns

## Export

The `VirtualThreadPool` type is exported in the public API:

```rust
#[cfg(feature = "threads")]
pub mod thread {
    pub use crate::wasi::thread::{
        DirectThreadPool, ThreadAccess, ThreadRunner, VirtualThread, 
        VirtualThreadPool, root_spawn,
    };
}
```
