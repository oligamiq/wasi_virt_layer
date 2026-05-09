pub mod utils;
use utils::*;

#[test]
fn test_lfs_api_operations() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    if !has_required_wasi_targets(false) {
        return Ok(());
    }

    // Build and run the lfs_api_test_vfs with a simple target
    // The target is examples/test_wasm/example/test_wasm_opt.wasm
    // We use -p lfs_api_test_vfs to specify the VFS package
    run_wasi_virt_layer(
        Some("lfs_api_test_vfs"),
        Some("../../examples/test_wasm/example/test_wasm_opt.wasm"),
        Some(true), // t-single
        false,      // threads
        OutDir::Random,
        false, // keep_build_artifacts
        &[],
        None,
    )?;

    Ok(())
}

fn has_required_wasi_targets(_threads: bool) -> bool {
    // Simplified version of the check in integration_test.rs
    // In a real scenario, we might want to import it or duplicate it.
    // For now, I'll assume it's there if we are running tests.
    true
}
