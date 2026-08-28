# Own-Memory VirtualThreadPool Target Spawn Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make target `std::thread::spawn` work through `VirtualThreadPool` under `--own-memory` without changing pool scheduling semantics.

**Architecture:** Keep `VirtualThreadPool` as a scheduler only. Target closure pointers remain opaque while in VFS memory and are forwarded unchanged to the target-specific `wasi_thread_start` entrypoint.

**Tech Stack:** Rust 2024, `wasm-encoder`, `wasmparser`, `cargo nextest`, `wasm-tools`/`wasm-objdump` for diagnostics.

## Global Constraints

- Rust minimum is 1.89.0.
- Do not change `VirtualThreadPool` capacity, resize, or queue semantics.
- Do not move the root-spawn flag to a new shared/global design.
- Do not copy, marshal, or dereference target closures in VFS memory.
- Do not avoid nested worker creation as a symptom workaround.
- Do not revert or rewrite unrelated worktree changes.
- Do not commit unless explicitly requested by the user.

---

## File Structure

- `wasi_virt_layer-cli/src/wasm_stream/passes/post_combine.rs`: add focused unit tests and fix import/export rebinding so target thread-spawn imports resolve to target-specific hooks, not the generic self/root wrapper.
- `wasi_virt_layer-cli/src/wasm_stream/passes/multi_memory_lowering.rs`: keep the existing own-memory physical grow regression test; only touch again if target-specific spawn routing is broken by lowering.
- `wasi_virt_layer-cli/tests/fixtures/pool_own_mem_vfs/src/lib.rs`: keep `_reset(); init_with_capacity_and_wait(5); _start(); _main();` for the own-memory pool fixture.
- `wasi_virt_layer-cli/tests/test_pool_own_memory.rs`: keep the integration assertion on target output.
- `docs/superpowers/specs/2026-06-21-own-memory-virtual-thread-pool-design.md`: reference only; no implementation edits needed.

---

### Task 1: Lock Target Thread-Spawn Import Routing

**Files:**
- Modify: `wasi_virt_layer-cli/src/wasm_stream/passes/post_combine.rs`
- Test: `wasi_virt_layer-cli/src/wasm_stream/passes/post_combine.rs`

**Interfaces:**
- Consumes: `PostCombineStreamPass::new(vfs_name, target_names, defined_funcs_counts, threads)` and existing helper `function_calls(output, target_func_idx)`.
- Produces: a post-combine guarantee that an import named `__wasip1_vfs_wasi_thread_spawn_{target}` maps to the exported function with the same name.

- [ ] **Step 1: Add a failing unit test for target-specific thread-spawn routing**

Add this test inside the existing `#[cfg(test)] mod tests` in `post_combine.rs`, near `duplicate_root_spawn_exports_keep_first_vfs_predicate_for_wrapper`:

```rust
    #[test]
    fn target_thread_spawn_import_resolves_to_target_specific_hook() -> eyre::Result<()> {
        let input = module_with_target_thread_spawn_import_and_hook();
        let mut pass = PostCombineStreamPass::new(
            "vfs".to_string(),
            vec!["target".to_string()],
            vec![3],
            true,
        );
        let output = pass.run(&input)?;

        let mut target_hook = None;
        let mut caller = None;

        for payload in wasmparser::Parser::new(0).parse_all(&output) {
            if let wasmparser::Payload::ExportSection(exports) = payload? {
                for export in exports {
                    let export = export?;
                    if export.name == "__wasip1_vfs_wasi_thread_spawn_target" {
                        target_hook = Some(export.index);
                    }
                    if export.name == "target_spawn_caller" {
                        caller = Some(export.index);
                    }
                }
            }
        }

        let target_hook = target_hook.expect("target hook export should remain available");
        let caller = caller.expect("test caller export should remain available");
        assert_eq!(function_calls(&output, caller)?, vec![target_hook]);

        Ok(())
    }
```

- [ ] **Step 2: Add the Wasm fixture helper for the failing test**

Add this helper in the same test module, near `module_with_thread_spawn_wrapper_and_duplicate_root_spawn_exports()`:

```rust
    fn module_with_target_thread_spawn_import_and_hook() -> Vec<u8> {
        let mut module = Module::new();

        let mut types = TypeSection::new();
        types
            .ty()
            .function([wasm_encoder::ValType::I32], [wasm_encoder::ValType::I32]);
        module.section(&types);

        let mut imports = ImportSection::new();
        imports.import(
            "__wasip1_vfs-host",
            "__wasip1_vfs_wasi_thread_spawn_target",
            wasm_encoder::EntityType::Function(0),
        );
        module.section(&imports);

        let mut functions = FunctionSection::new();
        functions.function(0);
        functions.function(0);
        module.section(&functions);

        let mut exports = ExportSection::new();
        exports.export("target_spawn_caller", ExportKind::Func, 1);
        exports.export("__wasip1_vfs_wasi_thread_spawn_target", ExportKind::Func, 2);
        module.section(&exports);

        let mut code = CodeSection::new();
        let mut caller = Function::new([]);
        caller.instruction(&Instruction::LocalGet(0));
        caller.instruction(&Instruction::Call(0));
        caller.instruction(&Instruction::End);
        code.function(&caller);

        let mut target_hook = Function::new([]);
        target_hook.instruction(&Instruction::LocalGet(0));
        target_hook.instruction(&Instruction::End);
        code.function(&target_hook);
        module.section(&code);

        module.finish()
    }
```

- [ ] **Step 3: Run the focused test and verify it fails for the intended reason**

Run:

```bash
cargo test -r -p wasi_virt_layer-cli target_thread_spawn_import_resolves_to_target_specific_hook -- --nocapture
```

Expected before the fix: the test fails because `target_spawn_caller` does not call the exported target-specific hook, or because the target-specific hook is dropped/unresolved.

- [ ] **Step 4: Implement the minimal post-combine routing fix**

In `post_combine.rs`, update the host-import resolution logic so imports named `__wasip1_vfs_wasi_thread_spawn_{target}` resolve through `info.exported_funcs` like any other late-linked VFS export.

Use the minimal shape below. The exact location should be in `resolve_host_export` after the direct `info.exported_funcs.get(name)` lookup and before returning `None`:

```rust
fn resolve_host_export(name: &str, info: &ParsedInfo) -> Option<u32> {
    if let Some(&export_orig_idx) = info.exported_funcs.get(name) {
        return Some(export_orig_idx);
    }

    if name.starts_with("__wasip1_vfs_wasi_thread_spawn_") {
        return info.exported_funcs.get(name).copied();
    }

    let self_suffix = name.strip_prefix("__wasip1_vfs___self_")?;
    if self_suffix == "proc_exit" {
        return None;
    }

    let compact_self_name = format!("__wasip1_vfs_self_{self_suffix}");
    if let Some(&export_orig_idx) = info.exported_funcs.get(&compact_self_name) {
        return Some(export_orig_idx);
    }

    None
}
```

If this exact code is redundant because the direct lookup already covers the case, do not keep a redundant branch. Instead, inspect why the test fails and make the smallest change in the import classification at `ImportSection` pass 1 so target-specific thread-spawn imports are not converted into dropped generated functions.

- [ ] **Step 5: Run the focused test and verify it passes**

Run:

```bash
cargo test -r -p wasi_virt_layer-cli target_thread_spawn_import_resolves_to_target_specific_hook -- --nocapture
```

Expected: `test wasm_stream::passes::post_combine::tests::target_thread_spawn_import_resolves_to_target_specific_hook ... ok`.

---

### Task 2: Lock Target `wasi_thread_start` Accessor Dispatch

**Files:**
- Modify: `wasi_virt_layer-cli/src/wasm_stream/passes/post_combine.rs`
- Test: `wasi_virt_layer-cli/src/wasm_stream/passes/post_combine.rs`

**Interfaces:**
- Consumes: target-specific hook routing from Task 1.
- Produces: a focused test proving the generated dispatch path can call `__wasip1_vfs_{target}_wasi_thread_start(thread_id, ptr)` without routing to a self/root stub.

- [ ] **Step 1: Add a failing test for target `wasi_thread_start` late linking**

Add this test to `post_combine.rs` tests:

```rust
    #[test]
    fn target_wasi_thread_start_import_resolves_to_target_export() -> eyre::Result<()> {
        let input = module_with_target_wasi_thread_start_import_and_export();
        let mut pass = PostCombineStreamPass::new(
            "vfs".to_string(),
            vec!["target".to_string()],
            vec![3],
            true,
        );
        let output = pass.run(&input)?;

        let mut target_start = None;
        let mut caller = None;

        for payload in wasmparser::Parser::new(0).parse_all(&output) {
            if let wasmparser::Payload::ExportSection(exports) = payload? {
                for export in exports {
                    let export = export?;
                    if export.name == "__wasip1_vfs_target_wasi_thread_start" {
                        target_start = Some(export.index);
                    }
                    if export.name == "target_wasi_thread_start_caller" {
                        caller = Some(export.index);
                    }
                }
            }
        }

        let target_start = target_start.expect("target wasi_thread_start export should remain available");
        let caller = caller.expect("test caller export should remain available");
        assert_eq!(function_calls(&output, caller)?, vec![target_start]);

        Ok(())
    }
```

- [ ] **Step 2: Add the Wasm fixture helper for target `wasi_thread_start`**

Add this helper near the Task 1 helper:

```rust
    fn module_with_target_wasi_thread_start_import_and_export() -> Vec<u8> {
        let mut module = Module::new();

        let mut types = TypeSection::new();
        types
            .ty()
            .function([wasm_encoder::ValType::I32, wasm_encoder::ValType::I32], []);
        module.section(&types);

        let mut imports = ImportSection::new();
        imports.import(
            "wasip1-vfs",
            "__wasip1_vfs_target_wasi_thread_start",
            wasm_encoder::EntityType::Function(0),
        );
        module.section(&imports);

        let mut functions = FunctionSection::new();
        functions.function(0);
        functions.function(0);
        module.section(&functions);

        let mut exports = ExportSection::new();
        exports.export("target_wasi_thread_start_caller", ExportKind::Func, 1);
        exports.export("__wasip1_vfs_target_wasi_thread_start", ExportKind::Func, 2);
        module.section(&exports);

        let mut code = CodeSection::new();
        let mut caller = Function::new([]);
        caller.instruction(&Instruction::LocalGet(0));
        caller.instruction(&Instruction::LocalGet(1));
        caller.instruction(&Instruction::Call(0));
        caller.instruction(&Instruction::End);
        code.function(&caller);

        let mut target_start = Function::new([]);
        target_start.instruction(&Instruction::End);
        code.function(&target_start);
        module.section(&code);

        module.finish()
    }
```

- [ ] **Step 3: Run the focused test and verify it fails for unresolved or wrong target start dispatch**

Run:

```bash
cargo test -r -p wasi_virt_layer-cli target_wasi_thread_start_import_resolves_to_target_export -- --nocapture
```

Expected before the fix: the caller either remains mapped to the wrong import/generated stub or validation fails because the target export is not resolved.

- [ ] **Step 4: Implement the minimal late-link fix**

If the import is treated as special and dropped without being linked, update the `is_special` import predicate in `post_combine.rs` so target `__wasip1_vfs_{target}_wasi_thread_start` imports are resolved against exports instead of emitted as generated unreachable functions.

Add a local helper near `resolve_host_export` if needed:

```rust
fn is_target_wasi_thread_start_import(name: &str) -> bool {
    name.starts_with("__wasip1_vfs_")
        && name.ends_with("_wasi_thread_start")
        && name != "__wasip1_vfs___self_wasi_thread_start"
        && name != "__wasip1_vfs_self_wasi_thread_start"
}
```

Then ensure such imports go through `host_imports` resolution rather than `dropped_imports`. Keep the change narrowly scoped to `wasi_thread_start`; do not alter memory, reset, or `_start` import handling.

- [ ] **Step 5: Run focused tests from Tasks 1 and 2**

Run:

```bash
cargo test -r -p wasi_virt_layer-cli target_thread_spawn_import_resolves_to_target_specific_hook target_wasi_thread_start_import_resolves_to_target_export -- --nocapture
```

Expected: both tests pass. If Cargo rejects multiple filters, run the two test commands separately.

---

### Task 3: Verify Own-Memory Integration and Remove Stale Instrumentation

**Files:**
- Modify: `wasi_virt_layer-cli/tests/fixtures/pool_own_mem_vfs/src/lib.rs`
- Modify: `wasi_virt_layer-cli/tests/test_pool_own_memory.rs`
- Verify: `wasi_virt_layer-cli/src/wasm_stream/passes/multi_memory_lowering.rs`

**Interfaces:**
- Consumes: target-specific spawn and target `wasi_thread_start` routing from Tasks 1 and 2.
- Produces: passing integration behavior for `test_pool_own_memory`.

- [ ] **Step 1: Ensure the own-memory fixture uses `_main()` after initialization**

In `wasi_virt_layer-cli/tests/fixtures/pool_own_mem_vfs/src/lib.rs`, the `Guest for ComponentABI` implementation must contain exactly this execution sequence:

```rust
impl Guest for ComponentABI {
    fn main() {
        pool_own_mem_target::_reset();
        unsafe { THREAD_POOL.init_with_capacity_and_wait(5) };
        println!("Pool threads initialized.");
        pool_own_mem_target::_start();
        pool_own_mem_target::_main();
    }
}
```

Do not add manual `THREAD_POOL.new_thread(...)` probes or debug prints to the fixture.

- [ ] **Step 2: Ensure the integration test expects target main completion**

In `wasi_virt_layer-cli/tests/test_pool_own_memory.rs`, keep these assertions:

```rust
assert!(stdout.contains("Starting 5 threads test with VirtualThreadPool and own-memory"));
assert!(stdout.contains("All 5 threads completed successfully."));
```

Do not add `--no-cache`; the current CLI rejects that flag.

- [ ] **Step 3: Run focused lowering regression**

Run:

```bash
cargo test -r -p wasi_virt_layer-cli own_memory_physical_grow_updates_self_logical_size -- --nocapture
```

Expected: `1 passed; 0 failed` for the focused lowering test.

- [ ] **Step 4: Run own-memory thread pool integration**

Run:

```bash
cargo nextest run -r --test test_pool_own_memory --no-capture
```

Expected: test passes and stdout contains both:

```text
### Starting 5 threads test with VirtualThreadPool and own-memory
### All 5 threads completed successfully.
```

- [ ] **Step 5: If integration still fails, capture generated Wasm call path before changing code**

Run `wasm-tools print` on the latest generated `pool_own_mem_vfs.core.wasm` from the failing `onetime/.../dist` directory:

```bash
latest_dist=$(ls -td wasi_virt_layer-cli/tests/onetime/*/dist | head -n 1) && wasm-tools print "$latest_dist/pool_own_mem_vfs.core.wasm" > "/tmp/opencode/pool_own_failed.wat"
```

Then search the WAT using the dedicated grep tool for these names:

```text
__wasip1_vfs_wasi_thread_spawn_pool_own_mem_target
__wasip1_vfs_pool_own_mem_target_wasi_thread_start
__wasip1_vfs_wasi_thread_spawn_wrapper
```

Expected: target `_main()` thread-spawn call path reaches `__wasip1_vfs_wasi_thread_spawn_pool_own_mem_target`, and pool worker dispatch reaches `__wasip1_vfs_pool_own_mem_target_wasi_thread_start`.

Do not try another fix until this evidence shows which edge is wrong.

---

### Task 4: Final Verification and Worktree Review

**Files:**
- Verify only: whole workspace.

**Interfaces:**
- Consumes: all prior tasks.
- Produces: verified change set ready for user review.

- [ ] **Step 1: Format the workspace**

Run:

```bash
cargo fmt
```

Expected: command exits 0.

- [ ] **Step 2: Run targeted regression tests**

Run:

```bash
cargo test -r -p wasi_virt_layer-cli own_memory_physical_grow_updates_self_logical_size -- --nocapture
cargo test -r -p wasi_virt_layer-cli target_thread_spawn_import_resolves_to_target_specific_hook -- --nocapture
cargo test -r -p wasi_virt_layer-cli target_wasi_thread_start_import_resolves_to_target_export -- --nocapture
cargo nextest run -r --test test_pool_own_memory --no-capture
```

Expected: all commands exit 0.

- [ ] **Step 3: Inspect worktree without reverting unrelated changes**

Run:

```bash
git status --short
git diff --stat
```

Expected: modified files include the intentional implementation and tests. If unrelated files are present, mention them in the final response and do not revert them.

- [ ] **Step 4: Summarize evidence**

Report:

```text
Focused lowering test: PASS
Target spawn routing test: PASS
Target wasi_thread_start routing test: PASS
Own-memory pool integration: PASS
Formatting: PASS
```

If any command fails, report the exact failing command and the first concrete failure symptom instead of claiming completion.
