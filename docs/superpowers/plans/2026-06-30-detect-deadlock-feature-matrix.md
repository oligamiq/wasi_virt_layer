# Detect Deadlock Feature Matrix Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add integration coverage for representative `wasi_virt_layer/detect-deadlock` feature combinations.

**Architecture:** Reuse the existing `test_deadlock_detection` WAT false-positive target and VFS fixture for combinations that can run against a raw target. Cover `own-memory` with the existing `pool_own_mem_vfs` / `pool_own_mem_target` fixture because that fixture already exercises valid own-memory thread spawning. Add a matrix test that invokes the CLI with `--detect-deadlock` plus one extra structural or compiled VFS feature at a time and asserts the run finishes without `deadlock detected`.

**Tech Stack:** Rust 2024, Cargo integration tests, `cargo nextest`, WAT fixtures, `run_wasi_virt_layer` test helper.

## Global Constraints

- Rust edition is 2024; minimum rustc is 1.89.0.
- Use `OutDir::Random` for integration tests.
- Use `--validate` and `--detect-deadlock` in all new matrix cases.
- Keep matrix scope to representative feature combinations; do not add full powerset testing.
- Do not change detector runtime behavior unless a new matrix test exposes a concrete bug.
- Full clippy is known to fail on existing repository-wide lint/doc debt and is not a success criterion for this matrix-only change.

---

## File Structure

- Modify `wasi_virt_layer-cli/tests/test_deadlock_detection.rs`: add feature-matrix test cases around the existing false-positive WAT target, the existing own-memory pool fixture, and assertion helpers.

---

### Task 1: Detect Deadlock Feature Matrix Integration Coverage

**Files:**
- Modify: `wasi_virt_layer-cli/tests/test_deadlock_detection.rs`

**Interfaces:**
- Consumes: `run_wasi_virt_layer(...)` helper from `wasi_virt_layer-cli/tests/utils.rs`.
- Consumes: `FALSE_POSITIVE_TARGET_WAT` and `write_target(wat)` from `test_deadlock_detection.rs`.
- Consumes: `pool_own_mem_vfs` / `pool_own_mem_target` fixtures for the own-memory case.
- Produces: one integration test that validates `--detect-deadlock` with representative structural and compiled VFS feature combinations.

- [ ] **Step 1: Add a failing matrix test**

Add this test after `detector_allows_notified_wait_to_finish`:

```rust
#[test]
fn detector_feature_matrix_allows_notified_wait_to_finish() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    if !has_required_wasi_targets(true) {
        return Ok(());
    }

    let own_memory_dir = run_wasi_virt_layer(
        Some("pool_own_mem_vfs"),
        Some("pool_own_mem_target"),
        None,
        true,
        OutDir::Random,
        false,
        &["--own-memory", "--validate", "--detect-deadlock"],
        Some(Duration::from_secs(20)),
    )?;
    assert_pool_own_memory_without_deadlock("own-memory", &own_memory_dir)?;

    // VFS `--features` must appear before the target positional path; otherwise
    // feature_extractor assigns them to target options instead of VFS options.
    let matrix: &[(&str, Option<bool>, &[&str])] = &[
        ("single_memory", Some(true), &[]),
        ("multi_memory", Some(false), &[]),
        ("dynamic-fs", None, &["--features", "wasi_virt_layer/dynamic-fs"]),
        ("multiple-fs", None, &["--features", "wasi_virt_layer/multiple-fs"]),
        ("trace", None, &["--features", "wasi_virt_layer/trace"]),
        (
            "detect-wasi-reentrancy",
            None,
            &["--features", "wasi_virt_layer/detect-wasi-reentrancy"],
        ),
    ];

    for (name, target_single, feature_args) in matrix {
        let (_target_dir, target_path) = write_target(FALSE_POSITIVE_TARGET_WAT)?;
        let mut args = vec!["--validate", "--detect-deadlock"];
        args.extend_from_slice(feature_args);
        args.push(target_path.as_str());
        let dir = run_wasi_virt_layer(
            Some("deadlock_detection_vfs"),
            None,
            *target_single,
            true,
            OutDir::Random,
            false,
            &args,
            Some(Duration::from_secs(20)),
        )
        .map_err(|err| color_eyre::eyre::eyre!("feature matrix case {name} failed: {err:?}"))?;

        assert_success_without_deadlock(name, &dir)?;
    }

    Ok(())
}
```

- [ ] **Step 2: Extract shared success assertion helper**

Replace the inline stdout/stderr assertions in `detector_allows_notified_wait_to_finish` with this helper:

```rust
fn assert_success_without_deadlock(case_name: &str, dir: &TestDir) -> color_eyre::Result<()> {
    let stdout = std::fs::read_to_string(dir.0.join(".deno-test-stdout.log"))?;
    let stderr = std::fs::read_to_string(dir.0.join(".deno-test-stderr.log"))?;
    assert!(
        stdout.contains("Deadlock detection false-positive test passed"),
        "case {case_name} stdout did not contain success marker:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        !stdout.contains("deadlock detected"),
        "case {case_name} stdout contained deadlock diagnostic:\n{stdout}"
    );
    assert!(
        !stderr.contains("deadlock detected"),
        "case {case_name} stderr contained deadlock diagnostic:\n{stderr}"
    );
    Ok(())
}
```

Also add `assert_pool_own_memory_without_deadlock(case_name, dir)` with the same stdout/stderr `deadlock detected` checks and the own-memory success marker `All 5 threads completed successfully.`.

Then update `detector_allows_notified_wait_to_finish` to call:

```rust
assert_success_without_deadlock("baseline", &dir)?;
```

- [ ] **Step 3: Run the new test and verify failures are feature-related if any**

Run:

```bash
cargo test -p wasi_virt_layer-cli --test test_deadlock_detection detector_feature_matrix_allows_notified_wait_to_finish -- --nocapture
```

Expected before fixes: FAIL if structural CLI flags or VFS feature placement are missing; PASS after the matrix exercises the intended CLI paths.

- [ ] **Step 4: Make minimal fixes only if Step 3 exposes a concrete issue**

If a feature case fails because the CLI cannot pass feature args to the VFS, place VFS feature args before the target positional path. If a feature case is incompatible with the raw WAT fixture, use an existing fixture that validly exercises that representative combination.

- [ ] **Step 5: Run targeted integration verification**

Run:

```bash
cargo nextest run -r -p wasi_virt_layer-cli --test test_deadlock_detection --fail-fast
```

Expected: PASS for all tests in `test_deadlock_detection`.

- [ ] **Step 6: Run workspace check**

Run:

```bash
cargo check -r
```

Expected: PASS.

- [ ] **Step 7: Commit**

Run:

```bash
git add wasi_virt_layer-cli/tests/test_deadlock_detection.rs docs/superpowers/plans/2026-06-30-detect-deadlock-feature-matrix.md
git commit -m "test: cover deadlock detector feature matrix"
```

---

## Self-Review

- Spec coverage: The plan adds representative `detect-deadlock + X` integration coverage, including structural `--own-memory` and `-t multi` paths, and avoids powerset testing.
- Placeholder scan: no placeholders or TBDs remain.
- Type consistency: assertion helpers take `case_name: &str` and `dir: &TestDir`, matching `utils::TestDir` imported by `use utils::*`.
