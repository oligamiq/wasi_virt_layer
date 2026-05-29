# Threads VFS Example

This example demonstrates how to use WASI Virtual Layer with multi-threaded WASI modules.

## Features
- Shared memory between the VFS and the target module.
- Support for `wasip1-threads` ABI.
- Thread-safe virtual file system access.

## Running the example
```bash
cargo run -r -- -p threads_vfs test_threads -t single --threads true
```
