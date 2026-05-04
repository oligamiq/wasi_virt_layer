# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.3] - 2026-05-04

### Added
- **Internal Macro Refactoring**: Redesigned the `__if_feature!` macro system into an extensible matrix-based architecture. This ensures that feature-gated logic in exported macros (like `plug_thread!`) correctly evaluates based on the library's internal configuration, preventing breakage in downstream crates while keeping the exported API surface minimal.
- **Unified Tracing System**: Consolidated all debug and tracing output behind the `trace` feature flag and its sub-features (`trace-fs`, `trace-thread`). Unified the tracing hierarchy where `trace` enables all subsystems and `unstable_print_debug` automatically enables `trace`.

## [0.3.2] - 2026-05-04

### Added
- **New Example**: Added `examples/vfs/dynamic_args_vfs` demonstrating the use of dynamic arguments.
- **Test Infrastructure**: Added `examples/test_wasm/test_args` helper WASM for argument verification and `test_dynamic_args_vfs_example` integration test.

### Fixed
- **plug_args! Macro**: Resolved a bug in the `plug_args!` macro where function definitions were duplicated and incorrect types were used in `@dynamic` mode.
- **Macro Type Safety**: Fixed a compilation error in `plug_args!` where unnamable closure types were used in a `const` context; migrated to `let` for correct type inference.
- **Compiler Warnings**: Fixed unused variable warnings in threaded contexts when tracing features are disabled.

## [0.3.0] - 2026-05-04

### Added
- **New Example**: Added `minimal_repro_virtual` example demonstrating virtual environment support for WASI.
- **Initialization Refactor**: Introduced a more robust startup sequence handling in `starts.rs`, managing various initialization phases like `init_offset_global`, `save_target_memory`, and `thread_patch`.
- **Deep Analysis Documentation**: Added several detailed analysis reports regarding memory flow, corruption root causes, and layout diagrams (`MEMORY_CORRUPTION_ROOT_CAUSE.md`, `MEMORY_FLOW_BUG_ANALYSIS.md`, etc.).

### Changed
- **Wasm Generator Overhaul**: Refactored the internal Wasm start section generation and shared global management to better handle complex multi-threading and multi-memory scenarios.
- **Version Bump**: Updated workspace version to 0.3.0.

### Fixed
- **Shared Global Stability**: Replaced `GlobalGet`/`GlobalSet` constant instructions with dynamic function calls to shared global handlers, ensuring correctness in multi-threaded environments after multi-memory lowering.

## [0.2.17] - 2026-05-03

### Added
- **Utility Macros**: Introduced `if_threads!`, `if_not_threads!`, `if_multi_memory!`, and `if_not_multi_memory!` macros for better conditional compilation management in the core library.
- **New Test Fixtures**: Added `minimal_repro` and `repro_multi_target_table_bug` examples to verify complex multi-target linking and memory management scenarios.

### Changed
- **Multi-target Threading Stability**: Fixed a critical bug where thread functionality failed when virtualizing multiple WASM modules simultaneously. This was resolved by decentralizing memory management, replacing the shared `ALT_GLOBAL_VAR` with per-module bridge functions to prevent cross-module state interference.
- **Multi-memory Refactor**: Refactored memory director access to better support multi-memory Wasm modules.
- **Memory Management**: Ensured that maximum memory limits are explicitly set for unshared memories in `TemporaryRefugeMemory`, improving compatibility with certain runtimes.
- **Unique Name Generation**: Enhanced the CLI to generate target-specific unique names for shared global functions.

### Fixed
- **PlugThread Validation**: Improved error reporting when `thread_spawn` is imported by a target but the `plug_thread!` macro is missing in the VFS. The CLI now explicitly suggests using the `plug_thread!` macro.
- **Package Name Normalization**: Fixed a linking bug where package names containing hyphens were not correctly normalized to underscores in Wasm module identifiers.

## [0.2.16] - 2026-05-02

### Changed
- **Thread Handling Refactor**: Updated the `VirtualThread` trait to use `&self` instead of `&mut self` for `new_thread` and `sched_yield`. This allows virtual thread implementations to be stored in `static` rather than `static mut`, improving safety and idiomatic Rust usage.
- **Thread Pool Examples**: Refactored all thread-related examples (`pool-threads-vfs`, `thread_pool_vfs`, etc.) to use thread-safe `static` pools.
- **`plug_thread!` Macro**: Enhanced the macro to support immutable references to thread pools and added a compile-time check to ensure the pool expression is valid.

## [0.2.15] - 2026-05-02

### Added
- **`UnsafeOnceCell`**: Introduced a minimal, thread-safe `UnsafeOnceCell` in `wasi_virt_layer::utils` for safe one-time initialization of values in `static` contexts.

### Changed
- **`VirtualThreadPool`**: Refactored to use `UnsafeOnceCell` for internal state initialization, enabling its use from immutable references.

## [0.2.14] - 2026-05-02

### Changed
- **Memory Handling**: Enhanced the generator to ensure the VFS memory is imported at index 0. This is a critical requirement for compatibility with `wasm-opt` during multi-memory lowering.
- **CLI**: Improved logging by cleaning up warning messages during thread-enabled transpilation.

## [0.2.13] - 2026-05-02

### Added
- **Wasm Optimization**: Enhanced `wasm-opt` integration to explicitly enable `--enable-threads` and `--enable-multimemory` flags.
- **Multi-memory Lowering**: Implemented shared memory tracking in `TemporaryRefugeMemory` to ensure the `shared` flag is correctly preserved during multi-memory lowering transformations.

### Changed
- **Test Fixtures**: Updated `c_target.wasm` fixtures and added `.wasm` binary attribute to `.gitattributes` to ensure proper handling of Wasm binaries in the repository.

## [0.2.11] - 2026-05-02

### Changed
- **VFS Refactor**: Decoupled `fd_seek` from Local File System (LFS) traits. Centralized file descriptor cursor management and seek logic within the VFS layer (`StandardMultipleFileSystem`, `StandardEmbeddedFileSystem`, and `StandardDynamicFileSystem`).

### Fixed
- **CLI Validation**: Fixed a regression in `CheckUseLibrary` that prevented self-virtualizing modules (e.g., `self_vfs`) from passing validation when the VFS name matched a declared Wasm name.

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
