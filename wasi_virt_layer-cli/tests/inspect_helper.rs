// Minimal test to just print generated helper content
use std::fs;

#[test]
fn inspect_generated_helper() -> color_eyre::Result<()> {
    // Check onetime directory for latest generated files
    let onetime = "wasi_virt_layer-cli/tests/onetime";

    if let Ok(dirs) = fs::read_dir(onetime) {
        // Get all directories, sort by modification time descending
        let mut dir_entries: Vec<_> = dirs
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .collect();

        dir_entries.sort_by_key(|e| {
            e.metadata()
                .ok()
                .and_then(|m| m.modified().ok())
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
        });

        dir_entries.reverse();

        // Check first few directories for helper files
        for entry in dir_entries.iter().take(3) {
            let dist_dir = entry.path().join("dist");
            if dist_dir.exists() {
                println!("\n=== Checking {:?} ===", entry.path().file_name());

                if let Ok(files) = fs::read_dir(&dist_dir) {
                    for file_entry in files {
                        if let Ok(file_entry) = file_entry {
                            let path = file_entry.path();
                            if let Some(name) = path.file_name() {
                                if name.to_string_lossy().ends_with(".helper.ts") {
                                    println!("Found helper: {:?}", name);
                                    if let Ok(content) = fs::read_to_string(&path) {
                                        println!("Content:\n{}\n", content);
                                        return Ok(());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("No helper file found");
    Ok(())
}
