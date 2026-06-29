pub mod utils;
use utils::*;

#[test]
fn test_wait_poll() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    if !has_required_wasi_targets(false) {
        return Ok(());
    }

    let _test_dir = run_wasi_virt_layer(
        Some("wait_poll_vfs"),
        Some("test_poll"),
        Some(true), // single memory
        false,      // no threads
        OutDir::Random,
        false,
        &[],
        Some(std::time::Duration::from_secs(30)),
    )
    .expect("Failed to run WaitPoll test");

    Ok(())
}
