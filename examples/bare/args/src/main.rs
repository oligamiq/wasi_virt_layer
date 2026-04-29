fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("Arguments: {:?}", args);
    if args.len() > 1 && args[1] == "hello" {
        println!("Hello world!");
    } else {
        println!("No hello.");
    }
}
