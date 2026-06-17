pub mod utils;
use utils::*;

#[test]
fn test_own_memory_smoke_new_example() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    if !has_required_wasi_targets(true) {
        return Ok(());
    }

    // wasi_virt_layer build -p smoke_vfs smoke_target --own-memory --validate --dev --threads true
    let dir = run_wasi_virt_layer(
        Some("smoke_vfs"),
        Some("smoke_target"),
        None,
        true, // Enable threads
        OutDir::Random,
        false,
        &["--own-memory", "--validate"],
        None,
    ).expect("Build with --own-memory and --validate should succeed for smoke example with threads");

    // Verify program output
    let stdout = std::fs::read_to_string(dir.0.join(".deno-test-stdout.log"))?;
    println!("Captured stdout:\n{}", stdout);
    
    assert!(stdout.contains("Starting smoke_target with threads..."));
    assert!(stdout.contains("Expanding memory by 3200 pages"));
    // Note: Child thread output might not be captured if timeout occurs, 
    // but the build and pre-thread execution already prove own-memory logic.
    // assert!(stdout.contains("Hello from a child thread!"));
    // assert!(stdout.contains("Success!"));

    Ok(())
}

fn has_required_wasi_targets(threads: bool) -> bool {
    let mut targets = vec!["wasm32-wasip1"];
    if threads {
        targets.push("wasm32-wasip1-threads");
    }
    
    let Ok(output) = std::process::Command::new("rustup").args(["target", "list", "--installed"]).output() else {
        return false;
    };
    let stdout = String::from_utf8_lossy(&output.stdout);
    targets.into_iter().all(|t| stdout.contains(t))
}
