pub mod utils;
use utils::*;

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

            assert!(stdout.contains("Starting 5 threads test with VirtualThreadPool and own-memory"));
            assert!(stdout.contains("All 5 threads completed successfully."));
            Ok(())
        }
        Err(e) => {
            println!("Runtime error: {}", e);
            Err(e)
        }
    }
}

fn has_required_wasi_targets(threads: bool) -> bool {
    let mut targets = vec!["wasm32-wasip1"];
    if threads {
        targets.push("wasm32-wasip1-threads");
    }

    let Ok(output) = std::process::Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output()
    else {
        return false;
    };
    let stdout = String::from_utf8_lossy(&output.stdout);
    targets.into_iter().all(|t| stdout.contains(t))
}
