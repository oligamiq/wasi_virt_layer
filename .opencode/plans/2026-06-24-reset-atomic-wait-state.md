# Reset Atomic Wait State on _reset Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Clear `memory.atomic.wait32`/`wait64`/`notify` emulation state (the `WAIT_MAP`) for a target when its `_reset()` is called, so stale wait-queue entries from a previous run do not interfere with re-runs.

**Architecture:** Add a `#[no_mangle]` Rust function `__vfs_atomic_reset_target(wasm_id)` in `wasi_virt_layer/src/wasi/thread.rs` that (1) notifies all parked threads on the target's wait cells (removing them from the runtime's wait queue), then (2) removes those cells from the global `WAIT_MAP`. Emit a call to this function from the generated `_reset` body in `post_combine.rs`, passing `wasm_id = target_index` (the same index `AtomicPatchStreamPass` assigns).

**Tech Stack:** Rust (edition 2024), wasm-encoder, wasmparser, WAT (for test target), deno (test runner).

## Global Constraints

- Rust edition 2024; minimum rustc 1.89.0 (`Cargo.toml`).
- Workspace members: `wasi_virt_layer` (lib) and `wasi_virt_layer-cli` (CLI).
- Check: `cargo check -r`. Lint: `cargo clippy --all-targets --all-features -D warnings`. Format: `cargo fmt`.
- Test: `cargo nextest run -r --fail-fast` (install via `cargo binstall cargo-nextest -y`).
- Integration tests require `wasm32-wasip1` and `wasm32-wasip1-threads` rustup targets; tests no-op via `has_required_wasi_targets` if absent.
- Doc comments: add as many as possible (AGENTS.md convention).
- No comments in code unless explicitly asked (AGENTS.md convention).
- The `WAIT_MAP` key layout is `((wasm_id as u64) << 32) | (relative_addr as u64)` (`thread.rs:1218`).
- `AtomicPatchStreamPass` assigns `target_id = self.target_index` (0-based) as `wasm_id` (`atomic_patch.rs:509`).
- In `post_combine.rs`, `wasm_mem = target_index + 1` (1-based memory index), so `wasm_id = wasm_mem - 1`.
- `__wvl_atomic_notify_vfs(ptr, count)` is raw `MemoryAtomicNotify` on VFS memory (Memory 0) — it does NOT modify `*ptr`, only wakes parked threads (`post_combine.rs:1270-1279`).
- The `_reset` body in `post_combine.rs` executes in this order: (0) [NEW] atomic reset, (1) reset_globals, (2) memory.fill(0), (3) memory.init, (4) flesh_target_start.
- `_reset` is NOT thread-safe (documented at `memory.rs:1228-1244`). Callers must ensure threads have unwound or accept the consequences.

---

## File Structure

| File | Action | Responsibility |
|------|--------|----------------|
| `wasi_virt_layer/src/wasi/thread.rs` | Modify | Add `__vfs_atomic_reset_target` function in `vfs_atomic` mod |
| `wasi_virt_layer-cli/src/wasm_stream/passes/post_combine.rs` | Modify | Emit call to `__vfs_atomic_reset_target` in `_reset` body |
| `wasi_virt_layer-cli/tests/fixtures/atomic_wait_reset_vfs/Cargo.toml` | Create | Test VFS crate manifest |
| `wasi_virt_layer-cli/tests/fixtures/atomic_wait_reset_vfs/wit/main.wit` | Create | Test VFS WIT world |
| `wasi_virt_layer-cli/tests/fixtures/atomic_wait_reset_vfs/src/lib.rs` | Create | Test VFS: `_start()` -> `_reset()` -> `_start()` -> `_main()` |
| `wasi_virt_layer-cli/tests/test_atomic_wait_reset.rs` | Create | Integration test: builds WAT target, runs VFS, asserts output |

---

## Task 1: Add `__vfs_atomic_reset_target` to `vfs_atomic` mod

**Files:**
- Modify: `wasi_virt_layer/src/wasi/thread.rs` (after line 1271, end of `__vfs_atomic_notify`, before the module closing `}`)

**Interfaces:**
- Produces: `pub unsafe extern "C" fn __vfs_atomic_reset_target(wasm_id: u32)` — a `#[no_mangle]` export callable from wasm. Notifies all parked threads on cells belonging to `wasm_id`, then removes those cells from `WAIT_MAP`.

- [ ] **Step 1: Write the failing test**

Add a `#[cfg(test)]` submodule inside the `vfs_atomic` module (after `__vfs_atomic_notify` at line 1271, before the module closing `}`):

```rust
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn reset_target_removes_only_matching_wasm_id_entries() {
            WAIT_MAP.clear();
            let key_a = (1u64 << 32) | 0x100;
            let key_b = (1u64 << 32) | 0x200;
            let key_c = (2u64 << 32) | 0x100;
            WAIT_MAP.insert(key_a, Box::new(0));
            WAIT_MAP.insert(key_b, Box::new(0));
            WAIT_MAP.insert(key_c, Box::new(0));
            assert_eq!(WAIT_MAP.len(), 3);

            unsafe { __vfs_atomic_reset_target(1) };

            assert!(!WAIT_MAP.contains_key(&key_a));
            assert!(!WAIT_MAP.contains_key(&key_b));
            assert!(WAIT_MAP.contains_key(&key_c));
            assert_eq!(WAIT_MAP.len(), 1);

            WAIT_MAP.clear();
        }
    }
```

- [ ] **Step 2: Run test to verify it fails (compilation error)**

Run: `cargo nextest run -r -p wasi_virt_layer thread::vfs_atomic::tests`
Expected: FAIL — `cannot find function __vfs_atomic_reset_target`

- [ ] **Step 3: Write minimal implementation**

Add after `__vfs_atomic_notify` (line 1271), inside the `vfs_atomic` module, before the module closing `}`:

```rust
    /// Resets the atomic-wait state for a single target, freeing its wait cells.
    ///
    /// Called from the generated `_reset` body. For each wait cell belonging to
    /// `wasm_id`, this first notifies (wakes) all threads parked on the cell so
    /// they are removed from the runtime's wait queue, then removes the cell from
    /// `WAIT_MAP`, freeing the VFS memory it occupied.
    ///
    /// This prevents a stale wait queue entry from being woken when the target is
    /// re-run after reset: without the notify, a zombie thread parked on a
    /// reused address would be woken instead of the new thread.
    ///
    /// # Safety
    ///
    /// Must only be called when no thread is still blocked on those cells -- i.e.
    /// under the same preconditions as `_reset()`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __vfs_atomic_reset_target(wasm_id: u32) {
        let cells: Vec<(u64, *const u32)> = WAIT_MAP
            .iter()
            .filter(|e| (*e.key() >> 32) as u32 == wasm_id)
            .map(|e| (*e.key(), &**e.value() as *const u32))
            .collect();

        for (_, ptr) in &cells {
            unsafe { __wvl_atomic_notify_vfs(*ptr, u32::MAX) };
        }

        for (key, _) in cells {
            WAIT_MAP.remove(&key);
        }
    }
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo nextest run -r -p wasi_virt_layer thread::vfs_atomic::tests`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add wasi_virt_layer/src/wasi/thread.rs
git commit -m "feat: add __vfs_atomic_reset_target to clear wait state per target"
```

---

## Task 2: Emit `__vfs_atomic_reset_target` call from `_reset` body

**Files:**
- Modify: `wasi_virt_layer-cli/src/wasm_stream/passes/post_combine.rs` (inside the `else if name.ends_with("_reset")` branch, after `wasm_mem` is computed at line 1352, before "1. Reset globals" at line 1354)

**Interfaces:**
- Consumes: `__vfs_atomic_reset_target(wasm_id: u32)` — the `#[no_mangle]` export from Task 1.
- Consumes: `info.exported_funcs: HashMap<String, u32>` (line 268) — collects all func exports by name; `__vfs_atomic_reset_target` will be found here because it is a VFS-crate `#[no_mangle]` export.
- Consumes: `rebinder.function(idx)` — maps original func index to new index.
- Consumes: `wasm_mem: u32` — the target's memory index (1-based); `wasm_id = wasm_mem - 1`.

- [ ] **Step 1: Read the existing test helpers in post_combine.rs**

Read `post_combine.rs` lines 1433-1550 to understand the existing test patterns (e.g. `reset_data_segments_are_scoped_to_the_target_memory`). Identify available helper functions like `module_with_flesh_vfs_start()`.

- [ ] **Step 2: Write the failing test**

Add a test to the `#[cfg(test)] mod tests` block in `post_combine.rs`. The test verifies that the generated `_reset` function body contains a `Call` instruction (to `__vfs_atomic_reset_target`).

```rust
    #[test]
    fn reset_body_calls_atomic_reset_target() -> eyre::Result<()> {
        let input = module_with_flesh_vfs_start_and_reset();
        let mut pass = PostCombineStreamPass::new(
            "vfs".to_string(),
            vec!["test_target".to_string()],
            vec![1],
            true,
        );
        let output = pass.run(&input)?;

        let mut reset_func_idx = None;
        let mut atomic_reset_idx = None;
        let mut found_call_to_atomic_reset = false;

        for payload in wasmparser::Parser::new(0).parse_all(&output) {
            match payload? {
                wasmparser::Payload::ExportSection(exports) => {
                    for export in exports {
                        let export = export?;
                        if export.name == "__wasip1_vfs_test_target_reset" {
                            reset_func_idx = Some(export.index);
                        }
                        if export.name == "__vfs_atomic_reset_target" {
                            atomic_reset_idx = Some(export.index);
                        }
                    }
                }
                wasmparser::Payload::CodeSectionEntry(body) => {
                    if let Some(_) = reset_func_idx {
                        let ops = body
                            .get_operators_reader()?
                            .into_iter()
                            .collect::<Result<Vec<_>, _>>()?;
                        for op in &ops {
                            if let wasmparser::Operator::Call { function_index } = op {
                                if let Some(ari) = atomic_reset_idx {
                                    if *function_index == ari {
                                        found_call_to_atomic_reset = true;
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        assert!(reset_func_idx.is_some(), "_reset export not found");
        assert!(
            found_call_to_atomic_reset,
            "_reset body should call __vfs_atomic_reset_target"
        );
        Ok(())
    }

    fn module_with_flesh_vfs_start_and_reset() -> Vec<u8> {
        let mut module = wasm_encoder::Module::new();

        let mut types = wasm_encoder::TypeSection::new();
        types.ty().function([], []);
        types.ty().function([wasm_encoder::ValType::I32], []);
        module.section(&types);

        let mut functions = wasm_encoder::FunctionSection::new();
        functions.function(0);
        functions.function(1);
        module.section(&functions);

        let mut exports = wasm_encoder::ExportSection::new();
        exports.export(
            "__wasip1_vfs_test_target_reset",
            wasm_encoder::ExportKind::Func,
            0,
        );
        exports.export(
            "__vfs_atomic_reset_target",
            wasm_encoder::ExportKind::Func,
            1,
        );
        module.section(&exports);

        let mut code = wasm_encoder::CodeSection::new();
        let mut reset_func = wasm_encoder::Function::new([]);
        reset_func.instruction(&wasm_encoder::Instruction::End);
        code.function(&reset_func);
        let mut atomic_func = wasm_encoder::Function::new([wasm_encoder::ValType::I32]);
        atomic_func.instruction(&wasm_encoder::Instruction::End);
        code.function(&atomic_func);
        module.section(&code);

        module.finish().to_vec()
    }
```

- [ ] **Step 3: Run test to verify it fails**

Run: `cargo nextest run -r -p wasi_virt_layer-cli reset_body_calls_atomic_reset_target`
Expected: FAIL — the `_reset` body is just `End`, no `Call` to `__vfs_atomic_reset_target`.

- [ ] **Step 4: Write minimal implementation**

In `post_combine.rs`, inside the `else if name.ends_with("_reset")` branch, after the `wasm_mem` computation (line 1352, after `+ 1;`) and before the `// 1. Reset globals` comment (line 1354), insert:

```rust
                if let Some(&reset_atomic_idx) =
                    info.exported_funcs.get("__vfs_atomic_reset_target")
                {
                    func.instruction(&wasm_encoder::Instruction::I32Const(
                        wasm_mem as i32 - 1,
                    ));
                    func.instruction(&wasm_encoder::Instruction::Call(
                        rebinder.function(reset_atomic_idx),
                    ));
                }
```

- [ ] **Step 5: Run test to verify it passes**

Run: `cargo nextest run -r -p wasi_virt_layer-cli reset_body_calls_atomic_reset_target`
Expected: PASS

- [ ] **Step 6: Run clippy and check**

Run: `cargo clippy --all-targets --all-features -D warnings`
Run: `cargo check -r`
Expected: No warnings, no errors.

- [ ] **Step 7: Commit**

```bash
git add wasi_virt_layer-cli/src/wasm_stream/passes/post_combine.rs
git commit -m "feat: emit __vfs_atomic_reset_target call in _reset body"
```

---

## Task 3: Create test VFS fixture

**Files:**
- Create: `wasi_virt_layer-cli/tests/fixtures/atomic_wait_reset_vfs/Cargo.toml`
- Create: `wasi_virt_layer-cli/tests/fixtures/atomic_wait_reset_vfs/wit/main.wit`
- Create: `wasi_virt_layer-cli/tests/fixtures/atomic_wait_reset_vfs/src/lib.rs`

**Interfaces:**
- Produces: A VFS crate named `atomic_wait_reset_vfs` that imports `atomic_wait_reset_target` wasm, plugs threads/poll/clock/process/env/fs, and exposes a `main()` that calls `_start()`, `_reset()`, `_start()`, `_main()` and prints `"Atomic wait reset test passed"`.

- [ ] **Step 1: Create Cargo.toml**

Create `wasi_virt_layer-cli/tests/fixtures/atomic_wait_reset_vfs/Cargo.toml`:

```toml
[package]
name = "atomic_wait_reset_vfs"
version = "0.0.1"
edition = "2024"
publish = false

[dependencies]
wit-bindgen = "0.54.0"
const_struct = "0.6.4"

[dependencies.wasi_virt_layer]
path = "../../../../wasi_virt_layer"
default-features = false
features = ["threads", "unstable_print_debug", "embedded-fs"]

[lib]
crate-type = ["cdylib"]
```

- [ ] **Step 2: Create WIT file**

Create `wasi_virt_layer-cli/tests/fixtures/atomic_wait_reset_vfs/wit/main.wit`:

```wit
package tests:atomic-wait-reset;

world component-abi {
  export main: func();
}
```

- [ ] **Step 3: Create VFS lib.rs**

Create `wasi_virt_layer-cli/tests/fixtures/atomic_wait_reset_vfs/src/lib.rs`:

```rust
use const_struct::const_struct;
use wasi_virt_layer::{
    file::*, plug_thread, poll::*, prelude::*, process::*, thread::VirtualThreadPool,
};

struct ComponentABI;

wit_bindgen::generate!({
    world: "component-abi",
});

import_wasm!(atomic_wait_reset_target);

static THREAD_POOL: VirtualThreadPool<ThreadAccessor> =
    unsafe { VirtualThreadPool::new_const(2) };

impl Guest for ComponentABI {
    fn main() {
        unsafe { THREAD_POOL.init_with_capacity_and_wait(2) };
        println!("Starting atomic wait reset test");
        atomic_wait_reset_target::_start();
        atomic_wait_reset_target::_reset();
        atomic_wait_reset_target::_start();
        atomic_wait_reset_target::_main();
        println!("Atomic wait reset test passed");
    }
}

#[cfg(not(test))]
export!(ComponentABI);

plug_thread!({ &THREAD_POOL }, atomic_wait_reset_target, self);
plug_poll!(DefaultWaitPoll, atomic_wait_reset_target);

wasi_virt_layer::plug_clock!(
    wasi_virt_layer::clock::StandardClock,
    atomic_wait_reset_target,
    self
);

mod process {
    use super::*;
    plug_process!(
        wasi_virt_layer::process::StandardProcess,
        atomic_wait_reset_target,
        self
    );
}

mod env {
    use super::*;
    #[const_struct]
    const HOST_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
        environ: &["HOME=~/"],
    };
    plug_env!(@embedded, HostEnvTy, atomic_wait_reset_target, self);
}

mod fs {
    use super::*;
    const FILE_COUNT: usize = 2;
    #[const_struct]
    const EMBEDDED_FILES: StandardEmbeddedFiles<WasiEmbeddedFile<&'static str>, { FILE_COUNT }> =
        EmbeddedFiles!([("/", [("dummy.txt", WasiEmbeddedFile::new("dummy"))])]);

    type LFS = StandardEmbeddedNormalLFS<
        EmbeddedFilesTy,
        WasiEmbeddedFile<&'static str>,
        FILE_COUNT,
        DefaultStdIO,
    >;

    static VIRTUAL_FILE_SYSTEM: StandardEmbeddedFileSystem<LFS, FILE_COUNT> =
        StandardEmbeddedFileSystem::new_const(StandardEmbeddedNormalLFS::new_const());

    plug_fs!(&VIRTUAL_FILE_SYSTEM, atomic_wait_reset_target, self);
}
```

**Key design decisions:**
- Thread pool capacity = 2: one for the zombie worker from run 1, one for the new worker. The pool auto-expands if needed.
- `main()` calls `_start()` (run 1: worker parks, `_start` returns), `_reset()` (fix: notify zombie + remove cell; re-runs `_start` which spawns a new worker), `_start()` (run 2: spawns another worker, parks, `_start` returns), `_main()` (notify worker, wait for done signal).
- The `_reset()` re-runs `_start` internally (step 4 of `_reset` body), so there are 3 `_start` executions total. The VFS's explicit `_start()` after `_reset()` is the "real" run that `_main()` completes the handshake with.

- [ ] **Step 4: Commit**

```bash
git add wasi_virt_layer-cli/tests/fixtures/atomic_wait_reset_vfs/
git commit -m "test: add atomic_wait_reset_vfs fixture"
```

---

## Task 4: Create integration test

**Files:**
- Create: `wasi_virt_layer-cli/tests/test_atomic_wait_reset.rs`

**Interfaces:**
- Consumes: `utils::run_wasi_virt_layer` (from `tests/utils.rs`) — builds and runs the VFS.
- Consumes: `utils::has_required_wasi_targets` (from `tests/utils.rs`) — checks rustup targets.
- Consumes: The `atomic_wait_reset_vfs` fixture from Task 3.

**WAT Target Design:**

The WAT target uses a cookie-based mechanism to distinguish zombie threads (woken by `_reset`'s atomic reset) from fresh threads:

- **addr 0**: handshake state (0=initial, 1=worker ready, 3=worker done)
- **addr 4**: park address / "go" signal (0=initial, 2=go)
- **addr 16**: cookie (0xDEAD=valid, 0=zeroed by _reset)
- Worker parks on `wait32(addr 4, expected=0, timeout=-1)` — infinite wait, no loop
- After being woken, worker checks cookie at addr 16: if `0xDEAD`, signals done; if `0` (zeroed by _reset), silently exits
- `_start`: spawns worker, waits for "ready" signal (addr 0 == 1), returns WITHOUT sending "go"
- `__main_void`: sends "go" (store 2 at addr 4, notify addr 4), waits for "done" (addr 0 == 3)

**Why this fails without the fix (and passes with it):**

Without fix: `_reset` does not clear `WAIT_MAP`. The zombie from run 1 remains in the wait queue at the cell for `(wasm_id, addr 4)`. When `_main` calls `notify(addr 4, 1)`, the runtime wakes the zombie (FIFO) instead of the new worker. The zombie checks its cookie (zeroed by `_reset`'s `memory.fill(0)`), silently exits. The new worker stays parked. `_main` hangs waiting for "done" -> test times out -> FAIL.

With fix: `_reset` calls `__vfs_atomic_reset_target(wasm_id)` which notifies the zombie (removing it from the wait queue) and removes the cell. The new worker gets a fresh cell. `_main`'s notify wakes the new worker. The new worker checks its cookie (intact: `0xDEAD`), signals done. `_main` sees "done" -> PASS.

**Scheduling assumption:** In deno's WASI implementation, the woken zombie is not scheduled until the main thread yields (cooperative). By the time the zombie runs, `_reset`'s `memory.fill(0)` has already zeroed the cookie. This is a reasonable assumption for the deno runtime used by the test harness.

- [ ] **Step 1: Create the test file**

Create `wasi_virt_layer-cli/tests/test_atomic_wait_reset.rs`:

```rust
pub mod utils;
use utils::*;

const ATOMIC_WAIT_RESET_TARGET_WAT: &str = r#"
(module
  (import "env" "__wasip1_vfs_wasi_thread_spawn_wrapper"
    (func $thread_spawn (param i32) (result i32)))

  (memory (export "memory") 1 1 shared)

  ;; addr 0:  handshake state (0=init, 1=ready, 3=done)
  ;; addr 4:  park address / go signal (0=init, 2=go)
  ;; addr 16: cookie (0xDEAD=valid, 0=zeroed by reset)

  (func $start
    (i32.atomic.store align=4 (i32.const 0) (i32.const 0))
    (i32.atomic.store align=4 (i32.const 4) (i32.const 0)))
  (start $start)

  (func $_start (export "_start")
    (i32.atomic.store align=4 (i32.const 0) (i32.const 0))
    (i32.atomic.store align=4 (i32.const 4) (i32.const 0))
    (drop (call $thread_spawn (i32.const 4)))
    (loop $wait_ready
      (if
        (i32.ne
          (i32.atomic.load align=4 (i32.const 0))
          (i32.const 1))
        (then
          (drop
            (memory.atomic.wait32 align=4
              (i32.const 0)
              (i32.atomic.load align=4 (i32.const 0))
              (i64.const 100000000)))
          (br $wait_ready)))))

  (func $__main_void (export "__main_void") (result i32)
    (i32.atomic.store align=4 (i32.const 4) (i32.const 2))
    (drop (memory.atomic.notify (i32.const 4) (i32.const 1)))
    (loop $wait_done
      (if
        (i32.ne
          (i32.atomic.load align=4 (i32.const 0))
          (i32.const 3))
        (then
          (drop
            (memory.atomic.wait32 align=4
              (i32.const 0)
              (i32.atomic.load align=4 (i32.const 0))
              (i64.const 100000000)))
          (br $wait_done))))
    (i32.const 0))

  (func $wasi_thread_start (export "wasi_thread_start")
    (param $thread_id i32)
    (param $start_arg i32)
    (i32.store align=4 (i32.const 16) (i32.const 0xDEAD))
    (i32.atomic.store align=4 (i32.const 0) (i32.const 1))
    (drop (memory.atomic.notify (i32.const 0) (i32.const 1)))
    (drop
      (memory.atomic.wait32 align=4
        (local.get $start_arg)
        (i32.const 0)
        (i64.const -1)))
    (if
      (i32.eq
        (i32.load align=4 (i32.const 16))
        (i32.const 0xDEAD))
      (then
        (i32.atomic.store align=4 (i32.const 0) (i32.const 3))
        (drop (memory.atomic.notify (i32.const 0) (i32.const 1)))))))
"#;

#[test]
fn test_atomic_wait_reset() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    if !has_required_wasi_targets(true) {
        return Ok(());
    }

    let target_dir = tempfile::tempdir()?;
    let target_path = target_dir.path().join("atomic_wait_reset_target.wasm");
    std::fs::write(&target_path, wat::parse_str(ATOMIC_WAIT_RESET_TARGET_WAT)?)?;
    let target_path = target_path
        .to_str()
        .ok_or_else(|| color_eyre::eyre::eyre!("target path is not UTF-8"))?;

    let dir = run_wasi_virt_layer(
        Some("atomic_wait_reset_vfs"),
        Some(target_path),
        None,
        true,
        OutDir::Random,
        false,
        &["--validate"],
        None,
    )?;

    let stdout = std::fs::read_to_string(dir.0.join(".deno-test-stdout.log"))?;
    println!("Captured stdout:\n{}", stdout);

    assert!(stdout.contains("Starting atomic wait reset test"));
    assert!(stdout.contains("Atomic wait reset test passed"));

    Ok(())
}
```

- [ ] **Step 2: Run the test (expect PASS with fix from Tasks 1+2)**

Run: `cargo nextest run -r -p wasi_virt_layer-cli test_atomic_wait_reset`
Expected: PASS (if wasm targets installed) or no-op (if not).

Note: If the test times out, it means either the fix is not working or the scheduling assumption does not hold for deno. Debug by:
1. Adding `--keep-build-artifacts` to inspect the generated wasm.
2. Using `wasm-tools print` on the generated wasm to verify `__vfs_atomic_reset_target` is called in `_reset`.
3. Checking if the zombie thread interferes despite the fix.

- [ ] **Step 3: Verify the test FAILS without the fix**

Temporarily comment out the `__vfs_atomic_reset_target` call in `post_combine.rs` (Task 2 Step 4 code), rebuild, and run:

Run: `cargo nextest run -r -p wasi_virt_layer-cli test_atomic_wait_reset`
Expected: FAIL (timeout — `_main` hangs because notify wakes zombie instead of new worker).

Then restore the fix.

- [ ] **Step 4: Run full clippy and check**

Run: `cargo clippy --all-targets --all-features -D warnings`
Run: `cargo check -r`
Expected: No warnings, no errors.

- [ ] **Step 5: Commit**

```bash
git add wasi_virt_layer-cli/tests/test_atomic_wait_reset.rs
git commit -m "test: add integration test for atomic wait state reset"
```

---

## Task 5: Full workspace verification

- [ ] **Step 1: Run format**

Run: `cargo fmt`

- [ ] **Step 2: Run clippy across workspace**

Run: `cargo clippy --all-targets --all-features -D warnings`
Expected: No warnings.

- [ ] **Step 3: Run workspace check**

Run: `cargo check -r`
Expected: No errors.

- [ ] **Step 4: Run nextest**

Run: `cargo nextest run -r --fail-fast`
Expected: All tests pass (tests requiring wasm targets no-op if targets are not installed).

- [ ] **Step 5: Final commit if any formatting changes**

```bash
git add -A
git commit -m "fmt: format workspace" || echo "No formatting changes needed"
```

---

## Self-Review

**1. Spec coverage:**
- User request: "reset memory.atomic.wait32 state on _reset" -> Task 1 (adds `__vfs_atomic_reset_target`), Task 2 (calls it from `_reset` body). Yes.
- User clarification: "notify + remove cells, do not keep them" -> Task 1 Step 3: notify then remove. Yes.
- User clarification: "with abort, test failure can be created" -> Task 4: WAT target with cookie-based zombie detection, test fails without fix (timeout), passes with fix. Yes.

**2. Placeholder scan:** No TBD/TODO/etc. All code blocks are complete. All file paths are exact.

**3. Type consistency:**
- `__vfs_atomic_reset_target(wasm_id: u32)` — defined in Task 1, called in Task 2 with `I32Const(wasm_mem as i32 - 1)` (u32 value as i32). Yes.
- `info.exported_funcs.get("__vfs_atomic_reset_target")` — matches the `#[no_mangle]` export name from Task 1. Yes.
- `WAIT_MAP` key layout `(wasm_id << 32) | relative_addr` — consistent between `__vfs_atomic_wait32` (thread.rs:1218) and the filter in `__vfs_atomic_reset_target` (`*e.key() >> 32`). Yes.
- `wasm_mem - 1` matches `AtomicPatchStreamPass`'s `target_id = self.target_index` (0-based) which becomes `wasm_id` in the WAIT_MAP key. Yes.
