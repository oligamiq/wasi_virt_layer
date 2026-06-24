pub mod utils;
use utils::*;

#[test]
fn test_vfs_features_invalid() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    if !has_required_wasi_targets(false) {
        return Ok(());
    }

    // Try to build with a non-existent feature, which should fail
    let result = run_wasi_virt_layer(
        Some("minimal_repro"),
        Some("fixtures/c_target.wasm"),
        Some(false), // multi memory
        false,       // no threads
        OutDir::Random,
        false,
        &["--features", "non-existent-feature"],
        None,
    );

    assert!(
        result.is_err(),
        "Build should fail with non-existent feature"
    );
    let err_msg = format!("{}", result.unwrap_err());
    assert!(
        err_msg.contains("non-existent-feature") || err_msg.contains("Build failed"),
        "Error message should indicate build failure. Got: {}",
        err_msg
    );

    Ok(())
}

