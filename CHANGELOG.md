# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Fixed
- **Threaded target reset lifecycle**:
    - `_reset()` now cancels target atomic waits, waits for all queued/running logical threads from the old target generation to finish before mutating target state, then frees atomic-wait cells, restores target state, and restarts the target. This prevents stale threads from racing with reset memory and stealing or missing notifications.
    - Thread activity is counted from enqueue time so child threads queued by an old-generation worker are included in the reset barrier.
    - Thread activity tracking is internal and keyed per `ThreadAccessor` type/target, so the public `ThreadAccess` trait remains unchanged for downstream implementations.
- **Threaded integration test logging**:
    - Deno stdout/stderr are written directly to test log files instead of undrained pipes, avoiding pipe backpressure during verbose threaded runs.
- **Patch-release API compatibility**:
    - Restored the public three-argument `VirtualThreadPool::run` API while keeping the new reset activity tracking internal, preserving compatibility with downstream callers.
- **Future Rust compatibility**:
    - Removed expression-position `eyre::bail!` usages that trigger Rust future-incompatibility lints, and fixed bare rustdoc URLs in the core and CLI crates.
- **Dependency security and lockfiles**:
    - Updated `crossbeam-epoch` to 0.9.20 to resolve RUSTSEC-2026-0204 and refreshed the `cxx` dependency family to 1.0.199 to remove its unsound advisory.
    - Removed stale workspace-member `Cargo.lock` files so the audited root lockfile is the single dependency resolution source; GitHub Dependabot now reports no open alerts.
- **Publishing metadata**:
    - Crate packages now include the repository README, and `wasi_virt_layer` uses the repository's actual `LICENSE` file instead of the previous incorrect `MIT OR Apache-2.0` metadata declaration.
    - Crate metadata now declares the tested MSRV explicitly: Rust 1.89.0 for `wasi_virt_layer` and Rust 1.93.0 for `wasi_virt_layer-cli`.
- **Release safety**:
    - Added a staged release workflow and `scripts/release-preflight.sh` to verify repository state, target-version availability, Dependabot/RustSec status, regression tests, SemVer compatibility, MSRV builds, strict rustdoc, and publish dry-runs before tagging.
    - Added `scripts/release-postflight.sh` to verify tag/workflow/release commit identity, the cargo-dist asset set, release checksums, installer version references, and a packaged Linux CLI smoke test after tagging, then confirm both exact crates.io versions after publication.

## [0.7.0] - 2026-08-28
### Fixed
- **WASI threads toolchain compatibility**:
    - Removed the obsolete manual `__wasi_init_tp` / `__wasm_call_ctors` initializer workaround and its synthetic `__thread_patch` startup placeholder after the upstream WASI TLS initialization fix.
    - Stopped forcing `+nightly` solely for `wasm32-wasip1-threads`; Rust 1.92.0 and later include the stable fix for rust-lang/rust#146721.
    - Threaded reactor/library builds now rely on the upstream wasi-sdk 34 initialization behavior available from nightly-2026-08-27 and, on stable, Rust 1.100.0; older toolchains are rejected before the VFS build starts.

## [0.6.1] - 2026-07-18
### Fixed
- **Wasm Generation**:
    - Preserved Wasm tag sections after combining modules.
- **Path Resolution**:
    - Treated dot-prefixed names (e.g., `.cargo`) as normal path components rather than special tokens.

## [0.6.0] - 2026-07-04
### Added
- **Reset State Management**:
    - Synthesized `_reset()` functions now correctly restore the target's own-memory logical sizes to their initial states.
    - `_reset()` now clears the `unreachable` trap wrapper state flags.
    - Added `own_memory_reset_vfs` and `unreachable_reset_vfs` integration tests to verify state clearing behavior.
- **Documentation**:
    - Added `rubrc-rustc-vtp-followup-2026-07.md` detailing the `VirtualThreadPool` architecture and thread re-initialization invariants.

### Fixed
- **Deadlock Detector**:
    - Fixed a false positive where the deadlock detector would incorrectly flag the VFS shell thread during a host idle wait (thread ID `1_000_000`, wasm ID `4`).
- **Tests**:
    - Fixed `pool_reused_direct_export_vfs` test to correctly verify target reinitialization on reused non-main workers.
    - Fixed `test_stderr_reentrancy_vfs` to explicitly assert a trap rather than a timeout.
    - Corrected the target name in `write-single-vfs` to `test_write_single` to align with expected test inputs.

## [0.5.14] - 2026-06-30
### Added
- **Deadlock Detection**:
    - Added `--deadlock-detection` CLI gate (`wasi_virt_layer-cli`) to enable automatic deadlock detection for threaded Wasm builds.
    - Injects per-target wasm thread ID globals so the host can associate atomic.wait callers with logical threads.
    - Observes atomic writes (`i32.atomic.store` variants) to detect when a thread has released a lock.
    - Detects closed atomic wait deadlocks when a waiter times out without the expected notify after a reasonable interval.
    - Routes all host-side wait calls through a central `DeadlockDetector` that logs suspected deadlocked thread IDs and their backtrace context.
    - Added unit and integration tests covering single-thread wait, multi-thread acquire, and feature-matrix permutations.

### Fixed
- **test_minimal_repro / test_minimal_repro_virtual**:
    - Inverted success condition in both integration tests — they previously returned `Err` when `output.status.success()` was true, causing false failures.
    - Added missing `plug_sched!(DefaultSched, ls, self)` in `examples/vfs/minimal_repro_virtual/src/lib.rs` so the `ls` target can resolve `sched_yield` imports.

## [0.5.13] - 2026-06-24
### Fixed
- **Atomic Wait State Reset**:
    - Cleared `WAIT_MAP` state on `_reset()` calls per-target using `__vfs_atomic_reset_target` to prevent stale zombie threads from stealing atomic notify signals on subsequent runs.
- **Tests**:
    - Fixed a bug in `has_required_wasi_targets` that incorrectly skipped tests when WASI targets were successfully detected.
    - Deduplicated `has_required_wasi_targets` logic across the test suite into `utils.rs` to improve performance and stability.
    - Tightened `post_combine` codegen tests to assert exact argument instructions.

### Added
- **Tests**:
    - Added `test_atomic_wait_reset.rs` integration test to ensure `_reset()` correctly clears memory.atomic wait states for a target.

## [0.5.12] - 2026-06-24
### Fixed
- **Thread spawning and pooling logic**:
    - Spun up workers when pool is under capacity and avoided AddThread deadlock in `run()`.
    - Routed VFS thread spawn wrapper directly to host thread path, fixing TLS destruction errors.
### Added
- **Tests**:
    - Enhanced spawn_main test with higher thread counts and re-entrancy.
    - Added test for spawning main in a new thread.

## [0.5.11] - 2026-06-23
### Fixed
- **VirtualThreadPool worker reuse skipping Wasm start-section reinitialization**:
    - When a `VirtualThreadPool` worker thread processed multiple logical threads sequentially (via `Run` messages in the shared queue), the Wasm start section was called only on the first logical thread. Subsequent logical threads on the same worker skipped TLS initialization and global constructors, causing runtime errors (e.g., `unreachable` traps in downstream threading projects).
    - Added per-worker `thread_local! WORKER_HAS_RUN_BEFORE: Cell<bool>` to detect reuse; on non-first `Run` messages, `call_thread_start_init()` invokes the Wasm start section function (exported as `__wasip1_vfs_<target>__thread_start`) before executing the logical thread body.
    - Added `StartsPreStreamPass` to dual-export the Wasm start section as both `__flesh_<target>_start` (preserving existing contract) and `__wasip1_vfs_<target>__thread_start` (reuse detection contract).
    - Added `post_combine.rs` classification for `__thread_start` imports/exports to resolve the cross-module call chain.
    - `DirectThreadPool` and `TestThreadAccessor` implement `call_thread_start_init` as no-ops (fresh threads don't need re-init).
    - Added integration test `test_pool_thread_reinitialization` with a WAT target whose start section clears a marker, verifying that the marker is cleared on every logical thread, not just the first.

## [0.5.10] - 2026-06-22
### Fixed
- **Dynamic LFS UTF-8 filename corruption**:
    - Fixed a bug where raw UTF-8 path bytes were cast to `char` via `b as char`, causing Latin-1 reinterpretation and byte-doubling for all non-ASCII sequences. Files created with Unicode names (e.g. `あ`, `👋`) were stored under garbled keys and became unfindable (`ENOENT`) on subsequent lookups.
    - Replaced byte-by-byte `push(b as char)` with a `bytes_to_smallstring` helper that preserves raw UTF-8 bytes via `core::str::from_utf8`.
    - Added regression tests for Unicode filename create, lookup, DirMap key verification, and 4-byte emoji handling.

## [0.5.9] - 2026-06-22
### Added
- **Documentation**:
    - Added a detailed Japanese mdBook architecture page (`book/src/ja/architecture/own-memory.md`) explaining the final own-memory architecture, physical vs logical memory bounds, lowering contracts, remapping, and regression testing checkpoints.
    - Integrated the own-memory architecture section into the Japanese `SUMMARY.md` and Index pages.

### Changed
- **Refactoring & Own-Memory ABI Alignment**:
    - Centralized all own-memory ABI prefixes/suffixes, host exports, memory director exports/parsers, target sanitization, and copy helpers into a new single contract helper module (`wasi_virt_layer-cli/src/wasm_stream/own_memory_abi.rs`).
    - Eliminated manual `replace("-", "_")` target-name mutations in `multi_memory_lowering.rs` and `post_combine.rs` in favor of consistent `own_memory_abi::sanitize_target_name` helpers, resolving several silent hyphenated target lookups/start/reset failures.
    - Replaced CLI-crashing `.unwrap()` and `panic!` invocations during target positions and string splits with safe `Result::Err(eyre::Error)` propagations.
- **Cleanup**:
    - Deleted obsolete script `rewrite.py` and dead duplicate lowering pass `own_memory_lowering.rs`.

## [0.5.8] - 2026-06-20
### Added
- **Self/Host own-memory API**:
    - `own_memory!` now unconditionally supports `memory_size_self()`, `memory_grow_self()`, and `memory_size::<__self>()` / `memory_grow::<__self>()` for querying and expanding the VFS/host own memory without passing a target Wasm.
    - `__self` marker type publicly re-exported via `wasi_virt_layer::prelude::*`.
- **Compile-time guard**: `own_memory!(self, ...)` and `own_memory!(__self, ...)` now produce a clear `compile_error!` with guidance.

### Changed
- CLI lowering (`own_memory_lowering`, `multi_memory_lowering`) maps `__self` imports to memory index 0, aligning host memory with the existing host logical exports.

## [0.5.7] - 2026-06-20
### Added
- **ThreadID Collision Safety**:
    - Introduced `ThreadIdGenerator` trait with `ReservedRangeThreadIdGenerator` (default base 1,000,000) for configurable guest thread ID generation in `VirtualThreadPool`.
    - Added `new_const_with_thread_id_generator()` constructor for custom ID generation.
    - `plug_thread!` now returns `ERRNO_AGAIN` instead of panicking on ID exhaustion.
    - `root_spawn`/`root_spawn_unchecked` use `RootSpawnFlagGuard` (Drop guard) for correct nested root-spawn state management.
- **Documentation**:
    - Documented collision contract: guest runtime uses a single thread-ID namespace shared by external (WasiRunner) and pool (VFS) threads.

### Changed
- **VirtualThreadPool refactoring**:
    - Thread ID generation moved from host-thread-derived `next_thread_id()` to configurable `Generator` field in the pool struct.
    - Added generic `Generator: ThreadIdGenerator` parameter to `VirtualThreadPool`.

### Removed
- Removed `VirtualThreadPool` dependency on `get_host_thread_id()` / `next_thread_id()` / `THREAD_LOCAL_COUNTER`.

## [0.5.6] - 2026-06-19
### Added
- **Feature Gating**:
    - Added `own-memory` Cargo feature to `wasi_virt_layer` to gate `own_memory!` macro, functions, and exports.
    - CLI `build` and `prebuild` now automatically inject `wasi_virt_layer/own-memory` for `--own-memory` builds.

### Changed
- **Memory Expansion & own_memory!**:
    - Moved own-memory logical-size exports out of `import_wasm!` and into `own_memory!` macro.
    - Rewrote own-memory logical `memory.grow` wrapper to use Atomic Compare-and-Swap (CAS) loop instead of layout locks.
    - Restored physical `memory_grow` contract to caller-quiesced execution without inserting broad layout locks.
    - Optimized `SharedGlobalStreamPass` to use lock-free/wait-free globals under `--own-memory` + threads.
- **Testing & Execution**:
    - Strengthened `test_own_memory_smoke_new_example` integration test to assert threaded worker run success and clean exit.
    - Threaded integration test runs now write and preserve Deno worker output to `.deno-test-stdout.log` / `.deno-test-stderr.log`.

### Removed
- **Debug Artifacts**:
    - Removed pre-existing hardcoded debug file generation (`DEBUG_INPUT.wasm`, `DEBUG_OUTPUT.wasm`) from core stream pipeline.

## [0.5.5] - 2026-06-16
### Added
- **Memory Expansion & own_memory!**:
    - Implemented `own_memory!` macro in `wasi_virt_layer` allowing VFS modules to expand target memory.
    - Added `own_memory_lowering` and `check_range` stream passes to the CLI to support memory expansion.
- **Export Stack Handoff Isolation**:
    - Implemented export stack handoff design and ABI to protect exported Wasm functions from stack collisions when sharing linear memory.
    - Added `ExportStackArenaStreamPass` for multi-memory target arena isolation (including atomic slot acquire/release).
    - Integrated `ExportStackMultiMemoryTargetStreamPass` into the generator.
    - Added `configure_wasm_stack!` and `protect_wasm_exports!` macros along with CLI argument parsing for configuring stack sizes and isolation.
- **Threading & Execution**:
    - Improved thread ID generation using host thread IDs and a TLS counter to ensure unique IDs across thread pools, preventing Rayon TLS isolation issues.
    - Added WASI re-entrancy detection feature (`detect-wasi-reentrancy`) to trap synchronous re-entries of `non_recursive_wasi_snapshot_preview1!` calls via thread-local guards.
    - Added synthesis warnings when a `__main_void` entrypoint is generated.
- **Tests**:
    - Added integration tests for single memory virtualization, 4 argument passing, and `stderr_reentrancy_vfs`.
    - Added unit tests for memory growth edge cases and stack size skip logic in `ExportStackPreTargetStreamPass`.
    - Added unit tests for memory strip, localization, and renaming logic in `TemporaryRefugeMemoryStreamPass`.
    - Added comprehensive unit tests for `ExportStackMultiMemoryTargetStreamPass`.
    - Added component validation tests and `wit-component` trampoline regression tests.

### Fixed
- **Stack Export Passes**:
    - Fixed `CodeSectionStart` parsing by using `CodeSectionReader` with the correct range, resolving function body corruption in various export stack passes.
    - Corrected `TargetRebinder` shift offset in `ExportStackPreTargetStreamPass` to prevent incorrect function indices in `call` instructions.
    - Fixed incorrect function index calculation for `slot_acquire` in the stack ensure wrapper.
- **Memory Passes**:
    - Fixed `MultiMemoryLoweringStreamPass` function index rebinding and element section handling.
    - Fixed refuge pass by adding explicit `TypeSection` pre-scan and `CodeSectionStart` handler.
    - Unconditionally cleared the `shared` flag in the refuge pass.
- **Dependencies**:
    - Upgraded `wit-component` to 0.252.0 and `js-component-bindgen` to 2.0.1.

## [0.5.4] - 2026-06-11
### Added
- **Tests**:
    - Added `test_workspace_overwrite.rs` to verify workspace file preservation.
- **Documentation**:
    - Added `docs/plans/handoff_rustc_v_fix.md`.

### Changed
- **CLI Improvements**:
    - Refined Wasm file existence checks in the generator to allow non-existent files during certain build phases.
    - Improved ABI validation and command handling in the CLI tool.
- **Examples**:
    - Updated various examples with improved formatting and minor logic refinements.
- **Dependencies**:
    - Updated project version and dependencies via `cargo update`.

## [0.5.3] - 2026-06-11
### Added
- **Unwind Support**:
    - Added support for compiling Wasm modules with exception handling using `-Cpanic=unwind -Cllvm-args=-wasm-use-legacy-eh=false` and `-Zbuild-std=std,panic_unwind`.
    - Introduced `--wasm-unwind` and `--vfs-unwind` flags to the CLI tool to control unwind compatibility.
    - Added `test_unwind_target` and `test_unwind_vfs` examples demonstrating `std::panic::catch_unwind` behavior inside virtualized environments.
    - Implemented integration tests to verify successful builds with the new unwind flags.


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
