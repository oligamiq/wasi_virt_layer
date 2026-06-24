pub mod utils;
use utils::*;


const ATOMIC_WAIT_RESET_TARGET_WAT: &str = r#"
(module
  (import "env" "__wasip1_vfs_wasi_thread_spawn_wrapper"
    (func $thread_spawn (param i32) (result i32)))

  (memory (export "memory") 1 1 shared)

  ;; addr 0:  handshake state (0=init, 1=ready, 3=done)
  ;; addr 4:  park address / go signal (0=init, 2=go)
  ;; addr 16: cookie (0xDEAD=valid, 0=zeroed by reset)

  (func $start
    (i32.atomic.store align=4 (i32.const 0) (i32.const 0))
    (i32.atomic.store align=4 (i32.const 4) (i32.const 0)))
  (start $start)

  (func $_start (export "_start")
    (i32.atomic.store align=4 (i32.const 0) (i32.const 0))
    (i32.atomic.store align=4 (i32.const 4) (i32.const 0))
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
              (i32.atomic.load align=4 (i32.const 0))
              (i64.const 100000000)))
          (br $wait_ready)))))

  (func $__main_void (export "__main_void") (result i32)
    (i32.atomic.store align=4 (i32.const 4) (i32.const 2))
    (drop (memory.atomic.notify (i32.const 4) (i32.const 1)))
    (loop $wait_done
      (if
        (i32.ne
          (i32.atomic.load align=4 (i32.const 0))
          (i32.const 3))
        (then
          (drop
            (memory.atomic.wait32 align=4
              (i32.const 0)
              (i32.atomic.load align=4 (i32.const 0))
              (i64.const 100000000)))
          (br $wait_done))))
    (i32.const 0))

  (func $wasi_thread_start (export "wasi_thread_start")
    (param $thread_id i32)
    (param $start_arg i32)
    (i32.store align=4 (i32.const 16) (i32.const 0xDEAD))
    (i32.atomic.store align=4 (i32.const 0) (i32.const 1))
    (drop (memory.atomic.notify (i32.const 0) (i32.const 1)))
    (drop
      (memory.atomic.wait32 align=4
        (local.get $start_arg)
        (i32.const 0)
        (i64.const -1)))
    (if
      (i32.eq
        (i32.load align=4 (i32.const 16))
        (i32.const 0xDEAD))
      (then
        (i32.atomic.store align=4 (i32.const 0) (i32.const 3))
        (drop (memory.atomic.notify (i32.const 0) (i32.const 1)))))))
"#;

#[test]
fn test_atomic_wait_reset() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    if !has_required_wasi_targets(true) {
        return Ok(());
    }

    let target_dir = tempfile::tempdir()?;
    let target_path = target_dir.path().join("atomic_wait_reset_target.wasm");
    std::fs::write(&target_path, wat::parse_str(ATOMIC_WAIT_RESET_TARGET_WAT)?)?;
    let target_path = target_path
        .to_str()
        .ok_or_else(|| color_eyre::eyre::eyre!("target path is not UTF-8"))?;

    let dir = run_wasi_virt_layer(
        Some("atomic_wait_reset_vfs"),
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

    assert!(stdout.contains("Starting atomic wait reset test"));
    assert!(stdout.contains("Atomic wait reset test passed"));

    Ok(())
}
