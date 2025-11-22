// cargo r -r -- -p example_vfs examples/test_wasm/example/test_wasm_opt.wasm
// cargo r -r -- -p threads_vfs test_threads -t single --threads true

pub mod utils;
use camino::Utf8PathBuf;
use eyre::Context;
use utils::*;
use wasi_virt_layer_cli::util;

static MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn lock() -> std::sync::MutexGuard<'static, ()> {
    let mut count = 0;
    loop {
        if let Ok(guard) = MUTEX.lock() {
            return guard;
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
        count += 1;

        if count % 300 == 0 {
            println!("Waiting for lock...");
        }
    }
}

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
    let _lock = lock();
    color_eyre::install().ok();

    let _test_dir = build_out_dir().wrap_err("Failed to build with out-dir")?;
    println!("Out dir build done.");

    core::mem::drop(_lock);

    Ok(())
}

/// Tests the build process for both normal and threaded VFS in "multi" memory mode.
#[test]
fn test_build_multi() -> color_eyre::Result<()> {
    let _lock = lock();
    color_eyre::install().ok();

    let _test_dir_normal = build_normal(false).wrap_err("Failed to build normal multi")?;
    println!("Normal multi build done.");
    let _test_dir_threads = build_threads(false).wrap_err("Failed to build threads multi")?;
    println!("Threads multi build done.");

    core::mem::drop(_lock);

    Ok(())
}

/// Tests the build process for both normal and threaded VFS in "single" memory mode.
#[test]
fn test_build_single() -> color_eyre::Result<()> {
    let _lock = lock();
    color_eyre::install().ok();

    let _test_dir_normal = build_normal(true).wrap_err("Failed to build normal single")?;
    println!("Normal single build done.");
    let _test_dir_threads = build_threads(true).wrap_err("Failed to build threads single")?;
    println!("Threads single build done.");

    core::mem::drop(_lock);

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
        &[],
    )
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
                util::CRATE_NAME,
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
    let _lock = lock();
    color_eyre::install().ok();

    let run = || -> color_eyre::Result<TestDir> {
        run_wasi_virt_layer(
            Some("no_std_vfs"),
            Some("test_wasm"),
            None,
            false,
            OutDir::Random,
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

    core::mem::drop(_lock);

    Ok(())
}

/// Verifies that the `threads_vfs` can be compiled with various feature flag combinations that include the "threads" feature.
/// Each combination is run in an isolated directory.
#[test]
fn all_features_with_threads() -> color_eyre::Result<()> {
    let _lock = lock();
    color_eyre::install().ok();

    let run = || -> color_eyre::Result<TestDir> {
        run_wasi_virt_layer(
            Some("threads_vfs"),
            Some("test_threads"),
            None,
            true,
            OutDir::Random,
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

    core::mem::drop(_lock);

    Ok(())
}

/// Tests a specific edge case: a VFS that enables the "threads" feature flag in wasi_virt_layer
/// but does not export thread-related functions itself.
/// This ensures the build process succeeds even if the VFS doesn't fully utilize the threaded capabilities it enables.
#[test]
fn test_no_thread_with_thread_feature_vfs() -> color_eyre::Result<()> {
    let _lock = lock();
    color_eyre::install().ok();

    let fn_ = |m: bool| -> color_eyre::Result<TestDir> {
        run_wasi_virt_layer(
            Some("no_thread_with_thread_feature_vfs"),
            Some("test_wasm"),
            Some(m),
            true,
            OutDir::Random,
            &[],
        )
    };

    let _t1 = fn_(false).wrap_err("Failed to run no_thread_with_thread_feature_vfs single")?;
    let _t2 = fn_(true).wrap_err("Failed to run no_thread_with_thread_feature_vfs multi")?;

    core::mem::drop(_lock);

    Ok(())
}
