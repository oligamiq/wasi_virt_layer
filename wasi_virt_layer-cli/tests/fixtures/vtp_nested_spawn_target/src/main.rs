use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

static RUN_COUNT: AtomicU32 = AtomicU32::new(0);

fn main() {
    let run = RUN_COUNT.fetch_add(1, Ordering::SeqCst);
    println!("### Starting nested spawn VirtualThreadPool test run {run}");

    if run == 0 {
        let started = Arc::new(AtomicU32::new(0));
        for background_id in 0..2 {
            let started = Arc::clone(&started);
            std::thread::spawn(move || {
                println!("### Background thread {background_id} starting");
                started.fetch_add(1, Ordering::SeqCst);
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(60));
                }
            });
        }

        while started.load(Ordering::SeqCst) < 2 {
            std::thread::yield_now();
        }
        println!("### Background threads are parked");
        return;
    }

    let completed_children = Arc::new(AtomicU32::new(0));
    let mut parents = Vec::new();

    for parent_id in 0..2 {
        let completed_children = Arc::clone(&completed_children);
        parents.push(std::thread::spawn(move || {
            println!("### Parent thread {parent_id} starting");

            let child = std::thread::spawn(move || {
                println!("### Child thread {parent_id} starting");
                completed_children.fetch_add(1, Ordering::SeqCst);
                println!("### Child thread {parent_id} finishing");
            });

            child.join().unwrap();
            println!("### Parent thread {parent_id} finishing");
        }));
    }

    for parent in parents {
        parent.join().unwrap();
    }

    assert_eq!(completed_children.load(Ordering::SeqCst), 2);
    println!("### Nested spawn VirtualThreadPool test completed.");
}
