pub mod utils;
use std::time::Duration;
use utils::*;

#[test]
fn test_virtual_thread_pool_new() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    if !has_required_wasi_targets(true) {
        return Ok(());
    }

    let dir_result = run_wasi_virt_layer(
        Some("vtp_5threads_vfs"),
        Some("vtp_5threads_target"),
        None,
        true, // Enable threads
        OutDir::Random,
        false,
        &["--validate"], // standard shared memory, no --own-memory
        None,
    );

    match dir_result {
        Ok(dir) => {
            // Verify program output
            let stdout = std::fs::read_to_string(dir.0.join(".deno-test-stdout.log"))?;
            println!("Captured stdout:\n{}", stdout);

            assert!(stdout.contains("Starting custom 5 threads test with VirtualThreadPool"));
            assert!(stdout.contains("All custom 5 threads completed successfully."));
            Ok(())
        }
        Err(e) => {
            println!("Runtime error: {}", e);
            Err(e)
        }
    }
}

#[test]
fn test_virtual_thread_pool_nested_spawn_starvation() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    if !has_required_wasi_targets(true) {
        return Ok(());
    }

    let dir = run_wasi_virt_layer(
        Some("vtp_nested_spawn_vfs"),
        Some("vtp_nested_spawn_target"),
        None,
        true,
        OutDir::Random,
        false,
        &["--validate"],
        Some(Duration::from_secs(10)),
    )?;

    let stdout = std::fs::read_to_string(dir.0.join(".deno-test-stdout.log"))?;
    println!("Captured stdout:\n{}", stdout);

    assert!(stdout.contains("Starting nested spawn VirtualThreadPool test"));
    assert!(stdout.contains("Nested spawn VirtualThreadPool test completed."));
    Ok(())
}
