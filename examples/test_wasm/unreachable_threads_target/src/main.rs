fn main() {
    println!("Hello from target!");
    // Trigger unreachable. The generator should catch this,
    // set the flag, and gracefully exit.
    let (sender, waiter) = std::sync::mpsc::sync_channel(0);

    std::thread::spawn(move || {
        sender.send(()).unwrap();

        unreachable!("This should trigger the wrap_unreachable mechanism");
    });

    println!("Spawned thread, sleeping to wait for it to hit the unreachable...");

    // Wait for the thread to start and hit the unreachable.
    std::thread::sleep(std::time::Duration::from_millis(500));
    println!("Woke up, but the thread should have hit the unreachable by now!");

    waiter.recv().unwrap();

    unreachable!("This should trigger the wrap_unreachable mechanism");
}
