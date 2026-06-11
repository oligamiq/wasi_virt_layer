fn main() {
    std::thread::spawn(|| {
        let args: Vec<String> = std::env::args().collect();
        dbg!(args);
    })
    .join()
    .unwrap();
}
