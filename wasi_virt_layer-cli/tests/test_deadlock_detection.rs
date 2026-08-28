pub mod utils;
use std::time::Duration;
use utils::*;

const DEADLOCK_TARGET_WAT: &str = r#"
(module
  (import "env" "__wasip1_vfs_wasi_thread_spawn_wrapper"
    (func $thread_spawn (param i32) (result i32)))

  (memory (export "memory") 1 1 shared)

  ;; addr 0: worker ready flag
  ;; addr 4: worker wait location
  ;; addr 8: main wait location

  (func $_start (export "_start")
    (drop (call $thread_spawn (i32.const 4)))
    (loop $wait_ready
      (if
        (i32.ne
          (i32.atomic.load align=4 (i32.const 0))
          (i32.const 1))
        (then
          (drop
            (memory.atomic.wait32 align=4
              (i32.const 0)
              (i32.const 0)
              (i64.const 100000000)))
          (br $wait_ready))))
    (drop
      (memory.atomic.wait32 align=4
        (i32.const 8)
        (i32.const 0)
        (i64.const -1))))

  (func $__main_void (export "__main_void") (result i32)
    (i32.const 0))

  (func $wasi_thread_start (export "wasi_thread_start")
    (param $thread_id i32)
    (param $start_arg i32)
    (i32.atomic.store align=4 (i32.const 0) (i32.const 1))
    (drop (memory.atomic.notify (i32.const 0) (i32.const 1)))
    (drop
      (memory.atomic.wait32 align=4
        (i32.const 4)
        (i32.const 0)
        (i64.const -1))))
)
"#;

const FALSE_POSITIVE_TARGET_WAT: &str = r#"
(module
  (import "env" "__wasip1_vfs_wasi_thread_spawn_wrapper"
    (func $thread_spawn (param i32) (result i32)))

  (memory (export "memory") 1 1 shared)

  ;; addr 0: worker ready flag
  ;; addr 4: worker wait location
  ;; addr 8: worker done flag

  (func $_start (export "_start")
    (drop (call $thread_spawn (i32.const 4)))
    (loop $wait_ready
      (if
        (i32.ne
          (i32.atomic.load align=4 (i32.const 0))
          (i32.const 1))
        (then
          (drop
            (memory.atomic.wait32 align=4
              (i32.const 0)
              (i32.const 0)
              (i64.const 100000000)))
          (br $wait_ready))))
    (i32.atomic.store align=4 (i32.const 4) (i32.const 1))
    (drop (memory.atomic.notify (i32.const 4) (i32.const 1)))
    (loop $wait_done
      (if
        (i32.ne
          (i32.atomic.load align=4 (i32.const 8))
          (i32.const 1))
        (then
          (drop
            (memory.atomic.wait32 align=4
              (i32.const 8)
              (i32.const 0)
              (i64.const 100000000)))
          (br $wait_done)))))

  (func $__main_void (export "__main_void") (result i32)
    (i32.const 0))

  (func $wasi_thread_start (export "wasi_thread_start")
    (param $thread_id i32)
    (param $start_arg i32)
    (i32.atomic.store align=4 (i32.const 0) (i32.const 1))
    (drop (memory.atomic.notify (i32.const 0) (i32.const 1)))
    (drop
      (memory.atomic.wait32 align=4
        (i32.const 4)
        (i32.const 0)
        (i64.const -1)))
    (i32.atomic.store align=4 (i32.const 8) (i32.const 1))
    (drop (memory.atomic.notify (i32.const 8) (i32.const 1))))
)
"#;

#[test]
fn deadlock_detection_traps_before_timeout() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    if !has_required_wasi_targets(true) {
        return Ok(());
    }

    let (_target_dir, target_path) = write_target(DEADLOCK_TARGET_WAT)?;
    let result = run_wasi_virt_layer(
        Some("deadlock_detection_vfs"),
        Some(target_path.as_str()),
        None,
        true,
        OutDir::Random,
        false,
        &["--validate", "--detect-deadlock"],
        Some(Duration::from_secs(20)),
    );

    let error = format!("{:?}", result.expect_err("deadlock fixture should fail"));
    assert!(error.contains("deadlock detected"), "{error}");
    assert!(!error.contains("Process timed out"), "{error}");
    Ok(())
}

#[test]
fn detector_allows_notified_wait_to_finish() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    if !has_required_wasi_targets(true) {
        return Ok(());
    }

    let (_target_dir, target_path) = write_target(FALSE_POSITIVE_TARGET_WAT)?;
    let dir = run_wasi_virt_layer(
        Some("deadlock_detection_vfs"),
        Some(target_path.as_str()),
        None,
        true,
        OutDir::Random,
        false,
        &["--validate", "--detect-deadlock"],
        Some(Duration::from_secs(20)),
    )?;

    assert_success_without_deadlock("baseline", &dir)?;
    Ok(())
}

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
    )
    .map_err(|err| color_eyre::eyre::eyre!("feature matrix case own-memory failed: {err:?}"))?;
    assert_pool_own_memory_without_deadlock("own-memory", &own_memory_dir)?;

    // VFS `--features` must appear before the target positional path; otherwise
    // feature_extractor assigns them to target options instead of VFS options.
    let matrix: &[(&str, Option<bool>, &[&str])] = &[
        ("single_memory", Some(true), &[]),
        ("multi_memory", Some(false), &[]),
        (
            "dynamic-fs",
            None,
            &["--features", "wasi_virt_layer/dynamic-fs"],
        ),
        (
            "multiple-fs",
            None,
            &["--features", "wasi_virt_layer/multiple-fs"],
        ),
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

fn assert_pool_own_memory_without_deadlock(
    case_name: &str,
    dir: &TestDir,
) -> color_eyre::Result<()> {
    let stdout = std::fs::read_to_string(dir.0.join(".deno-test-stdout.log"))?;
    let stderr = std::fs::read_to_string(dir.0.join(".deno-test-stderr.log"))?;
    assert!(
        stdout.contains("All 5 threads completed successfully."),
        "case {case_name} stdout did not contain own-memory success marker:\n{stdout}\nstderr:\n{stderr}"
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

fn write_target(wat: &str) -> color_eyre::Result<(tempfile::TempDir, camino::Utf8PathBuf)> {
    let target_dir = tempfile::tempdir()?;
    let target_path = target_dir.path().join("deadlock_detection_target.wasm");
    std::fs::write(&target_path, wat::parse_str(wat)?)?;
    let target_path = camino::Utf8PathBuf::from_path_buf(target_path)
        .map_err(|path| color_eyre::eyre::eyre!("target path is not UTF-8: {path:?}"))?;
    Ok((target_dir, target_path))
}
