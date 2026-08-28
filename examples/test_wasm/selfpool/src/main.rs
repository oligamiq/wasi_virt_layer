use std::sync::atomic::{AtomicUsize, Ordering};

thread_local! {
    static MY_TLS: std::cell::RefCell<usize> = std::cell::RefCell::new(0);
}

fn main() {
    println!("### Starting selfpool test");

    MY_TLS.with(|v| *v.borrow_mut() = 42);

    let pool = rayon::ThreadPoolBuilder::new().num_threads(2).build().unwrap();

    let counter = std::sync::Arc::new(AtomicUsize::new(0));

    pool.install(|| {
        println!("### Inside pool.install");
        // Check if TLS is intact for the main thread when it runs inside the pool.
        MY_TLS.with(|v| {
            println!("### TLS value: {}", *v.borrow());
            assert_eq!(*v.borrow(), 42, "TLS was corrupted in pool.install");
        });

        rayon::scope(|s| {
            for i in 0..10 {
                let counter = std::sync::Arc::clone(&counter);
                s.spawn(move |_| {
                    counter.fetch_add(1, Ordering::SeqCst);
                });
            }
        });
    });

    let count = counter.load(Ordering::SeqCst);
    println!("### Completed all tasks. Count = {}", count);
    assert_eq!(count, 10);
}
