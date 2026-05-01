fn main() {
    println!("Hello from unreachable target 1!");
    unreachable!("This should trigger the wrap_unreachable mechanism");
}
