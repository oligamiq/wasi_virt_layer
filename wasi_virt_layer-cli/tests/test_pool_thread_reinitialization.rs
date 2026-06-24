pub mod utils;
use utils::*;

const POOL_REUSED_START_TARGET_WAT: &str = r#"
(module
  (import "env" "__wasip1_vfs_wasi_thread_spawn_wrapper"
    (func $thread_spawn (param i32) (result i32)))

  (memory (export "memory") 1 1 shared)

  ;; The worker start section should run before each logical thread start.
  ;; It clears the marker that the previous logical thread dirtied.
  (func $start
    (i32.atomic.store align=4 (i32.const 8) (i32.const 0)))
  (start $start)

  (func $_start (export "_start")
    (i32.atomic.store align=4 (i32.const 0) (i32.const 0))
    (i32.atomic.store align=4 (i32.const 4) (i32.const 0))

    (drop (call $thread_spawn (i32.const 4)))
    (loop $wait_first
      (if
        (i32.ne
          (i32.atomic.load align=4 (i32.const 0))
          (i32.const 1))
        (then
          (drop
            (memory.atomic.wait32 align=4
              (i32.const 0)
              (i32.const 0)
              (i64.const -1)))
          (br $wait_first))))

    (drop (call $thread_spawn (i32.const 8)))
    (loop $wait_second
      (if
        (i32.ne
          (i32.atomic.load align=4 (i32.const 0))
          (i32.const 2))
        (then
          (drop
            (memory.atomic.wait32 align=4
              (i32.const 0)
              (i32.const 1)
              (i64.const -1)))
          (br $wait_second))))

    (if
      (i32.ne
        (i32.atomic.load align=4 (i32.const 4))
        (i32.const 0))
      (then unreachable)))

  (func $wasi_thread_start (export "wasi_thread_start")
    (param $thread_id i32)
    (param $start_arg i32)

    (if
      (i32.ne
        (i32.atomic.load align=4 (i32.const 8))
        (i32.const 0))
      (then
        (i32.atomic.store align=4 (i32.const 4) (i32.const 1))))

    (i32.atomic.store align=4 (i32.const 8) (local.get $start_arg))

    (if
      (i32.eq (local.get $start_arg) (i32.const 4))
      (then
        (i32.atomic.store align=4 (i32.const 0) (i32.const 1))
        (drop (memory.atomic.notify (i32.const 0) (i32.const 1))))
      (else
        (i32.atomic.store align=4 (i32.const 0) (i32.const 2))
        (drop (memory.atomic.notify (i32.const 0) (i32.const 1)))))))
"#;

#[test]
fn test_pool_reused_thread_runs_start_section_for_each_logical_thread() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    if !has_required_wasi_targets(true) {
        return Ok(());
    }

    let target_dir = tempfile::tempdir()?;
    let target_path = target_dir.path().join("pool_reused_start_target.wasm");
    std::fs::write(&target_path, wat::parse_str(POOL_REUSED_START_TARGET_WAT)?)?;
    let target_path = target_path
        .to_str()
        .ok_or_else(|| color_eyre::eyre::eyre!("target path is not UTF-8"))?;

    let dir = run_wasi_virt_layer(
        Some("pool_reused_start_vfs"),
        Some(target_path),
        None,
        true,
        OutDir::Random,
        false,
        &["--validate"],
        None,
    )?;

    let stdout = std::fs::read_to_string(dir.0.join(".deno-test-stdout.log"))?;
    println!("Captured stdout:\n{}", stdout);

    assert!(stdout.contains("Starting pool reused start-section test"));
    assert!(stdout.contains("Pool reused start-section test passed"));

    Ok(())
}

