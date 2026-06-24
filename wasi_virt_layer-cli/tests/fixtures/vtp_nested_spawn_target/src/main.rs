use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

fn main() {
    println!("### Starting nested spawn VirtualThreadPool test");

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
