pub mod utils;
use utils::*;

#[test]
fn test_spawn_main() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    if !has_required_wasi_targets(true) {
        return Ok(());
    }

    // 1. Test without feature (normal call)
    let dir_result = run_wasi_virt_layer(
        Some("spawn_main_vfs"),
        Some("spawn_main_target"),
        None,
        true, // Enable threads
        OutDir::Random,
        false,
        &["--validate"], // standard shared memory, no --own-memory
        None,
    );

    match dir_result {
        Ok(dir) => {
            let stdout = std::fs::read_to_string(dir.0.join(".deno-test-stdout.log"))?;
            println!("Captured stdout without feature:\n{}", stdout);

            assert!(stdout.contains("### Starting custom 8 threads test with VirtualThreadPool"));
            assert!(stdout.contains("### All custom 8 threads completed successfully."));
            assert!(!stdout.contains("Spawning main in a new thread."));
        }
        Err(e) => {
            println!("Runtime error (without feature): {}", e);
            return Err(e);
        }
    }

    // 2. Test with feature (spawned thread)
    let dir_result_with_feature = run_wasi_virt_layer(
        Some("spawn_main_vfs"),
        None,
        None,
        true, // Enable threads
        OutDir::Random,
        false,
        &[
            "--features",
            "spawn_main",
            "spawn_main_target",
            "--validate",
        ],
        None,
    );

    match dir_result_with_feature {
        Ok(dir) => {
            let stdout = std::fs::read_to_string(dir.0.join(".deno-test-stdout.log"))?;
            println!("Captured stdout with feature:\n{}", stdout);

            assert!(stdout.contains("Spawning main in a new thread."));
            assert!(stdout.contains("### Starting custom 8 threads test with VirtualThreadPool"));
            assert!(stdout.contains("### All custom 8 threads completed successfully."));
        }
        Err(e) => {
            println!("Runtime error (with feature): {}", e);
            return Err(e);
        }
    }

    Ok(())
}
