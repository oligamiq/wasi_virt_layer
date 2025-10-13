use assert_cmd::{Command, assert::OutputAssertExt as _};

pub const EXAMPLE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../examples");
pub const THIS_FOLDER: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests");

pub fn run_non_thread(out_dir: &str) -> color_eyre::Result<()> {
    Command::new("deno")
        .args(["add", "npm:@bjorn3/browser_wasi_shim"])
        .current_dir(out_dir)
        .assert()
        .success();

    std::process::Command::new("deno")
        .args(["run", "--allow-read", "--allow-env", "test_run.ts"])
        .current_dir(out_dir)
        .assert()
        .success();

    Ok(())
}

pub fn run_thread(out_dir: &str) -> color_eyre::Result<()> {
    let bun_or_npm = if std::process::Command::new("bun")
        .arg("--version")
        .output()
        .is_ok()
    {
        "bun"
    } else {
        "npm"
    };

    println!("Using {bun_or_npm} for threads");

    Command::new(bun_or_npm)
        .args(["i"])
        .current_dir(out_dir)
        .assert()
        .success();

    std::process::Command::new(bun_or_npm)
        .args(["run", "run"])
        .current_dir(out_dir)
        .assert()
        .success();

    Ok(())
}
