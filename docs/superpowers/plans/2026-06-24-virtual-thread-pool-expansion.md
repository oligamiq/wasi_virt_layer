# VirtualThreadPool Expansion Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix `VirtualThreadPool` so queued work can cause worker creation when the pool is under capacity and so worker creation does not depend on idle existing workers.

**Architecture:** Preserve the current auto-growth behavior where overload may increase `max_threads`. When work is queued and `worker_count < capacity`, call `flush_capacity()`. For capacity increases, create the bootstrap worker out-of-band with `root_spawn` instead of queueing `AddThread` behind guest work.

**Tech Stack:** Rust 2024, `flume`, `parking_lot`, unit tests in `wasi_virt_layer/src/wasi/thread.rs`.

## Global Constraints

- Rust edition is 2024; minimum rustc is 1.89.0.
- Keep changes focused on `VirtualThreadPool` expansion behavior.
- Do not change import/export ABI names.
- Do not commit unless explicitly requested.

---

### Task 1: Add Regression Tests

**Files:**
- Modify: `wasi_virt_layer/src/wasi/thread.rs`
- Test: `wasi_virt_layer/src/wasi/thread.rs`

**Interfaces:**
- Consumes: `VirtualThreadPool::init`, `VirtualThreadPool::new_thread`, `ThreadRunner::__new`.
- Produces: tests proving lazy spin-up and expansion while a worker is blocked.

- [ ] **Step 1: Add native test helper to execute `ThreadRunner` closures**

Inside `#[cfg(test)] mod tests`, update `TestThreadAccessor::call_wasi_thread_start` so native tests execute the boxed closure from `ThreadRunner`.

- [ ] **Step 2: Add failing test for `init()` plus capacity spin-up**

Add `virtual_thread_pool_spawns_worker_for_queued_task_when_under_capacity`.

- [ ] **Step 3: Add failing test for expansion while the only worker is blocked**

Add `virtual_thread_pool_expands_when_existing_worker_is_blocked`.

- [ ] **Step 4: Verify RED**

Run: `cargo test -p wasi_virt_layer virtual_thread_pool_spawns_worker_for_queued_task_when_under_capacity virtual_thread_pool_expands_when_existing_worker_is_blocked -- --nocapture`

Expected: at least one test fails because queued tasks do not get a worker under the current scheduler.

---

### Task 2: Implement案A Scheduler Fix

**Files:**
- Modify: `wasi_virt_layer/src/wasi/thread.rs:525-672`
- Test: `wasi_virt_layer/src/wasi/thread.rs`

**Interfaces:**
- Consumes: existing `flush_capacity()` and `run()` APIs.
- Produces: `run()` calls `flush_capacity()` when queued work exists and `worker_count < capacity`; `flush_capacity()` uses out-of-band bootstrap spawning for capacity increases.

- [ ] **Step 1: Simplify increasing branch in `flush_capacity()`**

Remove the queue-delivered `AddThread` path for increases. Always spawn one bootstrap worker with `root_spawn`, pass it `AddThread(count - 1, ...)`, push its handle into `kept_workers_pool`, and return `WaitThreadJoin::Recv(recv)`.

- [ ] **Step 2: Spin up under-capacity pools from `run()`**

In `run()`, when `need_expansion` is true, call `flush_capacity()` if `current < max`. Keep existing `current >= max` CAS auto-growth behavior.

- [ ] **Step 3: Verify GREEN**

Run: `cargo test -p wasi_virt_layer virtual_thread_pool -- --nocapture`

Expected: all `virtual_thread_pool` unit tests pass.

- [ ] **Step 4: Broader verification**

Run: `cargo check -r`

Expected: workspace check completes successfully.
