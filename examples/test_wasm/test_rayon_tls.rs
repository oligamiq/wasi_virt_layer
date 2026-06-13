use std::sync::atomic::{AtomicUsize, Ordering};

thread_local! {
    static MY_TLS: std::cell::RefCell<usize> = std::cell::RefCell::new(0);
}

fn main() {
    MY_TLS.with(|v| *v.borrow_mut() = 42);

    let pool = rayon::ThreadPoolBuilder::new().num_threads(2).build().unwrap();

    let counter = std::sync::Arc::new(AtomicUsize::new(0));

    pool.install(|| {
        MY_TLS.with(|v| {
            assert_eq!(*v.borrow(), 42, "TLS was corrupted in pool.install");
        });
    });
}
