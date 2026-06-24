pub mod utils;
use utils::*;

#[test]
fn test_lfs_api_operations() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    if !has_required_wasi_targets(false) {
        return Ok(());
    }

    // Build and run the lfs_api_test_vfs with a simple target
    // The target is the test_wasm package, built from source by cargo.
    // We use -p lfs_api_test_vfs to specify the VFS package
    run_wasi_virt_layer(
        Some("lfs_api_test_vfs"),
        Some("test_wasm"),
        Some(true), // t-single
        false,      // threads
        OutDir::Random,
        false, // keep_build_artifacts
        &["--run-with-opt"],
        None,
    )?;

    Ok(())
}

