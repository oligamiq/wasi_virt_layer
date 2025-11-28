use std::{io::Read as _, process::Stdio, time::Duration};

use assert_cmd::assert::OutputAssertExt as _;
use camino::{Utf8Path, Utf8PathBuf}; // Added Utf8Path
use uuid::Uuid;
use wait_timeout::ChildExt;

pub const EXAMPLE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../examples");
pub const THIS_FOLDER: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests");

pub fn run_non_thread(out_dir: &str) -> color_eyre::Result<()> {
    std::process::Command::new("deno")
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

    std::process::Command::new(bun_or_npm)
        .args(["i"])
        .current_dir(out_dir)
        .assert()
        .success();

    let mut child = std::process::Command::new(bun_or_npm)
        .args(["run", "run"])
        .current_dir(out_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let msg = match child.wait_timeout(Duration::from_secs(120))? {
        Some(status) => {
            if status.success() {
                return Ok(());
            }
            format!("Process exited with status: {status}")
        }
        None => {
            child.kill()?;
            let code = child.wait()?.code();
            format!("Process timed out and was killed. Exit code: {:?}", code)
        }
    };

    let mut stdout = String::new();
    let mut stderr = String::new();

    if let Some(mut out) = child.stdout.take() {
        let _ = out.read_to_string(&mut stdout);
    }
    if let Some(mut err) = child.stderr.take() {
        let _ = err.read_to_string(&mut stderr);
    }

    Err(color_eyre::eyre::eyre!(
        "{msg}\nSTDOUT:\n{stdout}\nSTDERR:\n{stderr}"
    ))
}

pub enum OutDir<'a> {
    Default,
    Path(&'a str),
    Random,
}

/// A wrapper around a directory path that is automatically deleted when it goes out of scope.
#[derive(Debug)]
pub struct TestDir(pub Utf8PathBuf);

impl TestDir {
    pub fn new(path: impl Into<Utf8PathBuf>) -> Self {
        Self(path.into())
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        // The path often contains a "dist" folder which is inside the actual temporary folder.
        // We need to delete the parent of "dist".
        if let Some(parent) = self.0.parent() {
            if parent.starts_with(THIS_FOLDER) && parent != Utf8Path::new(THIS_FOLDER) {
                let _ = std::fs::remove_dir_all(parent);
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn run_wasi_virt_layer(
    p_vfs: Option<&str>,
    wasm: Option<&str>,
    t_single: Option<bool>,
    threads: bool,
    out_dir: OutDir,
    keep_build_artifacts: bool,
    other_args: &[&str],
) -> color_eyre::Result<TestDir> {
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

    if keep_build_artifacts {
        cmd.arg("--keep-build-artifacts");
    }

    let out_dir_path = match out_dir {
        OutDir::Default => format!("{THIS_FOLDER}/dist"),
        OutDir::Path(p) => p.to_string(),
        OutDir::Random => format!("{THIS_FOLDER}/onetime/{}", Uuid::new_v4()),
    };
    let final_dist_path = format!("{out_dir_path}/dist");
    cmd.args(["--out-dir", &final_dist_path]);

    if !other_args.is_empty() {
        cmd.args(other_args);
    }

    cmd.current_dir(THIS_FOLDER).assert().try_success()?;

    println!("Output directory: {final_dist_path}");

    if threads {
        run_thread(&final_dist_path)?;
    } else {
        run_non_thread(&final_dist_path)?;
    }

    println!("Test run successful");

    Ok(TestDir::new(final_dist_path))
}
