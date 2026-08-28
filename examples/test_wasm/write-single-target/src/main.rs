fn main() {
    let path = "./test-single.txt";
    let content = "Hello Single Memory Write";
    std::fs::write(path, content).unwrap();
    let read_content = std::fs::read_to_string(path).unwrap();
    assert_eq!(read_content, content);
    println!("write and read single memory success");
}
