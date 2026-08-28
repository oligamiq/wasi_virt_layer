# Own-Memory Root Spawn Design

## Problem

`test_pool_own_memory` fails at runtime when `VirtualThreadPool` is used with `--own-memory`. The generated Deno run times out after a worker aborts with:

```text
fatal runtime error: current thread handle already set during thread spawn, aborting
```

The failure happens during `VirtualThreadPool::init_with_capacity_and_wait(5)`: the first pool worker is spawned, then that worker tries to create the remaining workers.

## Root Cause

`VirtualThreadPool` uses `root_spawn` for pool-internal worker creation. `root_spawn` sets a root-spawn flag so the generated `wasi_thread_spawn` wrapper can route that spawn to the host thread-spawn import instead of recursively routing it through the virtual thread pool.

With `--own-memory`, the generated core Wasm contains multiple copies of the root-spawn flag logic. The pool side sets one copy of the flag, while the thread-spawn wrapper reads another copy. As a result, the wrapper treats root-spawn calls as normal guest thread spawns and routes them back into `__wasip1_vfs_wasi_thread_spawn_<target>`. That recursively re-enters the pool path from inside a worker and triggers the runtime abort.

Evidence from the failing WAT:

- `__wasip1_vfs_wasi_thread_spawn_pool_own_mem_target` is exported as `func 83` and routes directly to the pool spawn path.
- The wrapper-like function `func 1099` correctly branches between host `thread_spawn` and pool spawn, but it reads root-spawn state through `func 343`.
- `func 343` reads state at `global 1 + 2`, while the pool-side root-spawn flag initialization path uses `global 1 + 0`.

## Chosen Approach

Fix the generator/lowering path so all target `wasi_thread_spawn` calls in own-memory builds route through a single VFS-owned root-spawn wrapper.

The wrapper must keep this behavior:

```text
if VFS-owned __wasip1_vfs_is_root_spawn() {
    call host real thread_spawn(data_ptr)
} else {
    call __wasip1_vfs_wasi_thread_spawn_<target>(data_ptr)
}
```

The important invariant is that the `root_spawn` setter and the wrapper's `__wasip1_vfs_is_root_spawn` reader refer to the same VFS-owned state after own-memory lowering.

## Non-Goals

- Do not change `VirtualThreadPool` scheduling or resize behavior.
- Do not move the root-spawn flag into a new shared global/thread-local design.
- Do not rewrite thread pool initialization to avoid nested worker creation; that would only avoid this particular symptom and leave root-spawn routing broken.

## Implementation Areas

- `wasi_virt_layer-cli/src/wasm_stream/passes/post_combine.rs`: verify and adjust synthesized `__wasip1_vfs_wasi_thread_spawn_wrapper` emission for own-memory output.
- `wasi_virt_layer-cli/src/wasm_stream/passes/own_memory_lowering.rs`: verify that removed/rebound imports do not bypass the wrapper or bind target thread-spawn imports directly to pool exports.
- ABI/export naming docs if function routing names change.

## Testing

- Run `cargo nextest run -r --fail-fast` after the fix.
- Run `cargo test -r -p wasi_virt_layer-cli --test test_pool_own_memory -- --nocapture` or the nextest equivalent for focused verification.
- Inspect the generated WAT for the fixture if needed: the target thread-spawn path should call the root-spawn wrapper, and the wrapper should call the VFS-owned root-spawn predicate before selecting host thread-spawn versus pool spawn.
