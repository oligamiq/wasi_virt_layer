fn main() {
    println!("Hello, world!");

    std::thread::spawn(|| {
        println!("Hello from example thread!");
    })
    .join()
    .unwrap();
}
