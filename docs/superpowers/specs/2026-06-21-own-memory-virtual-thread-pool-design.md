# Own-Memory VirtualThreadPool Target Spawn Design

## Problem

`--own-memory` plus `VirtualThreadPool` now initializes pool workers and reaches the target program's `_main()`, but target-side `std::thread::spawn` still fails with WASI `EAGAIN` / `WouldBlock`.

The earlier packet-reuse abort was a separate memory-lowering bug: physical `memory.grow` succeeded without updating own-memory logical size. That fix is covered by `own_memory_physical_grow_updates_self_logical_size`.

The remaining failure is in the target spawn path. A target `std::thread::spawn` passes a target-memory pointer to a `Box<dyn FnOnce()>`. The VFS cannot dereference or reinterpret that pointer because target and VFS memories are separate under own-memory.

## Goal

Support target `std::thread::spawn` through `VirtualThreadPool` under `--own-memory`.

The pool should schedule target thread work, but the target closure pointer must remain opaque until execution reaches the target's own `wasi_thread_start`.

## Non-Goals

- Do not change `VirtualThreadPool` capacity, resize, or queue semantics.
- Do not move the root-spawn flag to a new shared/global design.
- Do not copy, marshal, or dereference target closures in VFS memory.
- Do not avoid nested worker creation as a symptom workaround.

## Architecture

`VirtualThreadPool` remains a scheduler only. It owns the queue, worker handles, and guest-visible thread ID allocation. It does not own target memory semantics.

Target `std::thread::spawn` must resolve to the target-specific VFS hook:

`__wasip1_vfs_wasi_thread_spawn_{target}`

That hook wraps the target closure pointer in `ThreadRunner` as an opaque integer, allocates a pool thread ID, enqueues `Run(runner, ThreadAccessor::{target}, id)`, and returns the ID synchronously.

An idle pool worker receives `Run` and calls `ThreadAccessor::{target}.call_wasi_thread_start(runner, id)`. The generated accessor must call the target-specific exported start function:

`__wasip1_vfs_{target}_wasi_thread_start(thread_id, runner_ptr)`

The target `wasi_thread_start` is responsible for interpreting the target closure pointer in target memory and running the closure.

The generic root/self thread-spawn wrapper remains for VFS/self root worker creation. It must not replace target-specific spawn imports.

## Data Flow

1. Target `_main()` calls Rust `std::thread::spawn`.
2. Target WASI thread-spawn import calls `__wasip1_vfs_wasi_thread_spawn_{target}`.
3. VFS hook treats `data_ptr` as opaque and queues `ThreadRunner` with target accessor.
4. Pool worker dequeues the message.
5. Target accessor calls target `wasi_thread_start(thread_id, data_ptr)`.
6. Target runtime initializes its thread context and runs the closure from target memory.

## Error Handling

If `VirtualThreadPool::new_thread` cannot allocate an ID or enqueue work, the hook may return WASI `EAGAIN` as it does today. The fix should not hide these real failures; it should ensure valid target spawns do not accidentally take the wrong path or execute with the wrong ABI.

## Testing

- Keep the existing logical-size unit test for physical grow.
- Add focused post-combine/lowering tests that assert target `thread-spawn` imports resolve to the target-specific hook, not the self/root wrapper.
- Add or update a focused test proving the generated target accessor calls target-specific `wasi_thread_start` with the opaque pointer unchanged.
- Verify `cargo nextest run -r --test test_pool_own_memory --no-capture` reaches `All 5 threads completed successfully.`.

## Open Constraints

The current worktree contains unrelated modified files. Implementation must avoid reverting or rewriting unrelated changes.
