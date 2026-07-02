pub mod utils;
use utils::*;

const OWN_MEMORY_RESET_TARGET_WAT: &str = r#"
(module
  (memory (export "memory") 1 10)

  (func $_start (export "_start"))

  (func $__main_void (export "__main_void") (result i32)
    (drop (memory.grow (i32.const 2)))
    (i32.const 0))
)
"#;

#[test]
fn test_pool_own_memory() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    if !has_required_wasi_targets(true) {
        return Ok(());
    }

    let dir_result = run_wasi_virt_layer(
        Some("pool_own_mem_vfs"),
        Some("pool_own_mem_target"),
        None,
        true, // Enable threads
        OutDir::Random,
        false,
        &["--own-memory", "--validate"],
        None,
    );

    match dir_result {
        Ok(dir) => {
            // Verify program output
            let stdout = std::fs::read_to_string(dir.0.join(".deno-test-stdout.log"))?;
            println!("Captured stdout:\n{}", stdout);

            assert!(
                stdout.contains("Starting 5 threads test with VirtualThreadPool and own-memory")
            );
            assert!(stdout.contains("All 5 threads completed successfully."));
            Ok(())
        }
        Err(e) => {
            println!("Runtime error: {}", e);
            Err(e)
        }
    }
}

#[test]
fn test_own_memory_reset_restores_logical_size() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    if !has_required_wasi_targets(false) {
        return Ok(());
    }

    let target_dir = tempfile::tempdir()?;
    let target_path = target_dir.path().join("own_memory_reset_target.wasm");
    std::fs::write(&target_path, wat::parse_str(OWN_MEMORY_RESET_TARGET_WAT)?)?;
    let target_path = target_path
        .to_str()
        .ok_or_else(|| color_eyre::eyre::eyre!("target path is not UTF-8"))?;

    let dir = run_wasi_virt_layer(
        Some("own_memory_reset_vfs"),
        Some(target_path),
        None,
        false,
        OutDir::Random,
        false,
        &["--own-memory", "--validate"],
        None,
    )?;

    let stdout = std::fs::read_to_string(dir.0.join(".deno-test-stdout.log"))?;
    println!("Captured stdout:\n{}", stdout);

    assert!(stdout.contains("own-memory reset grown logical size = 3"));
    assert!(stdout.contains("own-memory reset after-reset logical size = 1"));
    assert!(stdout.contains("own-memory reset second-grown logical size = 3"));
    assert!(stdout.contains("own-memory reset logical-size test passed"));

    Ok(())
}

#[test]
fn test_own_memory_reset_clears_unreachable_wrapper_state() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    if !has_required_wasi_targets(false) {
        return Ok(());
    }

    let dir = run_wasi_virt_layer(
        Some("unreachable_reset_vfs"),
        Some("test_unreachable_target"),
        None,
        false,
        OutDir::Random,
        false,
        &["--own-memory", "--validate"],
        None,
    )?;

    let stdout = std::fs::read_to_string(dir.0.join(".deno-test-stdout.log"))?;
    println!("Captured stdout:\n{}", stdout);

    assert!(stdout.contains("unreachable reset initial flag = 0"));
    assert!(stdout.contains("unreachable reset after-first flag = 1"));
    assert!(stdout.contains("unreachable reset after-reset flag = 0"));
    assert!(stdout.contains("unreachable reset after-second flag = 1"));
    assert!(stdout.contains("unreachable reset flag test passed"));

    Ok(())
}
