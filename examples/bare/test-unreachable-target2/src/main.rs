fn main() {
    println!("Hello from unreachable target 2!");
    unreachable!("This should trigger the wrap_unreachable mechanism");
}
