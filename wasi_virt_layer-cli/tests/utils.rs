use std::{io::Read as _, process::Stdio, time::Duration};

use assert_cmd::assert::OutputAssertExt as _;
use camino::{Utf8Path, Utf8PathBuf};
use eyre::Context;
// Added Utf8Path
use uuid::Uuid;
use wait_timeout::ChildExt;

pub const EXAMPLE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../examples");
pub const THIS_FOLDER: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests");

pub fn run_non_thread(out_dir: &str, timeout: Duration) -> color_eyre::Result<()> {
    std::process::Command::new("deno")
        .args(["add", "npm:@bjorn3/browser_wasi_shim"])
        .current_dir(out_dir)
        .assert()
        .success();

    let mut child = std::process::Command::new("deno")
        .args(["run", "--allow-read", "--allow-env", "test_run.ts"])
        .current_dir(out_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let msg = match child.wait_timeout(timeout)? {
        Some(status) => {
            if status.success() {
                return Ok(());
            }

            let mut stdout = String::new();
            let mut stderr = String::new();

            if let Some(mut out) = child.stdout.take() {
                let _ = out.read_to_string(&mut stdout);
            }
            if let Some(mut err) = child.stderr.take() {
                let _ = err.read_to_string(&mut stderr);
            }

            // Check if this is a proc_exit error (which is expected behavior)
            if stderr.contains("exit with exit code 0") && stdout.contains("[WASI stdout]") {
                return Ok(());
            }

            format!("deno execution failed: {}\nstdout: {}\nstderr: {}", status, stdout, stderr)
        }
        None => {
            child.kill()?;
            let code = child.wait()?.code();
            format!("Process timed out after {:?} and was killed. Exit code: {:?}", timeout, code)
        }
    };

    Err(color_eyre::eyre::eyre!(msg))
}

pub fn run_thread(out_dir: &str, timeout: Duration) -> color_eyre::Result<()> {
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

    let msg = match child.wait_timeout(timeout)? {
        Some(status) => {
            if status.success() {
                return Ok(());
            }
            format!("Process exited with status: {status}")
        }
        None => {
            child.kill()?;
            let code = child.wait()?.code();
            format!("Process timed out after {:?} and was killed. Exit code: {:?}", timeout, code)
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

static TEST_DIRS: std::sync::OnceLock<
    std::sync::Arc<std::sync::Mutex<std::collections::HashSet<Utf8PathBuf>>>,
> = std::sync::OnceLock::new();

fn get_test_dirs() -> std::sync::Arc<std::sync::Mutex<std::collections::HashSet<Utf8PathBuf>>> {
    TEST_DIRS
        .get_or_init(|| {
            let dirs = std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashSet::<
                Utf8PathBuf,
            >::new()));
            let dirs_clone = dirs.clone();

            let _ = ctrlc::set_handler(move || {
                if let Ok(dirs) = dirs_clone.lock() {
                    for dir in dirs.iter() {
                        if let Some(parent) = dir.parent() {
                            if parent.starts_with(THIS_FOLDER)
                                && parent != Utf8Path::new(THIS_FOLDER)
                            {
                                let _ = std::fs::remove_dir_all(parent);
                            }
                        }
                    }
                }
                std::process::exit(130);
            });

            dirs
        })
        .clone()
}

impl TestDir {
    pub fn new(path: impl Into<Utf8PathBuf>) -> Self {
        let path = path.into();
        get_test_dirs().lock().unwrap().insert(path.clone());
        Self(path)
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        get_test_dirs().lock().unwrap().remove(&self.0);

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
    timeout: Option<Duration>,
) -> color_eyre::Result<TestDir> {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("wasi_virt_layer");
    cmd.arg("build");

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

    let cmd_line = {
        let mut args = vec!["cargo r -r --".to_string()];

        let mut skip_next = false;
        for a in cmd.get_args() {
            if skip_next {
                skip_next = false;
                continue;
            }

            let s = a.to_string_lossy();
            if s == "--out-dir" {
                skip_next = true;
                continue;
            }

            if s == "--lock-file" {
                skip_next = true;
                continue;
            }

            args.push(s.into_owned());
        }

        args.join(" ")
    };

    let result = || -> color_eyre::Result<TestDir> {
        cmd.current_dir(THIS_FOLDER).assert().try_success()?;

        println!("Output directory: {final_dist_path}");

        let execution_timeout = timeout.unwrap_or(if threads {
            Duration::from_secs(120)
        } else {
            Duration::from_secs(60)
        });

        if threads {
            run_thread(&final_dist_path, execution_timeout)?;
        } else {
            run_non_thread(&final_dist_path, execution_timeout)?;
        }

        println!("Test run successful");

        Ok(TestDir::new(final_dist_path))
    };

    result().context(format!(
        "Error with cmd {cmd:?} in dir {THIS_FOLDER}. Try running: {cmd_line}"
    ))
}
