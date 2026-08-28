/// Deno execution tests for generated TypeScript helpers
pub mod utils;
use eyre::Context;
use std::fs;

fn get_workspace_root() -> std::path::PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    std::path::Path::new(manifest_dir)
        .parent()
        .unwrap()
        .to_path_buf()
}

/// Test SharedMemory VFS export detection
#[test]
fn test_shared_memory_export_detection() -> color_eyre::Result<()> {
    use wasi_virt_layer_cli::gen_ts_helper::detect_vfs_exports;

    let workspace_root = get_workspace_root();

    // Build test_helper_shared_memory_vfs as WASM
    let build_output = std::process::Command::new("cargo")
        .args([
            "build",
            "-p",
            "test_helper_shared_memory_vfs",
            "--target",
            "wasm32-wasip1-threads",
            "--release",
        ])
        .current_dir(&workspace_root)
        .output()
        .wrap_err("Failed to build test_helper_shared_memory_vfs")?;

    if !build_output.status.success() {
        let stderr = String::from_utf8_lossy(&build_output.stderr);
        eprintln!("Build failed: {}", stderr);
        return Err(eyre::eyre!("Failed to build test_helper_shared_memory_vfs"));
    }

    // Load the WASM binary
    let wasm_path = workspace_root
        .join("target/wasm32-wasip1-threads/release/test_helper_shared_memory_vfs.wasm");

    if !wasm_path.exists() {
        eprintln!("WASM not found at: {:?}", wasm_path);
        return Err(eyre::eyre!("WASM binary not found"));
    }

    // Read the WASM binary
    let wasm_bytes = fs::read(&wasm_path)
        .wrap_err_with(|| format!("Failed to read WASM file: {:?}", wasm_path))?;

    let mut export_names = Vec::new();
    for payload in wasmparser::Parser::new(0).parse_all(&wasm_bytes) {
        if let wasmparser::Payload::ExportSection(s) = payload.wrap_err("Failed to parse WASM")? {
            for export in s {
                export_names.push(export.wrap_err("Failed to parse export")?.name.to_string());
            }
        }
    }
    let export_names: Vec<&str> = export_names.iter().map(|s| s.as_str()).collect();

    // Check SharedMemory exports
    let exports = detect_vfs_exports(&export_names);

    println!("Detected SharedMemory exports: {:#?}", exports);
    assert!(
        !exports.is_empty(),
        "Should detect at least one SharedMemory export"
    );

    // We expect SHARED_MEMORY_VFS export
    let has_shared_memory = exports.iter().any(|e| e.holder_name == "SHARED_MEMORY_VFS");
    assert!(
        has_shared_memory,
        "Should detect SHARED_MEMORY_VFS export, found: {:?}",
        exports.iter().map(|e| &e.holder_name).collect::<Vec<_>>()
    );

    println!("✓ SharedMemory export detection works correctly");
    Ok(())
}

/// Test that SharedMemory helper code is properly generated
#[test]
fn test_shared_memory_helper_generation() -> color_eyre::Result<()> {
    use wasi_virt_layer_cli::gen_ts_helper::{detect_vfs_exports, generate_ts_helper};

    let workspace_root = get_workspace_root();

    // Build test_helper_shared_memory_vfs as WASM
    let build_output = std::process::Command::new("cargo")
        .args([
            "build",
            "-p",
            "test_helper_shared_memory_vfs",
            "--target",
            "wasm32-wasip1-threads",
            "--release",
        ])
        .current_dir(&workspace_root)
        .output()
        .wrap_err("Failed to build")?;

    if !build_output.status.success() {
        return Err(eyre::eyre!("Failed to build test_helper_shared_memory_vfs"));
    }

    // Load the WASM binary
    let wasm_path = workspace_root
        .join("target/wasm32-wasip1-threads/release/test_helper_shared_memory_vfs.wasm");
    let wasm_bytes = fs::read(&wasm_path).wrap_err("Failed to read WASM")?;

    let mut export_names = Vec::new();
    for payload in wasmparser::Parser::new(0).parse_all(&wasm_bytes) {
        if let wasmparser::Payload::ExportSection(s) = payload.wrap_err("Failed to parse WASM")? {
            for export in s {
                export_names.push(export.wrap_err("Failed to parse export")?.name.to_string());
            }
        }
    }

    // Extract export names and detect exports
    let export_names: Vec<&str> = export_names.iter().map(|s| s.as_str()).collect();
    let vfs_exports = detect_vfs_exports(&export_names);

    // Generate helper
    let helper_code = generate_ts_helper("test_helper_shared_memory_vfs", &vfs_exports, &[]);

    // Verify it's SharedMemory mode
    assert!(
        helper_code.contains("SharedMemoryWorkerConfig"),
        "SharedMemory helper should export SharedMemoryWorkerConfig"
    );
    assert!(
        !helper_code.contains("registerPseudoWasmTarget"),
        "SharedMemory helper should not contain PseudoWasm types"
    );

    println!("✓ SharedMemory helper generated correctly");
    println!("Generated content:\n{}", helper_code);
    Ok(())
}

/// Test that generated PseudoWasm helper can be executed with Deno
#[test]
fn test_pseudo_wasm_deno_execution() -> color_eyre::Result<()> {
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
        .output()?;

    if !build_output.status.success() {
        return Err(eyre::eyre!("Failed to build test_helper_vfs"));
    }

    // Load and generate helper
    let wasm_path = workspace_root.join("target/wasm32-wasip1/release/test_helper_vfs.wasm");
    let wasm_bytes = fs::read(&wasm_path)?;
    let mut export_names = Vec::new();
    for payload in wasmparser::Parser::new(0).parse_all(&wasm_bytes) {
        if let wasmparser::Payload::ExportSection(s) = payload.wrap_err("Failed to parse WASM")? {
            for export in s {
                export_names.push(export.wrap_err("Failed to parse export")?.name.to_string());
            }
        }
    }

    let export_names: Vec<&str> = export_names.iter().map(|s| s.as_str()).collect();
    let vfs_exports = detect_vfs_exports(&export_names);
    let helper_code = generate_ts_helper("test_helper_vfs", &vfs_exports, &[]);

    // Create temp directory
    let temp_dir = std::env::temp_dir().join(format!("deno_ts_{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&temp_dir)?;

    // Write helper file
    fs::write(temp_dir.join("test_helper_vfs.helper.ts"), &helper_code)?;

    // Create test script
    let test_script = r#"
import { registerPseudoWasmTarget } from "./test_helper_vfs.helper.ts";
if (typeof registerPseudoWasmTarget !== "function") throw new Error("Not a function");
console.log("✓ PseudoWasm helper works with Deno");
"#;
    fs::write(temp_dir.join("test.ts"), test_script)?;

    // Run Deno
    let output = std::process::Command::new("deno")
        .args(["run", "--allow-read", "test.ts"])
        .current_dir(&temp_dir)
        .output()?;

    let _ = std::fs::remove_dir_all(&temp_dir);

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let code = output.status.code().unwrap_or(1);
        return Err(eyre::eyre!(
            "Deno execution failed with status {}: {}",
            code,
            stderr
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("✓ PseudoWasm Deno execution passed:\n{}", stdout);

    Ok(())
}

/// Test that generated SharedMemory helper can be executed with Deno
#[test]
fn test_shared_memory_deno_execution() -> color_eyre::Result<()> {
    use wasi_virt_layer_cli::gen_ts_helper::{detect_vfs_exports, generate_ts_helper};

    let workspace_root = get_workspace_root();

    // Build test_helper_shared_memory_vfs
    let build_output = std::process::Command::new("cargo")
        .args([
            "build",
            "-p",
            "test_helper_shared_memory_vfs",
            "--target",
            "wasm32-wasip1-threads",
            "--release",
        ])
        .current_dir(&workspace_root)
        .output()?;

    if !build_output.status.success() {
        return Err(eyre::eyre!("Failed to build test_helper_shared_memory_vfs"));
    }

    // Load and generate helper
    let wasm_path = workspace_root
        .join("target/wasm32-wasip1-threads/release/test_helper_shared_memory_vfs.wasm");
    let wasm_bytes = fs::read(&wasm_path)?;
    let mut export_names = Vec::new();
    for payload in wasmparser::Parser::new(0).parse_all(&wasm_bytes) {
        if let wasmparser::Payload::ExportSection(s) = payload.wrap_err("Failed to parse WASM")? {
            for export in s {
                export_names.push(export.wrap_err("Failed to parse export")?.name.to_string());
            }
        }
    }

    let export_names: Vec<&str> = export_names.iter().map(|s| s.as_str()).collect();
    let vfs_exports = detect_vfs_exports(&export_names);
    let helper_code = generate_ts_helper("test_helper_shared_memory_vfs", &vfs_exports, &[]);

    // Create temp directory
    let temp_dir = std::env::temp_dir().join(format!("deno_shmem_{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&temp_dir)?;

    // Write helper file
    let helper_path = temp_dir.join("test_helper_shared_memory_vfs.helper.ts");
    fs::write(&helper_path, &helper_code)?;

    // Debug: print file content
    let file_content = fs::read_to_string(&helper_path)?;
    println!(
        "Helper file content (first 500 chars):\n{}",
        &file_content.chars().take(500).collect::<String>()
    );

    // Create test script - check what's exported
    let test_script = r#"
import { createSharedMemoryWorkerConfig, initializeSharedMemoryTarget } from "./test_helper_shared_memory_vfs.helper.ts";
console.log("✓ SharedMemory exports found");
if (typeof createSharedMemoryWorkerConfig !== "function") throw new Error("createSharedMemoryWorkerConfig not found");
if (typeof initializeSharedMemoryTarget !== "function") throw new Error("initializeSharedMemoryTarget not found");
console.log("✓ SharedMemory helper works with Deno");
"#;
    fs::write(temp_dir.join("test.ts"), test_script)?;

    // Run Deno
    let output = std::process::Command::new("deno")
        .args(["run", "--allow-read", "test.ts"])
        .current_dir(&temp_dir)
        .output()?;

    let _ = std::fs::remove_dir_all(&temp_dir);

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let code = output.status.code().unwrap_or(1);
        return Err(eyre::eyre!(
            "Deno execution failed with status {}: {}",
            code,
            stderr
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("✓ SharedMemory Deno execution passed:\n{}", stdout);

    Ok(())
}
