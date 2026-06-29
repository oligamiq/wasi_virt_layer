//! Component generation validation: verifies that components generated with
//! various memory configurations are valid according to `wasm-tools validate`.
//!
//! Covers spec B (Component 包括検証) — confirms that components with
//! shared/non-shared, single/multi, and stack-handoff memory layouts
//! all pass the component-model validator.

use camino::Utf8PathBuf;
use std::process::Command;
use uuid::Uuid;

mod utils;
use utils::*;

/// Helper to build with `--run-with-opt` (no --dev) so that the component
/// file is generated, then validate it.
fn build_and_validate_component(
    p_vfs: &str,
    wasm: &str,
    t_single: bool,
    threads: bool,
    other_args: &[&str],
) -> color_eyre::Result<()> {
    let mut combined = vec!["--run-with-opt"];
    combined.extend(other_args);

    let dist_dir = utils::run_wasi_virt_layer(
        Some(p_vfs),
        Some(wasm),
        Some(t_single),
        threads,
        OutDir::Random,
        false,
        &combined,
        None,
    )?;

    let out_dir = dist_dir.0.as_str();
    let component_path = Utf8PathBuf::from(out_dir).join(format!("{p_vfs}.component.wasm"));

    if component_path.exists() {
        let output = Command::new("wasm-tools")
            .args([
                "validate",
                "--features=component-model",
                component_path.as_str(),
            ])
            .output()
            .expect("failed to run wasm-tools");

        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            output.status.success(),
            "Component validation failed for {} ({})\nstderr:\n{}",
            p_vfs,
            component_path,
            stderr
        );
    }

    Ok(())
}

/// Build with `--dev` (the default) and validate the core wasm.
/// `other_args` should NOT include `--dev` or `--run-with-opt` —
/// `--dev` is added automatically by the test harness.
fn build_and_validate_dev(
    p_vfs: &str,
    wasm: &str,
    t_single: bool,
    threads: bool,
    other_args: &[&str],
) -> color_eyre::Result<()> {
    let dist_dir = utils::run_wasi_virt_layer(
        Some(p_vfs),
        Some(wasm),
        Some(t_single),
        threads,
        OutDir::Random,
        false,
        other_args,
        None,
    )?;

    let out_dir = dist_dir.0.as_str();
    let core_path = Utf8PathBuf::from(out_dir).join(format!("{p_vfs}.core.wasm"));

    if core_path.exists() {
        let output = Command::new("wasm-tools")
            .args(["validate", core_path.as_str()])
            .output()
            .expect("failed to run wasm-tools");

        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            output.status.success(),
            "Core wasm validation failed for {} ({})\nstderr:\n{}",
            p_vfs,
            core_path,
            stderr
        );
    }

    Ok(())
}

// ── Component validation tests ────────────────────────────────────────────

#[test]
fn validate_component_single_memory_no_threads() -> color_eyre::Result<()> {
    color_eyre::install().ok();
    if !has_required_wasi_targets(false) {
        return Ok(());
    }
    build_and_validate_component("example_vfs", "test_wasm", true, false, &[])?;
    Ok(())
}

#[test]
fn validate_component_multi_memory_no_threads() -> color_eyre::Result<()> {
    color_eyre::install().ok();
    if !has_required_wasi_targets(false) {
        return Ok(());
    }
    build_and_validate_component("example_vfs", "test_wasm", false, false, &[])?;
    Ok(())
}

#[test]
fn validate_component_single_memory_threads() -> color_eyre::Result<()> {
    color_eyre::install().ok();
    if !has_required_wasi_targets(true) {
        return Ok(());
    }
    build_and_validate_component("threads_vfs", "test_threads", true, true, &[])?;
    Ok(())
}

#[test]
fn validate_component_multi_memory_threads() -> color_eyre::Result<()> {
    color_eyre::install().ok();
    if !has_required_wasi_targets(true) {
        return Ok(());
    }
    build_and_validate_component("threads_vfs", "test_threads", false, true, &[])?;
    Ok(())
}

// ── Dev-flag combination tests (spec D) ───────────────────────────────────
// validate_component tests above already exercise --stack-size paths;
// these confirm core-wasm validity under --dev for key combinations.

/// Build-only helper (no deno execution) for configurations where
/// `run_wasi_virt_layer`'s runtime step would fail (e.g. own_memory).
fn build_only(
    p_vfs: &str,
    wasm: &str,
    t_single: bool,
    threads: bool,
    other_args: &[&str],
    keep_build_artifacts: bool,
) -> color_eyre::Result<Utf8PathBuf> {
    let out_dir = format!("{THIS_FOLDER}/onetime/{}/dist", Uuid::new_v4());
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin("wasi_virt_layer"));
    cmd.arg("build").args(["-p", p_vfs]).arg(wasm);
    if threads {
        cmd.args(["--threads", "true"]);
    }
    match t_single {
        true => {
            cmd.args(["-t", "single"]);
        }
        false => {
            cmd.args(["-t", "multi"]);
        }
    }
    cmd.args(["--dev", "--out-dir", &out_dir]);
    if keep_build_artifacts {
        cmd.arg("--keep-build-artifacts");
    }
    cmd.args(other_args);
    cmd.current_dir(THIS_FOLDER);

    let output = cmd.output()?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !output.status.success() {
        return Err(color_eyre::eyre::eyre!(
            "Build failed for {} with args {:?}\nstderr:\n{}",
            p_vfs,
            other_args,
            stderr
        ));
    }
    Ok(Utf8PathBuf::from(out_dir))
}

#[test]
fn dev_with_own_memory_validates() -> color_eyre::Result<()> {
    color_eyre::install().ok();
    if !has_required_wasi_targets(false) {
        return Ok(());
    }
    let out_dir = build_only(
        "own_memory_vfs",
        "big_alloc",
        false,
        false,
        &["--own-memory"],
        false,
    )?;
    let core_path = out_dir.join("own_memory_vfs.core.wasm");
    assert!(core_path.exists(), "core.wasm not found at {}", core_path);
    let output = Command::new("wasm-tools")
        .args(["validate", core_path.as_str()])
        .output()?;
    assert!(
        output.status.success(),
        "core wasm invalid: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(())
}

#[test]
fn dev_with_threads_single_validates() -> color_eyre::Result<()> {
    color_eyre::install().ok();
    if !has_required_wasi_targets(true) {
        return Ok(());
    }
    build_and_validate_dev("threads_vfs", "test_threads", true, true, &[])?;
    Ok(())
}

// ── Stack size boundary value tests (spec E) ──────────────────────────────

#[test]
fn dev_rejects_zero_stack_size() -> color_eyre::Result<()> {
    color_eyre::install().ok();
    if !has_required_wasi_targets(true) {
        return Ok(());
    }
    let out_dir = format!("{THIS_FOLDER}/onetime/{}/dist", Uuid::new_v4());
    let output = Command::new(assert_cmd::cargo::cargo_bin("wasi_virt_layer"))
        .args([
            "build",
            "-p",
            "threads_vfs",
            "test_threads",
            "-t",
            "single",
            "--threads",
            "true",
            "--dev",
            "--out-dir",
            &out_dir,
            "--stack-size",
            "vfs=0",
        ])
        .current_dir(THIS_FOLDER)
        .output()?;
    assert!(
        !output.status.success(),
        "--stack-size vfs=0 should be rejected"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("greater than zero") || stderr.contains("must be greater"),
        "expected zero-rejection message, got: {}",
        stderr
    );
    Ok(())
}

#[test]
fn dev_accepts_minimal_stack_size() -> color_eyre::Result<()> {
    color_eyre::install().ok();
    if !has_required_wasi_targets(true) {
        return Ok(());
    }
    let out_dir = build_only(
        "threads_vfs",
        "test_threads",
        true,
        true,
        &["--stack-size", "vfs=1"],
        false,
    )?;
    let core_path = out_dir.join("threads_vfs.core.wasm");
    assert!(core_path.exists(), "core.wasm not found at {}", core_path);
    let output = Command::new("wasm-tools")
        .args(["validate", core_path.as_str()])
        .output()?;
    assert!(
        output.status.success(),
        "core wasm invalid with stack-size=1: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(())
}

#[test]
fn dev_accepts_large_stack_size() -> color_eyre::Result<()> {
    color_eyre::install().ok();
    if !has_required_wasi_targets(true) {
        return Ok(());
    }
    let out_dir = build_only(
        "threads_vfs",
        "test_threads",
        true,
        true,
        &["--stack-size", "vfs=16777216"],
        false,
    )?;
    let core_path = out_dir.join("threads_vfs.core.wasm");
    assert!(core_path.exists(), "core.wasm not found at {}", core_path);
    let output = Command::new("wasm-tools")
        .args(["validate", core_path.as_str()])
        .output()?;
    assert!(
        output.status.success(),
        "core wasm invalid with stack-size=16MiB: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(())
}
