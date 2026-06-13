use std::sync::atomic::{AtomicUsize, Ordering};

thread_local! {
    static MY_TLS: std::cell::RefCell<usize> = std::cell::RefCell::new(0);
}

fn main() {
    MY_TLS.with(|v| *v.borrow_mut() = 42);

    let pool = rayon::ThreadPoolBuilder::new().num_threads(2).build().unwrap();

    let counter = std::sync::Arc::new(AtomicUsize::new(0));

    pool.install(|| {
        rayon::scope(|s| {
            for _ in 0..100 {
                let counter = counter.clone();
                s.spawn(move |_| {
                    counter.fetch_add(1, Ordering::SeqCst);
                });
            }
        });
    });

    // Check TLS in the main thread AFTER using the pool
    MY_TLS.with(|v| {
        assert_eq!(*v.borrow(), 42, "Main thread TLS was corrupted after using pool");
    });
    
    println!("Native TLS test passed");
}
