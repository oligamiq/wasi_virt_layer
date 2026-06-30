# Detect Deadlock Feature Matrix Integration Test Design

## Goal

Add integration coverage for representative `wasi_virt_layer/detect-deadlock` feature combinations so the detector is exercised with the other feature families most likely to affect threaded VFS generation and runtime behavior.

## Scope

Use the existing `test_deadlock_detection` integration fixture and add matrix cases that run the false-positive target with `--detect-deadlock` plus one extra feature family at a time:

- `wasi_virt_layer/own-memory`
- `wasi_virt_layer/multi_memory`
- `wasi_virt_layer/dynamic-fs`
- `wasi_virt_layer/multiple-fs`
- `wasi_virt_layer/trace`
- `wasi_virt_layer/detect-wasi-reentrancy`

The current deadlock and false-positive tests remain the behavioral baseline. Matrix cases focus on build and runtime compatibility for successful execution; they must not report `deadlock detected`.

## Approach

Extend the existing test helper path by passing additional VFS cargo features through the CLI feature argument mechanism already used by `run_wasi_virt_layer`. Keep the same WAT false-positive target and `deadlock_detection_vfs` fixture to avoid adding redundant crates or target modules.

Each matrix test case should use `OutDir::Random`, `--validate`, `--detect-deadlock`, and a bounded timeout. Assertions should verify the success marker appears and stderr/stdout do not contain `deadlock detected`.

## Non-Goals

- Do not run the full feature powerset in integration tests.
- Do not add `cargo-hack` or CI tooling in this change.
- Do not change detector runtime behavior unless the new matrix tests expose a concrete bug.

## Verification

Run:

```bash
cargo test -p wasi_virt_layer-cli --test test_deadlock_detection -- --nocapture
cargo nextest run -r -p wasi_virt_layer-cli --test test_deadlock_detection --fail-fast
cargo check -r
```

Full clippy is known to fail on existing repository-wide lint/doc debt and is not a success criterion for this matrix-only change.

## Self-Review

- No placeholders remain.
- The scope is limited to representative `detect-deadlock` combinations.
- The design avoids the cost and flakiness of powerset integration testing.
