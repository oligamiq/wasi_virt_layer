pub mod utils;
use utils::*;

struct RemoveDirOnDrop(String);

impl Drop for RemoveDirOnDrop {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn test_own_memory_smoke_new_example() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    if !has_required_wasi_targets(true) {
        return Ok(());
    }

    // wasi_virt_layer build -p smoke_vfs smoke_target --own-memory --validate --dev --threads true
    let dir = run_wasi_virt_layer(
        Some("smoke_vfs"),
        Some("smoke_target"),
        None,
        true, // Enable threads
        OutDir::Random,
        false,
        &["--own-memory", "--validate"],
        None,
    )
    .expect("Build with --own-memory and --validate should succeed for smoke example with threads");

    // Verify program output
    let stdout = std::fs::read_to_string(dir.0.join(".deno-test-stdout.log"))?;
    println!("Captured stdout:\n{}", stdout);

    assert!(stdout.contains("Starting smoke_target with threads..."));
    assert!(stdout.contains("Expanding memory by 3200 pages"));
    assert!(stdout.contains("Hello from a child thread!"));
    assert!(stdout.contains("VFS: inside root_spawn closure"));
    assert!(stdout.contains("[WASI main] done."));

    Ok(())
}

#[test]
fn test_own_memory_smoke_auto_detects_exports_without_flag() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    if !has_required_wasi_targets(true) {
        return Ok(());
    }

    let out_root = format!("{THIS_FOLDER}/onetime/{}", uuid::Uuid::new_v4());
    let _cleanup = RemoveDirOnDrop(out_root.clone());
    let out_dir = format!("{out_root}/dist");
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("wasi_virt_layer");
    cmd.args([
        "build",
        "-p",
        "smoke_vfs",
        "--features",
        "wasi_virt_layer/own-memory",
        "smoke_target",
        "--threads",
        "true",
        "--out-dir",
        &out_dir,
        "--dev",
        "--validate",
    ]);
    cmd.current_dir(THIS_FOLDER).assert().success();

    run_thread(&out_dir, std::time::Duration::from_secs(120), false)
        .expect("generated smoke example should run after auto-detected own-memory lowering");

    let stdout = std::fs::read_to_string(format!("{out_dir}/.deno-test-stdout.log"))?;
    println!("Captured stdout:\n{}", stdout);

    assert!(stdout.contains("Starting smoke_target with threads..."));
    assert!(stdout.contains("Expanding memory by 3200 pages"));
    assert!(stdout.contains("Hello from a child thread!"));
    assert!(stdout.contains("VFS: inside root_spawn closure"));
    assert!(stdout.contains("[WASI main] done."));

    Ok(())
}

