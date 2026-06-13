use std::sync::atomic::{AtomicUsize, Ordering};

thread_local! {
    static MY_TLS: std::cell::RefCell<usize> = std::cell::RefCell::new(0);
}

fn main() {
    println!("### Starting TLS isolation test");

    MY_TLS.with(|v| *v.borrow_mut() = 42);

    let pool = rayon::ThreadPoolBuilder::new().num_threads(2).build().unwrap();

    let counter = std::sync::Arc::new(AtomicUsize::new(0));

    pool.install(|| {
        rayon::scope(|s| {
            for i in 0..10 {
                let counter = std::sync::Arc::clone(&counter);
                s.spawn(move |_| {
                    // This runs on a worker thread.
                    // It should have its own TLS, initialized to 0.
                    // We modify it to see if it affects the main thread.
                    MY_TLS.with(|v| *v.borrow_mut() = 100 + i);
                    counter.fetch_add(1, Ordering::SeqCst);
                });
            }
        });
    });

    let count = counter.load(Ordering::SeqCst);
    println!("### Completed all tasks. Count = {}", count);
    assert_eq!(count, 10);

    // Now check if main thread's TLS is still intact
    MY_TLS.with(|v| {
        let val = *v.borrow();
        println!("### Main thread TLS value: {}", val);
        assert_eq!(val, 42, "TLS was corrupted! Worker threads overwrote main thread's TLS.");
    });
}
