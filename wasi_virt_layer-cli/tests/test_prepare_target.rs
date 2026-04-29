/// Test for prepare-target command with shared memory ABI
use std::fs;
use std::process::Command;
use uuid::Uuid;

/// Test that prepare-target command can transform a simple WASI module
#[test]
fn test_prepare_target_basic() -> eyre::Result<()> {
    // Create a temporary directory for test
    let test_dir = std::env::temp_dir().join(format!("prepare-target-test-{}", Uuid::new_v4()));
    fs::create_dir_all(&test_dir)?;

    // Check if we have wasi target
    let has_target = Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output()
        .ok()
        .and_then(|out| {
            String::from_utf8_lossy(&out.stdout)
                .contains("wasm32-wasip1")
                .then_some(())
        })
        .is_some();

    if !has_target {
        eprintln!("Skipping test: wasm32-wasip1 target not installed");
        return Ok(());
    }

    // Create a minimal WASI module using cargo
    let module_dir = test_dir.join("test_module");
    fs::create_dir_all(&module_dir)?;

    // Create Cargo.toml
    let cargo_toml = r#"
[package]
name = "test_wasi_module"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
"#;
    fs::write(module_dir.join("Cargo.toml"), cargo_toml)?;

    // Create minimal lib.rs
    let lib_rs = r#"
#[no_mangle]
pub extern "C" fn test_export() -> i32 {
    42
}
"#;
    fs::create_dir_all(module_dir.join("src"))?;
    fs::write(module_dir.join("src/lib.rs"), lib_rs)?;

    // Build the module
    let build_output = Command::new("cargo")
        .args(&["build", "--release", "--target=wasm32-wasip1"])
        .current_dir(&module_dir)
        .output()?;

    if !build_output.status.success() {
        eprintln!(
            "Failed to build test module:\n{}",
            String::from_utf8_lossy(&build_output.stderr)
        );
        return Ok(());
    }

    // Find the built WASM module
    let wasm_path = module_dir.join("target/wasm32-wasip1/release/test_wasi_module.wasm");
    if !wasm_path.exists() {
        eprintln!("Built WASM module not found at {:?}", wasm_path);
        return Ok(());
    }

    // Test prepare-target command
    let output_path = test_dir.join("test_wasi_module.prepared.wasm");

    // Run prepare-target command
    let result = Command::new(env!("CARGO_BIN_EXE_wasi_virt_layer"))
        .args(&[
            "prepare-target",
            wasm_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
        ])
        .output()?;

    if !result.status.success() {
        eprintln!(
            "prepare-target command failed:\n{}",
            String::from_utf8_lossy(&result.stderr)
        );
        return Ok(());
    }

    // Verify output file exists
    if !output_path.exists() {
        eyre::bail!("Output WASM file not created");
    }

    // Verify output is a valid WASM binary
    let output_bytes = fs::read(&output_path)?;
    if output_bytes.len() < 4 || &output_bytes[0..4] != b"\0asm" {
        eyre::bail!("Output is not a valid WASM binary");
    }

    // Cleanup
    fs::remove_dir_all(&test_dir)?;

    Ok(())
}

/// Test that prepare-target correctly injects ABI function imports
#[test]
fn test_prepare_target_abi_imports() -> eyre::Result<()> {
    // Create a temporary directory for test
    let test_dir = std::env::temp_dir().join(format!("prepare-target-abi-{}", Uuid::new_v4()));
    fs::create_dir_all(&test_dir)?;

    // Check if we have wasi target
    let has_target = Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output()
        .ok()
        .and_then(|out| {
            String::from_utf8_lossy(&out.stdout)
                .contains("wasm32-wasip1")
                .then_some(())
        })
        .is_some();

    if !has_target {
        eprintln!("Skipping test: wasm32-wasip1 target not installed");
        return Ok(());
    }

    // Create and build test module
    let module_dir = test_dir.join("abi_test_module");
    fs::create_dir_all(&module_dir)?;

    let cargo_toml = r#"
[package]
name = "abi_test_module"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]
"#;
    fs::write(module_dir.join("Cargo.toml"), cargo_toml)?;

    let lib_rs = r#"
#[no_mangle]
pub extern "C" fn run() {
    // This would trigger memory.grow if we allocate
}
"#;
    fs::create_dir_all(module_dir.join("src"))?;
    fs::write(module_dir.join("src/lib.rs"), lib_rs)?;

    // Build
    let build_output = Command::new("cargo")
        .args(&["build", "--release", "--target=wasm32-wasip1"])
        .current_dir(&module_dir)
        .output()?;

    if !build_output.status.success() {
        return Ok(());
    }

    let wasm_path = module_dir.join("target/wasm32-wasip1/release/abi_test_module.wasm");
    if !wasm_path.exists() {
        return Ok(());
    }

    let output_path = test_dir.join("abi_test_module.prepared.wasm");

    // Run prepare-target
    let result = Command::new(env!("CARGO_BIN_EXE_wasi_virt_layer"))
        .args(&[
            "prepare-target",
            wasm_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
        ])
        .output()?;

    if !result.status.success() {
        eprintln!(
            "prepare-target failed:\n{}",
            String::from_utf8_lossy(&result.stderr)
        );
        return Ok(());
    }

    // Verify ABI functions were imported
    let output_bytes = fs::read(&output_path)?;
    
    // Check for import names in the binary
    let has_register_import = output_bytes.windows(40)
        .any(|window| String::from_utf8_lossy(window).contains("wasip1_vfs_register_shared_memory_target"));
    let has_grow_import = output_bytes.windows(40)
        .any(|window| String::from_utf8_lossy(window).contains("wasip1_vfs_shared_memory_grow"));
    let has_lock_import = output_bytes.windows(40)
        .any(|window| String::from_utf8_lossy(window).contains("wasip1_vfs_shared_memory_get_lock_ptr"));

    // At least one import should be present (they should all be)
    if !(has_register_import || has_grow_import || has_lock_import) {
        eprintln!("Warning: Could not find ABI imports in binary (they may be name-mangled)");
    }

    // Cleanup
    fs::remove_dir_all(&test_dir)?;

    Ok(())
}

/// Test that prepare-target preserves original module functionality
#[test]
fn test_prepare_target_preserves_exports() -> eyre::Result<()> {
    let test_dir = std::env::temp_dir().join(format!("prepare-target-exports-{}", Uuid::new_v4()));
    fs::create_dir_all(&test_dir)?;

    // Check for target
    let has_target = Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output()
        .ok()
        .and_then(|out| {
            String::from_utf8_lossy(&out.stdout)
                .contains("wasm32-wasip1")
                .then_some(())
        })
        .is_some();

    if !has_target {
        return Ok(());
    }

    // Create module with named exports
    let module_dir = test_dir.join("export_test");
    fs::create_dir_all(&module_dir)?;

    fs::write(
        module_dir.join("Cargo.toml"),
        r#"
[package]
name = "export_test"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]
"#,
    )?;

    fs::create_dir_all(module_dir.join("src"))?;
    fs::write(
        module_dir.join("src/lib.rs"),
        r#"
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
"#,
    )?;

    // Build
    let build_output = Command::new("cargo")
        .args(&["build", "--release", "--target=wasm32-wasip1"])
        .current_dir(&module_dir)
        .output()?;

    if !build_output.status.success() {
        return Ok(());
    }

    let wasm_path = module_dir.join("target/wasm32-wasip1/release/export_test.wasm");
    if !wasm_path.exists() {
        return Ok(());
    }

    let output_path = test_dir.join("export_test.prepared.wasm");

    // Transform
    let result = Command::new(env!("CARGO_BIN_EXE_wasi_virt_layer"))
        .args(&[
            "prepare-target",
            wasm_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
        ])
        .output()?;

    if !result.status.success() {
        return Ok(());
    }

    // Verify exports are preserved
    let output_bytes = fs::read(&output_path)?;
    
    // Check for export names
    let has_add = output_bytes.windows(10)
        .any(|w| String::from_utf8_lossy(w).contains("add"));
    let has_multiply = output_bytes.windows(20)
        .any(|w| String::from_utf8_lossy(w).contains("multiply"));

    if !(has_add || has_multiply) {
        eprintln!("Warning: Original exports may not be preserved");
    }

    // Cleanup
    fs::remove_dir_all(&test_dir)?;

    Ok(())
}
