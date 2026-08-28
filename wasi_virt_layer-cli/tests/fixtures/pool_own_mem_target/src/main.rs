use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

fn main() {
    println!("### Starting 5 threads test with VirtualThreadPool and own-memory");

    let counter = Arc::new(AtomicU32::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let counter = Arc::clone(&counter);
        handles.push(std::thread::spawn(move || {
            println!("### Thread {i} started");
            counter.fetch_add(1, Ordering::SeqCst);
            println!("### Thread {i} completed");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(counter.load(Ordering::SeqCst), 5);
    println!("### All 5 threads completed successfully.");
}
