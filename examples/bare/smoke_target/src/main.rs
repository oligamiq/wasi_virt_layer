use std::thread;

fn main() {
    println!("Starting smoke_target with threads...");

    let handle = thread::spawn(|| {
        println!("Hello from a child thread!");
        println!("Child thread finishing...");
    });

    handle.join().expect("Thread failed");

    println!("Success!");
}
