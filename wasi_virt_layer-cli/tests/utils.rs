use assert_cmd::{assert::OutputAssertExt as _, Command};
use uuid::Uuid;

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

pub enum OutDir<'a> {
    Default,
    Path(&'a str),
    Random,
}

#[allow(clippy::too_many_arguments)]
pub fn run_wasi_virt_layer(
    p_vfs: Option<&str>,
    wasm: Option<&str>,
    t_single: Option<bool>,
    threads: bool,
    out_dir: OutDir,
    other_args: &[&str],
) -> color_eyre::Result<()> {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("wasi_virt_layer");

    if let Some(p_vfs) = p_vfs {
        cmd.args(["-p", p_vfs]);
    }
    if let Some(wasm) = wasm {
        cmd.arg(wasm);
    }

    if let Some(t_single) = t_single {
        cmd.args(["-t", if t_single { "single" } else { "multi" }]);
    }

    if threads {
        cmd.args(["--threads", "true"]);
    }

    let out_dir_path = match out_dir {
        OutDir::Default => format!("{THIS_FOLDER}/dist"),
        OutDir::Path(p) => p.to_string(),
        OutDir::Random => format!("{THIS_FOLDER}/{}/dist", Uuid::new_v4()),
    };
    cmd.args(["--out-dir", &out_dir_path]);

    if !other_args.is_empty() {
        cmd.args(other_args);
    }

    cmd.current_dir(THIS_FOLDER).assert().try_success()?;

    if threads {
        run_thread(&out_dir_path)?;
    } else {
        run_non_thread(&out_dir_path)?;
    }

    Ok(())
}

