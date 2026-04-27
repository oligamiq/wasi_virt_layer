use std::sync::Mutex;
use wasi_virt_layer_cli::fallback_command::{check_gag, get_fallback_command, FallbackCommand};

static MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_fallback_command_integration() {
    let _lock = MUTEX.lock().unwrap();
    if !check_gag() {
        return;
    }

    let mut cmd = FallbackCommand::new("non_existent_binary_12345", |args: &[String]| {
        println!("Fallback triggered with args: {:?}", args);
        0
    });
    cmd.arg("test-arg");

    let child = cmd.spawn().expect("Failed to spawn command");
    let output = child.wait_with_output().expect("Failed to wait for output");

    assert!(output.success);
    let stdout_str = String::from_utf8_lossy(&output.stdout);
    assert!(stdout_str.contains("Fallback triggered with args: [\"test-arg\"]"));
}

#[cfg(feature = "fallback")]
#[test]
fn test_fallback_wasm_merge_version() {
    let _lock = MUTEX.lock().unwrap();
    if !check_gag() {
        return;
    }

    let mut cmd = get_fallback_command("wasm-merge");
    cmd.arg("--version");

    let child = cmd.spawn().expect("Failed to spawn command");
    let output = child.wait_with_output().expect("Failed to wait for output");

    assert!(output.success);
}

#[cfg(feature = "fallback")]
#[test]
fn test_fallback_wasm_opt_version() {
    let _lock = MUTEX.lock().unwrap();
    if !check_gag() {
        return;
    }

    let mut cmd = get_fallback_command("wasm-opt");
    cmd.arg("--version");

    let child = cmd.spawn().expect("Failed to spawn command");
    let output = child.wait_with_output().expect("Failed to wait for output");

    assert!(output.success);
}
