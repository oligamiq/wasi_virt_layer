// cargo +nightly b -r --target wasm32-wasip1-threads -p test_atomic_wait
//
// This test exercises `memory.atomic.wait32` and `memory.atomic.notify`
// via Rust's standard `Condvar` / `Mutex` primitives.
// When running under wasi_virt_layer with multi-memory lowering,
// the `AtomicPatch` generator must redirect these instructions to VFS
// memory; otherwise the memory offset shift will corrupt the wait address
// and the condvar handshake will hang or panic.

use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

fn main() {
    println!("### AtomicWait/Notify test: starting...");

    // ── Test 1: basic Condvar signal ──────────────────────────────
    {
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair2 = Arc::clone(&pair);

        let handle = std::thread::spawn(move || {
            // Give the main thread a moment to reach the wait.
            std::thread::sleep(Duration::from_millis(50));

            let (lock, cvar) = &*pair2;
            let mut started = lock.lock().unwrap();
            *started = true;
            cvar.notify_one(); // ← triggers `memory.atomic.notify`
            drop(started);
            println!("### Worker: signalled condvar");
        });

        let (lock, cvar) = &*pair;
        let mut started = lock.lock().unwrap();
        while !*started {
            // ← triggers `memory.atomic.wait32`
            started = cvar.wait(started).unwrap();
        }
        assert!(*started, "condvar must have been signalled");
        handle.join().unwrap();
        println!("### Test 1 (basic condvar signal): PASSED");
    }

    // ── Test 2: ping-pong between two threads ────────────────────
    {
        let counter = Arc::new((Mutex::new(0u32), Condvar::new()));
        let c2 = Arc::clone(&counter);
        const ROUNDS: u32 = 20;

        let handle = std::thread::spawn(move || {
            let (lock, cvar) = &*c2;
            for _ in 0..ROUNDS {
                let mut val = lock.lock().unwrap();
                while *val % 2 == 0 {
                    val = cvar.wait(val).unwrap();
                }
                *val += 1;
                cvar.notify_one();
            }
        });

        let (lock, cvar) = &*counter;
        for _ in 0..ROUNDS {
            let mut val = lock.lock().unwrap();
            while *val % 2 != 0 {
                val = cvar.wait(val).unwrap();
            }
            *val += 1;
            cvar.notify_one();
        }
        handle.join().unwrap();

        let final_val = *counter.0.lock().unwrap();
        assert_eq!(
            final_val,
            ROUNDS * 2,
            "ping-pong counter should reach {}",
            ROUNDS * 2
        );
        println!("### Test 2 (ping-pong {ROUNDS} rounds): PASSED");
    }

    println!("### AtomicWait/Notify test: all passed!");
}
