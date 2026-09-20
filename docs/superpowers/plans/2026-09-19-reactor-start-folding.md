# Threaded reactor start folding implementation plan

**Goal:** Eliminate wit-component's reactor start-shim core module by calling the
official `_initialize` from WVL's synthesized start and removing its export.

**Architecture:** Keep the v0.6.1 order, replacing its manual thread initializer
slot with a direct call to Rust's reactor initializer. A WVL-owned shared atomic
word coordinates that call across core instances; the libc initializer traps on
repeated invocation, so its own guard is not an idempotent once primitive.

**Tech stack:** Rust, wasmparser/wasm-encoder/wit-component 0.252.0,
js-component-bindgen 2.0.9 (locked version), Deno.

## Constraints and initialization dependencies

- Threaded builds only. Missing `_initialize` retains the existing sequence.
- Never copy/inline the reactor body, remove its call, restore the old manual
  TLS initializer, or hide a second module in JS.
- `StartsPreStreamPass` exports the original Wasm start as `__flesh_vfs_start`.
  It performs linker memory initialization before reactor constructors.
- Execute: flesh VFS start, reactor initialization, offset globals, target memory
  snapshot, target starts, debug pre-init. This is the slot occupied by
  `thread_patch` / `wasi_thread_initializer` in tag v0.6.1.
- Unlike a Component reactor shim, core start executes before canonical host
  bindings can use this instance's exports. As with the old WVL start sequence,
  constructors must not depend on memory-dependent canonical imports or host
  re-entry during core instantiation.
- Add `__wasip1_vfs_reactor_init_state`, an aligned zero-initialized static
  AtomicU32, to threaded WVL builds. Its export supplies an immutable i32 address.
  State 0 = unclaimed, 1 = initializing, 2 = complete. CAS elects one initializer;
  other instances wait for completion. A trapping initializer leaves state 1:
  its shared runtime must be discarded, just as for a failed libc initialization.
- Require the state ABI for reactors being folded; incompatible prebuilt VFSs
  produce a rebuild diagnostic rather than silently producing broken workers.
- Preserve existing rubrc changes; verification outputs use fresh directories.

## Tasks

1. Add `tests/test_reactor_start_folding.rs` using WAT fixtures and the public
   PostCombineStreamPass. Assert validation, exact rebound call order, one
   `_initialize` call, absent export, component module count and JS core count.
   Include malformed exports/types, non-threaded and no-initialize cases.
   Run `cargo test -r -p wasi_virt_layer-cli --test test_reactor_start_folding` and
   observe failures before implementation.
2. Add the shared static ABI in `wasi_virt_layer/src/lib.rs`. Add a focused reactor
   resolver/emitter module alongside `post_combine.rs`; validate export type,
   uniqueness, VFS function ownership, static address and duplicate start calls.
   Validate ownership before merging; carry reserved function/state exports
   across optimizer reordering instead of relying on pre-optimization counts.
   Resolve into `ResolvedStartFuncs` and emit in `FnInStarts::emit_start_body`.
   Rename `_initialize` to an internal optimizer anchor; remove the state export
   on successful folding. Protect the function with selective native Binaryen
   `--no-inline` on every invocation and strip the anchor from the final core.
   The old fallback backend must reject this unsupported pass ordering explicitly.
3. Run structural tests and a Deno shared-memory instantiation test with a
   nonempty constructor and a trap-on-second-call initializer. Verify that
   original memory init runs before constructors and target starts see their
   effects, including worker instances and repeated synthesized start calls.
4. Run `cargo check -r`, `cargo nextest run -r --fail-fast`, and `cargo fmt
   --check`. Verify real threaded VFS generation with the supported installed
   nightly and inspect output for one core module, no module1 or second
   instantiateCore. Review the diff and document any environment blockers.

## Verification results

- `cargo check -r`: passed.
- `cargo nextest run -r --fail-fast`: 187 passed, 2 skipped. Includes the 14
  reactor regression tests, optimized/unoptimized threaded output, shared worker
  instances, worker reuse, and existing integration tests.
- The first suite run exposed the unwind fixture's missing `clock_time_get`
  connection. Adding its `plug_clock!(StandardClock, ...)` fixed that test.
- rubrc's actual five-target build with local WVL, nightly, `--dev --own-memory
  --vfs-unwind --threads true --keep-build-artifacts --validate` succeeded into
  `/tmp/opencode/rubrc-c2-retry`. Both component and final core validated. Core
  module count = 1, final initializer exports = 0, direct start call to the
  original initializer = 1. No `core2.wasm` or `module1`; one `instantiateCore`.
- Deno instantiation of rubrc's generated JS/core completed with shared memory:
  compile count = 1, core-module lookup count = 1, instantiation count = 1.
  The probe rejects host operations during core start; it does not claim to run
  rubrc's full compiler/LSP application workflow.
- An initial rubrc artifact had a one-byte opcode corruption outside reactor
  code. A fresh pipeline probe and normal rebuild did not reproduce it. The
  successful result above is from an ordinary rebuild, not a patched artifact.
- `git diff --check`: passed. Changed Rust files are rustfmt-formatted.
  Workspace `cargo fmt --check` still reports pre-existing formatting differences
  in `wasi_virt_layer/src/memory.rs`.
- `cargo clippy --all-targets --all-features -- -D warnings` reported 205 existing
  library lint errors (including raw-pointer API and missing-doc lints); the
  command then timed out waiting for remaining build jobs. Lint is not green.
