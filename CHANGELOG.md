# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.2] - 2026-06-11
### Added
- **Example Enhancements**:
    - Added `plug_sched!` and `plug_args!` to `lfs_api_test_vfs` example.
    - Added `plug_args!`, `plug_random!`, and WASI socket stubs (`sock_recv`, `sock_send`, `sock_accept`, `sock_shutdown`) to `wait_poll_vfs` example.
- **CLI Validation & Compatibility**:
    - Enhanced `validate_unresolved_imports` to correctly handle and validate `__self_` prefixed custom imports.
    - Whitelisted `wasi_snapshot_preview1` in the streaming merger's post-combine pass to allow direct pass-through of standard WASI imports when necessary.
- **Internal Cleanup**:
    - Improved documentation and added doc comments for `Wasip1ABIName`, `MemoryUniqueName`, and other internal CLI types.
    - Cleaned up unused imports and refined internal module organization.

### Fixed
- **Integration Tests**:
    - Enabled optimizations in `test_lfs_api_operations` to verify LFS operations under realistic build conditions.

## [0.5.1] - 2026-06-10
### Fixed
- **CLI Validation Improvements**:
    - Restored and refined unresolved WASI import validation in the streaming pipeline.
    - Improved error message formatting and context chaining for missing imports.
    - Fixed validation to correctly ignore intentionally dropped imports.
- **Test Fixes**:
    - Fixed single-memory thread VFS tests for Deno.
    - Cleaned up unused WASI imports in `c_target.wat` test fixture.

## [0.5.0] - 2026-06-09
### Added
- **Enhanced Streaming Merger**: Replaced the external `wasm-merge` dependency with a robust, in-process streaming merger.
    - **Import Deduplication**: Automatically deduplicates identical external function imports during the merge process.
    - **Robust Memory Restoration**: Implemented consistent memory state restoration, including zero-filling target memory, restoring active data segments from passive storage, and re-running target module starts.
    - **Integrated Poll Support**: Integrated `poll_wait` logic directly into the post-combine pass, utilizing `atomic.wait` for efficient thread suspension in multi-threaded builds.
- **Improved Memory Safety**:
    - **Guarded Memory Access**: Refactored `memory_director` to use guarded closures (`with_directed_memory`), preventing pointer invalidation during shared global updates or memory growth.
    - **Shared Global Optimization**: Optimized `SharedGlobalStreamPass` to use lock-free/wait-free global access for memory grow/size helpers.
- **Thread Pool Enhancements**:
    - **VirtualThreadPool Improvements**: Added initialization helpers, capacity reporting, and synchronous resizing to `VirtualThreadPool`.
- **Custom Metadata Sections**: Added `wvl.multi_memory_lowering.helpers.v1` custom section to share metadata and control flags between different streaming passes.

### Fixed
- **ABI Connection Refinement**: Improved the ABI connection pass to be more selective, only renaming imports when matching exports are present in the module.
- **Memory Copy Instruction Order**: Corrected the operand order for `memory.copy` instructions in generated director functions.
- **Start Function Synthesis**: Ensured the synthesized `_start` function correctly orchestrates thread initialization and target runtime startup sequences.

### Changed
- **Version Bump**: Updated workspace version to 0.5.0.

## [0.4.12] - 2026-05-23
### Changed
- **Parallelized Wasm Processing**: Introduced `rayon` to parallelize Wasm modification and generation steps (e.g., rewriting multi-memory, shared globals, atomic waits, unreachable wrappers, etc.) inside the CLI for single-module processing. This significantly improves transpilation speed for large Wasm modules without increasing memory overhead via parallel target generation.

## [0.4.10] - 2026-05-22
### Fixed
- **Development Mode Optimization**: Fixed an issue where the `--dev` flag did not completely skip `wasm-opt` for target Wasm modules. Ensure target Wasm optimizations are fully bypassed when using `--dev`.
- **Test Compatibility**: Updated VFS test modules (`threads_vfs`, `anonymous_threads_vfs`, `no_thread_with_thread_feature_vfs`) to include `plug_clock!` to satisfy unmodified target import constraints during unoptimized testing.

## [0.4.9] - 2026-05-17
### Added
- **Full WASI Snapshot Preview 1 Compliance**: Extended `Wasip1FileTrait` (and `Wasip1LFSBase` / `Wasip1FileSystem`) to include all missing Snapshot Preview 1 filesystem operations, such as `pwrite`, `pwrite_raw`, `advise`, `allocate`, `datasync`, `sync`, `filestat_set_size`, and `filestat_set_times`.
- **WasiEmbeddedFile Implementation**: Added complete implementations for all missing methods inside the read-only `WasiEmbeddedFile`. Mutating methods now correctly return `ERRNO_ROFS` (Read-Only File System), while `advise` and `sync` gracefully return `Ok(())`.

### Fixed
- **Compiler and Parsing Failures**:
  - Restored trailing cursor updates and successfully closed blocks in `fd_seek_raw` within `multiple/lfs.rs`, resolving "unclosed delimiter" compilation failures in downstream VFS test modules.
  - Corrected `u64` vs `i64` type discrepancies inside `fd_seek_raw`'s invocation of `Wasm::store_le`.
  - Cleaned up type aliasing duplicate definitions for `Advice` and `Fstflags` to prevent Rust compilation conflicts.
- **Robust Path Traversal**: Added missing `RootDir` variants to path resolution component matchers in `StandardDynamicLFS` to safely handle directory transitions up to root.

## [0.4.8] - 2026-05-17
### Added
- **Integrated Shared Global Management**: Consolidated the thread-safe memory management pipeline by merging `SharedGlobal` functionality directly into `MultiMemoryLowering`. This handles replacement of mutable offset globals with VFS shared-memory-backed equivalents, eliminating the separate `post_lower_memory` pass and improving pipeline cohesion.

### Fixed
- **Double-Locking Hazard**: Resolved a potential deadlock issue where the `SharedGlobal` generator wrapped memory growth with its own write lock, nesting under `MultiMemoryLowering`'s already locked grow functions. Now, `MultiMemoryLowering` manages the locking directly and safely using optimized `_no_wait` and `_with_lock` variants.
- **Float and Vector Local Support**: Fixed a Walrus lowering crash (`internal error: entered unreachable code`) by expanding type support for float (`f32`, `f64`) and vector (`v128`) instruction rewrites inside the temporary local allocation helper of `MultiMemoryLowering`.

## [0.4.7] - 2026-05-16
### Added
- **CLI Development Mode**: Added a new `--dev` flag to the `build` and `prebuild` commands. This flag skips WebAssembly optimizations (bypassing `wasm-opt`), significantly speeding up the development cycle.
- **Enhanced Test Infrastructure**: Updated the internal test runner to use `--dev` mode by default for all existing integration tests, ensuring all features are verified in unoptimized builds. Added explicit optimized test paths (`test_build_multi_opt`, `test_build_single_opt`) to maintain production pipeline reliability.

### Fixed
- **Unoptimized Memory Lowering Bug**: Resolved a critical issue in the native `multi_memory_lowering` pass where recreated memory imports were not correctly associated with their memory IDs in Walrus. This fix ensures that `wit-component` translation succeeds even when optimizations are disabled.

## [0.4.6] - 2026-05-12
### Added
- **Native Multi-Memory Lowering**: Replaced the external dependency on `wasm-opt --multi-memory-lowering` with a native, robust implementation using `walrus`. This pass correctly handles memory merging, data segment offsetting, and instruction rewriting without requiring external tools.
- **Thread-Safe Memory Instrumentation**: Implemented deep instruction rewriting to inject thread-safe synchronization points (`lock_read_acquire`/`release` and `lock_write_acquire`/`release`) and dynamic memory offset tracking for all load, store, and atomic operations in multi-threaded environments.
- **Robust `memory.grow` & `memory.size` Handling**: Added specialized helper functions that handle dynamic memory shifting and offset global updates during `memory.grow`, ensuring memory consistency across all integrated Wasm modules.

## [0.4.5] - 2026-05-10
### Added
- **Optimized `WaitPoll` Performance**: Replaced the busy-wait loop in `poll_oneoff` with a non-blocking `Atomics.wait`-based mechanism.
- **`PollWait` Generator**: Added a new CLI generator that automatically replaces `__wvl_poll_atomic_wait` imports with `memory.atomic.wait32` when threads are enabled, ensuring efficient thread suspension.
- **Improved Test Infrastructure**: Refactored `lfs_api_test_vfs` to build its target from source, eliminating the need for pre-built artifacts and increasing test reliability.

## [0.4.3] - 2026-05-10
### Fixed
- **Multi-Module Atomic Isolation**: Introduced `wasm_id` to the atomic synchronization layer to prevent wait/notify collisions between different Wasm modules sharing the same VFS.
- **Dynamic Target Memory Dispatch**: Implemented a dynamic dispatch mechanism in `AtomicPatch` to correctly load memory values from the specific target module's memory space during atomic waits.
### Added
- **Expanded Atomic Tests**: Added a new test case to the `test_atomic_wait` example that verifies non-zero offset support and concurrent VFS I/O activity during atomic waits.

## [0.4.2] - 2026-05-09
### Fixed
- **VFS Atomics Initialization**: Changed the `DashMap` hasher to `FxHasher` (deterministic) to prevent `random_get` calls during module initialization. This fixes failures in `module.start` where imported functions may not yet be available or are restricted.

## [0.4.1] - 2026-05-09
### Added
- **AtomicPatch Non-Zero Offset Support**: Enhanced `AtomicPatch` to support atomic instructions with non-zero offsets by dynamically generating wrapper functions. This ensures that static offsets in Wasm bytecode are correctly applied when redirecting to the VFS-managed synchronization layer.

## [0.4.0] - 2026-05-09
### Added
- **AtomicPatch Synchronization**: Introduced `AtomicPatch` generator to redirect standard WebAssembly atomic instructions (`memory.atomic.wait32` and `memory.atomic.notify`) to a stable VFS-managed synchronization layer.
  - Implemented `vfs_atomic` module in `wasi/thread.rs` using `LazyLock` and `dashmap::DashMap` for efficient and scalable thread coordination.
  - Added `test_atomic_wait` target and `atomic_wait_vfs` example to verify `std::sync::Condvar` behavior in virtualized environments.
- **Integration Tests**: Added `test_atomic_wait_vfs` to the integration test suite to ensure correct atomic instruction rewriting during multi-memory lowering.

### Fixed
- **Atomics Multi-Memory Race**: Resolved a critical issue where memory growth or offset shifts in multi-memory lowering would invalidate atomic wait addresses, causing synchronization primitives (like `Mutex` and `Condvar`) to hang or trap.

### Changed
- **Version Bump**: Updated workspace version to 0.4.0.

## [0.3.9] - 2026-05-09

### Added
- **WaitPoll Example**: Added `wait_poll_vfs` example and `test_wait_poll` integration test to verify blocking sleep behavior in virtualized environments.

### Fixed
- **WaitPoll Clock ID**: `WaitPoll` now correctly respects the `clock_id` from WASI subscriptions instead of hardcoding `CLOCKID_REALTIME`. This fixes issues where `std::thread::sleep` (which typically uses `CLOCKID_MONOTONIC`) would return immediately or panic.
- **WaitPoll Memory Corruption**: Resolved a critical memory corruption issue in `WaitPoll` by using `crate::__self::__self` for local buffer operations, preventing incorrect pointer redirection by target-specific memory directors.

### Changed
- **StandardClock Refactoring**: Refactored `StandardClock` to use the safer `Wasm::store_le` API for memory writes, removing manual `memory_director_mut` calls and `unsafe` pointer operations.
- **Test Infrastructure**: Increased `test_wait_poll` execution timeout to 30 seconds to ensure reliability on slower CI/CD environments.
- **Version Bump**: Updated workspace version to 0.3.9.

## [0.3.8] - 2026-05-07

### Added
- **JS Test Runner Enhancement**: Improved the `custom_instantiate` function in the JS test runner to automatically bind and expose all WIT-exported functions from the root instance. This enables seamless access to WIT exports when using the virtualized module in JavaScript environments.

### Changed
- **Version Bump**: Updated workspace version to 0.3.8.

## [0.3.7] - 2026-05-06

### Added
- **Enhanced WASI Path Functions**: Added delegation and routing support for path operations in all VFS layers (`StandardDynamicFileSystem`, `StandardEmbeddedFileSystem`, `StandardMultipleFileSystem`). This ensures consistent behavior across different filesystem implementations when using path-based WASI functions.

### Changed
- **Test Infrastructure**: Updated `@oligami/browser_wasi_shim-threads` to `^0.3.2` and improved worker import logic in the JS test runner.
- **Dependencies**: Updated workspace dependencies to their latest versions.
- **Version Bump**: Updated workspace version to 0.3.7.

## [0.3.6] - 2026-05-06

### Added
- **WASI Path Functions**: Implemented core `path_create_directory`, `path_link`, `path_remove_directory`, `path_rename`, and `path_unlink_file` functions.
  - Added low-level implementation logic in `StandardDynamicLFS`.
  - Updated `plug_fs!` macro to export these functions, completing the standard WASI file system interface.

### Changed
- **Version Bump**: Updated workspace version to 0.3.6.

## [0.3.5] - 2026-05-05

### Added
- **VFS Host Communication**: Implemented import resolution for VFS functions, allowing target modules to call functions provided by the VFS (host-side virtualization). Added `vfs_host_test_vfs` example and `vfs_host_test_target`.
- **Per-Target Feature Configuration**: Enhanced the CLI to allow per-target feature configuration. Improved feature extraction logic to better handle dependencies between target features and the virtual layer.

### Changed
- **CLI Robustness**: Refactored generator logic for shared globals and start functions to be more robust with per-target configurations.
- **Documentation**: Enhanced documentation for the `wasm-merge` module naming and integration process in `wasi_virt_layer/src/memory.rs`.

## [0.3.4] - 2026-05-05

### Added
- **Dynamic File System Operations**: Implemented `Lfs` (Local File System) based dynamic file system operations in `wasi_virt_layer`. Added `examples/vfs/lfs_api_test_vfs` and associated integration tests to verify full CRUD operations on the virtualized file system.
- **VFS Build Options**: Added `--features` and `--no-default-features` flags to the CLI `build` and `prebuild` commands, allowing fine-grained control over the VFS module's feature set during compilation.
- **Concurrent Execution Support**: Refactored the CLI's command locking mechanism to support multi-lock concurrency. This allows multiple `wasi_virt_layer` instances to run in parallel while safely managing shared resources like the `target` directory.
- **Error Reporting**: Enhanced error reporting in the `wasm-merge` utility to provide more diagnostic information when module merging fails.

### Fixed
- **Macro Logic**: Fixed edge cases in the `__if_feature!` macro evaluation to correctly handle negated feature gates (e.g., `not_trace_thread`).
- **Compiler Warnings**: Suppressed unused type warnings in the `plug_process!` macro when certain features are disabled.

### Changed
- **Documentation**: Enhanced API documentation for `VirtualThread` and the `wrap_unreachable` macro. Updated `plug_` macro documentation with better examples and error message descriptions.
- **Integration Tests**: Refactored several integration tests to use the new `ls2` helper module for more reliable verification of multi-module scenarios.

## [0.3.3] - 2026-05-04

### Added
- **JS Dynamic Wasm Support**: Introduced early-stage support for dynamic Wasm module loading in JavaScript/Deno environments. This enables the virtual layer to dynamically resolve and link additional Wasm modules at runtime within a JS-hosted environment.
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
