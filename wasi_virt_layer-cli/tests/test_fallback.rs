#[cfg(feature = "fallback")]
use std::sync::Mutex;

use wasi_virt_layer_cli::fallback_command::FallbackCommand;
#[cfg(feature = "fallback")]
use wasi_virt_layer_cli::fallback_command::{check_gag, get_fallback_command};

#[cfg(feature = "fallback")]
static MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_fallback_command_integration() {
    let mut cmd = FallbackCommand::new("non_existent_binary_12345", |_| 0);
    cmd.arg("test-arg");

    let child = cmd.spawn().expect("Failed to spawn command");
    let output = child.wait_with_output().expect("Failed to wait for output");

    assert!(output.success);
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
