# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.10] - 2026-05-02

### Added
- **C/C++ Target Support**: Added support for non-Rust WASI modules (e.g., C/C++ via LLVM/Clang) that export `_start` but lack `__main_void`. The CLI now automatically synthesizes a `__main_void` wrapper.
- **`import_wasm!` Validation**: The CLI now validates that every `import_wasm!` declaration in the VFS has a matching target module provided in the command-line arguments.
- Added `test_c_target_wasm` integration test to verify non-Rust target compatibility.

### Changed
- Improved `import_wasm!` macro to provide a helpful `compile_error!` when used without arguments.
- Enhanced Deno test runner to properly await main execution and gracefully handle promise rejections triggered by `proc_exit`.
- Replaced hard errors with informative warnings when a target module is detected as non-Rust.

### Fixed
- Resolved call-graph rewriting issues for synthesized `__main_void` functions in C/C++ targets.

## [0.2.9] - 2026-05-02

### Added
- Introduced `plug_random!` and `plug_sched!` macros for full WASI ABI virtualization.
- Added `WasmAccessName` trait to support dynamic module name resolution in VFS operations.
- Added `PseudoRandom` (Xorshift64) and `StandardRandom` implementations for WASI random handling.
- Added `DefaultSched` implementation for WASI scheduler handling.

### Changed
- Enhanced WASI file, environment, and process modules to utilize `WasmAccessName` for improved type safety and dynamic module support.
- Updated `ProcessExit` trait to incorporate `WasmAccessName`.
- Improved CLI templates to include `plug_random!` and `plug_sched!` by default.

### Fixed
- Standardized workspace authorship by adding `authors.workspace` to all internal crates.

## [0.2.8] - 2026-05-01

### Added
- Manifest path support (`-m`, `--manifest-path`) in the CLI for more flexible build configurations.

### Changed
- Simplified internal Wasm IR manipulation by removing redundant tracking in `rewrite_inner` and `retain_inner` functions.

### Fixed
- Resolved multi-target export naming collisions in `wrap_unreachable` mechanism.
- Fixed export wiring bugs for multi-target modules.

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
