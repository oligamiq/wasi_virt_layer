fn main() {
    println!("=== Target executable start ===");

    // Read files from the virtual filesystem (including ones the VFS wrote)
    println!("Reading files:");
    for entry in std::fs::read_dir(".").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let content = std::fs::read_to_string(&path).unwrap();
            println!("  {}: {content}", path.display());
        } else {
            println!("  {} (dir)", path.display());
        }
    }

    // Write our own file back into the virtual filesystem
    std::fs::write("./target-was-here.txt", "Written by the target executable").unwrap();
    println!("Target wrote target-was-here.txt");

    println!("=== Target executable end ===");
}
