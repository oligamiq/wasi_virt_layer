fn main() {
    println!("We will try to allocate 500 MB of memory");
    println!("Starting allocation...");
    
    let mut chunks = Vec::new();
    for i in 0..=5 {
        println!("Allocated {} MB", i * 100);
        if i > 0 {
            let mut chunk: Vec<u8> = Vec::with_capacity(100 * 1024 * 1024);
            unsafe {
                chunk.set_len(100 * 1024 * 1024);
            }
            chunks.push(chunk);
        }
    }
    
    println!("Success!");
}
