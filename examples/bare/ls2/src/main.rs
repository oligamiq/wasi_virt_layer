fn main() {
    // ls
    for entry in std::fs::read_dir(".").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        println!("{}", path.display());
    }
}
