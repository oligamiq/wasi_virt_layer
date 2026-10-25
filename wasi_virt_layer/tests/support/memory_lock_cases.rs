use super::*;
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::{Arc, Condvar, Mutex, mpsc};
use std::thread;
use std::time::Duration;

const WATCHDOG: Duration = Duration::from_secs(5);
const MARKER: i32 = 1 << 30;
type Hook = Box<dyn Fn(&AtomicI32, i32) + Send + Sync>;

#[derive(Default)]
struct Parking {
    gate: Mutex<()>,
    wake: Condvar,
    notifications: Mutex<Vec<u32>>,
    parked: Option<mpsc::Sender<i32>>,
    before_wait: Option<Hook>,
    before_register: Option<Hook>,
    spurious_once: AtomicBool,
    saturation_seen: AtomicBool,
}

impl WaitNotify for Parking {
    fn wait(&self, state: &AtomicI32, expected: i32) {
        if let Some(hook) = &self.before_wait {
            hook(state, expected);
        }
        if self.spurious_once.swap(false, Ordering::Relaxed) {
            return;
        }
        let guard = self.gate.lock().unwrap();
        if state.load(Ordering::Relaxed) != expected {
            return;
        }
        if let Some(sender) = &self.parked {
            sender.send(expected).unwrap();
        }
        let (_guard, timeout) = self.wake.wait_timeout(guard, WATCHDOG).unwrap();
        assert!(
            !timeout.timed_out(),
            "lost wakeup: native wait watchdog expired"
        );
    }

    fn notify(&self, _: &AtomicI32, count: u32) {
        let _guard = self.gate.lock().unwrap();
        self.notifications.lock().unwrap().push(count);
        if count == 1 {
            self.wake.notify_one();
        } else {
            self.wake.notify_all();
        }
    }

    fn before_writer_register(&self, state: &AtomicI32, expected: i32) {
        if let Some(hook) = &self.before_register {
            hook(state, expected);
        }
    }

    fn reader_saturated(&self, state: &AtomicI32) {
        assert!(!self.saturation_seen.swap(true, Ordering::Relaxed));
        // Model an existing reader leaving; the actual release path is tested elsewhere.
        state.fetch_sub(1, Ordering::Release);
    }
}

fn paused_hook() -> (Hook, mpsc::Receiver<i32>, mpsc::Sender<()>) {
    let (entered, receive) = mpsc::channel();
    let (resume, permit) = mpsc::channel();
    let permit = Mutex::new(permit);
    let once = AtomicBool::new(false);
    let hook = Box::new(move |_: &AtomicI32, expected| {
        if !once.swap(true, Ordering::Relaxed) {
            entered.send(expected).unwrap();
            permit.lock().unwrap().recv_timeout(WATCHDOG).unwrap();
        }
    });
    (hook, receive, resume)
}

fn writer(lock: Arc<MemoryAccessLock>, parking: Arc<Parking>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        lock.write_acquire(&*parking);
        assert_eq!(lock.state.load(Ordering::Relaxed), -1);
        lock.write_release(&*parking);
    })
}

#[test]
fn overlapping_readers_notify_only_when_the_last_one_leaves() {
    let lock = MemoryAccessLock::new();
    let parking = Parking::default();
    lock.read_acquire(&parking);
    lock.read_acquire(&parking);
    lock.read_release(&parking);
    assert!(parking.notifications.lock().unwrap().is_empty());
    lock.read_release(&parking);
    let expected = if cfg!(feature = "waiter-aware-memory-lock") {
        vec![]
    } else {
        vec![1]
    };
    assert_eq!(*parking.notifications.lock().unwrap(), expected);
    assert_eq!(lock.state.load(Ordering::Relaxed), 0);
}

#[test]
fn parked_writer_is_woken_by_the_last_reader() {
    let lock = Arc::new(MemoryAccessLock::new());
    let (send, receive) = mpsc::channel();
    let parking = Arc::new(Parking {
        parked: Some(send),
        ..Default::default()
    });
    lock.read_acquire(&*parking);
    lock.read_acquire(&*parking);
    let handle = writer(lock.clone(), parking.clone());
    let expected = receive.recv_timeout(WATCHDOG).unwrap();
    assert_eq!(
        expected,
        if cfg!(feature = "waiter-aware-memory-lock") {
            MARKER | 2
        } else {
            2
        }
    );
    lock.read_release(&*parking);
    assert!(parking.notifications.lock().unwrap().is_empty());
    lock.read_release(&*parking);
    handle.join().unwrap();
    assert_eq!(*parking.notifications.lock().unwrap(), vec![1, u32::MAX]);
}

#[cfg(feature = "waiter-aware-memory-lock")]
#[test]
fn last_reader_leaves_before_writer_registration() {
    let lock = Arc::new(MemoryAccessLock::new());
    let (hook, entered, resume) = paused_hook();
    let parking = Arc::new(Parking {
        before_register: Some(hook),
        ..Default::default()
    });
    lock.read_acquire(&*parking);
    let handle = writer(lock.clone(), parking.clone());
    assert_eq!(entered.recv_timeout(WATCHDOG).unwrap(), 1);
    lock.read_release(&*parking);
    assert!(parking.notifications.lock().unwrap().is_empty());
    resume.send(()).unwrap();
    handle.join().unwrap();
    assert_eq!(lock.state.load(Ordering::Relaxed), 0);
}

#[test]
fn notification_before_actual_park_is_not_lost() {
    let lock = Arc::new(MemoryAccessLock::new());
    let (hook, entered, resume) = paused_hook();
    let parking = Arc::new(Parking {
        before_wait: Some(hook),
        ..Default::default()
    });
    lock.read_acquire(&*parking);
    let handle = writer(lock.clone(), parking.clone());
    entered.recv_timeout(WATCHDOG).unwrap();
    lock.read_release(&*parking);
    resume.send(()).unwrap();
    handle.join().unwrap();
    assert_eq!(lock.state.load(Ordering::Relaxed), 0);
}

#[test]
fn reader_count_aba_still_wakes_the_writer() {
    let lock = Arc::new(MemoryAccessLock::new());
    let (hook, entered, resume) = paused_hook();
    let (send, receive) = mpsc::channel();
    let parking = Arc::new(Parking {
        before_wait: Some(hook),
        parked: Some(send),
        ..Default::default()
    });
    lock.read_acquire(&*parking);
    let handle = writer(lock.clone(), parking.clone());
    let expected = entered.recv_timeout(WATCHDOG).unwrap();
    lock.read_release(&*parking);
    lock.read_acquire(&*parking);
    assert_eq!(lock.state.load(Ordering::Relaxed), expected);
    resume.send(()).unwrap();
    assert_eq!(receive.recv_timeout(WATCHDOG).unwrap(), expected);
    lock.read_release(&*parking);
    handle.join().unwrap();
}

#[test]
fn writer_release_broadcasts_to_a_reader() {
    let lock = Arc::new(MemoryAccessLock::new());
    let (send, receive) = mpsc::channel();
    let parking = Arc::new(Parking {
        parked: Some(send),
        ..Default::default()
    });
    lock.write_acquire(&*parking);
    let other_lock = lock.clone();
    let other_parking = parking.clone();
    let handle = thread::spawn(move || {
        other_lock.read_acquire(&*other_parking);
        other_lock.read_release(&*other_parking);
    });
    assert_eq!(receive.recv_timeout(WATCHDOG).unwrap(), -1);
    lock.write_release(&*parking);
    handle.join().unwrap();
    assert_eq!(parking.notifications.lock().unwrap()[0], u32::MAX);
}

#[test]
fn multiple_waiting_writers_complete() {
    let lock = Arc::new(MemoryAccessLock::new());
    let (send, receive) = mpsc::channel();
    let parking = Arc::new(Parking {
        parked: Some(send),
        ..Default::default()
    });
    lock.read_acquire(&*parking);
    let first = writer(lock.clone(), parking.clone());
    receive.recv_timeout(WATCHDOG).unwrap();
    let second = writer(lock.clone(), parking.clone());
    receive.recv_timeout(WATCHDOG).unwrap();
    lock.read_release(&*parking);
    first.join().unwrap();
    second.join().unwrap();
    assert_eq!(lock.state.load(Ordering::Relaxed), 0);
}

#[test]
fn spurious_wait_return_retries_acquisition() {
    let lock = Arc::new(MemoryAccessLock::new());
    let (send, receive) = mpsc::channel();
    let parking = Arc::new(Parking {
        parked: Some(send),
        spurious_once: AtomicBool::new(true),
        ..Default::default()
    });
    lock.read_acquire(&*parking);
    let handle = writer(lock.clone(), parking.clone());
    receive.recv_timeout(WATCHDOG).unwrap();
    lock.read_release(&*parking);
    handle.join().unwrap();
}

#[cfg(feature = "waiter-aware-memory-lock")]
#[test]
fn saturated_readers_do_not_overflow_the_marker() {
    for marker in [0, MARKER] {
        let lock = MemoryAccessLock {
            state: AtomicI32::new(marker | (MARKER - 1)),
        };
        let parking = Parking::default();
        lock.read_acquire(&parking);
        assert!(parking.saturation_seen.load(Ordering::Relaxed));
        assert_eq!(lock.state.load(Ordering::Relaxed), marker | (MARKER - 1));
        lock.read_release(&parking);
    }
}

#[test]
fn readers_and_writers_preserve_exclusion_and_payload() {
    let lock = MemoryAccessLock::new();
    let parking = Parking::default();
    let occupants = AtomicI32::new(0);
    let first = AtomicUsize::new(0);
    let second = AtomicUsize::new(!0);
    thread::scope(|scope| {
        for index in 0..4 {
            let (lock, parking, occupants, first, second) =
                (&lock, &parking, &occupants, &first, &second);
            scope.spawn(move || {
                for iteration in 0..100 {
                    if index % 2 == 0 {
                        lock.write_acquire(parking);
                        assert_eq!(occupants.swap(-1, Ordering::Relaxed), 0);
                        first.store(iteration, Ordering::Relaxed);
                        second.store(!iteration, Ordering::Relaxed);
                        assert_eq!(occupants.swap(0, Ordering::Relaxed), -1);
                        lock.write_release(parking);
                    } else {
                        lock.read_acquire(parking);
                        assert!(occupants.fetch_add(1, Ordering::Relaxed) >= 0);
                        assert_eq!(
                            first.load(Ordering::Relaxed),
                            !second.load(Ordering::Relaxed)
                        );
                        assert!(occupants.fetch_sub(1, Ordering::Relaxed) > 0);
                        lock.read_release(parking);
                    }
                }
            });
        }
    });
    assert_eq!(lock.state.load(Ordering::Relaxed), 0);
}
