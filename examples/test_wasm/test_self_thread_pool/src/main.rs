use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    println!("### Starting self thread pool test");

    // Create a rayon thread pool.
    let pool = rayon::ThreadPoolBuilder::new().num_threads(4).build().unwrap();

    let counter = std::sync::Arc::new(AtomicUsize::new(0));

    // pool.install allows the current thread to act as a worker thread in the pool.
    // If TLS is broken when self acts as a thread in its own thread pool, this should panic.
    pool.install(|| {
        println!("### Inside pool.install");
        rayon::scope(|s| {
            for i in 0..10 {
                let counter = std::sync::Arc::clone(&counter);
                s.spawn(move |_| {
                    println!("### Executing task {}", i);
                    counter.fetch_add(1, Ordering::SeqCst);
                });
            }
        });
    });

    let count = counter.load(Ordering::SeqCst);
    println!("### Completed all tasks. Count = {}", count);
    assert_eq!(count, 10);
}
