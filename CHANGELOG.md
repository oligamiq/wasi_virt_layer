# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.7] - 2026-05-01

### Added
- Refinement of terminology unification and stabilization of `StandardMultipleFileSystem`.
- Improved multi-module start-up sequence handling.
- Enhanced memory management for dynamic name resolution in Wasm access.

## [0.2.6] - 2026-04-30

### Changed
- **Major Refactor**: Terminology unification across the entire workspace.
    - `const`/`static` renamed to `Embedded`.
    - `changeable`/`dynamic` renamed to `Dynamic`.
    - `Default` implementations renamed to `Standard` (e.g., `StandardEmbeddedFileSystem`, `StandardDynamicFileSystem`).
    - Feature flags updated: `const-fs` -> `embedded-fs`, `changeable-fs` -> `dynamic-fs`.
- Standardized process handling across all examples.

### Added
- Implement `PollOneoff` for handling clock subscriptions in the polling module.
- Implement `WaitPoll` (and `DefaultWaitPoll`) for blocking sleep/yield in `poll_oneoff`.
- Introduce `DirectThreadPool` and `VirtualThreadPool` for robust thread management.
- Add `pool-threads-vfs` and `complex-threads-vfs` examples.

### Fixed
- Improved compatibility for Deno and Node.js in worker handling logic.
- Fixed deadlocks in multi-threaded VFS scenarios.

## [0.2.5] - 2026-04-29

### Added
- **Zero-Copy Shared Memory ABI**:
    - New CLI command: `prepare-target` to transform modules for zero-copy memory sharing.
    - New ABI functions: `wasip1_vfs_register_shared_memory_target`, `wasip1_vfs_shared_memory_grow`, `wasip1_vfs_shared_memory_get_lock_ptr`.
    - Support for replacing `memory.grow` instructions with ABI calls.
- `wasip1_vfs_shared_memory` module in core library.
- Performance benchmarks for shared memory ABI.

## [0.2.4] - 2026-04-20

### Added
- **Trap Interception**: Introduced `wrap_unreachable!` macro and `WrapUnreachableGenerator` to catch Wasm `unreachable` instructions.
- Added `PseudoRandom` implementation using Xorshift64.
- Added `plug_random!` and `plug_sched!` macros for WASI ABI virtualization.
- Updated CLI project templates to include `plug_random!` and `plug_sched!` by default.
- Removed "not implemented" warnings for Random, Clock, Poll, and Sched in the CLI.
- Support for anonymous VFS target generation.
- `trace-fs` feature for tracing filesystem operations.

## [0.2.3] - 2026-04-15

### Added
- `producer` field support in generated Wasm metadata.
- `simple-debug` feature for fast debugging prints in Wasm.
- Enhanced `WasmAccess` with `WasmAccessMemoryUtilUpper` for advanced memory operations.

## [0.1.2] - 2026-03-30

### Added
- Introduced the `wasi_virt_layer new` command for project scaffolding.
- Added comprehensive examples for `ls` and `ls-vfs`.
- Added Japanese localization support for documentation (mdBook).

### Changed
- **Breaking Change**: Significant refactoring of the CLI structure and command-line arguments.

## [0.1.0] - 2026-03-15

### Added
- Initial release of the WASI Virtual Layer (WVL) framework.
- Basic implementation of `plug_fs!`, `plug_env!`, and `plug_args!`.
