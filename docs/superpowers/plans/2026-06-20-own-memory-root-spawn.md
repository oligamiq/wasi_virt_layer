# Own-Memory Root Spawn Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix `VirtualThreadPool` with `--own-memory` so pool-internal `root_spawn` calls route to the host thread-spawn import instead of recursively re-entering the virtual pool.

**Architecture:** Keep `VirtualThreadPool` unchanged. Fix `post_combine` export collection so duplicate export names do not let a later target-side copy overwrite the earlier VFS-owned implementation. Use the existing `test_pool_own_memory` integration test as the end-to-end regression.

**Tech Stack:** Rust 2024, `wasmparser`, `wasm-encoder`, `wasm-tools`, `cargo nextest`, Deno threaded generated runner.

## Global Constraints

- Minimum rustc is 1.89.0.
- Do not change `VirtualThreadPool` scheduling or resize behavior.
- Do not move the root-spawn flag into a new shared global/thread-local design.
- Do not rewrite thread pool initialization to avoid nested worker creation.
- If ABI naming changes, consult `wasi_virt_layer-cli/IMPORTS_EXPORTS_EVOLUTION_DETAILED.md` and keep import/export names in sync.
- Use `apply_patch` for manual edits.
- Do not commit unless the user explicitly requests a commit.

---

## File Structure

- Modify `wasi_virt_layer-cli/src/wasm_stream/passes/post_combine.rs`: preserve the first/VFS-owned export for duplicate names and add unit coverage for duplicate root-spawn predicate exports.
- Read `wasi_virt_layer-cli/src/wasm_stream/passes/own_memory_lowering.rs`: confirm it is not removing or rewriting the thread-spawn wrapper import.
- Use existing `wasi_virt_layer-cli/tests/test_pool_own_memory.rs`: no test source change is required unless focused command output is insufficient during debugging.
- Reference `docs/superpowers/specs/2026-06-20-own-memory-root-spawn-design.md`: root cause and accepted design.

---

### Task 1: Reproduce And Capture The Red Regression

**Files:**
- Read: `wasi_virt_layer-cli/tests/test_pool_own_memory.rs`
- Read: `wasi_virt_layer-cli/tests/fixtures/pool_own_mem_vfs/src/lib.rs`
- Read: `wasi_virt_layer-cli/tests/fixtures/pool_own_mem_target/src/main.rs`

**Interfaces:**
- Consumes: existing `run_wasi_virt_layer(...)` helper.
- Produces: a generated failing dist directory and WAT artifact used by Task 2.

- [ ] **Step 1: Run the focused failing test**

Run:

```bash
cargo nextest run -r --test test_pool_own_memory --no-capture
```

Expected: FAIL or SIGTERM/timeout. The output directory path appears in stdout after `Output directory:`.

- [ ] **Step 2: Run the generated Deno runner directly with a short timeout**

Replace `<dist>` with the output directory from Step 1.

```bash
timeout 20s deno run -A test_run.ts > /tmp/opencode/pool_own_mem.out 2> /tmp/opencode/pool_own_mem.err; printf 'status=%s\n' "$?"
```

Run this in `<dist>`.

Expected: `status=124`, `/tmp/opencode/pool_own_mem.err` contains `fatal runtime error: current thread handle already set during thread spawn, aborting`, and `/tmp/opencode/pool_own_mem.out` shows `Creating 4 threads in the thread pool...`.

- [ ] **Step 3: Print the generated core Wasm to WAT**

Run this in `<dist>`:

```bash
wasm-tools print pool_own_mem_vfs.core.wasm -o /tmp/opencode/pool_own_mem_vfs.before.wat
```

Expected: command exits successfully.

- [ ] **Step 4: Confirm the broken routing pattern**

Search `/tmp/opencode/pool_own_mem_vfs.before.wat` for these exact patterns:

```text
call 1099
call 343
call 82
call 0
```

Expected: there is a wrapper-like function that calls a root-spawn predicate, branches to host import `call 0` on true, and calls a pool spawn function on false. The predicate call target is the later duplicate predicate because `ParsedInfo.exported_funcs` overwrites the earlier VFS-owned `__wasip1_vfs_is_root_spawn` export.

---

### Task 2: Add A Focused Unit Test For Duplicate Root-Spawn Predicate Exports

**Files:**
- Modify: `wasi_virt_layer-cli/src/wasm_stream/passes/post_combine.rs`

**Interfaces:**
- Consumes: `PostCombineStreamPass::run(&mut self, input_wasm: &[u8]) -> eyre::Result<Vec<u8>>`.
- Produces: a unit test that fails on current code because the second duplicate `__wasip1_vfs_is_root_spawn` export overwrites the first/VFS-owned one.

- [ ] **Step 1: Add a helper that extracts function calls from a specific function index**

Add this helper inside `#[cfg(test)] mod tests` near `start_calls`:

```rust
fn function_calls(output: &[u8], target_func_idx: u32) -> eyre::Result<Vec<u32>> {
    let mut calls = Vec::new();
    let mut imported_funcs = 0;
    let mut defined_func_idx = 0;

    for payload in wasmparser::Parser::new(0).parse_all(output) {
        match payload? {
            wasmparser::Payload::ImportSection(imports) => {
                for group in imports {
                    for import in group?.into_iter() {
                        let (_, import) = import?;
                        if matches!(import.ty, wasmparser::TypeRef::Func(_)) {
                            imported_funcs += 1;
                        }
                    }
                }
            }
            wasmparser::Payload::CodeSectionEntry(body) => {
                let absolute_idx = imported_funcs + defined_func_idx;
                if absolute_idx == target_func_idx {
                    for op in body.get_operators_reader()? {
                        if let wasmparser::Operator::Call { function_index } = op? {
                            calls.push(function_index);
                        }
                    }
                }
                defined_func_idx += 1;
            }
            _ => {}
        }
    }

    Ok(calls)
}
```

- [ ] **Step 2: Add a minimal module builder with duplicate `__wasip1_vfs_is_root_spawn` exports**

Add this helper inside the same test module:

```rust
fn module_with_thread_spawn_wrapper_and_duplicate_root_spawn_exports() -> Vec<u8> {
    let mut module = Module::new();

    let mut types = TypeSection::new();
    types.ty().function([wasm_encoder::ValType::I32], [wasm_encoder::ValType::I32]);
    types.ty().function([], [wasm_encoder::ValType::I32]);
    module.section(&types);

    let mut imports = wasm_encoder::ImportSection::new();
    imports.import(
        "env",
        "__wasip1_vfs_wasi_thread_spawn_wrapper",
        wasm_encoder::EntityType::Function(0),
    );
    imports.import(
        "wasip1-vfs:host/virtual-file-system-wasip1-threads-import",
        "[static]wasip1-threads.thread-spawn-import",
        wasm_encoder::EntityType::Function(0),
    );
    module.section(&imports);

    let mut functions = FunctionSection::new();
    functions.function(1); // first/VFS-owned __wasip1_vfs_is_root_spawn
    functions.function(1); // later duplicate target-side __wasip1_vfs_is_root_spawn
    functions.function(0); // __wasip1_vfs_wasi_thread_spawn___self
    module.section(&functions);

    let mut exports = ExportSection::new();
    exports.export("__wasip1_vfs_is_root_spawn", ExportKind::Func, 2);
    exports.export("__wasip1_vfs_is_root_spawn", ExportKind::Func, 3);
    exports.export("__wasip1_vfs_wasi_thread_spawn___self", ExportKind::Func, 4);
    module.section(&exports);

    let mut code = CodeSection::new();
    let mut vfs_predicate = Function::new([]);
    vfs_predicate.instruction(&Instruction::I32Const(1));
    vfs_predicate.instruction(&Instruction::End);
    code.function(&vfs_predicate);

    let mut duplicate_predicate = Function::new([]);
    duplicate_predicate.instruction(&Instruction::I32Const(0));
    duplicate_predicate.instruction(&Instruction::End);
    code.function(&duplicate_predicate);

    let mut self_spawn = Function::new([]);
    self_spawn.instruction(&Instruction::LocalGet(0));
    self_spawn.instruction(&Instruction::End);
    code.function(&self_spawn);
    module.section(&code);

    module.finish()
}
```

- [ ] **Step 3: Add the failing unit test**

Add this test inside the same test module:

```rust
#[test]
fn duplicate_root_spawn_exports_keep_first_vfs_predicate_for_wrapper() -> eyre::Result<()> {
    let input = module_with_thread_spawn_wrapper_and_duplicate_root_spawn_exports();
    let mut pass = PostCombineStreamPass::new("vfs".to_string(), vec!["target".to_string()], vec![3], true);
    let output = pass.run(&input)?;

    let mut wrapper_calls = None;
    let mut imported_funcs = 0;
    let mut defined_func_idx = 0;

    for payload in wasmparser::Parser::new(0).parse_all(&output) {
        match payload? {
            wasmparser::Payload::ImportSection(imports) => {
                for group in imports {
                    for import in group?.into_iter() {
                        let (_, import) = import?;
                        if matches!(import.ty, wasmparser::TypeRef::Func(_)) {
                            imported_funcs += 1;
                        }
                    }
                }
            }
            wasmparser::Payload::CodeSectionEntry(body) => {
                let absolute_idx = imported_funcs + defined_func_idx;
                let calls = function_calls(&output, absolute_idx)?;
                if calls.len() == 3 && calls[1] == 0 && calls[2] == 3 {
                    wrapper_calls = Some(calls);
                }
                let _ = body;
                defined_func_idx += 1;
            }
            _ => {}
        }
    }

    assert_eq!(wrapper_calls, Some(vec![1, 0, 3]));

    Ok(())
}
```

Expected before implementation: FAIL with `Some(vec![2, 0, 3])` because current code lets the later duplicate root-spawn export overwrite the first/VFS-owned export.

- [ ] **Step 4: Run the focused unit test**

Run:

```bash
cargo test -r -p wasi_virt_layer-cli wasm_stream::passes::post_combine::tests::duplicate_root_spawn_exports_keep_first_vfs_predicate_for_wrapper -- --nocapture
```

Expected: FAIL before implementation and PASS after Task 3.

---

### Task 3: Preserve First Export Mapping For Duplicate Names

**Files:**
- Modify: `wasi_virt_layer-cli/src/wasm_stream/passes/post_combine.rs`
- Read: `wasi_virt_layer-cli/src/wasm_stream/passes/own_memory_lowering.rs`

**Interfaces:**
- Consumes: `ParsedInfo.exported_funcs: HashMap<String, u32>` and export iteration in `PostCombineStreamPass::run`.
- Produces: `ParsedInfo.exported_funcs` keeps the first occurrence for duplicate export names, so VFS-owned implementations are not overwritten by later target-side copies.

- [ ] **Step 1: Change export collection to preserve first mapping**

Replace this block in `post_combine.rs`:

```rust
info.exported_funcs
    .insert(export.name.to_string(), export.index);
```

with:

```rust
info.exported_funcs
    .entry(export.name.to_string())
    .or_insert(export.index);
```

- [ ] **Step 2: Keep wrapper synthesis lookup unchanged**

Leave this existing lookup unchanged, because Task 3 Step 1 changes `exported_funcs` to retain the first/VFS-owned export for duplicate names:

```rust
let is_root_spawn_orig_idx = info
    .exported_funcs
    .get("__wasip1_vfs_is_root_spawn")
    .unwrap();
let is_root_spawn_new_idx = func_map.get(is_root_spawn_orig_idx).unwrap();
```

- [ ] **Step 3: Inspect own-memory lowering for import/export rebinding bypass**

In `own_memory_lowering.rs`, inspect the import rewrite loop at `Payload::ImportSection`. Confirm that only imports matching these predicates are removed:

```rust
let is_own_memory_size = |name: &str| name.starts_with("__wasip1_vfs_own_memory_size_");
let is_own_memory_grow = |name: &str| name.starts_with("__wasip1_vfs_own_memory_grow_");
```

Expected: no thread-spawn wrapper import is removed here. Do not edit `own_memory_lowering.rs` unless inspection shows code that directly rewrites `__wasip1_vfs_wasi_thread_spawn_wrapper` calls; the known failing evidence is explained by duplicate export overwrite in `post_combine.rs`.

- [ ] **Step 4: Run the unit test from Task 2**

Run:

```bash
cargo test -r -p wasi_virt_layer-cli wasm_stream::passes::post_combine::tests::duplicate_root_spawn_exports_keep_first_vfs_predicate_for_wrapper -- --nocapture
```

Expected: PASS.

---

### Task 4: Verify End-To-End Own-Memory Thread Pool Runtime

**Files:**
- Read: `wasi_virt_layer-cli/tests/test_pool_own_memory.rs`
- Generated output: `wasi_virt_layer-cli/tests/onetime/<uuid>/dist/`

**Interfaces:**
- Consumes: fixed transform pipeline from Task 3.
- Produces: passing `test_pool_own_memory` and WAT evidence that the wrapper path uses a single root-spawn predicate state.

- [ ] **Step 1: Run the focused integration test**

Run:

```bash
cargo nextest run -r --test test_pool_own_memory --no-capture
```

Expected: PASS. Output should contain:

```text
Starting 5 threads test with VirtualThreadPool and own-memory
All 5 threads completed successfully.
```

- [ ] **Step 2: Inspect generated WAT after the fix**

Run this in the generated `dist` directory:

```bash
wasm-tools print pool_own_mem_vfs.core.wasm -o /tmp/opencode/pool_own_mem_vfs.after.wat
```

Expected: command exits successfully.

- [ ] **Step 3: Confirm wrapper routing in WAT**

Search `/tmp/opencode/pool_own_mem_vfs.after.wat` for the synthesized wrapper function. Confirm it branches like this:

```wat
call <vfs-owned-is-root-spawn>
if (result i32)
  local.get 0
  call <host-thread-spawn-import>
else
  local.get 0
  call <pool-spawn-function>
end
```

Expected: the predicate call target is the same implementation updated by the VFS `root_spawn` path, not a duplicate target-side predicate.

- [ ] **Step 4: Run the full fail-fast suite**

Run:

```bash
cargo nextest run -r --fail-fast
```

Expected: PASS for all tests that are not skipped by existing target/environment gates.

---

### Task 5: Documentation And Final Sanity Checks

**Files:**
- Modify only if ABI names changed: `wasi_virt_layer-cli/IMPORTS_EXPORTS_EVOLUTION_DETAILED.md`
- Read: `docs/superpowers/specs/2026-06-20-own-memory-root-spawn-design.md`

**Interfaces:**
- Consumes: verified code and tests from Tasks 3 and 4.
- Produces: final checked working tree with no unrelated edits.

- [ ] **Step 1: Check whether ABI names changed**

Run:

```bash
git diff -- wasi_virt_layer-cli/src/wasm_stream/passes/post_combine.rs wasi_virt_layer-cli/src/wasm_stream/passes/own_memory_lowering.rs
```

Expected: if no import/export names changed, do not edit `IMPORTS_EXPORTS_EVOLUTION_DETAILED.md`. If names changed, update that doc with the exact old and new names.

- [ ] **Step 2: Format**

Run:

```bash
cargo fmt
```

Expected: exits successfully.

- [ ] **Step 3: Check working tree**

Run:

```bash
git status --short
```

Expected: only intended files are modified or added.

- [ ] **Step 4: Prepare commit command if requested**

If the user explicitly asks for a commit, run:

```bash
git add docs/superpowers/specs/2026-06-20-own-memory-root-spawn-design.md docs/superpowers/plans/2026-06-20-own-memory-root-spawn.md wasi_virt_layer-cli/src/wasm_stream/passes/post_combine.rs wasi_virt_layer-cli/src/wasm_stream/passes/own_memory_lowering.rs
git commit -m "fix: preserve root spawn routing for own memory threads"
```

Expected: commit succeeds. If the user has not requested a commit, do not run these commands.
