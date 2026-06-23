use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

fn main() {
    println!("### Starting custom 5 threads test with VirtualThreadPool");

    let counter = Arc::new(AtomicU32::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let counter = Arc::clone(&counter);
        handles.push(std::thread::spawn(move || {
            println!("### Custom thread {i} starting");
            counter.fetch_add(1, Ordering::SeqCst);
            println!("### Custom thread {i} finishing");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(counter.load(Ordering::SeqCst), 5);
    println!("### All custom 5 threads completed successfully.");
}
