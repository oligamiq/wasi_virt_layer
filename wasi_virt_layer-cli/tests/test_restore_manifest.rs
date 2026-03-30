use camino::Utf8PathBuf;
use std::fs;
use tempfile::tempdir;
use wasi_virt_layer_cli::config_checker::{TomlRestorer, TomlRestorers};

#[test]
fn test_restore_on_panic() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("Cargo.toml");
    let original_content = r#"[package]
name = "test"
version = "0.1.0"
"#;
    fs::write(&file_path, original_content).unwrap();

    let utf8_path = Utf8PathBuf::from_path_buf(file_path.clone()).unwrap();
    let utf8_path_thread = utf8_path.clone();

    // We run the panic logic in a separate thread to isolate it
    let result = std::thread::spawn(move || {
        let mut restorers = TomlRestorers::new();

        // Create a restorer that changes the file
        let changed_content = r#"[package]
name = "test"
version = "0.2.0" # CHANGED
"#;
        // Manually simulate what FeatureChecker::set or similar does.
        // using with_write which writes the file and returns a restorer
        let restorer =
            TomlRestorer::with_write(&utf8_path_thread, changed_content.to_string()).unwrap();
        restorers.push(restorer);

        // Verify file is changed
        let current_content = fs::read_to_string(&utf8_path_thread).unwrap();
        assert_eq!(current_content, changed_content);

        // Panic!
        panic!("Simulated panic");

        // restorers is dropped here during unwind
    })
    .join();

    // Verify thread panicked
    assert!(result.is_err());

    // Verify file is restored
    let final_content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(
        final_content, original_content,
        "Cargo.toml should be restored to original content after panic"
    );
}
