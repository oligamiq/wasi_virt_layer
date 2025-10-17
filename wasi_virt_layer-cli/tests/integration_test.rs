use assert_cmd::Command;

// cargo r -r -- -p example_vfs examples/test_wasm/example/test_wasm_opt.wasm
// cargo r -r -- -p threads_vfs test_threads -t single --threads true

pub mod utils;
use camino::Utf8PathBuf;
use eyre::Context;
use utils::*;
use wasi_virt_layer_cli::util;

static MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

// alloc
// multi_memory
// std
// threads
// unstable_print_debug
// multi_memory + std
// multi_memory + threads
// multi_memory + unstable_print_debug
// threads + unstable_print_debug
// multi_memory + threads + unstable_print_debug

#[test]
fn test_build_out_dir() -> color_eyre::Result<()> {
    let _lock = MUTEX.lock().unwrap();
    color_eyre::install().ok();

    build_out_dir().wrap_err("Failed to build with out-dir")?;
    println!("Out dir build done.");

    core::mem::drop(_lock);

    Ok(())
}

#[test]
fn test_build_multi() -> color_eyre::Result<()> {
    let _lock = MUTEX.lock().unwrap();
    color_eyre::install().ok();

    build_normal(false).wrap_err("Failed to build normal multi")?;
    println!("Normal multi build done.");
    build_threads(false).wrap_err("Failed to build threads multi")?;
    println!("Threads multi build done.");

    core::mem::drop(_lock);

    Ok(())
}

#[test]
fn test_build_single() -> color_eyre::Result<()> {
    let _lock = MUTEX.lock().unwrap();
    color_eyre::install().ok();

    build_normal(true).wrap_err("Failed to build normal single")?;
    println!("Normal single build done.");
    build_threads(true).wrap_err("Failed to build threads single")?;
    println!("Threads single build done.");

    core::mem::drop(_lock);

    Ok(())
}

fn build_normal(single: bool) -> color_eyre::Result<()> {
    Command::cargo_bin("wasi_virt_layer")?
        .args([
            "-p",
            "example_vfs",
            "test_wasm",
            "-t",
            if single { "single" } else { "multi" },
        ])
        .current_dir(THIS_FOLDER)
        .assert()
        .try_success()?;

    utils::run_non_thread(&format!("{THIS_FOLDER}/dist"))?;

    Ok(())
}

fn build_out_dir() -> color_eyre::Result<()> {
    Command::cargo_bin("wasi_virt_layer")?
        .args([
            "-p",
            "example_vfs",
            "test_wasm",
            "-t",
            "single",
            "--out-dir",
            &format!("{THIS_FOLDER}/tmp/dist"),
        ])
        .current_dir(THIS_FOLDER)
        .assert()
        .try_success()?;

    utils::run_non_thread(&format!("{THIS_FOLDER}/tmp/dist"))?;

    Ok(())
}

fn build_threads(single: bool) -> color_eyre::Result<()> {
    Command::cargo_bin("wasi_virt_layer")?
        .args([
            "-p",
            "threads_vfs",
            "test_threads",
            "-t",
            if single { "single" } else { "multi" },
            "--threads",
            "true",
            "--out-dir",
            &format!("{THIS_FOLDER}/threads/dist"),
        ])
        .current_dir(THIS_FOLDER)
        .assert()
        .try_success()?;

    utils::run_thread(&format!("{THIS_FOLDER}/threads/dist"))?;

    Ok(())
}

fn set_features<T>(features: &[&str], fn_: impl FnOnce() -> T) -> color_eyre::Result<T> {
    let manifest_path = Utf8PathBuf::from(EXAMPLE_DIR.to_owned() + "./vfs/no_std_vfs/Cargo.toml");
    let root_manifest_path = Utf8PathBuf::from(EXAMPLE_DIR.to_owned() + "./../Cargo.toml");
    let original = std::fs::read_to_string(&manifest_path)
        .wrap_err("Failed to read Cargo.toml for feature checking")?;
    features
        .iter()
        .map(|&feature| {
            wasi_virt_layer_cli::config_checker::FeatureChecker::new(
                feature,
                &manifest_path,
                &root_manifest_path,
                util::CRATE_NAME,
            )
        })
        .map(|c| c.set(true))
        .collect::<color_eyre::Result<Vec<_>>>()?;

    let t = fn_();

    let _resetter = Resetter {
        manifest_path: &manifest_path,
        original,
    };

    Ok(t)
}

struct Resetter<'a> {
    manifest_path: &'a Utf8PathBuf,
    original: String,
}

impl core::ops::Drop for Resetter<'_> {
    fn drop(&mut self) {
        std::fs::write(self.manifest_path, &self.original).unwrap();
    }
}

#[test]
fn all_features_without_threads() -> color_eyre::Result<()> {
    let _lock = MUTEX.lock().unwrap();
    color_eyre::install().ok();

    let run = || -> color_eyre::Result<()> {
        Command::cargo_bin("wasi_virt_layer")?
            .args(["-p", "no_std_vfs", "test_wasm"])
            .current_dir(THIS_FOLDER)
            .assert()
            .try_success()?;

        utils::run_non_thread(&format!("{THIS_FOLDER}/dist"))?;

        Ok(())
    };

    set_features(&[], run)
        .flatten()
        .wrap_err("Failed to run without features")?;
    set_features(&["alloc"], run)
        .flatten()
        .wrap_err("Failed to run with alloc")?;
    set_features(&["std"], run)
        .flatten()
        .wrap_err("Failed to run with std")?;
    set_features(&["multi_memory"], run)
        .flatten()
        .wrap_err("Failed to run with multi_memory")?;
    set_features(&["unstable_print_debug"], run)
        .flatten()
        .wrap_err("Failed to run with unstable_print_debug")?;
    set_features(&["multi_memory", "std"], run)
        .flatten()
        .wrap_err("Failed to run with multi_memory + std")?;
    set_features(&["multi_memory", "unstable_print_debug"], run)
        .flatten()
        .wrap_err("Failed to run with multi_memory + unstable_print_debug")?;

    Ok(())
}

#[test]
fn all_features_with_threads() -> color_eyre::Result<()> {
    let _lock = MUTEX.lock().unwrap();
    color_eyre::install().ok();

    let run = || -> color_eyre::Result<()> {
        Command::cargo_bin("wasi_virt_layer")?
            .args(["-p", "threads_vfs", "test_threads", "--threads", "true"])
            .current_dir(THIS_FOLDER)
            .assert()
            .try_success()?;

        utils::run_thread(&format!("{THIS_FOLDER}/threads/dist"))?;

        Ok(())
    };
    set_features(&["threads"], run)
        .flatten()
        .wrap_err("Failed to run without features")?;
    set_features(&["multi_memory", "threads"], run)
        .flatten()
        .wrap_err("Failed to run with multi_memory + threads")?;
    set_features(&["threads", "unstable_print_debug"], run)
        .flatten()
        .wrap_err("Failed to run with threads + unstable_print_debug")?;
    set_features(&["multi_memory", "threads", "unstable_print_debug"], run)
        .flatten()
        .wrap_err("Failed to run with multi_memory + threads + unstable_print_debug")?;
    Ok(())
}
