use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

fn main() {
    println!("Testing virtual thread pool...");

    let counter = Arc::new(AtomicU32::new(0));
    let mut handles = vec![];

    // Spawn 10 threads to test the pool
    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            println!("Thread {} started", i);
            counter.fetch_add(1, Ordering::SeqCst);
            std::thread::sleep(std::time::Duration::from_millis(10));
            println!("Thread {} completed", i);
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    let final_count = counter.load(Ordering::SeqCst);
    println!("All threads completed. Counter: {}", final_count);

    assert_eq!(final_count, 10, "Expected counter to be 10");
    println!("Test passed!");
}
