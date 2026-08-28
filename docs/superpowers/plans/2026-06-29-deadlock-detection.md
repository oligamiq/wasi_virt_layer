# Deadlock Detection Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add opt-in deadlock detection for threaded WASI builds by tracking wasm thread IDs and routing target/host atomic waits through a detector.

**Architecture:** The target pre-pipeline injects a mutable `i32` thread-id global and initializes it from `wasi_thread_start(thread_id, data_ptr)` or reserved main-thread id `0`. Atomic rewrite wrappers read that global and pass it to VFS detector hooks for wait/notify/write/RMW operations. Runtime hooks maintain a wait graph keyed by wasm thread id and atomic locations, while host atomic waits use finite slices so the detector can trap instead of hanging indefinitely.

**Tech Stack:** Rust 2024, `wasmparser`, `wasm-encoder`, WASI preview1 threads, `dashmap`, `parking_lot`, `flume`, CLI `clap`.

## Global Constraints

- Rust edition is 2024; minimum rustc is 1.89.0.
- Use `cargo check -r` for workspace checks.
- Use `cargo nextest run -r --fail-fast` when practical.
- Use `cargo clippy --all-targets --all-features -D warnings` for lint verification.
- Use `cargo fmt` after Rust edits.
- Do not change detector-disabled output behavior except where existing passes already rewrite atomics.
- Do not hook `root_spawn`; thread identity comes from `wasi_thread_start` and target entry wrappers.
- `0` is the reserved main-thread id; positive ids come from guest-visible `wasi_thread_start`.
- Host/runtime waits inside this repo must avoid indefinite raw `memory.atomic.wait*` when deadlock detection is enabled.

---

## File Structure

- Modify `wasi_virt_layer/Cargo.toml`: add the `detect-deadlock` feature.
- Modify `wasi_virt_layer-cli/src/args.rs`: add `--detect-deadlock` to build/prebuild args.
- Modify `wasi_virt_layer-cli/src/lib.rs`: auto-enable VFS crate feature from CLI flag.
- Modify `wasi_virt_layer-cli/src/generator/mod.rs`: carry `detect_deadlock` through generation and pass construction.
- Create `wasi_virt_layer-cli/src/wasm_stream/passes/deadlock_thread_id.rs`: append target thread-id global and wrap thread/main entry points.
- Modify `wasi_virt_layer-cli/src/wasm_stream/passes/mod.rs`: export the new pass.
- Modify `wasi_virt_layer-cli/src/wasm_stream/passes/atomic_patch.rs`: pass current thread id into detector hooks and rewrite wait/notify/write/RMW atomics when enabled.
- Modify `wasi_virt_layer-cli/src/wasm_stream/passes/post_combine.rs`: recognize detector helpers as special imports and ensure host wait helpers remain bounded when detector is enabled.
- Modify `wasi_virt_layer/src/wasi/thread.rs`: implement detector state, lifecycle hooks, wait hooks, notify/write/RMW hooks, and sliced waits.
- Add integration fixtures/tests under `wasi_virt_layer-cli/tests/` for trap and false-positive coverage.

---

### Task 1: Feature And CLI Gate

**Files:**
- Modify: `wasi_virt_layer/Cargo.toml`
- Modify: `wasi_virt_layer-cli/src/args.rs`
- Modify: `wasi_virt_layer-cli/src/lib.rs`
- Modify: `wasi_virt_layer-cli/src/generator/mod.rs`
- Test: `wasi_virt_layer-cli/src/args.rs` unit parse coverage if existing parser tests are nearby; otherwise verify with `cargo check -r`.

**Interfaces:**
- Produces: `GeneratorCtx { detect_deadlock: bool, .. }`.
- Produces: CLI flag `--detect-deadlock` on `build` and `prebuild`.
- Produces: VFS cargo feature string `wasi_virt_layer/detect-deadlock`.

- [ ] **Step 1: Write the failing compile change expectation**

Add the new field where `BuildArgs` and `PreBuildArgs` are defined:

```rust
/// Enable atomic wait deadlock detection for threaded builds.
#[arg(long, default_value = "false")]
pub detect_deadlock: bool,
```

Run: `cargo check -r`
Expected: FAIL because `GeneratorCtx` and command handling do not yet know this field.

- [ ] **Step 2: Add the crate feature**

In `wasi_virt_layer/Cargo.toml`, add:

```toml
detect-deadlock = ["threads"]
```

- [ ] **Step 3: Add CLI flag propagation**

In `wasi_virt_layer-cli/src/lib.rs`, after the existing `own_memory` feature insertion, add equivalent logic:

```rust
if build_args.detect_deadlock
    && !vfs_opts
        .features
        .iter()
        .any(|f| f == "wasi_virt_layer/detect-deadlock")
{
    vfs_opts
        .features
        .push("wasi_virt_layer/detect-deadlock".to_string());
}
```

Repeat the same block for `prebuild_args.detect_deadlock`.

- [ ] **Step 4: Add generator context field**

Add `detect_deadlock: bool` to `GeneratorCtx` and initialize it from parsed args wherever `GeneratorRunner` is constructed.

- [ ] **Step 5: Run compile check**

Run: `cargo check -r`
Expected: PASS or unrelated existing compile errors only. Any `detect_deadlock` missing-field error must be fixed in this task.

- [ ] **Step 6: Commit**

Run:

```bash
git add wasi_virt_layer/Cargo.toml wasi_virt_layer-cli/src/args.rs wasi_virt_layer-cli/src/lib.rs wasi_virt_layer-cli/src/generator/mod.rs
git commit -m "feat: add deadlock detection CLI gate"
```

---

### Task 2: Target Thread ID Global Pass

**Files:**
- Create: `wasi_virt_layer-cli/src/wasm_stream/passes/deadlock_thread_id.rs`
- Modify: `wasi_virt_layer-cli/src/wasm_stream/passes/mod.rs`
- Modify: `wasi_virt_layer-cli/src/generator/mod.rs`
- Test: unit tests in `deadlock_thread_id.rs`

**Interfaces:**
- Consumes: `detect_deadlock: bool` from `GeneratorCtx`.
- Produces: target mutable global named `__wvl_current_thread_id` with type `i32` and initial value `0`.
- Produces: custom section `wvl.deadlock_thread_id.v1` containing the appended global index as little-endian `u32`.
- Produces: wrappers that set the global before calling `wasi_thread_start`, `_start`, and `__main_void` when those exports exist.

- [ ] **Step 1: Write pass unit test for global injection**

Create a fixture wasm in the test with one exported `wasi_thread_start` and assert after running the pass:

```rust
assert!(has_custom_section(&output, "wvl.deadlock_thread_id.v1"));
assert!(has_mutable_i32_global(&output));
assert!(export_exists(&output, "wasi_thread_start"));
```

Run: `cargo test -p wasi_virt_layer-cli deadlock_thread_id --lib`
Expected: FAIL because the pass does not exist.

- [ ] **Step 2: Implement pass skeleton**

Define:

```rust
pub struct DeadlockThreadIdPreTargetStreamPass {
    enabled: bool,
}

impl DeadlockThreadIdPreTargetStreamPass {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
```

If `enabled` is false, return `input_wasm.to_vec()`.

- [ ] **Step 3: Append thread-id global and metadata**

Count imported globals and local globals. Append:

```rust
wasm_encoder::GlobalType {
    val_type: wasm_encoder::ValType::I32,
    mutable: true,
    shared: false,
}
```

with `ConstExpr::i32_const(0)`. Emit custom section `wvl.deadlock_thread_id.v1` with the global index.

- [ ] **Step 4: Wrap entry functions**

When exports exist:
- `wasi_thread_start`: wrapper params `(i32, i32)` stores local `0` to the global, calls original.
- `_start`: wrapper stores `0`, calls original.
- `__main_void`: wrapper stores `0`, calls original and returns original `i32`.

Preserve original functions by appending wrappers and rebinding exports to wrapper indices.

- [ ] **Step 5: Register pass**

Export the module in `passes/mod.rs` and insert the pass after `WrapUnreachablePreTargetStreamPass` and before `AtomicPatchStreamPass` in `generator/mod.rs`.

- [ ] **Step 6: Run pass tests**

Run: `cargo test -p wasi_virt_layer-cli deadlock_thread_id --lib`
Expected: PASS.

- [ ] **Step 7: Commit**

Run:

```bash
git add wasi_virt_layer-cli/src/wasm_stream/passes/deadlock_thread_id.rs wasi_virt_layer-cli/src/wasm_stream/passes/mod.rs wasi_virt_layer-cli/src/generator/mod.rs
git commit -m "feat: inject wasm thread id global"
```

---

### Task 3: Runtime Detector Skeleton And Sliced Waits

**Files:**
- Modify: `wasi_virt_layer/src/wasi/thread.rs`
- Test: `wasi_virt_layer/src/wasi/thread.rs` unit tests where target cfg allows pure detector helpers.

**Interfaces:**
- Produces: feature-gated module functions under `vfs_atomic`:
  - `__vfs_deadlock_thread_enter(thread_id: i32)`
  - `__vfs_deadlock_thread_exit(thread_id: i32)`
  - `record_wait_start(thread_id, wasm_id, addr, width, expected_display)` internal helper
  - `record_wait_end(thread_id)` internal helper
  - `record_location_change(thread_id, wasm_id, addr, width)` internal helper

- [ ] **Step 1: Write detector state unit tests**

Add tests for:

```rust
detector_records_wait_edge();
detector_clears_wait_edge_on_end();
location_generation_increments_on_notify_or_write();
```

Run: `cargo test -p wasi_virt_layer detector --lib`
Expected: FAIL because helpers do not exist.

- [ ] **Step 2: Implement detector data structures**

Behind `#[cfg(feature = "detect-deadlock")]`, define:

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct AtomicLocation {
    wasm_id: u32,
    relative_addr: u32,
    width: u8,
}

#[derive(Clone, Copy, Debug)]
struct ThreadWait {
    location: AtomicLocation,
    expected_low: u64,
    generation_at_wait: u64,
}
```

Use `DashMap<i32, ThreadWait>` for active waiters and `DashMap<AtomicLocation, AtomicU64>` or a lock-protected map for generations.

- [ ] **Step 3: Add sliced wait constants**

Use:

```rust
const DEADLOCK_WAIT_SLICE_NS: i64 = 50_000_000;
```

Detector-disabled code keeps current direct wait behavior.

- [ ] **Step 4: Run tests**

Run: `cargo test -p wasi_virt_layer detector --lib`
Expected: PASS.

- [ ] **Step 5: Commit**

Run:

```bash
git add wasi_virt_layer/src/wasi/thread.rs
git commit -m "feat: add deadlock detector state"
```

---

### Task 4: Atomic Patch Thread ID And Wait/Notify Hooks

**Files:**
- Modify: `wasi_virt_layer-cli/src/wasm_stream/passes/atomic_patch.rs`
- Modify: `wasi_virt_layer/src/wasi/thread.rs`
- Test: `atomic_patch.rs` unit tests.

**Interfaces:**
- Consumes: custom section `wvl.deadlock_thread_id.v1` for global index.
- Produces detector-enabled import signatures:
  - `__vfs_atomic_wait32(i32 thread_id, i32 wasm_id, i32 addr, i32 expected, i64 timeout) -> i32`
  - `__vfs_atomic_wait64(i32 thread_id, i32 wasm_id, i32 addr, i64 expected, i64 timeout) -> i32`
  - `__vfs_atomic_notify(i32 thread_id, i32 wasm_id, i32 addr, i32 count) -> i32`

- [ ] **Step 1: Write rewrite tests**

Use a WAT fixture with wait32, wait64, notify. Assert detector-enabled output imports extended signatures and wrapper bodies contain `global.get` for the thread-id global.

Run: `cargo test -p wasi_virt_layer-cli atomic_patch --lib`
Expected: FAIL.

- [ ] **Step 2: Add `detect_deadlock` to pass constructor**

Change constructor to:

```rust
pub fn new(threads: bool, target_index: u32, detect_deadlock: bool) -> Self
```

Update call sites.

- [ ] **Step 3: Parse thread-id global metadata**

Read `wvl.deadlock_thread_id.v1` when `detect_deadlock` is true. If missing, return an error explaining that deadlock thread-id pass must run before atomic patch.

- [ ] **Step 4: Emit detector-enabled wait/notify wrappers**

For wait32 wrapper body:

```wasm
global.get $__wvl_current_thread_id
i32.const <target_id>
local.get 0
i32.const <offset>
i32.add
local.get 1
local.get 2
call $__vfs_atomic_wait32
end
```

Use equivalent order for wait64 and notify.

- [ ] **Step 5: Update runtime function signatures under feature**

In `thread.rs`, provide detector-enabled versions that accept `thread_id` first. Keep existing signatures when feature is disabled to avoid changing existing generated wasm.

- [ ] **Step 6: Run tests**

Run: `cargo test -p wasi_virt_layer-cli atomic_patch --lib`
Expected: PASS.

- [ ] **Step 7: Commit**

Run:

```bash
git add wasi_virt_layer-cli/src/wasm_stream/passes/atomic_patch.rs wasi_virt_layer/src/wasi/thread.rs wasi_virt_layer-cli/src/generator/mod.rs
git commit -m "feat: pass wasm thread ids to atomic hooks"
```

---

### Task 5: Hook Atomic Writes And RMW Operations

**Files:**
- Modify: `wasi_virt_layer-cli/src/wasm_stream/passes/atomic_patch.rs`
- Modify: `wasi_virt_layer/src/wasi/thread.rs`
- Test: `atomic_patch.rs` unit tests for at least one store, one cmpxchg, one RMW add.

**Interfaces:**
- Produces detector observer hooks that preserve original atomic semantics and return values.
- Produces location generation updates for atomic writes/RMWs.

- [ ] **Step 1: Write failing tests for store/RMW rewrite**

Use WAT with:
- `i32.atomic.store`
- `i64.atomic.store`
- `i32.atomic.rmw.cmpxchg`
- `i32.atomic.rmw.add`

Assert rewritten module validates and imports detector hooks.

- [ ] **Step 2: Add wrapper generation with locals**

For multi-operand atomics, store operands into synthetic locals in wrapper functions so the call order preserves wasm semantics. For cmpxchg, wrapper must return the original old value.

- [ ] **Step 3: Implement runtime observer hooks**

After performing the underlying atomic operation or after observing that the wrapper executed the original op, call `record_location_change(thread_id, wasm_id, addr, width)`.

- [ ] **Step 4: Cover 8/16-bit variants**

Add rewrite cases for atomic store/RMW 8-bit and 16-bit operators exposed by `wasmparser::Operator`.

- [ ] **Step 5: Run validation tests**

Run: `cargo test -p wasi_virt_layer-cli atomic_patch --lib`
Expected: PASS.

- [ ] **Step 6: Commit**

Run:

```bash
git add wasi_virt_layer-cli/src/wasm_stream/passes/atomic_patch.rs wasi_virt_layer/src/wasi/thread.rs
git commit -m "feat: observe atomic writes for deadlock detection"
```

---

### Task 6: Host Wait Coverage And Post-Combine Imports

**Files:**
- Modify: `wasi_virt_layer-cli/src/wasm_stream/passes/post_combine.rs`
- Modify: `wasi_virt_layer/src/wasi/thread.rs`
- Test: post-combine unit tests.

**Interfaces:**
- Consumes: detector helper import names from atomic patch.
- Produces: post-combine special import handling for detector helpers.
- Produces: no unbounded raw host wait path for detector-enabled VFS atomic waits.

- [ ] **Step 1: Write failing post-combine test**

Add a fixture with detector imports and assert post-combine does not treat them as unresolved host imports.

- [ ] **Step 2: Update special import allowlist**

In `PostCombineStreamPass`, mark detector helpers as special imports when module/name match the generated detector hook names.

- [ ] **Step 3: Ensure sliced waits are used**

In detector-enabled `__vfs_atomic_wait32/64`, loop over finite slices and call `__wvl_atomic_wait32_vfs` with `min(remaining, DEADLOCK_WAIT_SLICE_NS)` or the fixed slice for infinite waits.

- [ ] **Step 4: Run post-combine tests**

Run: `cargo test -p wasi_virt_layer-cli post_combine --lib`
Expected: PASS.

- [ ] **Step 5: Commit**

Run:

```bash
git add wasi_virt_layer-cli/src/wasm_stream/passes/post_combine.rs wasi_virt_layer/src/wasi/thread.rs
git commit -m "feat: route host waits through deadlock detector"
```

---

### Task 7: Deadlock Detection Semantics

**Files:**
- Modify: `wasi_virt_layer/src/wasi/thread.rs`
- Test: detector unit tests.

**Interfaces:**
- Produces: panic message beginning with `deadlock detected`.
- Produces: graph evaluation using active threads, wait edges, and location generations.

- [ ] **Step 1: Write closed-component failing test**

Create detector state with two running threads, both waiting on unchanged locations, no generation changes across two checks. Assert `check_deadlock()` returns a diagnostic containing both thread ids.

- [ ] **Step 2: Write non-deadlock generation-change test**

Create a waiter, increment its location generation, then assert `check_deadlock()` does not report deadlock.

- [ ] **Step 3: Implement graph check**

Implement closed observed component rule:
- active observed threads are either running or waiting;
- if any active observed thread is running, do not panic;
- if all active observed threads are waiting and every waited location generation is unchanged across two checks, report deadlock.

- [ ] **Step 4: Add diagnostic formatting**

Include thread ids, wasm ids, addresses, widths, expected values, and generations.

- [ ] **Step 5: Run detector tests**

Run: `cargo test -p wasi_virt_layer detector --lib`
Expected: PASS.

- [ ] **Step 6: Commit**

Run:

```bash
git add wasi_virt_layer/src/wasi/thread.rs
git commit -m "feat: detect closed atomic wait deadlocks"
```

---

### Task 8: Integration Tests

**Files:**
- Create: `wasi_virt_layer-cli/tests/test_deadlock_detection.rs`
- Create fixtures as needed under `wasi_virt_layer-cli/tests/fixtures/`
- Modify: `Cargo.toml` if new fixture crates are workspace members.

**Interfaces:**
- Consumes: CLI `--detect-deadlock`.
- Produces: integration coverage that traps before test timeout.

- [ ] **Step 1: Add deadlock fixture**

Create a target that spawns two threads. Both threads execute `memory.atomic.wait32` forever on unchanged values. No notify/write occurs.

- [ ] **Step 2: Add false-positive fixture**

Create a target where worker waits and main thread later stores and notifies. This must finish successfully with detection enabled.

- [ ] **Step 3: Add integration tests**

Use `run_wasi_virt_layer(..., & ["--validate", "--detect-deadlock"], Some(Duration::from_secs(20)))`.

Assert deadlock test fails with output containing `deadlock detected`, not `Process timed out`.

Assert false-positive test succeeds and prints a completion marker.

- [ ] **Step 4: Run targeted integration tests**

Run: `cargo nextest run -r -p wasi_virt_layer-cli test_deadlock_detection --fail-fast`
Expected: PASS.

- [ ] **Step 5: Commit**

Run:

```bash
git add Cargo.toml wasi_virt_layer-cli/tests/test_deadlock_detection.rs wasi_virt_layer-cli/tests/fixtures
git commit -m "test: cover atomic deadlock detection"
```

---

### Task 9: Full Verification

**Files:**
- No intended source changes except formatting.

**Interfaces:**
- Consumes: all previous tasks.
- Produces: verified workspace.

- [ ] **Step 1: Format**

Run: `cargo fmt`
Expected: no errors.

- [ ] **Step 2: Check**

Run: `cargo check -r`
Expected: PASS.

- [ ] **Step 3: Targeted tests**

Run:

```bash
cargo nextest run -r -p wasi_virt_layer-cli test_deadlock_detection --fail-fast
cargo nextest run -r -p wasi_virt_layer-cli test_atomic_wait_reset --fail-fast
cargo nextest run -r -p wasi_virt_layer-cli test_pool_thread_reinitialization --fail-fast
```

Expected: PASS or skipped only for missing WASI targets.

- [ ] **Step 4: Clippy**

Run: `cargo clippy --all-targets --all-features -D warnings`
Expected: PASS.

- [ ] **Step 5: Commit verification fixes**

If formatting or lint fixes changed files, run:

```bash
git add .
git commit -m "chore: verify deadlock detection"
```

---

## Self-Review

- Spec coverage: feature/CLI gate is Task 1; thread identity via `wasi_thread_start` and globals is Task 2; target/host wait hooks are Tasks 4 and 6; broad atomic write observation is Task 5; detector semantics are Task 7; tests are Task 8; verification is Task 9.
- Placeholder scan: no TBD/TODO placeholders are intentionally left.
- Type consistency: `detect_deadlock`, `__wvl_current_thread_id`, and `wvl.deadlock_thread_id.v1` names are consistent across tasks.
