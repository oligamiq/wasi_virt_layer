use assert_cmd::{Command, assert::OutputAssertExt as _};

pub const EXAMPLE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../examples");
pub const THIS_FOLDER: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests");

pub fn run_non_thread(out_dir: &str) -> color_eyre::Result<()> {
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
