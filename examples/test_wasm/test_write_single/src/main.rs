fn main() {
    let path = "test_file_single_mem.txt";
    let content = "Hello, single memory write test! Testing if VFS file writes work properly in single memory mode.";
    
    // VFS でファイルへの書き込みが正常にできるかテストする
    std::fs::write(path, content).expect("Failed to write to file");
    
    // 読み込んで確認する
    let read_content = std::fs::read_to_string(path).expect("Failed to read from file");
    assert_eq!(content, read_content, "Content mismatch");
    
    println!("File write and read success!");
}
