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

