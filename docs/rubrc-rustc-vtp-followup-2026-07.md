# Rubrc `rustc` / VTP follow-up

Date: 2026-07-02

This note records the `wasi_virt_layer`-level behavior that remains suspicious
after the latest `rubrc` WebShell `rustc` investigation. The observed Rubrc
hang was avoided by a `wasi-shell` call-path change, not by fixing
`wasi_virt_layer`.

## Current state

- `wasi_virt_layer` has no final changes from this investigation.
- The attempted `wasi_virt_layer` diagnostics and behavior changes were
  reverted/cleaned.
- The final functional change is in
  `/home/oligami/projects/sl-rust/crates/wasi-shell/src/lib.rs`.
- `rubrc` and `wasi_virt_layer` worktrees were clean after cleanup.

The `wasi-shell` change avoids one problematic path:

- `handle_parallel(vec![single_command])` now executes inline on the caller
  thread instead of spawning an extra worker thread.
- `handle_pipeline(single_stage)` also executes inline.
- Multi-line parallel execution still spawns threads.

This reduces exposure to reused VTP/direct-export state, but it does not fix
that lower-level behavior.

## Verified after cleanup

In `sl-rust`:

```bash
cargo test -p wasi-shell
```

Result:

```text
54 passed; 0 failed
```

Only warning observed:

```text
warning: unused variable: `chars`
  --> crates/wasibox-core/src/utils/ls.rs:184:13
```

In `rubrc`, a light shell-path check also returned to the prompt for four runs:

```bash
VFS_DEBUG_THREADS=8 VFS_DEBUG_RUNS=4 VFS_DEBUG_TIMEOUT_MS=60000 \
  deno run -A scripts/vfs_debug_shell.ts rustc
```

This check used the existing page bindings and bare `rustc` with no input file.
It is not the heavier compile repro with `/src/main.rs`.

## Previous stronger repro evidence

Before cleanup, a temporary typed-shell repro that compiled `/src/main.rs`
passed after the `wasi-shell` inline fix:

```bash
VFS_DEBUG_THREADS=8 VFS_DEBUG_RUNS=4 VFS_DEBUG_TIMEOUT_MS=60000 \
  deno run -A scripts/debug_rustc_shell_thread_spawn.ts
```

Recorded passing outputs from that temporary repro:

- `/home/oligami/.local/share/opencode/tool-output/tool_f1dadef97001uCevbouP9tZzbl`
  - reserve pages minimized
- `/home/oligami/.local/share/opencode/tool-output/tool_f1daf047c001wq225n1rq0WLc4`
  - default reserves

That temporary repro script was removed during final cleanup because the chosen
scope was "minimal fix only".

## Important paths

Typed shell command path before the `wasi-shell` fix:

```text
keystrokes
  -> vfs_shell_dispatch
  -> wasi_shell::handle_parallel(vec![line])
  -> spawned per-command worker thread
  -> handle_pipeline
  -> vfs-shell fallback
  -> vfs_execute_command
  -> handle_command("rustc")
  -> rustc_opt::_reset()
  -> rustc_opt::_main()
```

Typed shell command path after the `wasi-shell` fix:

```text
keystrokes
  -> vfs_shell_dispatch
  -> wasi_shell::handle_parallel(vec![line])
  -> inline on caller thread
  -> handle_pipeline(single stage)
  -> inline on caller thread
  -> vfs-shell fallback
  -> vfs_execute_command
  -> handle_command("rustc")
  -> rustc_opt::_reset()
  -> rustc_opt::_main()
```

The direct debug path used during investigation:

```text
EVENT_TYPE_DEBUG_FIXED_RUSTC
  -> temporary host/guest thread
  -> rustc_opt::_reset()
  -> rustc_opt::_main()
```

The direct path repeatedly passed even while the `_reset()` / own-memory high
water behavior described below was visible. That means the memory high-water
issue is real, but was not sufficient by itself to explain the observed shell
hang.

## Failed `wasi_virt_layer` hypothesis

Hypothesis tested: change direct-export thread reinitialization behavior around
VTP workers.

Result: failed. The shell repro still timed out, so the change was reverted.

Recorded failed output:

```text
/home/oligami/.local/share/opencode/tool-output/tool_f1d9c113c001yjKyN0zMFHYRbv
```

Conclusion: caller-side direct-export reinit behavior may still be relevant,
but the tested exclusion was not a sufficient fix.

## Invalid diagnostic path

A 64-thread diagnostic run should not be used as root-cause evidence.

During thread-pool expansion it showed startup argument corruption or unaligned
data-like behavior. That run was useful as a stress signal, but it conflates
pool expansion problems with the repeated `rustc` issue.

## Remaining suspicious `wasi_virt_layer` behavior

### 1. Reused VTP worker direct exports

Repeated calls into a target module from a VTP/reused worker still look risky.
The relevant area is the generated/imported `_main` wrapper that decides whether
to re-run target thread-start initialization before a direct export.

Files to inspect:

```text
/home/oligami/projects/wasi_virt_layer/wasi_virt_layer/src/memory.rs
/home/oligami/projects/wasi_virt_layer/wasi_virt_layer/src/wasi/thread.rs
```

Symbols/functions involved during investigation:

```text
should_reinitialize_direct_export_thread
is_virtual_thread_pool_worker
mark_wasi_thread_started
call_thread_start_init
WORKER_HAS_RUN_BEFORE
```

The important distinction is:

- a thread entered through the target module's WASI thread-start entrypoint;
- a VTP worker thread reused to call a target direct export;
- a root/non-VTP thread calling a direct export.

The failing hypothesis changed this space but did not fix the symptom. A new
fix should have a focused test that exercises repeated direct exports from a
reused VTP worker.

### 2. `_reset()` does not reset own-memory logical size

Observed facts from diagnostics:

- `rustc_opt::_reset()` resets mutable globals via generated reset code.
- It restores linear memory contents.
- It resets VFS atomic wait-map state for the target wasm id.
- It reruns the target module start path.
- It does not shrink wasm memory.
- It does not reset own-memory logical size atomics.

This creates a state gap: after one large `rustc` run, the logical own-memory
high-water value remains high across `_reset()`.

Relevant generated/runtime areas:

```text
/home/oligami/projects/wasi_virt_layer/wasi_virt_layer/src/memory.rs
/home/oligami/projects/wasi_virt_layer/wasi_virt_layer-cli/src/wasm_stream/passes/post_combine.rs
```

In `rubrc`, this surfaced as traces like:

```text
[vfs-debug] rustc:memory:before-reset pages=1730
[vfs-debug] rustc:memory:after-reset pages=1730
```

The direct repeated path passed despite this, so it is not proven to be the
shell hang root cause. It is still likely incorrect lifecycle state for a module
that is expected to be reset and rerun.

### 3. Reserve/ensure compares against logical size

In `rubrc`, `MEMORY_MANAGER.ensure()` uses
`wasi_virt_layer::memory_size::<target>()` as its capacity signal.

Observed warning shape:

```text
[memory] failed to reserve 4096 pages for rustc_opt (have 1730, need 4096)
```

The `have` value is the target logical own-memory size, not necessarily a clear
physical reservation/quota signal. Repeated reset/rerun can therefore report a
misleading or stale capacity state.

Rubrc files involved:

```text
/home/oligami/projects/rubrc/crates/vfs/src/memory_manager.rs
/home/oligami/projects/rubrc/crates/vfs/src/command.rs
```

Potential WVL-level clarification/fix:

- separate logical target memory size from physical reserved capacity;
- expose a reserve-capacity query distinct from target `memory.size`;
- reset target logical own-memory size during `_reset()` if it is intended to
  represent post-reset module state.

### 4. Atomic wait/noisy late notifications

Investigation logs often included messages like:

```text
notify failed, waiter is late
notify number is 0. ref is late?
invoke_func_loop is late
```

These appeared frequently in passing and failing runs, so they are not by
themselves proof of the hang. They are still useful breadcrumbs if a WVL fix
touches wait-map reset, VTP reuse, or deadlock detection.

Related areas:

```text
/home/oligami/projects/wasi_virt_layer/wasi_virt_layer/src/wasi/thread.rs
vfs_atomic / deadlock detection / wait-map reset paths
```

## Recommended WVL tests

Add focused tests before changing behavior.

### Reused VTP direct export test

Construct a fixture where:

1. A root VFS module creates a VTP pool.
2. A VTP worker calls a target module direct export.
3. The same worker is reused for a second direct export call.
4. The test verifies target thread-local/start-section state is correct on both
   calls and does not rely on stale TLS/global state.

This should fail or expose the current ambiguous behavior before the fix.

### `_reset()` own-memory state test

Construct a target that grows own memory, calls `_reset()`, then checks:

- generated memory contents are restored;
- target globals are restored;
- target own-memory logical size is either intentionally restored to the
  snapshot size or explicitly documented as persistent;
- reserve-capacity reporting remains distinct and coherent.

### Reserve reporting test

Test that a target which previously grew to `N` pages does not cause a later
post-reset reserve/ensure call to treat stale logical high-water as current
capacity.

## Build/copy caveat for Rubrc validation

Rubrc page repros load generated bindings from:

```text
/home/oligami/projects/rubrc/page/src/worker_process/vfs_bindings/
```

The WVL CLI build writes to `rubrc/dist` first. After rebuilding, copy artifacts
while preserving `inst.ts` and excluding nested dependencies:

```bash
rsync -a \
  --exclude 'inst.ts' \
  --exclude 'node_modules/' \
  "/home/oligami/projects/rubrc/dist/" \
  "/home/oligami/projects/rubrc/page/src/worker_process/vfs_bindings/"
```

Do not validate against stale `page/src/worker_process/vfs_bindings` artifacts.

## Known non-goals from this investigation

- Do not rely on the old stderr-recursion handoff as the current root cause.
  That was a previous issue.
- Do not treat the `wasi-shell` inline fix as a WVL correctness fix.
- Do not treat the failed VTP direct-export exclusion as proof that VTP state is
  correct. It only proves that one attempted change was insufficient.
- Do not use 64-thread pool-expansion diagnostics as primary evidence for the
  repeated `rustc` hang.

## Practical success criteria for a WVL fix

A WVL-side fix should be considered credible only after all of these pass:

1. A WVL unit/integration test for repeated direct exports on a reused VTP
   worker.
2. A WVL unit/integration test for `_reset()` plus own-memory logical state.
3. Existing WVL VTP and deadlock-detection tests.
4. Rubrc shell repro with repeated `rustc` compile commands after rebuilding and
   copying generated page bindings.
