fn main() {
    println!("Hello from target!");
    // Trigger unreachable. The generator should catch this,
    // set the flag, and gracefully exit.
    unreachable!("This should trigger the wrap_unreachable mechanism");
}
