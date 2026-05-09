use std::time::{Duration, Instant};

fn main() {
    println!("### Starting WaitPoll test...");
    
    let duration = Duration::from_millis(200);
    let start = Instant::now();
    
    println!("### Sleeping for {:?}...", duration);
    std::thread::sleep(duration);
    
    let elapsed = start.elapsed();
    println!("### Woke up. Elapsed: {:?}", elapsed);
    
    if elapsed < duration {
        panic!("WaitPoll failed: elapsed time {:?} is less than requested duration {:?}", elapsed, duration);
    }
    
    println!("### WaitPoll test passed!");
}
