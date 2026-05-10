use std::fs;
use std::path::Path;

fn main() {
    let path = "a/b/c";
    println!("Creating nested directory: {}", path);
    
    if let Err(e) = fs::create_dir_all(path) {
        eprintln!("Error creating directory: {}", e);
        std::process::exit(1);
    }
    
    if Path::new(path).exists() {
        println!("Successfully created directory: {}", path);
    } else {
        eprintln!("Directory does not exist after creation!");
        std::process::exit(1);
    }
    
    let file_path = format!("{}/hello.txt", path);
    println!("Writing to file: {}", file_path);
    if let Err(e) = fs::write(&file_path, "Hello from nested directory!") {
        eprintln!("Error writing file: {}", e);
        std::process::exit(1);
    }
    
    println!("Reading from file: {}", file_path);
    match fs::read_to_string(&file_path) {
        Ok(content) => println!("File content: {}", content),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    }
    
    println!("All tests passed!");
}
