# Core Traits

The following traits are the primary interfaces for implementing virtualized WASI behaviors.

## `Wasip1FileSystem`

This trait defines the interface for a virtual file system. Implementing this allows you to intercept and handle all file-related WASI calls.

### Key Methods
- `fd_read_raw`: Read from a file descriptor.
- `fd_write_raw`: Write to a file descriptor.
- `path_open_raw`: Open a file or directory.
- `fd_readdir_raw`: Read directory entries.

## `VirtualEnv`

Defines the interface for virtual environment variables.

## `VirtualArgs`

Defines the interface for virtual command-line arguments.

## `Clock`

Defines the interface for time-related operations (e.g., getting the current time).
