use assert_cmd::{Command, assert::OutputAssertExt};

const EXAMPLE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../examples");
const THIS_FOLDER: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests");

#[test]
fn build_normal() -> color_eyre::Result<()> {
    Command::cargo_bin("wasip1_vfs-cli")?
        .args([
            "-p",
            "example_vfs",
            &format!("{EXAMPLE_DIR}/test_wasm/example/test_wasm_opt.wasm"),
            "--out-dir",
            &format!("{THIS_FOLDER}/dist"),
        ])
        .assert()
        .success();

    // deno run --allow-read dist/test_run.ts
    std::process::Command::new("deno")
        .args([
            "run",
            "--allow-read",
            &format!("{THIS_FOLDER}/dist/test_run.ts"),
        ])
        .assert()
        .success();

    Ok(())
}
