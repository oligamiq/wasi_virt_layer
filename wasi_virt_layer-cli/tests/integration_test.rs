use assert_cmd::{Command, assert::OutputAssertExt as _};

// cargo r -r -- -p example_vfs examples/test_wasm/example/test_wasm_opt.wasm
// cargo r -r -- -p threads_vfs test_threads -t single --threads true

pub mod utils;
use eyre::Context;
use utils::*;

#[test]
fn build_all() -> color_eyre::Result<()> {
    color_eyre::install()?;

    build_out_dir().wrap_err("Failed to build with out-dir")?;
    println!("Out dir build done.");
    build_normal(false).wrap_err("Failed to build normal multi")?;
    println!("Normal multi build done.");
    build_threads(false).wrap_err("Failed to build threads multi")?;
    println!("Threads multi build done.");
    build_normal(true).wrap_err("Failed to build normal single")?;
    println!("Normal single build done.");
    build_threads(true).wrap_err("Failed to build threads single")?;
    println!("Threads single build done.");

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
        .success();

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
        .assert()
        .success();

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
        .assert()
        .success();

    utils::run_thread(&format!("{THIS_FOLDER}/threads/dist"))?;

    Ok(())
}
