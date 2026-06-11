

use assert_cmd::Command;
use uuid::Uuid;

#[test]
fn test_unwind_flags() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    let out_dir = format!("tests/onetime/{}", Uuid::new_v4());

    let mut cmd = Command::cargo_bin("wasi_virt_layer")?;
    cmd.args([
        "build",
        "-p",
        "lfs_api_test_vfs",
        "test_wasm",
        "-t",
        "single",
        "--out-dir",
        &out_dir,
        "--dev",
        "--vfs-unwind",
        "--wasm-unwind",
        "true",
        "--adjust-abi",
    ]);

    cmd.assert().success();

    // Clean up
    let _ = std::fs::remove_dir_all(&out_dir);

    Ok(())
}

#[test]
fn test_unwind_target_compile_with_unwind() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    let out_dir = format!("tests/onetime/{}", Uuid::new_v4());

    let mut cmd = Command::cargo_bin("wasi_virt_layer")?;
    // Build the unwinding target with unwind enabled.
    // Deno execution is currently disabled because `wit-component` and `walrus` 
    // strip the Wasm Exception Handling `tag` section, causing `Invalid tag index: 0` at runtime.
    cmd.args([
        "build",
        "-p",
        "test_unwind_vfs",
        "test_unwind_target",
        "-t",
        "single",
        "--out-dir",
        &out_dir,
        "--dev",
        "--vfs-unwind",
        "--wasm-unwind",
        "true",
    ]);

    cmd.assert().success();

    // Clean up
    let _ = std::fs::remove_dir_all(&out_dir);

    Ok(())
}

#[test]
fn test_unwind_target_compile_without_unwind() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    let out_dir = format!("tests/onetime/{}", Uuid::new_v4());

    let mut cmd = Command::cargo_bin("wasi_virt_layer")?;
    // Build the unwinding target with unwind disabled.
    // It should compile successfully, but would abort at runtime since `catch_unwind` won't catch panic.
    cmd.args([
        "build",
        "-p",
        "test_unwind_vfs",
        "test_unwind_target",
        "-t",
        "single",
        "--out-dir",
        &out_dir,
        "--dev",
    ]);

    cmd.assert().success();

    // Clean up
    let _ = std::fs::remove_dir_all(&out_dir);

    Ok(())
}
