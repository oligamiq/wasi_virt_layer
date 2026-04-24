// cargo r -r -- -p example_vfs examples/test_wasm/example/test_wasm_opt.wasm
// cargo r -r -- -p threads_vfs test_threads -t single --threads true

pub mod utils;
use camino::Utf8PathBuf;
use eyre::Context;
use glob;
use itertools::Itertools;
use utils::*;
use uuid::Uuid;
use wasi_virt_layer_cli::unique_name::UniqueName;

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

/// Tests the build process with the `--out-dir` argument, ensuring output is directed to a specific temporary directory.
#[test]
fn test_build_out_dir() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    // rm onetime dir if it exists
    let _ = std::fs::remove_dir_all(format!("{THIS_FOLDER}/tmp"));

    let _test_dir = build_out_dir().wrap_err("Failed to build with out-dir")?;
    println!("Out dir build done.");

    Ok(())
}

/// Tests the build process for both normal and threaded VFS in "multi" memory mode.
#[test]
fn test_build_multi() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    let _test_dir_normal = build_normal(false).wrap_err("Failed to build normal multi")?;
    println!("Normal multi build done.");
    let _test_dir_threads = build_threads(false).wrap_err("Failed to build threads multi")?;
    println!("Threads multi build done.");

    Ok(())
}

/// Tests the build process for both normal and threaded VFS in "single" memory mode.
#[test]
fn test_build_single() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    let _test_dir_normal = build_normal(true).wrap_err("Failed to build normal single")?;
    println!("Normal single build done.");
    let _test_dir_threads = build_threads(true).wrap_err("Failed to build threads single")?;
    println!("Threads single build done.");

    Ok(())
}

/// Helper function to build a wasm component with a "normal" (non-threaded) VFS.
/// It uses the default output directory.
fn build_normal(single: bool) -> color_eyre::Result<TestDir> {
    run_wasi_virt_layer(
        Some("example_vfs"),
        Some("test_wasm"),
        Some(single),
        false,
        OutDir::Default,
        false, // keep_build_artifacts
        &[],
    )
}

/// Helper function to test the `--out-dir` argument.
/// It builds a wasm component and directs the output to a specific path.
fn build_out_dir() -> color_eyre::Result<TestDir> {
    run_wasi_virt_layer(
        Some("example_vfs"),
        Some("test_wasm"),
        Some(true),
        false,
        OutDir::Path(&format!("{THIS_FOLDER}/tmp/dist")),
        false, // keep_build_artifacts
        &[],
    )
}

/// Helper function to build a wasm component with a threaded VFS.
/// It uses a random output directory to ensure test isolation.
fn build_threads(single: bool) -> color_eyre::Result<TestDir> {
    run_wasi_virt_layer(
        Some("threads_vfs"),
        Some("test_threads"),
        Some(single),
        true,
        OutDir::Random,
        false, // keep_build_artifacts
        &[],
    )
}

/// Tests the self-virtualizing read/write VFS example in single-memory mode.
/// This validates that a VFS can act as both the virtualizer and an executable-style workload.
#[test]
fn test_self_rw_vfs_example() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    let _test_dir = run_wasi_virt_layer(
        Some("self-rw-vfs"),
        Some("ls"),
        Some(true),
        false,
        OutDir::Random,
        false,
        &[],
    )
    .wrap_err("Failed to run self-rw-vfs example")?;

    Ok(())
}

/// Tests the threaded self-virtualizing read/write VFS example in single-memory mode.
/// This validates that the threaded pair builds successfully.
///
/// Runtime execution is intentionally skipped here because the Node/Bun test runner backend
/// used by `run_thread` does not support `parking_lot` parking in this environment.
#[test]
fn test_self_rw_threads_vfs_example() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    let out_dir = format!("{THIS_FOLDER}/onetime/{}/dist", Uuid::new_v4());

    let mut cmd = std::process::Command::new(assert_cmd::cargo::cargo_bin("wasi_virt_layer"));
    cmd.current_dir(THIS_FOLDER).args([
        "build",
        "-p",
        "self-rw-threads-vfs",
        "self_rw_example",
        "-t",
        "single",
        "--threads",
        "true",
        "--out-dir",
        &out_dir,
    ]);

    let status = cmd.status().wrap_err("Failed to execute wasi_virt_layer")?;
    if !status.success() {
        return Err(color_eyre::eyre::eyre!(
            "self-rw-threads-vfs build failed with status: {status}"
        ));
    }

    let _test_dir = TestDir::new(Utf8PathBuf::from(out_dir));

    Ok(())
}

fn set_features_inner<T>(
    features: &[&str],
    p: &str,
    fn_: impl FnOnce() -> color_eyre::Result<T>,
) -> color_eyre::Result<T> {
    let manifest_path = Utf8PathBuf::from(EXAMPLE_DIR.to_owned() + "./vfs/" + p + "/Cargo.toml");
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
                UniqueName::CRATE_NAME,
            )
        })
        .map(|c| c.set(true))
        .collect::<color_eyre::Result<Vec<_>>>()?;

    let t = fn_()?; // Call fn_ and propagate error

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

/// Verifies that the `no_std_vfs` can be compiled with various feature flag combinations, excluding threads.
/// Each combination is run in an isolated directory.
#[test]
fn all_features_without_threads() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    let run = || -> color_eyre::Result<TestDir> {
        run_wasi_virt_layer(
            Some("no_std_vfs"),
            Some("test_wasm"),
            None,
            false,
            OutDir::Random,
            false, // keep_build_artifacts
            &[],
        )
    };

    fn set_features(
        features: &[&str],
        run: impl FnOnce() -> color_eyre::Result<TestDir>,
    ) -> color_eyre::Result<TestDir> {
        set_features_inner(features, "no_std_vfs", run)
    }

    let _t1 = set_features(&[], run).wrap_err("Failed to run without features")?;
    let _t2 = set_features(&["alloc"], run).wrap_err("Failed to run with alloc")?;
    let _t3 = set_features(&["std"], run).wrap_err("Failed to run with std")?;
    let _t4 = set_features(&["multi_memory"], run).wrap_err("Failed to run with multi_memory")?;
    let _t5 = set_features(&["unstable_print_debug"], run)
        .wrap_err("Failed to run with unstable_print_debug")?;
    let _t6 = set_features(&["multi_memory", "std"], run)
        .wrap_err("Failed to run with multi_memory + std")?;
    let _t7 = set_features(&["multi_memory", "unstable_print_debug"], run)
        .wrap_err("Failed to run with multi_memory + unstable_print_debug")?;

    Ok(())
}

/// Verifies that the `threads_vfs` can be compiled with various feature flag combinations that include the "threads" feature.
/// Each combination is run in an isolated directory.
#[test]
fn all_features_with_threads() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    let run = || -> color_eyre::Result<TestDir> {
        run_wasi_virt_layer(
            Some("threads_vfs"),
            Some("test_threads"),
            None,
            true,
            OutDir::Random,
            false, // keep_build_artifacts
            &[],
        )
    };

    fn set_features(
        features: &[&str],
        run: impl FnOnce() -> color_eyre::Result<TestDir>,
    ) -> color_eyre::Result<TestDir> {
        set_features_inner(features, "threads_vfs", run)
    }

    let _t1 = set_features(&["threads"], run).wrap_err("Failed to run without features")?;
    let _t2 = set_features(&["multi_memory", "threads"], run)
        .wrap_err("Failed to run with multi_memory + threads")?;
    let _t3 = set_features(&["threads", "unstable_print_debug"], run)
        .wrap_err("Failed to run with threads + unstable_print_debug")?;
    let _t4 = set_features(&["multi_memory", "threads", "unstable_print_debug"], run)
        .wrap_err("Failed to run with multi_memory + threads + unstable_print_debug")?;

    Ok(())
}

/// Tests a specific edge case: a VFS that enables the "threads" feature flag in wasi_virt_layer
/// but does not export thread-related functions itself.
/// This ensures the build process succeeds even if the VFS doesn't fully utilize the threaded capabilities it enables.
#[test]
fn test_no_thread_with_thread_feature_vfs() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    let fn_ = |m: bool| -> color_eyre::Result<TestDir> {
        run_wasi_virt_layer(
            Some("no_thread_with_thread_feature_vfs"),
            Some("test_wasm"),
            Some(m),
            true,
            OutDir::Random,
            false, // keep_build_artifacts
            &[],
        )
    };

    let _t1 = fn_(false).wrap_err("Failed to run no_thread_with_thread_feature_vfs single")?;
    let _t2 = fn_(true).wrap_err("Failed to run no_thread_with_thread_feature_vfs multi")?;

    Ok(())
}

/// Tests the `--keep-build-artifacts` argument.
#[test]
fn test_keep_build_artifacts() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    // Test with keep_build_artifacts = true
    let test_dir_keep = run_wasi_virt_layer(
        Some("example_vfs"),
        Some("test_wasm"),
        Some(true),
        false,
        OutDir::Random,
        true, // keep_build_artifacts
        &[],
    )
    .wrap_err("Failed to run with keep_build_artifacts = true")?;

    let parent_dir_keep = test_dir_keep.0.parent().unwrap();

    // Check for intermediate files
    let adjusted_wasm_files: Vec<_> = glob::glob(&format!("{parent_dir_keep}/**/*.adjusted.wasm"))?
        .filter_map(Result::ok)
        .collect();
    let opt_wasm_files: Vec<_> = glob::glob(&format!("{parent_dir_keep}/**/*.opt.wasm"))?
        .filter_map(Result::ok)
        .collect();

    assert!(
        !adjusted_wasm_files.is_empty(),
        "Expected .adjusted.wasm files to exist when keep_build_artifacts is true"
    );
    assert!(
        !opt_wasm_files.is_empty(),
        "Expected .opt.wasm files to exist when keep_build_artifacts is true"
    );

    // Test with keep_build_artifacts = false
    let test_dir_no_keep = run_wasi_virt_layer(
        Some("example_vfs"),
        Some("test_wasm"),
        Some(true),
        false,
        OutDir::Random,
        false, // keep_build_artifacts
        &[],
    )
    .wrap_err("Failed to run with keep_build_artifacts = false")?;

    let parent_dir_no_keep = test_dir_no_keep.0.parent().unwrap();

    // Check for intermediate files - should not exist
    let adjusted_wasm_files_no_keep: Vec<_> =
        glob::glob(&format!("{parent_dir_no_keep}/**/*.adjusted.wasm"))?
            .filter_map(Result::ok)
            .collect();
    let opt_wasm_files_no_keep: Vec<_> =
        glob::glob(&format!("{parent_dir_no_keep}/**/*.opt.wasm"))?
            .filter_map(Result::ok)
            .collect();

    assert!(
        adjusted_wasm_files_no_keep.is_empty(),
        "Expected no .adjusted.wasm files to exist when keep_build_artifacts is false"
    );
    assert!(
        opt_wasm_files_no_keep.is_empty(),
        "Expected no .opt.wasm files to exist when keep_build_artifacts is false"
    );

    Ok(())
}

/// Generates documentation for import/export name changes across generator stages.
/// This test is ignored by default and should be run manually when the documentation needs to be updated.
#[test]
#[ignore]
fn doc_gen_imports_exports() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    fn process_wasm_file(
        md_output: &mut String,
        wasm_path: &std::path::Path,
        stage_name: &str,
    ) -> color_eyre::Result<()> {
        md_output.push_str(&format!("### Stage: `{}`\n\n", stage_name));

        let wasm_path_str = wasm_path
            .to_str()
            .ok_or_else(|| eyre::eyre!("Invalid UTF-8 path"))?;
        let wat_output = std::process::Command::new("wasm-tools")
            .args(["print", wasm_path_str])
            .output()?;

        if !wat_output.status.success() {
            let stderr = String::from_utf8_lossy(&wat_output.stderr);
            md_output.push_str(&format!(
                "Failed to process `{}`. Error:\n```\n{}\n```\n\n",
                stage_name, stderr
            ));
            return Ok(());
        }

        let wat = String::from_utf8_lossy(&wat_output.stdout);

        let imports: Vec<_> = wat
            .lines()
            .filter(|line| line.trim().starts_with("(import"))
            .map(|line| {
                let parts: Vec<_> = line.split_whitespace().collect();
                format!(
                    "| `{}` | `{}` |",
                    parts.get(1).unwrap_or(&""),
                    parts.get(2).unwrap_or(&"")
                )
            })
            .collect();

        if !imports.is_empty() {
            md_output.push_str("#### Imports\n\n");
            md_output.push_str("| Module | Name |\n");
            md_output.push_str("|---|---|\n");
            md_output.push_str(&imports.join("\n"));
            md_output.push_str("\n\n");
        }

        let exports: Vec<_> = wat
            .lines()
            .filter(|line| line.trim().starts_with("(export"))
            .map(|line| {
                let parts: Vec<_> = line.split_whitespace().collect();
                format!("| `{}` |", parts.get(1).unwrap_or(&""))
            })
            .collect();

        if !exports.is_empty() {
            md_output.push_str("#### Exports\n\n");
            md_output.push_str("| Name |\n");
            md_output.push_str("|---|\n");
            md_output.push_str(&exports.join("\n"));
            md_output.push_str("\n\n");
        }

        if imports.is_empty() && exports.is_empty() {
            md_output.push_str("*No imports or exports found.*\n\n");
        }
        Ok(())
    }

    let feature_combos: Vec<(&str, &[&str])> = vec![
        ("no_features", &[]),
        ("alloc", &["alloc"]),
        ("std", &["std"]),
        ("multi_memory", &["multi_memory"]),
        ("unstable_print_debug", &["unstable_print_debug"]),
        ("multi_memory_std", &["multi_memory", "std"]),
        (
            "multi_memory_unstable_print_debug",
            &["multi_memory", "unstable_print_debug"],
        ),
    ];

    let mut md_output = String::new();
    md_output.push_str("# Wasm Import/Export Evolution (Detailed)\n\n");
    md_output.push_str("This document exhaustively tracks the changes in Wasm import and export names through each generator stage for different feature combinations.\n\n");

    for (name, features) in feature_combos {
        println!("Processing feature combination: {name}");
        md_output.push_str(&format!("## Feature Combination: `{}`\n\n", name));

        let run = || -> color_eyre::Result<TestDir> {
            // Manually build and inspect initial modules
            md_output.push_str("### Stage 0: Initial Modules\n\n");

            std::process::Command::new("cargo")
                .args([
                    "build",
                    "--release",
                    "--target",
                    "wasm32-wasip1",
                    "-p",
                    "no_std_vfs",
                ])
                .status()?;
            let vfs_path = std::path::PathBuf::from("target/wasm32-wasip1/release/no_std_vfs.wasm");
            process_wasm_file(&mut md_output, &vfs_path, "no_std_vfs.wasm (initial)")?;

            std::process::Command::new("cargo")
                .args([
                    "build",
                    "--release",
                    "--target",
                    "wasm32-wasip1",
                    "-p",
                    "test_wasm",
                ])
                .status()?;
            let test_wasm_path =
                std::path::PathBuf::from("target/wasm32-wasip1/release/test_wasm.wasm");
            process_wasm_file(&mut md_output, &test_wasm_path, "test_wasm.wasm (initial)")?;

            run_wasi_virt_layer(
                Some("no_std_vfs"),
                Some("test_wasm"),
                None,
                false,
                OutDir::Random,
                true, // keep_build_artifacts is crucial
                &[],
            )
        };

        let test_dir = set_features_inner(features, "no_std_vfs", run)?;
        let parent_dir = test_dir.0.parent().unwrap();

        md_output.push_str("### Post-Merge Stages\n\n");
        let wasm_files: Vec<_> = glob::glob(&format!("{}/**/*.wasm", parent_dir.as_str()))?
            .filter_map(Result::ok)
            .sorted_by_key(|p| p.metadata().unwrap().created().unwrap())
            .collect();

        if wasm_files.is_empty() {
            md_output.push_str("*No post-merge wasm files found for this combination.*\n\n");
            continue;
        }

        for wasm_path in wasm_files {
            let file_name = wasm_path.file_name().unwrap().to_str().unwrap();
            process_wasm_file(&mut md_output, &wasm_path, file_name)?;
        }
    }

    std::fs::write("IMPORTS_EXPORTS_EVOLUTION_DETAILED.md", md_output)?;
    Ok(())
}
