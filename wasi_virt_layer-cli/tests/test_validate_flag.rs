pub mod utils;
use utils::*;

#[test]
fn test_validate_flag_success() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    if !has_required_wasi_targets(false) {
        return Ok(());
    }

    // Use a simple, known-working example to verify the --validate flag doesn't fail
    // on properly generated modules.
    let result = run_wasi_virt_layer(
        Some("args-vfs"),
        Some("args"),
        Some(true), // multi memory single
        false,      // no threads
        OutDir::Random,
        false,
        &["--validate"], // other_args
        None,
    );

    assert!(
        result.is_ok(),
        "Build with --validate failed: {:?}",
        result.err()
    );

    Ok(())
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
