pub mod utils;
use std::{collections::HashSet, process::Command, sync::OnceLock};
use utils::*;

static INSTALLED_TARGETS_STABLE: OnceLock<HashSet<String>> = OnceLock::new();

fn installed_targets() -> &'static HashSet<String> {
    INSTALLED_TARGETS_STABLE.get_or_init(|| {
        let output = Command::new("rustup")
            .args(["target", "list", "--installed"])
            .output();

        let Ok(output) = output else {
            return HashSet::new();
        };
        if !output.status.success() {
            return HashSet::new();
        }

        String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(ToOwned::to_owned)
            .collect()
    })
}

fn has_required_wasi_targets() -> bool {
    if !installed_targets().contains("wasm32-wasip1") {
        eprintln!(
            "Skipping test: missing rust target `wasm32-wasip1` (install with `rustup target add wasm32-wasip1`)"
        );
        return false;
    }
    true
}

#[test]
fn test_wait_poll() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    if !has_required_wasi_targets() {
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
        Some(std::time::Duration::from_secs(5)),
    )
    .expect("Failed to run WaitPoll test");

    Ok(())
}
