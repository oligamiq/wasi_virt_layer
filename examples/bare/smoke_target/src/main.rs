fn main() {
    println!("Starting smoke_target...");
    // Attempt to allocate 10MB (much smaller)
    let size = 10 * 1024 * 1024;
    println!("Attempting to allocate {} bytes...", size);
    
    let mut v = Vec::with_capacity(size);
    v.push(1u8);
    
    println!("Allocation successful! First element: {}", v[0]);
    println!("Success!");
}
