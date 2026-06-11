fn main() {
    let result = std::panic::catch_unwind(|| {
        println!("About to panic!");
        panic!("This is a test panic");
    });
    
    if result.is_err() {
        println!("Caught the panic!");
        std::process::exit(0);
    } else {
        println!("Panic did not happen?");
        std::process::exit(1);
    }
}
