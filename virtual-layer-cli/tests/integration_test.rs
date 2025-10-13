use assert_cmd::{Command, assert::OutputAssertExt as _};

// cargo r -r -- -p example_vfs examples/test_wasm/example/test_wasm_opt.wasm
// cargo r -r -- -p threads_vfs test_threads -t single --threads true

pub mod utils;
use utils::*;

#[test]
fn build_normal() -> color_eyre::Result<()> {
    Command::cargo_bin("wasip1_vfs-cli")?
        .args([
            "-p",
            "example_vfs",
            "test_wasm",
            "-t",
            "single",
            // "--out-dir",
            // &format!("{THIS_FOLDER}/dist"),
        ])
        .assert()
        .success();

    utils::run_non_thread(&format!("{THIS_FOLDER}/dist"))?;

    Ok(())
}
