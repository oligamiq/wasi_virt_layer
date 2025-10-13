use assert_cmd::{Command, assert::OutputAssertExt};

const EXAMPLE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../examples");
const THIS_FOLDER: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests");

// cargo r -r -- -p example_vfs examples/test_wasm/example/test_wasm_opt.wasm
// cargo r -r -- -p threads_vfs test_threads -t single --threads true

#[test]
fn build_normal() -> color_eyre::Result<()> {
    Command::cargo_bin("wasip1_vfs-cli")?
        .args([
            "-p",
            "example_vfs",
            "test_wasm",
            "-t",
            "single",
            "--out-dir",
            &format!("{THIS_FOLDER}/dist"),
        ])
        .assert()
        .success();

    Command::new("deno")
        .args(["add", "npm:@bjorn3/browser_wasi_shim"])
        .assert()
        .success();

    std::process::Command::new("deno")
        .args([
            "run",
            "--allow-read",
            "--allow-env",
            &format!("{THIS_FOLDER}/dist/test_run.ts"),
        ])
        .assert()
        .success();

    Ok(())
}
