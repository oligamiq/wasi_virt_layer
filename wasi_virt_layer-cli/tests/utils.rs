use std::{io::Read as _, process::Stdio, time::Duration};

use assert_cmd::assert::OutputAssertExt as _;
use camino::{Utf8Path, Utf8PathBuf};
use eyre::Context;
// Added Utf8Path
use uuid::Uuid;
use wait_timeout::ChildExt;

pub const EXAMPLE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../examples");
pub const THIS_FOLDER: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests");
pub const THREAD_TEST_TOOLCHAIN: &str = "nightly-2026-08-27";

static INSTALLED_TARGETS_STABLE: std::sync::OnceLock<std::collections::HashSet<String>> =
    std::sync::OnceLock::new();
static INSTALLED_TARGETS_NIGHTLY: std::sync::OnceLock<std::collections::HashSet<String>> =
    std::sync::OnceLock::new();

fn installed_targets(nightly: bool) -> &'static std::collections::HashSet<String> {
    let list = || {
        let mut cmd = std::process::Command::new("rustup");
        cmd.args(["target", "list", "--installed"]);
        if nightly {
            cmd.args(["--toolchain", THREAD_TEST_TOOLCHAIN]);
        }
        let output = cmd.output();

        let Ok(output) = output else {
            return std::collections::HashSet::new();
        };
        if !output.status.success() {
            return std::collections::HashSet::new();
        }

        String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(ToOwned::to_owned)
            .collect()
    };

    if nightly {
        INSTALLED_TARGETS_NIGHTLY.get_or_init(list)
    } else {
        INSTALLED_TARGETS_STABLE.get_or_init(list)
    }
}

pub fn has_required_wasi_targets(threads: bool) -> bool {
    if !installed_targets(false).contains("wasm32-wasip1") {
        eprintln!(
            "Skipping test: missing rust target `wasm32-wasip1` (install with `rustup target add wasm32-wasip1`)"
        );
        return false;
    }

    if threads && !installed_targets(true).contains("wasm32-wasip1-threads") {
        eprintln!(
            "Skipping test: missing rust target `wasm32-wasip1-threads` for {THREAD_TEST_TOOLCHAIN} (install with `rustup target add --toolchain {THREAD_TEST_TOOLCHAIN} wasm32-wasip1-threads`)"
        );
        return false;
    }

    true
}

pub fn run_non_thread(out_dir: &str, timeout: Duration) -> color_eyre::Result<()> {
    std::process::Command::new("deno")
        .args(["add", "npm:@bjorn3/browser_wasi_shim@0.4"])
        .current_dir(out_dir)
        .assert()
        .success();

    let stdout_path = Utf8Path::new(out_dir).join(".deno-test-stdout.log");
    let stderr_path = Utf8Path::new(out_dir).join(".deno-test-stderr.log");
    let stdout_file = std::fs::File::create(&stdout_path)?;
    let stderr_file = std::fs::File::create(&stderr_path)?;

    let mut child = std::process::Command::new("deno")
        .args(["run", "--allow-read", "--allow-env", "test_run.ts"])
        .current_dir(out_dir)
        .stdout(Stdio::from(stdout_file))
        .stderr(Stdio::from(stderr_file))
        .spawn()?;

    let msg = match child.wait_timeout(timeout)? {
        Some(status) => {
            let stdout = std::fs::read_to_string(&stdout_path).unwrap_or_default();
            let stderr = std::fs::read_to_string(&stderr_path).unwrap_or_default();
            println!(
                "Process exited with {}.\nstdout: {}\nstderr: {}",
                status, stdout, stderr
            );

            if status.success() {
                return Ok(());
            }

            format!(
                "deno execution failed with status: {}\nstdout: {}\nstderr: {}",
                status, stdout, stderr
            )
        }
        None => {
            child.kill()?;
            let code = child.wait()?.code();
            let stdout = std::fs::read_to_string(&stdout_path).unwrap_or_default();
            let stderr = std::fs::read_to_string(&stderr_path).unwrap_or_default();
            format!(
                "Process timed out after {:?} and was killed. Exit code: {:?}\nstdout: {}\nstderr: {}",
                timeout, code, stdout, stderr
            )
        }
    };

    Err(color_eyre::eyre::eyre!(msg))
}

pub fn run_thread(
    out_dir: &str,
    timeout: Duration,
    _is_single_memory: bool,
) -> color_eyre::Result<()> {
    let bun_or_npm = if std::process::Command::new("bun")
        .arg("--version")
        .output()
        .is_ok()
    {
        "bun"
    } else {
        "npm"
    };

    let bun_tmpdir = Utf8Path::new(out_dir).join(".bun-tmp");
    std::fs::create_dir_all(&bun_tmpdir)?;

    std::process::Command::new(bun_or_npm)
        .args(["i"])
        .current_dir(out_dir)
        .env("BUN_TMPDIR", bun_tmpdir.as_str())
        .assert()
        .success();

    let mut child = std::process::Command::new("deno")
        .args(["run", "-A", "test_run.ts"])
        .current_dir(out_dir)
        .env("BUN_TMPDIR", bun_tmpdir.as_str())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let msg = match child.wait_timeout(timeout)? {
        Some(status) => {
            use std::io::Read;
            let mut stdout = String::new();
            if let Some(mut s) = child.stdout.take() {
                s.read_to_string(&mut stdout).unwrap_or_default();
            }
            let mut stderr = String::new();
            if let Some(mut s) = child.stderr.take() {
                s.read_to_string(&mut stderr).unwrap_or_default();
            }
            std::fs::write(
                Utf8Path::new(out_dir).join(".deno-test-stdout.log"),
                &stdout,
            )?;
            std::fs::write(
                Utf8Path::new(out_dir).join(".deno-test-stderr.log"),
                &stderr,
            )?;
            println!(
                "Process exited with {}.\nstdout: {}\nstderr: {}",
                status, stdout, stderr
            );

            if status.success() {
                return Ok(());
            }
            format!("Process exited with status: {status}\nstdout: {stdout}\nstderr: {stderr}")
        }
        None => {
            child.kill()?;
            let code = child.wait()?.code();
            format!(
                "Process timed out after {:?} and was killed. Exit code: {:?}",
                timeout, code
            )
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
        // if let Some(parent) = self.0.parent() {
        //     if parent.starts_with(THIS_FOLDER) && parent != Utf8Path::new(THIS_FOLDER) {
        //         let _ = std::fs::remove_dir_all(parent);
        //     }
        // }
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
    run_wasi_virt_layer_inner(
        p_vfs,
        wasm,
        t_single,
        threads,
        threads,
        out_dir,
        keep_build_artifacts,
        other_args,
        timeout,
    )
}

#[allow(dead_code, clippy::too_many_arguments)]
pub fn run_wasi_virt_layer_with_thread_toolchain(
    p_vfs: Option<&str>,
    wasm: Option<&str>,
    t_single: Option<bool>,
    threads: bool,
    out_dir: OutDir,
    keep_build_artifacts: bool,
    other_args: &[&str],
    timeout: Option<Duration>,
) -> color_eyre::Result<TestDir> {
    run_wasi_virt_layer_inner(
        p_vfs,
        wasm,
        t_single,
        threads,
        true,
        out_dir,
        keep_build_artifacts,
        other_args,
        timeout,
    )
}

#[allow(clippy::too_many_arguments)]
fn run_wasi_virt_layer_inner(
    p_vfs: Option<&str>,
    wasm: Option<&str>,
    t_single: Option<bool>,
    threads: bool,
    use_thread_toolchain: bool,
    out_dir: OutDir,
    keep_build_artifacts: bool,
    other_args: &[&str],
    timeout: Option<Duration>,
) -> color_eyre::Result<TestDir> {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("wasi_virt_layer");
    cmd.arg("build");
    println!("COMMAND: {:?}", cmd.get_program());

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
    if use_thread_toolchain {
        // Threaded VFS modules are reactors/cdylibs, so exercise them with the
        // first nightly that contains the wasi-sdk 34 / rust-lang/rust#146843 fix.
        cmd.env("RUSTUP_TOOLCHAIN", THREAD_TEST_TOOLCHAIN);
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

    let mut use_dev = true;
    let mut filtered_args = Vec::new();
    for &arg in other_args {
        if arg == "--run-with-opt" {
            use_dev = false;
        } else {
            filtered_args.push(arg);
        }
    }

    if use_dev {
        cmd.arg("--dev");
    }

    if !filtered_args.is_empty() {
        cmd.args(filtered_args);
    }

    let cmd_line = {
        let mut args = vec![if use_thread_toolchain {
            format!("RUSTUP_TOOLCHAIN={THREAD_TEST_TOOLCHAIN} cargo r -r --")
        } else {
            "cargo r -r --".to_string()
        }];

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
            run_thread(
                &final_dist_path,
                execution_timeout,
                t_single.unwrap_or(false),
            )?;
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
