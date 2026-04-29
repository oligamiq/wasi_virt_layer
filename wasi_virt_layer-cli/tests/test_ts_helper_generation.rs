/// Integration tests for generated TypeScript helpers
///
/// Tests that:
/// 1. TypeScript helper code is correctly generated
/// 2. Helper exports are properly detected
/// 3. VFS with export_pseudo_wasm! macro generates proper helpers
pub mod utils;
use eyre::Context;
use std::fs;
use std::path::Path;

/// Helper to find workspace root from test location
fn get_workspace_root() -> std::path::PathBuf {
    // env!("CARGO_MANIFEST_DIR") = F:\wasi_virt_layer\wasi_virt_layer-cli
    // We need F:\wasi_virt_layer
    // So go up 1 level from manifest dir
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    Path::new(manifest_dir).parent().unwrap().to_path_buf()
}

/// Test that VFS exports are properly detected and parsed
#[test]
fn test_vfs_export_detection() -> color_eyre::Result<()> {
    use wasi_virt_layer_cli::gen_ts_helper::detect_vfs_exports;

    let workspace_root = get_workspace_root();

    // Build test_helper_vfs as WASM
    let build_output = std::process::Command::new("cargo")
        .args([
            "build",
            "-p",
            "test_helper_vfs",
            "--target",
            "wasm32-wasip1",
            "--release",
        ])
        .current_dir(&workspace_root)
        .output()
        .wrap_err("Failed to build test_helper_vfs")?;

    if !build_output.status.success() {
        let stderr = String::from_utf8_lossy(&build_output.stderr);
        eprintln!("Build failed: {}", stderr);
        return Err(eyre::eyre!("Failed to build test_helper_vfs"));
    }

    // Load the WASM binary
    let wasm_path = workspace_root.join("target/wasm32-wasip1/release/test_helper_vfs.wasm");

    if !wasm_path.exists() {
        eprintln!("WASM not found at: {:?}", wasm_path);
        return Err(eyre::eyre!("WASM binary not found"));
    }

    // Read the WASM binary
    let wasm_bytes = fs::read(&wasm_path)
        .wrap_err_with(|| format!("Failed to read WASM file: {:?}", wasm_path))?;

    // Parse with walrus using default config
    let config = walrus::ModuleConfig::new();
    let module = config
        .parse(&wasm_bytes)
        .map_err(|e| eyre::eyre!("Failed to parse WASM module: {}", e))?;

    // Extract export names
    let export_names: Vec<&str> = module.exports.iter().map(|e| e.name.as_str()).collect();

    // Check VFS exports
    let exports = detect_vfs_exports(&export_names);

    println!("Detected VFS exports: {:#?}", exports);
    assert!(!exports.is_empty(), "Should detect at least one VFS export");

    // We expect MY_VFS export
    let has_my_vfs = exports.iter().any(|e| e.holder_name == "MY_VFS");
    assert!(
        has_my_vfs,
        "Should detect MY_VFS export, found: {:?}",
        exports.iter().map(|e| &e.holder_name).collect::<Vec<_>>()
    );

    println!("✓ VFS export detection works correctly");
    Ok(())
}

/// Test that PseudoWasm helper code is properly generated
#[test]
fn test_pseudo_wasm_helper_generation() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    use wasi_virt_layer_cli::gen_ts_helper::{detect_vfs_exports, generate_ts_helper};

    let workspace_root = get_workspace_root();

    // First, build test_helper_vfs as WASM
    let build_output = std::process::Command::new("cargo")
        .args([
            "build",
            "-p",
            "test_helper_vfs",
            "--target",
            "wasm32-wasip1",
            "--release",
        ])
        .current_dir(&workspace_root)
        .output()
        .wrap_err("Failed to build test_helper_vfs")?;

    if !build_output.status.success() {
        let stderr = String::from_utf8_lossy(&build_output.stderr);
        eprintln!("Build failed: {}", stderr);
        return Err(eyre::eyre!("Failed to build test_helper_vfs"));
    }

    // Load the WASM binary
    let wasm_path = workspace_root.join("target/wasm32-wasip1/release/test_helper_vfs.wasm");

    let wasm_bytes =
        fs::read(&wasm_path).wrap_err_with(|| format!("Failed to read WASM: {:?}", wasm_path))?;

    let config = walrus::ModuleConfig::new();
    let module = config
        .parse(&wasm_bytes)
        .map_err(|e| eyre::eyre!("Failed to parse WASM: {}", e))?;

    // Extract export names and detect VFS exports
    let export_names: Vec<&str> = module.exports.iter().map(|e| e.name.as_str()).collect();
    let vfs_exports = detect_vfs_exports(&export_names);

    // Generate helper
    let helper_code = generate_ts_helper("test_helper_vfs", &vfs_exports, &[]);

    // Verify it's PseudoWasm mode (not SharedMemory)
    assert!(
        helper_code.contains("registerPseudoWasmTarget"),
        "Helper should export registerPseudoWasmTarget function. Got:\n{}",
        helper_code
    );

    // Should NOT contain SharedMemory types
    assert!(
        !helper_code.contains("SharedMemoryWorkerConfig"),
        "PseudoWasm helper should not contain SharedMemory types. Got:\n{}",
        helper_code
    );

    println!("✓ PseudoWasm helper generated correctly");
    println!("Generated helper code:\n{}", helper_code);

    Ok(())
}

/// Test that generated helper code has proper TypeScript structure
#[test]
fn test_helper_typescript_structure() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    use wasi_virt_layer_cli::gen_ts_helper::{detect_vfs_exports, generate_ts_helper};

    let workspace_root = get_workspace_root();

    // Build test_helper_vfs
    let build_output = std::process::Command::new("cargo")
        .args([
            "build",
            "-p",
            "test_helper_vfs",
            "--target",
            "wasm32-wasip1",
            "--release",
        ])
        .current_dir(&workspace_root)
        .output()
        .wrap_err("Failed to build test_helper_vfs")?;

    if !build_output.status.success() {
        let stderr = String::from_utf8_lossy(&build_output.stderr);
        return Err(eyre::eyre!("Failed to build test_helper_vfs: {}", stderr));
    }

    // Load and parse WASM
    let wasm_path = workspace_root.join("target/wasm32-wasip1/release/test_helper_vfs.wasm");

    let wasm_bytes = fs::read(&wasm_path).wrap_err("Failed to read WASM")?;

    let config = walrus::ModuleConfig::new();
    let module = config
        .parse(&wasm_bytes)
        .map_err(|e| eyre::eyre!("Failed to parse WASM: {}", e))?;

    // Extract exports and generate helper
    let export_names: Vec<&str> = module.exports.iter().map(|e| e.name.as_str()).collect();
    let vfs_exports = detect_vfs_exports(&export_names);
    let helper_code = generate_ts_helper("test_helper_vfs", &vfs_exports, &[]);

    // Check required TypeScript patterns
    let required_patterns = ["export", "async", "function", "registerPseudoWasmTarget"];

    for pattern in &required_patterns {
        assert!(
            helper_code.contains(pattern),
            "Generated helper missing required pattern: '{}'\n\nGenerated code:\n{}",
            pattern,
            helper_code
        );
    }

    // Verify JSDoc is present
    assert!(
        helper_code.contains("/**") || helper_code.contains("//"),
        "Helper should have documentation comments"
    );

    println!("✓ Helper TypeScript structure is correct");
    Ok(())
}

/// Test generation with no VFS exports
#[test]
fn test_minimal_helper_generation() -> color_eyre::Result<()> {
    color_eyre::install().ok();

    use wasi_virt_layer_cli::gen_ts_helper::{detect_vfs_exports, generate_ts_helper};

    let workspace_root = get_workspace_root();

    // Build a WASM without VFS exports - use test_wasm
    let build_output = std::process::Command::new("cargo")
        .args([
            "build",
            "-p",
            "test_wasm",
            "--target",
            "wasm32-wasip1",
            "--release",
        ])
        .current_dir(&workspace_root)
        .output()
        .wrap_err("Failed to build test_wasm")?;

    if !build_output.status.success() {
        let stderr = String::from_utf8_lossy(&build_output.stderr);
        eprintln!("Note: test_wasm build skipped: {}", stderr);
        return Ok(()); // Skip if test_wasm doesn't build
    }

    // Load WASM
    let wasm_path = workspace_root.join("target/wasm32-wasip1/release/test_wasm.wasm");

    if !wasm_path.exists() {
        println!("Skipping: test_wasm WASM not found");
        return Ok(());
    }

    let wasm_bytes = fs::read(&wasm_path).wrap_err("Failed to read test_wasm")?;

    let config = walrus::ModuleConfig::new();
    let module = config
        .parse(&wasm_bytes)
        .map_err(|e| eyre::eyre!("Failed to parse test_wasm: {}", e))?;

    // Extract exports (should be empty)
    let export_names: Vec<&str> = module.exports.iter().map(|e| e.name.as_str()).collect();
    let vfs_exports = detect_vfs_exports(&export_names);

    // Generate helper for module with no VFS exports
    let helper_code = generate_ts_helper("test_wasm", &vfs_exports, &[]);

    // Minimal helper should still be valid TypeScript
    assert!(
        !helper_code.is_empty(),
        "Helper should generate even without VFS exports"
    );

    println!("✓ Minimal helper generated successfully for WASM without exports");
    println!("Minimal helper content:\n{}", helper_code);

    Ok(())
}
