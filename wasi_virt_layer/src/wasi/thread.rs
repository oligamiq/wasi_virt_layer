use core::{
    cell::Cell,
    num::NonZero,
    ptr::NonNull,
    sync::atomic::{AtomicU32, AtomicUsize, Ordering},
};
use std::{sync::Arc, thread::JoinHandle};

#[allow(unused_imports)]
use crate::{__private::wasip1, memory::WasmAccess};
use crate::{memory::WasmAccessName, utils::UnsafeOnceCell};

const DEFAULT_RESERVED_THREAD_ID_BASE: u32 = 1_000_000;

thread_local! {
    // Tracks reusable worker state for direct and pooled thread reinitialization.
    static WORKER_HAS_RUN_BEFORE: Cell<bool> = const { Cell::new(false) };
    static IS_VIRTUAL_THREAD_POOL_WORKER: Cell<bool> = const { Cell::new(false) };
    static IS_WASI_STARTED_THREAD: Cell<bool> = const { Cell::new(false) };
}

/// Returns true when the current thread is a reusable [`VirtualThreadPool`] worker.
pub fn is_virtual_thread_pool_worker() -> bool {
    IS_VIRTUAL_THREAD_POOL_WORKER.with(Cell::get)
}

/// Returns true when a direct target export should refresh thread-local target state.
pub fn should_reinitialize_direct_export_thread() -> bool {
    is_virtual_thread_pool_worker()
        || IS_WASI_STARTED_THREAD.with(Cell::get)
        || current_thread_debug_id().is_some_and(|id| id != 1)
}

/// Marks the current thread as having entered through the WASI thread-start entrypoint.
pub fn mark_wasi_thread_started() {
    IS_WASI_STARTED_THREAD.with(|flag| flag.set(true));
}

fn mark_virtual_thread_pool_worker() {
    IS_VIRTUAL_THREAD_POOL_WORKER.with(|flag| flag.set(true));
}

static NEXT_HOST_THREAD_ID: AtomicU32 = AtomicU32::new(1);

fn get_host_thread_id() -> u32 {
    HOST_THREAD_ID.with(|&id| id)
}

fn current_thread_debug_id() -> Option<u32> {
    let id_str = format!("{:?}", std::thread::current().id());
    id_str
        .trim_start_matches("ThreadId(")
        .trim_end_matches(')')
        .parse()
        .ok()
}

thread_local! {
    static HOST_THREAD_ID: u32 = NEXT_HOST_THREAD_ID.fetch_add(1, Ordering::Relaxed);
    static THREAD_LOCAL_COUNTER: Cell<u32> = Cell::new(0);
}

/// Generates a unique thread ID using the host thread ID and a TLS counter.
fn next_thread_id() -> u32 {
    let host_id = get_host_thread_id();
    let counter = THREAD_LOCAL_COUNTER.with(|c| {
        let val = c.get();
        // Rotate (wrap around) when it reaches 100,000
        c.set((val + 1) % 100_000);
        val
    });

    // Multiply external (host) thread ID by 100,000 and add the local counter
    host_id.wrapping_mul(100_000).wrapping_add(counter)
}

/// Generates guest-visible thread IDs for virtual thread pools.
///
/// # Collision contract
///
/// The guest runtime maintains a **single namespace** for thread IDs. IDs
/// returned by this generator must never collide with IDs assigned by the
/// external `WasiRunner` (host) for unmanaged threads, nor with IDs from
/// other [`VirtualThreadPool`] instances sharing the same guest namespace.
/// Any collision causes the guest runtime to confuse two different threads,
/// resulting in TLS corruption, `thread_join` / `thread_exit` errors,
/// scheduler panics, or undefined behavior.
///
/// The default [`ReservedRangeThreadIdGenerator`] uses a base offset of
/// 1,000,000 to avoid common host-side ID ranges. Custom implementors *must*
/// coordinate with the host runner to guarantee disjoint ID allocation, or
/// accept the risk of collisions.
pub trait ThreadIdGenerator: Send + Sync + 'static {
    /// Returns the next thread ID for the given thread accessor namespace.
    fn next_thread_id(&self, accessor: usize) -> Option<NonZero<u32>>;
}

/// Thread ID generator that allocates from a reserved positive `i32` range.
///
/// IDs are intentionally not reused because the generator cannot know when the
/// guest runtime has fully released a thread ID. If multiple virtual pools run
/// in the same guest thread namespace, use a shared custom generator or
/// disjoint ranges to avoid duplicate IDs.
///
/// # Collision with external (host) thread IDs
///
/// The guest runtime maintains a **single namespace** for thread IDs shared
/// between threads spawned by the external `WasiRunner` (host) and threads
/// spawned through this pool (VFS). If a host-side thread and a pool-side
/// thread receive the **same guest-visible ThreadID**, the guest runtime
/// will confuse the two — causing corruption in thread-local storage (TLS),
/// incorrect `thread_join` / `thread_exit` tracking, scheduler panics,
/// or arbitrary undefined behavior.
///
/// The default base of 1,000,000 is chosen to stay above the range that
/// typical host-side (WasiRunner) generators produce (usually small
/// monotonically increasing IDs). This is a **convention, not a guarantee**.
/// `WasiRunner` must also avoid allocating IDs within the pool's reserved
/// range. When this guarantee cannot be enforced (e.g. third-party runner),
/// use a custom [`ThreadIdGenerator`] that coordinates with the runner
/// (e.g. via a shared atomic counter or a disjoint bit‑partitioned scheme).
///
/// If the same guest-visible thread namespace contains **multiple**
/// `VirtualThreadPool` instances, each must use either a shared generator
/// or disjoint ranges to avoid cross-pool collisions.
pub struct ReservedRangeThreadIdGenerator {
    next: AtomicU32,
    max: u32,
}

impl ReservedRangeThreadIdGenerator {
    /// Creates a generator that starts at `base` and stops at `max`.
    pub const fn new(base: u32, max: u32) -> Self {
        Self {
            next: AtomicU32::new(if base == 0 { 1 } else { base }),
            max: if max > i32::MAX as u32 {
                i32::MAX as u32
            } else {
                max
            },
        }
    }

    /// Creates the default generator for IDs reserved away from external runners.
    pub const fn new_default() -> Self {
        Self::new(DEFAULT_RESERVED_THREAD_ID_BASE, i32::MAX as u32)
    }
}

impl ThreadIdGenerator for ReservedRangeThreadIdGenerator {
    fn next_thread_id(&self, _accessor: usize) -> Option<NonZero<u32>> {
        let mut current = self.next.load(Ordering::Relaxed);
        loop {
            if current > self.max {
                return None;
            }

            let next = current + 1;
            match self.next.compare_exchange_weak(
                current,
                next,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => return NonZero::new(current),
                Err(actual) => current = actual,
            }
        }
    }
}

/// Trait for a virtual thread implementation.
pub trait VirtualThread<ThreadAccessor: ThreadAccess> {
    /// Creates a new thread and returns its ID.
    fn new_thread(&self, accessor: ThreadAccessor, runner: ThreadRunner) -> Option<NonZero<u32>>;

    /// Yields the execution of the current thread.
    #[inline(always)]
    fn sched_yield<Wasm: WasmAccess + WasmAccessName + 'static>(&self) -> wasip1::Errno {
        #[cfg(target_os = "wasi")]
        {
            wasip1::ERRNO_SUCCESS
        }

        #[cfg(not(target_os = "wasi"))]
        {
            std::thread::yield_now();
            wasip1::ERRNO_SUCCESS
        }
    }
}

/// ref ~/.rustup/toolchains/stable-x86_64-pc-windows-msvc/lib/rustlib/src/rust/library/std/src/sys/pal/wasi/thread.rs
/// this type is *mut Box<dyn FnOnce()>
/// but we can't use it directly, because ABI was not designed with this in mind
/// A wrapper for a thread's entry point function.
#[derive(Debug)]
pub struct ThreadRunner {
    main: NonNull<Box<dyn FnOnce()>>,
}

unsafe impl Send for ThreadRunner {}

impl ThreadRunner {
    /// Creates a new `ThreadRunner` from a raw pointer to a boxed closure.
    pub fn __new(ptr: *mut Box<dyn FnOnce()>) -> Self {
        ThreadRunner {
            main: NonNull::new(ptr).unwrap(),
        }
    }

    /// Returns the underlying raw pointer.
    pub const fn inner(self) -> *mut Box<dyn FnOnce()> {
        self.main.as_ptr()
    }
}

/// Trait for accessing WASM thread start and identification.
pub trait ThreadAccess: Send + 'static + Copy {
    /// Calls the `wasi_thread_start` exported function in the WASM module.
    fn call_wasi_thread_start(&self, ptr: ThreadRunner, thread_id: Option<NonZero<u32>>);

    /// Calls the Wasm start section function to reinitialize the worker thread.
    ///
    /// This is called by `VirtualThreadPool` before `call_wasi_thread_start`
    /// when reusing a worker thread for a new logical thread. It re-runs
    /// global constructors and TLS initialization that would otherwise
    /// be lost because the worker thread was not freshly spawned.
    fn call_thread_start_init(&self);

    /// Returns the name of the WASM module.
    fn as_name(&self) -> &'static str;

    /// Returns the accessor as a unique `usize` value.
    fn as_usize(&self) -> usize;

    /// Creates an accessor from a unique `usize` value.
    fn from_usize(v: usize) -> Self
    where
        Self: Sized;
}

/// A wrapper for a `ThreadAccess` implementor that can be safely passed between threads.
pub struct ThreadAccessorWrapper<T: ThreadAccess> {
    inner: usize,
    _marker: core::marker::PhantomData<T>,
}

impl<T: ThreadAccess> ThreadAccessorWrapper<T> {
    /// Creates a new `ThreadAccessorWrapper`.
    pub fn new(accessor: T) -> Self {
        ThreadAccessorWrapper {
            inner: accessor.as_usize(),
            _marker: core::marker::PhantomData,
        }
    }

    /// Returns the original accessor.
    pub fn as_accessor(&self) -> T {
        T::from_usize(self.inner)
    }
}

#[derive(Clone)]
struct JoinPoolHandle {
    pool: Arc<parking_lot::Mutex<Vec<JoinHandle<()>>>>,
}

impl Default for JoinPoolHandle {
    fn default() -> Self {
        JoinPoolHandle {
            pool: Arc::new(parking_lot::Mutex::new(Vec::new())),
        }
    }
}

impl JoinPoolHandle {
    pub fn lock(&self) -> parking_lot::MutexGuard<'_, Vec<JoinHandle<()>>> {
        self.pool.lock()
    }

    pub fn extend<I: IntoIterator<Item = JoinHandle<()>>>(&self, iter: I) {
        let mut guard = self.pool.lock();
        guard.extend(iter);
    }
}

struct InFlightRunGuard(Arc<AtomicUsize>);

impl Drop for InFlightRunGuard {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::SeqCst);
    }
}

/// Options for waiting for threads in a thread pool to join.
pub enum WaitThreadJoin {
    /// No waiting required.
    None,
    /// Wait for a single signal.
    Recv(std::sync::mpsc::Receiver<()>),
    /// Wait for N signals.
    RecvN(flume::Receiver<()>, usize),
}

impl WaitThreadJoin {
    /// Block until the join condition is met.
    pub fn wait(self) {
        match self {
            WaitThreadJoin::None => {}
            WaitThreadJoin::Recv(recv) => {
                #[cfg(feature = "trace-thread")]
                println!("Waiting for thread pool flush to complete...");
                #[cfg(feature = "trace-thread")]
                println!("Thread ID: {:?}", std::thread::current().id());
                let _s = recv.recv();
                #[cfg(feature = "trace-thread")]
                println!("Thread pool flush completed: {:?}", _s);
            }
            WaitThreadJoin::RecvN(recv, n) => {
                #[cfg(feature = "trace-thread")]
                println!(
                    "Waiting for thread pool flush to complete for {} threads...",
                    n
                );
                for _ in 0..n {
                    let _s = recv.recv();
                    #[cfg(feature = "trace-thread")]
                    println!("Thread pool flush completed for one thread: {:?}", _s);
                }
            }
        }
    }
}

enum VirtualThreadPoolMessage<ThreadAccessor: ThreadAccess> {
    Run(
        ThreadRunner,
        ThreadAccessorWrapper<ThreadAccessor>,
        NonZero<u32>,
        InFlightRunGuard,
    ),
    AddThread(usize, std::sync::mpsc::SyncSender<()>, JoinPoolHandle),
    Terminate(flume::Sender<()>, JoinPoolHandle),
}

impl<ThreadAccessor: ThreadAccess> VirtualThreadPoolMessage<ThreadAccessor> {
    pub fn use_(self, queue: &flume::Receiver<VirtualThreadPoolMessage<ThreadAccessor>>) -> bool {
        match self {
            VirtualThreadPoolMessage::Run(runner, accessor_wrapper, thread_id, in_flight_guard) => {
                let accessor = accessor_wrapper.as_accessor();
                // Track whether this worker thread has already processed a Run
                // message. On the first Run, the Wasm start section was already
                // executed during module instantiation. On subsequent Runs, the
                // worker is being reused and needs start-section reinitialization.
                WORKER_HAS_RUN_BEFORE.with(|flag| {
                    if flag.get() {
                        accessor.call_thread_start_init();
                    }
                    flag.set(true);
                });
                let _in_flight_guard = in_flight_guard;
                accessor.call_wasi_thread_start(runner, Some(thread_id));
                #[cfg(feature = "trace-thread")]
                println!("[] Thread pool worker finished Run for thread {thread_id}");
            }
            VirtualThreadPoolMessage::AddThread(count, ref sender, ref kept_workers_pool) => {
                // Passing an iterator to kept_workers_pool causes the lock to hold for too long.
                let threads = self.create_thread(count, &queue).collect::<Vec<_>>();
                kept_workers_pool.extend(threads);
                let _s = sender.try_send(());
                #[cfg(feature = "trace-thread")]
                println!("Sent add thread completion signal: {:?}", _s);
            }
            VirtualThreadPoolMessage::Terminate(sender, pool) => {
                let thread_id = std::thread::current().id();
                let mut _guard = pool.lock();
                if let Some(pos) = _guard.iter().position(|h| h.thread().id() == thread_id) {
                    _guard.remove(pos);
                    core::mem::drop(_guard);
                } else {
                    panic!("Thread not found in pool during termination");
                }

                let _ = sender.send(());

                return false;
            }
        }
        true
    }

    fn listen(queue: &flume::Receiver<VirtualThreadPoolMessage<ThreadAccessor>>) {
        while let Ok(message) = queue.recv() {
            if !message.use_(queue) {
                break;
            }
        }
    }

    fn listen_with(
        queue: &flume::Receiver<VirtualThreadPoolMessage<ThreadAccessor>>,
        message: VirtualThreadPoolMessage<ThreadAccessor>,
    ) {
        if message.use_(queue) {
            Self::listen(queue);
        }
    }

    fn create_thread(
        &self,
        count: usize,
        queue: &flume::Receiver<VirtualThreadPoolMessage<ThreadAccessor>>,
    ) -> impl Iterator<Item = JoinHandle<()>> {
        #[cfg(feature = "trace-thread")]
        println!("Creating {count} threads in the thread pool...");
        core::iter::repeat_n(queue.clone(), count).map(move |queue| {
            let thread = root_spawn(std::thread::Builder::new(), move || {
                mark_virtual_thread_pool_worker();
                Self::listen(&queue);
            })
            .unwrap();
            thread
        })
    }
}

/// A thread pool implementation for virtual threads.
pub struct VirtualThreadPool<
    ThreadAccessor: ThreadAccess,
    Generator: ThreadIdGenerator = ReservedRangeThreadIdGenerator,
> {
    max_threads: AtomicUsize,
    read_kept_workers_pool_size: AtomicUsize,
    queue: parking_lot::Mutex<Option<flume::Sender<VirtualThreadPoolMessage<ThreadAccessor>>>>,
    queue_receiver: UnsafeOnceCell<flume::Receiver<VirtualThreadPoolMessage<ThreadAccessor>>>,
    kept_workers_pool: UnsafeOnceCell<JoinPoolHandle>,
    in_flight_runs: UnsafeOnceCell<Arc<AtomicUsize>>,
    thread_id_generator: Generator,
}

unsafe impl<ThreadAccessor: ThreadAccess, Generator: ThreadIdGenerator> Send
    for VirtualThreadPool<ThreadAccessor, Generator>
{
}
unsafe impl<ThreadAccessor: ThreadAccess, Generator: ThreadIdGenerator> Sync
    for VirtualThreadPool<ThreadAccessor, Generator>
{
}

impl<ThreadAccessor: ThreadAccess>
    VirtualThreadPool<ThreadAccessor, ReservedRangeThreadIdGenerator>
{
    /// Creates a new `VirtualThreadPool` without initialization.
    pub const unsafe fn new_const(max_threads: usize) -> Self {
        VirtualThreadPool {
            max_threads: AtomicUsize::new(max_threads),
            kept_workers_pool: UnsafeOnceCell::new(),
            in_flight_runs: UnsafeOnceCell::new(),
            queue: parking_lot::Mutex::new(None),
            queue_receiver: UnsafeOnceCell::new(),
            read_kept_workers_pool_size: AtomicUsize::new(0),
            thread_id_generator: ReservedRangeThreadIdGenerator::new_default(),
        }
    }
}

impl<ThreadAccessor: ThreadAccess, Generator: ThreadIdGenerator>
    VirtualThreadPool<ThreadAccessor, Generator>
{
    /// Creates a new `VirtualThreadPool` without initialization, using a custom thread ID generator.
    pub const unsafe fn new_const_with_thread_id_generator(
        max_threads: usize,
        generator: Generator,
    ) -> Self {
        VirtualThreadPool {
            max_threads: AtomicUsize::new(max_threads),
            kept_workers_pool: UnsafeOnceCell::new(),
            in_flight_runs: UnsafeOnceCell::new(),
            queue: parking_lot::Mutex::new(None),
            queue_receiver: UnsafeOnceCell::new(),
            read_kept_workers_pool_size: AtomicUsize::new(0),
            thread_id_generator: generator,
        }
    }

    /// Initializes the thread pool. This must be called before use.
    /// It is unsafe because it must only be called once, and the caller must ensure that no threads are using the pool until initialization is complete.
    pub unsafe fn init(&self) {
        if unsafe { self.kept_workers_pool.init_default().is_ok() } {
            unsafe {
                self.in_flight_runs
                    .init(Arc::new(AtomicUsize::new(0)))
                    .unwrap()
            };
            let (sender, receiver) = flume::unbounded();
            *self.queue.lock() = Some(sender);
            unsafe { self.queue_receiver.init(receiver).unwrap() };
        }
    }

    /// Initializes the thread pool, sets its capacity, and adjusts the worker count.
    ///
    /// The returned [`WaitThreadJoin`] can be used to wait until the requested
    /// worker count is ready.
    ///
    /// This has the same safety requirements as [`Self::init`].
    pub unsafe fn init_with_capacity(&self, max_threads: usize) -> WaitThreadJoin {
        unsafe { self.init() };
        self.resize(max_threads)
    }

    /// Initializes the thread pool, sets its capacity, and waits for workers to be ready.
    ///
    /// This has the same safety requirements as [`Self::init`].
    pub unsafe fn init_with_capacity_and_wait(&self, max_threads: usize) {
        unsafe { self.init_with_capacity(max_threads) }.wait();
    }

    /// Returns whether the thread pool has been initialized.
    pub fn is_initialized(&self) -> bool {
        self.queue.lock().is_some()
    }

    /// Returns the configured maximum capacity of the thread pool.
    pub fn capacity(&self) -> usize {
        self.max_threads.load(Ordering::SeqCst)
    }

    /// Returns the number of worker threads the pool currently tracks.
    pub fn worker_count(&self) -> usize {
        self.read_kept_workers_pool_size.load(Ordering::SeqCst)
    }

    /// Returns the number of pending queue items, or `None` if the pool is not initialized.
    pub fn queued_task_count(&self) -> Option<usize> {
        self.queue.lock().as_ref().map(flume::Sender::len)
    }

    /// Sets the maximum capacity of the thread pool.
    pub fn set_capacity(&self, max_threads: usize) {
        self.max_threads.store(max_threads, Ordering::SeqCst);
    }

    /// Sets the maximum capacity and adjusts the worker count to match it.
    ///
    /// The pool must already be initialized.
    pub fn resize(&self, max_threads: usize) -> WaitThreadJoin {
        self.set_capacity(max_threads);
        self.flush_capacity()
    }

    /// Sets the maximum capacity and waits until the worker count has been adjusted.
    ///
    /// The pool must already be initialized.
    pub fn resize_and_wait(&self, max_threads: usize) {
        self.resize(max_threads).wait();
    }

    fn add_queue_with<T>(
        &self,
        f: impl FnOnce(
            &mut flume::Sender<VirtualThreadPoolMessage<ThreadAccessor>>,
        ) -> Option<(VirtualThreadPoolMessage<ThreadAccessor>, T)>,
    ) -> Option<T> {
        let mut lock = self.queue.lock();
        let r = if let Some((msg, t)) = f(&mut lock.as_mut().unwrap()) {
            let _ = lock.as_mut().unwrap().send(msg).unwrap();
            Some(t)
        } else {
            None
        };
        core::mem::drop(lock);
        r
    }

    /// Adjusts the number of running worker threads to match the current capacity.
    pub fn flush_capacity(&self) -> WaitThreadJoin {
        let max_threads = self.max_threads.load(Ordering::SeqCst);

        let current_len = self.read_kept_workers_pool_size.load(Ordering::SeqCst);

        if current_len == max_threads {
            // no change
            return WaitThreadJoin::None;
        }

        if self
            .read_kept_workers_pool_size
            .compare_exchange(current_len, max_threads, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            // another thread is updating
            return WaitThreadJoin::None;
        }

        let mut pool = self.kept_workers_pool.lock();

        if current_len < max_threads {
            #[cfg(feature = "trace-thread")]
            println!("[] Increasing thread pool size from {current_len} to {max_threads}");

            let count = max_threads - current_len;
            let (send, recv) = std::sync::mpsc::sync_channel(count);
            #[cfg(feature = "trace-thread")]
            println!("[] count {count}");
            let msg = VirtualThreadPoolMessage::<ThreadAccessor>::AddThread(
                count - 1,
                send,
                self.kept_workers_pool.clone(),
            );

            let queue_receiver = self.queue_receiver.clone();

            let handle = root_spawn(std::thread::Builder::new(), move || {
                mark_virtual_thread_pool_worker();
                #[cfg(feature = "trace-thread")]
                println!("[] Thread pool addition thread started.");

                VirtualThreadPoolMessage::listen_with(&queue_receiver, msg);
            })
            .unwrap();

            pool.push(handle);
            return WaitThreadJoin::Recv(recv);
        } else {
            let mut sender = self.queue.lock();

            let count = current_len - max_threads;

            let (send, recv) = flume::bounded(count);

            for _ in 0..count {
                let _ = sender
                    .as_mut()
                    .unwrap()
                    .send(VirtualThreadPoolMessage::Terminate(
                        send.clone(),
                        self.kept_workers_pool.clone(),
                    ));
            }

            return WaitThreadJoin::RecvN(recv, count);
        }
    }

    /// Runs a thread runner on an available worker thread.
    ///
    /// When a worker thread is reused (has already processed a previous `Run`),
    /// the Wasm start section function is re-executed before
    /// `wasi_thread_start` to reinitialize global constructors and TLS state.
    /// Reuse detection is per-worker via a `thread_local` flag.
    pub fn run(&self, accessor: ThreadAccessor, runner: ThreadRunner, thread_id: NonZero<u32>) {
        let in_flight_runs = unsafe { self.in_flight_runs.get() }.clone();
        let need_expansion = {
            let mut sender_lock = self.queue.lock();
            let sender = sender_lock
                .as_mut()
                .expect("Thread pool queue not initialized");
            let in_flight_after_enqueue = in_flight_runs.fetch_add(1, Ordering::SeqCst) + 1;
            let in_flight_guard = InFlightRunGuard(in_flight_runs);

            sender
                .send(VirtualThreadPoolMessage::Run(
                    runner,
                    ThreadAccessorWrapper::new(accessor),
                    thread_id,
                    in_flight_guard,
                ))
                .unwrap();

            // Keep one spare worker once all target-capacity workers are occupied.
            // Some Wasm workloads synchronously spawn/join from worker threads; by
            // then the queue can be empty because the parent work was already consumed.
            let queue_len = sender.len();
            let max_threads = self.max_threads.load(Ordering::SeqCst);
            let need_expansion = queue_len > 0 || in_flight_after_enqueue >= max_threads;
            #[cfg(feature = "trace-thread")]
            println!(
                "[] VTP enqueue thread {thread_id}: queue_len={queue_len} in_flight={in_flight_after_enqueue} max={max_threads} current={} need_expansion={need_expansion}",
                self.read_kept_workers_pool_size.load(Ordering::SeqCst)
            );
            need_expansion
        };

        if need_expansion {
            let current = self.read_kept_workers_pool_size.load(Ordering::SeqCst);
            let max = self.max_threads.load(Ordering::SeqCst);
            if current < max {
                let _ = self.flush_capacity();
            } else if self
                .max_threads
                .compare_exchange(max, max + 1, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
            {
                #[cfg(feature = "trace-thread")]
                println!(
                    "[] Automatically expanding thread pool capacity to {}",
                    max + 1
                );

                let _ = self.flush_capacity();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy)]
    struct TestThreadAccessor;

    impl ThreadAccess for TestThreadAccessor {
        fn call_wasi_thread_start(&self, ptr: ThreadRunner, _thread_id: Option<NonZero<u32>>) {
            unsafe {
                let boxed: Box<Box<dyn FnOnce()>> = Box::from_raw(ptr.inner());
                boxed();
            }
        }

        fn call_thread_start_init(&self) {}

        fn as_name(&self) -> &'static str {
            "test-thread-accessor"
        }

        fn as_usize(&self) -> usize {
            0
        }

        fn from_usize(_v: usize) -> Self {
            Self
        }
    }

    fn test_runner(f: impl FnOnce() + 'static) -> ThreadRunner {
        let boxed: Box<dyn FnOnce()> = Box::new(f);
        ThreadRunner::__new(Box::into_raw(Box::new(boxed)))
    }

    #[test]
    fn virtual_thread_pool_reports_capacity_and_initialization_state() {
        let pool = unsafe { VirtualThreadPool::<TestThreadAccessor>::new_const(3) };

        assert!(!pool.is_initialized());
        assert_eq!(pool.capacity(), 3);
        assert_eq!(pool.worker_count(), 0);
        assert_eq!(pool.queued_task_count(), None);

        unsafe { pool.init() };

        assert!(pool.is_initialized());
        assert_eq!(pool.queued_task_count(), Some(0));
    }

    #[test]
    fn virtual_thread_pool_can_initialize_and_resize_with_helpers() {
        let pool = unsafe { VirtualThreadPool::<TestThreadAccessor>::new_const(0) };

        unsafe { pool.init_with_capacity_and_wait(2) };

        assert!(pool.is_initialized());
        assert_eq!(pool.capacity(), 2);
        assert_eq!(pool.worker_count(), 2);
        assert_eq!(pool.queued_task_count(), Some(0));

        pool.resize_and_wait(0);

        assert_eq!(pool.capacity(), 0);
        assert_eq!(pool.worker_count(), 0);
    }

    #[test]
    fn test_reserved_range_thread_id_generator_default() {
        let generator = ReservedRangeThreadIdGenerator::new_default();
        let id1 = generator.next_thread_id(0).unwrap();
        assert_eq!(id1.get(), 1_000_000);
        let id2 = generator.next_thread_id(0).unwrap();
        assert_eq!(id2.get(), 1_000_001);
    }

    #[test]
    fn test_reserved_range_thread_id_generator_exhaustion() {
        let generator = ReservedRangeThreadIdGenerator::new(10, 11);
        assert_eq!(generator.next_thread_id(0).unwrap().get(), 10);
        assert_eq!(generator.next_thread_id(0).unwrap().get(), 11);
        assert!(generator.next_thread_id(0).is_none());
    }

    #[test]
    fn test_reserved_range_thread_id_generator_skips_zero() {
        let generator = ReservedRangeThreadIdGenerator::new(0, 2);
        assert_eq!(generator.next_thread_id(0).unwrap().get(), 1);
        assert_eq!(generator.next_thread_id(0).unwrap().get(), 2);
        assert!(generator.next_thread_id(0).is_none());
    }

    #[test]
    fn test_reserved_range_thread_id_generator_rejects_negative_i32_range() {
        let generator = ReservedRangeThreadIdGenerator::new(i32::MAX as u32, u32::MAX);
        assert_eq!(generator.next_thread_id(0).unwrap().get(), i32::MAX as u32);
        assert!(generator.next_thread_id(0).is_none());
    }

    #[test]
    fn test_virtual_thread_pool_custom_generator() {
        let generator = ReservedRangeThreadIdGenerator::new(50, 60);
        let pool = unsafe {
            VirtualThreadPool::<TestThreadAccessor, _>::new_const_with_thread_id_generator(
                2, generator,
            )
        };
        assert_eq!(
            pool.thread_id_generator.next_thread_id(0).unwrap().get(),
            50
        );
    }

    struct OutOfRangeThreadIdGenerator;

    impl ThreadIdGenerator for OutOfRangeThreadIdGenerator {
        fn next_thread_id(&self, _accessor: usize) -> Option<NonZero<u32>> {
            NonZero::new(i32::MAX as u32 + 1)
        }
    }

    #[test]
    fn virtual_thread_pool_rejects_custom_generator_ids_outside_i32_range() {
        let pool = unsafe {
            VirtualThreadPool::<TestThreadAccessor, _>::new_const_with_thread_id_generator(
                0,
                OutOfRangeThreadIdGenerator,
            )
        };
        assert!(
            pool.new_thread(TestThreadAccessor, test_runner(|| {}))
                .is_none()
        );
    }

    #[test]
    fn virtual_thread_pool_spawns_worker_for_queued_task_when_under_capacity() {
        use std::sync::atomic::AtomicBool;

        let pool = unsafe { VirtualThreadPool::<TestThreadAccessor>::new_const(2) };
        unsafe { pool.init() };

        let ran = std::sync::Arc::new(AtomicBool::new(false));
        let ran_clone = ran.clone();

        let runner = test_runner(move || {
            ran_clone.store(true, std::sync::atomic::Ordering::SeqCst);
        });

        let result = pool.new_thread(TestThreadAccessor, runner);
        assert!(result.is_some());

        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while !ran.load(std::sync::atomic::Ordering::SeqCst) && std::time::Instant::now() < deadline
        {
            std::thread::yield_now();
        }
        assert!(
            ran.load(std::sync::atomic::Ordering::SeqCst),
            "pool did not spawn a worker for the queued task"
        );
    }

    #[test]
    fn virtual_thread_pool_expands_when_existing_worker_is_blocked() {
        use std::sync::{Barrier, atomic::AtomicBool};

        let pool = unsafe { VirtualThreadPool::<TestThreadAccessor>::new_const(1) };
        unsafe { pool.init_with_capacity_and_wait(1) };

        // First task: take the only worker and block it on a barrier.
        let barrier = std::sync::Arc::new(Barrier::new(2));
        let barrier_clone = barrier.clone();
        let runner1 = test_runner(move || {
            barrier_clone.wait();
        });
        let result1 = pool.new_thread(TestThreadAccessor, runner1);
        assert!(result1.is_some());

        // Wait for the worker to pick up the task and block.
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Second task: should trigger auto-expansion because the sole worker is blocked.
        let ran = std::sync::Arc::new(AtomicBool::new(false));
        let ran_clone = ran.clone();
        let barrier2 = barrier.clone();
        let runner2 = test_runner(move || {
            ran_clone.store(true, std::sync::atomic::Ordering::SeqCst);
            barrier2.wait();
        });
        let result2 = pool.new_thread(TestThreadAccessor, runner2);
        assert!(result2.is_some());

        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while !ran.load(std::sync::atomic::Ordering::SeqCst) && std::time::Instant::now() < deadline
        {
            std::thread::yield_now();
        }
        assert!(
            ran.load(std::sync::atomic::Ordering::SeqCst),
            "pool did not expand for the second task while the only worker was blocked"
        );
    }

    #[test]
    fn virtual_thread_pool_expands_when_workers_are_busy_without_queue_backlog() {
        use std::sync::mpsc;

        let pool = unsafe { VirtualThreadPool::<TestThreadAccessor>::new_const(2) };
        unsafe { pool.init_with_capacity_and_wait(2) };

        let (started_tx, started_rx) = mpsc::sync_channel(2);
        let (release_tx, release_rx) = mpsc::sync_channel::<()>(2);
        let release_rx = std::sync::Arc::new(std::sync::Mutex::new(release_rx));

        for task_id in 0..2 {
            let started_tx = started_tx.clone();
            let release_rx = release_rx.clone();
            let runner = test_runner(move || {
                started_tx.send(task_id).unwrap();
                release_rx.lock().unwrap().recv().unwrap();
            });

            let result = pool.new_thread(TestThreadAccessor, runner);
            assert!(result.is_some());

            started_rx
                .recv_timeout(std::time::Duration::from_secs(5))
                .expect("worker did not start blocking task");

            assert_eq!(
                pool.queued_task_count(),
                Some(0),
                "task should be consumed immediately, leaving no queue backlog"
            );
        }

        assert!(
            pool.capacity() > 2,
            "pool should keep spare capacity when all existing workers are busy"
        );

        release_tx.send(()).unwrap();
        release_tx.send(()).unwrap();
    }
}

impl<ThreadAccessor: ThreadAccess, Generator: ThreadIdGenerator> VirtualThread<ThreadAccessor>
    for VirtualThreadPool<ThreadAccessor, Generator>
{
    fn new_thread(&self, accessor: ThreadAccessor, runner: ThreadRunner) -> Option<NonZero<u32>> {
        let thread_id_nz = self
            .thread_id_generator
            .next_thread_id(accessor.as_usize())?;
        if thread_id_nz.get() > i32::MAX as u32 {
            return None;
        }

        self.run(accessor, runner, thread_id_nz);

        Some(thread_id_nz)
    }
}

/// A "pool" that spawns native threads directly for each request.
pub struct DirectThreadPool<ThreadAccessor: ThreadAccess>(
    core::marker::PhantomData<ThreadAccessor>,
);

impl<ThreadAccessor: ThreadAccess> DirectThreadPool<ThreadAccessor> {
    /// Creates a new `DirectThreadPool`.
    pub const fn new_const() -> Self {
        DirectThreadPool(core::marker::PhantomData)
    }
}

mod spawn {
    use core::cell::UnsafeCell;

    // It is safe as it releases immediately.
    thread_local! {
        static IS_ROOT_THREAD: UnsafeCell<bool> = UnsafeCell::new(false);
    }

    struct RootSpawnFlagGuard(bool);

    impl RootSpawnFlagGuard {
        fn new() -> Self {
            let previous = IS_ROOT_THREAD.with(|flag| unsafe { flag.get().replace(true) });
            Self(previous)
        }
    }

    impl Drop for RootSpawnFlagGuard {
        fn drop(&mut self) {
            IS_ROOT_THREAD.with(|flag| {
                unsafe { flag.get().write(self.0) };
            });
        }
    }

    /// Spawn a new thread.
    /// If you call `std::thread::spawn` in ThreadPool, it will be looped.
    /// So, you should use `root_spawn` instead.
    pub fn root_spawn<F, T>(
        builder: std::thread::Builder,
        f: F,
    ) -> std::io::Result<std::thread::JoinHandle<T>>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
        let _guard = RootSpawnFlagGuard::new();
        builder.spawn(f)
    }

    /// Spawn a new thread using an unchecked closure.
    pub fn root_spawn_unchecked<F, T>(
        builder: std::thread::Builder,
        f: F,
    ) -> std::io::Result<std::thread::JoinHandle<T>>
    where
        F: FnOnce() -> T,
        F: Send,
        T: Send,
    {
        let _guard = RootSpawnFlagGuard::new();
        unsafe { builder.spawn_unchecked(f) }
    }

    #[cfg(target_os = "wasi")]
    #[unsafe(no_mangle)]
    /// When calling thread_spawn, first branch based on the result of this function.
    pub extern "C" fn __wasip1_vfs_is_root_spawn() -> bool {
        // get and turn off the flag
        IS_ROOT_THREAD.with(|flag| unsafe { flag.get().replace(false) })
    }
}

#[cfg(target_os = "wasi")]
pub use spawn::__wasip1_vfs_is_root_spawn;
pub use spawn::{root_spawn, root_spawn_unchecked};

impl<ThreadAccessor: ThreadAccess> VirtualThread<ThreadAccessor>
    for DirectThreadPool<ThreadAccessor>
{
    // new thread start function call by other wasm
    fn new_thread(&self, accessor: ThreadAccessor, runner: ThreadRunner) -> Option<NonZero<u32>> {
        let thread_id = next_thread_id();

        let thread_id_nz = NonZero::new(thread_id as u32)?;
        println!("VFS: Spawning thread {} with root_spawn", thread_id);

        let res = root_spawn(
            std::thread::Builder::new().name(format!("worker-{}", thread_id_nz)),
            move || {
                println!("VFS: inside root_spawn closure for {}", thread_id_nz);
                accessor.call_wasi_thread_start(runner, Some(thread_id_nz));
            },
        );
        match res {
            Ok(_) => {
                println!("VFS: root_spawn succeeded for {}", thread_id_nz);
            }
            Err(e) => {
                println!("VFS: root_spawn failed for {}: {}", thread_id_nz, e);
                return None;
            }
        }

        Some(thread_id_nz)
    }
}

#[cfg(feature = "detect-deadlock")]
mod deadlock_detector {
    use core::hash::BuildHasherDefault;

    use dashmap::DashMap;
    use fxhash::FxHasher;

    use super::DEFAULT_RESERVED_THREAD_ID_BASE;

    const VFS_SHELL_WASM_ID: u32 = 4;

    /// Atomic memory location observed by the deadlock detector.
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub(super) struct AtomicLocation {
        pub(super) wasm_id: u32,
        pub(super) relative_addr: u32,
        pub(super) width: u8,
    }

    #[derive(Clone, Copy, Debug)]
    struct ThreadWait {
        location: AtomicLocation,
        expected_low: u64,
        generation_at_wait: u64,
    }

    impl ThreadWait {
        fn is_possible_host_idle_wait(&self, thread_id: i32) -> bool {
            thread_id == DEFAULT_RESERVED_THREAD_ID_BASE as i32
                && self.location.wasm_id == VFS_SHELL_WASM_ID
                && self.location.width == 4
                && self.expected_low == u32::MAX as u64
        }
    }

    #[derive(Clone, Copy, Debug, Default)]
    struct LocationState {
        generation: u64,
        last_thread_id: i32,
    }

    /// Runtime wait graph state for atomic deadlock detection.
    #[derive(Default)]
    pub(super) struct DeadlockDetector {
        active_threads: DashMap<i32, (), BuildHasherDefault<FxHasher>>,
        waits: DashMap<i32, ThreadWait, BuildHasherDefault<FxHasher>>,
        locations: DashMap<AtomicLocation, LocationState, BuildHasherDefault<FxHasher>>,
    }

    impl DeadlockDetector {
        /// Marks a wasm thread as active for detector purposes.
        pub(super) fn record_thread_enter(&self, thread_id: i32) {
            self.active_threads.insert(thread_id, ());
        }

        /// Removes all detector state for a wasm thread.
        pub(super) fn record_thread_exit(&self, thread_id: i32) {
            self.waits.remove(&thread_id);
            self.active_threads.remove(&thread_id);
        }

        /// Records that a wasm thread is blocked on an atomic location.
        pub(super) fn record_wait_start(
            &self,
            thread_id: i32,
            location: AtomicLocation,
            expected_low: u64,
        ) {
            self.record_thread_enter(thread_id);
            let generation_at_wait = self.location_generation(location);
            self.waits.insert(
                thread_id,
                ThreadWait {
                    location,
                    expected_low,
                    generation_at_wait,
                },
            );
        }

        /// Clears a wasm thread's current wait edge.
        pub(super) fn record_wait_end(&self, thread_id: i32) {
            self.waits.remove(&thread_id);
        }

        /// Records a write, RMW, CAS, or notify that can advance a location.
        pub(super) fn record_location_change(&self, thread_id: i32, location: AtomicLocation) {
            self.record_thread_enter(thread_id);
            let mut entry = self.locations.entry(location).or_default();
            let state = entry.value_mut();
            state.generation = state.generation.wrapping_add(1);
            state.last_thread_id = thread_id;
        }

        /// Returns the location a thread is currently waiting on.
        pub(super) fn waiting_location(&self, thread_id: i32) -> Option<AtomicLocation> {
            self.waits.get(&thread_id).map(|wait| wait.location)
        }

        /// Returns the expected low bits recorded for a waiting thread.
        pub(super) fn waiting_expected_low(&self, thread_id: i32) -> Option<u64> {
            self.waits.get(&thread_id).map(|wait| wait.expected_low)
        }

        /// Returns the location generation captured when a thread started waiting.
        pub(super) fn waiting_generation_at_wait(&self, thread_id: i32) -> Option<u64> {
            self.waits
                .get(&thread_id)
                .map(|wait| wait.generation_at_wait)
        }

        /// Returns the observed generation for an atomic location.
        pub(super) fn location_generation(&self, location: AtomicLocation) -> u64 {
            self.locations
                .get(&location)
                .map(|state| state.generation)
                .unwrap_or(0)
        }

        /// Returns a deadlock diagnostic when all observed threads are waiting on unchanged locations.
        pub(super) fn check_deadlock(&self) -> Option<String> {
            let active_count = self.active_threads.len();
            if active_count == 0 {
                return None;
            }

            let mut waits = Vec::new();
            for thread in self.active_threads.iter() {
                let thread_id = *thread.key();
                let wait = self.waits.get(&thread_id)?;
                if active_count == 1 && wait.is_possible_host_idle_wait(thread_id) {
                    continue;
                }
                let current_generation = self.location_generation(wait.location);
                if current_generation != wait.generation_at_wait {
                    return None;
                }
                waits.push((thread_id, *wait));
            }

            if waits.is_empty() {
                return None;
            }

            let mut diagnostic = String::from("deadlock detected");
            for (thread_id, wait) in waits {
                diagnostic.push_str(&format!(
                    ": thread={thread_id} wasm={} addr={} width={} expected={} generation={}",
                    wait.location.wasm_id,
                    wait.location.relative_addr,
                    wait.location.width,
                    wait.expected_low,
                    wait.generation_at_wait
                ));
            }
            Some(diagnostic)
        }
    }
}

/// Plugs the thread ecosystem by defining necessary accessor enums and hooks.
/// Other plug_* macros can be split and used separately,
/// but due to the internal branching logic of this macro,
/// only one instance of this macro can be defined per VirtualThread.
///
/// ```rust,no_run
/// use wasi_virt_layer::prelude::*;
///
/// import_wasm!(test_wasm);
///
/// // Example: plug a thread pool to `test_wasm`
/// // plug_thread!({ &THREAD_POOL }, test_wasm, self);
/// ```
#[macro_export]
macro_rules! plug_thread {
    ($pool:tt, $($wasm:ident),* $(,)?) => {
        $crate::__as_t!(@through, $($wasm),* => $crate::plug_thread, @inner, $pool);
    };

    (@inner, $pool:tt, $($wasm:ident),* $(,)?) => {
        const _: () = {
            #[allow(unused)]
            let _ = || {
                $pool;
            };
        };

        $crate::__private::paste::paste! {
            #[allow(non_camel_case_types)]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[repr(usize)]
            pub(crate) enum ThreadAccessor {
                $(
                    $wasm,
                )*
            }

            impl $crate::thread::ThreadAccess for ThreadAccessor {
                fn call_wasi_thread_start(&self, ptr: $crate::thread::ThreadRunner, thread_id: Option<core::num::NonZero<u32>>) {
                    #[cfg(target_os = "wasi")]
                    {
                        $crate::__if_feature!(@trace_thread
                            println!("$$$ Calling wasi_thread_start in {}", self.as_name());
                        );
                        match *self {
                            $(
                                Self::$wasm => {
                                    $crate::__if_feature!(@trace_thread
                                        println!("Calling wasi_thread_start in {}", self.as_name());
                                        println!("  thread_id: {:?}", thread_id);
                                        println!("  data_ptr: {:?}", ptr);
                                    );
                                    unsafe { [<__wasip1_vfs_ $wasm _wasi_thread_start>](
                                        match thread_id {
                                            Some(id) => u32::from(id) as i32,
                                            None => -1,
                                        },
                                        ptr.inner() as i32,
                                    ) }
                                }
                            )*
                        }
                    }

                    #[cfg(not(target_os = "wasi"))]
                    {
                        panic!("This function is only available on WASI");
                    }
                }

                fn call_thread_start_init(&self) {
                    #[cfg(target_os = "wasi")]
                    {
                        match *self {
                            $(
                                Self::$wasm => {
                                    $crate::__if_feature!(@trace_thread
                                        println!("$$$ Calling thread_start_init in {}", self.as_name());
                                    );
                                    unsafe { [<__wasip1_vfs_ $wasm __thread_start>]() }
                                }
                            )*
                        }
                    }

                    #[cfg(not(target_os = "wasi"))]
                    {
                        panic!("This function is only available on WASI");
                    }
                }

                fn as_name(&self) -> &'static str {
                    match *self {
                        $(
                            Self::$wasm => {
                                $crate::__as_t!(@as_t, $wasm);
                                <T as $crate::memory::WasmAccessName>::NAME
                            }
                        )*
                    }
                }

                fn as_usize(&self) -> usize {
                    *self as usize
                }

                fn from_usize(v: usize) -> Self
                where
                    Self: Sized,
                {
                    match v {
                        $(
                            x if x == Self::$wasm as usize => Self::$wasm,
                        )*
                        _ => panic!("Invalid ThreadAccessor value: {v}"),
                    }
                }
            }

            $(
                #[cfg(target_os = "wasi")]
                #[doc(hidden)]
                #[link(wasm_import_module = "wasip1-vfs")]
                unsafe extern "C" {
                    pub fn [<__wasip1_vfs_ $wasm _wasi_thread_start>](
                        thread_id: i32,
                        ptr: i32,
                    );

                    pub fn [<__wasip1_vfs_ $wasm __thread_start>]();
                }

                #[cfg(target_os = "wasi")]
                #[unsafe(no_mangle)]
                unsafe extern "C" fn [<__wasip1_vfs_ $wasm _wasi_thread_start_anchor>](
                    thread_id: i32,
                    ptr: i32,
                ) {
                    unsafe {
                        [<__wasip1_vfs_ $wasm _wasi_thread_start>](thread_id, ptr);
                    }
                }

                #[cfg(target_os = "wasi")]
                #[unsafe(no_mangle)]
                unsafe extern "C" fn [<__wasip1_vfs_wasi_thread_spawn_ $wasm>](
                    data_ptr: *mut Box<dyn FnOnce()>,
                ) -> i32 {
                    use $crate::thread::{VirtualThread, ThreadAccess};
                    const ERRNO_AGAIN: i32 = -6;
                    const ACCESSOR: ThreadAccessor = ThreadAccessor::$wasm;
                    $crate::__if_feature!(@trace_thread
                        println!("$$$ Spawning a new thread in {}", ACCESSOR.as_name());
                    );

                    #[allow(unused_mut)]
                    let mut pool = $pool;

                    match pool.new_thread(ACCESSOR, $crate::thread::ThreadRunner::__new(data_ptr)) {
                        Some(thread_id) => {
                            return u32::from(thread_id) as i32;
                        },
                        None => {
                            return ERRNO_AGAIN;
                        }
                    }
                }

                $crate::plug_thread!(@sched_yield, $pool, $wasm);
            )*
        }
    };

    (@sched_yield, $pool:tt, $wasm:ident) => {
        $crate::__private::paste::paste! {
            #[unsafe(no_mangle)]
            #[cfg(target_os = "wasi")]
            pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _sched_yield>](
            ) -> $crate::__private::wasip1::Errno {
                use $crate::thread::VirtualThread;

                #[allow(unused_mut)]
                let mut pool = $pool;
                $crate::__as_t!(@as_t, $wasm);
                pool.sched_yield::<T>()
            }
        }
    };
}

// If a thread exists, it may be invoked multiple times.
// `Reset` is a process that must not be invoked multiple times.
#[cfg(feature = "threads")]
#[cfg(target_os = "wasi")]
mod reset_on_thread {
    use crate::utils::InitOnce;

    static INIT: InitOnce = InitOnce::new_const();

    #[link(wasm_import_module = "wasip1-vfs")]
    unsafe extern "C" {
        fn __wasip1_vfs_reset_on_thread_once();
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __wasip1_vfs_reset_on_thread() {
        INIT.call_once(|| {
            unsafe { __wasip1_vfs_reset_on_thread_once() };
        });
    }
}

#[cfg(target_os = "wasi")]
pub mod vfs_atomic {
    use dashmap::DashMap;
    use fxhash::FxHasher;
    use std::boxed::Box;
    use std::hash::BuildHasherDefault;
    use std::sync::LazyLock;

    #[cfg(feature = "detect-deadlock")]
    use super::deadlock_detector::{AtomicLocation, DeadlockDetector};

    #[cfg(feature = "detect-deadlock")]
    const DEADLOCK_WAIT_SLICE_NS: i64 = 50_000_000;

    #[cfg(feature = "detect-deadlock")]
    static DEADLOCK_DETECTOR: LazyLock<DeadlockDetector> = LazyLock::new(DeadlockDetector::default);

    #[cfg(feature = "detect-deadlock")]
    struct WaitGuard {
        thread_id: i32,
    }

    #[cfg(feature = "detect-deadlock")]
    impl Drop for WaitGuard {
        fn drop(&mut self) {
            DEADLOCK_DETECTOR.record_wait_end(self.thread_id);
        }
    }

    #[link(wasm_import_module = "wvl_atomic")]
    unsafe extern "C" {
        // Wait and notify on VFS memory (Memory 0)
        pub fn __wvl_atomic_wait32_vfs(addr: *const u32, expected: u32, timeout: i64) -> i32;
        pub fn __wvl_atomic_notify_vfs(addr: *const u32, count: u32) -> i32;

        // Lock operations on VFS memory (Memory 0)
        pub fn __wvl_atomic_cmpxchg32_vfs(addr: *mut u32, expected: u32, new: u32) -> u32;
        pub fn __wvl_atomic_store32_vfs(addr: *mut u32, val: u32);

        // Load operations on Target memory (Memory 1..N)
        pub fn __wvl_atomic_load32_target(wasm_id: u32, addr: *const u32) -> u32;
        pub fn __wvl_atomic_load64_target(wasm_id: u32, addr: *const u64) -> u64;
    }

    /// Marks a wasm thread as active in the deadlock detector.
    #[unsafe(no_mangle)]
    #[cfg(feature = "detect-deadlock")]
    pub unsafe extern "C" fn __vfs_deadlock_thread_enter(thread_id: i32) {
        DEADLOCK_DETECTOR.record_thread_enter(thread_id);
    }

    /// Removes a wasm thread from active deadlock detector state.
    #[unsafe(no_mangle)]
    #[cfg(feature = "detect-deadlock")]
    pub unsafe extern "C" fn __vfs_deadlock_thread_exit(thread_id: i32) {
        DEADLOCK_DETECTOR.record_thread_exit(thread_id);
    }

    static WAIT_MAP: LazyLock<DashMap<u64, Box<u32>, BuildHasherDefault<FxHasher>>> =
        LazyLock::new(DashMap::default);

    #[unsafe(no_mangle)]
    #[cfg(not(feature = "detect-deadlock"))]
    pub unsafe extern "C" fn __vfs_atomic_wait32(
        wasm_id: u32,
        relative_addr: u32,
        expected: u32,
        timeout: i64,
    ) -> i32 {
        let key = ((wasm_id as u64) << 32) | (relative_addr as u64);
        let (ptr, vfs_expected) = {
            let entry = WAIT_MAP.entry(key).or_insert_with(|| Box::new(0));
            unsafe {
                let val = __wvl_atomic_load32_target(wasm_id, relative_addr as *const u32);
                if val != expected {
                    return 1; // not-equal
                }
                let ptr = &**entry.value() as *const u32;
                let vfs_expected = *ptr;
                (ptr, vfs_expected)
            }
        };
        unsafe { __wvl_atomic_wait32_vfs(ptr, vfs_expected, timeout) }
    }

    #[unsafe(no_mangle)]
    #[cfg(feature = "detect-deadlock")]
    pub unsafe extern "C" fn __vfs_atomic_wait32(
        thread_id: i32,
        wasm_id: u32,
        relative_addr: u32,
        expected: u32,
        timeout: i64,
    ) -> i32 {
        let _ = thread_id;
        let key = ((wasm_id as u64) << 32) | (relative_addr as u64);
        let (ptr, vfs_expected) = {
            let entry = WAIT_MAP.entry(key).or_insert_with(|| Box::new(0));
            unsafe {
                let val = __wvl_atomic_load32_target(wasm_id, relative_addr as *const u32);
                if val != expected {
                    return 1; // not-equal
                }
                let ptr = &**entry.value() as *const u32;
                let vfs_expected = *ptr;
                (ptr, vfs_expected)
            }
        };
        let location = AtomicLocation {
            wasm_id,
            relative_addr,
            width: 4,
        };
        DEADLOCK_DETECTOR.record_wait_start(thread_id, location, expected as u64);
        let _guard = WaitGuard { thread_id };
        wait_with_deadlock_detection(ptr, vfs_expected, timeout)
    }

    #[unsafe(no_mangle)]
    #[cfg(not(feature = "detect-deadlock"))]
    pub unsafe extern "C" fn __vfs_atomic_wait64(
        wasm_id: u32,
        relative_addr: u32,
        expected: u64,
        timeout: i64,
    ) -> i32 {
        let key = ((wasm_id as u64) << 32) | (relative_addr as u64);
        let (ptr, vfs_expected) = {
            let entry = WAIT_MAP.entry(key).or_insert_with(|| Box::new(0));
            unsafe {
                let val = __wvl_atomic_load64_target(wasm_id, relative_addr as *const u64);
                if val != expected {
                    return 1; // not-equal
                }
                let ptr = &**entry.value() as *const u32;
                let vfs_expected = *ptr;
                (ptr, vfs_expected)
            }
        };
        unsafe { __wvl_atomic_wait32_vfs(ptr, vfs_expected, timeout) }
    }

    #[unsafe(no_mangle)]
    #[cfg(feature = "detect-deadlock")]
    pub unsafe extern "C" fn __vfs_atomic_wait64(
        thread_id: i32,
        wasm_id: u32,
        relative_addr: u32,
        expected: u64,
        timeout: i64,
    ) -> i32 {
        let _ = thread_id;
        let key = ((wasm_id as u64) << 32) | (relative_addr as u64);
        let (ptr, vfs_expected) = {
            let entry = WAIT_MAP.entry(key).or_insert_with(|| Box::new(0));
            unsafe {
                let val = __wvl_atomic_load64_target(wasm_id, relative_addr as *const u64);
                if val != expected {
                    return 1; // not-equal
                }
                let ptr = &**entry.value() as *const u32;
                let vfs_expected = *ptr;
                (ptr, vfs_expected)
            }
        };
        let location = AtomicLocation {
            wasm_id,
            relative_addr,
            width: 8,
        };
        DEADLOCK_DETECTOR.record_wait_start(thread_id, location, expected);
        let _guard = WaitGuard { thread_id };
        wait_with_deadlock_detection(ptr, vfs_expected, timeout)
    }

    #[cfg(feature = "detect-deadlock")]
    fn wait_with_deadlock_detection(ptr: *const u32, expected: u32, timeout: i64) -> i32 {
        let infinite = timeout < 0;
        let mut remaining = timeout;
        loop {
            let slice = if infinite {
                DEADLOCK_WAIT_SLICE_NS
            } else {
                remaining.min(DEADLOCK_WAIT_SLICE_NS)
            };
            let result = unsafe { __wvl_atomic_wait32_vfs(ptr, expected, slice) };
            if result != 2 {
                return result;
            }
            if let Some(diagnostic) = DEADLOCK_DETECTOR.check_deadlock() {
                panic!("{diagnostic}");
            }
            if !infinite {
                remaining = remaining.saturating_sub(slice);
                if remaining <= 0 {
                    return 2;
                }
            }
        }
    }

    #[unsafe(no_mangle)]
    #[cfg(not(feature = "detect-deadlock"))]
    pub unsafe extern "C" fn __vfs_atomic_notify(
        wasm_id: u32,
        relative_addr: u32,
        count: u32,
    ) -> i32 {
        let key = ((wasm_id as u64) << 32) | (relative_addr as u64);
        let ptr = {
            let mut entry = WAIT_MAP.entry(key).or_insert_with(|| Box::new(0));
            let val_mut = entry.value_mut().as_mut();
            *val_mut = val_mut.wrapping_add(1);
            val_mut as *const u32
        };
        unsafe { __wvl_atomic_notify_vfs(ptr, count) }
    }

    #[unsafe(no_mangle)]
    #[cfg(feature = "detect-deadlock")]
    pub unsafe extern "C" fn __vfs_atomic_notify(
        thread_id: i32,
        wasm_id: u32,
        relative_addr: u32,
        count: u32,
    ) -> i32 {
        let _ = thread_id;
        let key = ((wasm_id as u64) << 32) | (relative_addr as u64);
        DEADLOCK_DETECTOR.record_location_change(
            thread_id,
            AtomicLocation {
                wasm_id,
                relative_addr,
                width: 4,
            },
        );
        DEADLOCK_DETECTOR.record_location_change(
            thread_id,
            AtomicLocation {
                wasm_id,
                relative_addr,
                width: 8,
            },
        );
        let ptr = {
            let mut entry = WAIT_MAP.entry(key).or_insert_with(|| Box::new(0));
            let val_mut = entry.value_mut().as_mut();
            *val_mut = val_mut.wrapping_add(1);
            val_mut as *const u32
        };
        unsafe { __wvl_atomic_notify_vfs(ptr, count) }
    }

    #[unsafe(no_mangle)]
    #[cfg(feature = "detect-deadlock")]
    pub unsafe extern "C" fn __vfs_atomic_observe_write(
        thread_id: i32,
        wasm_id: u32,
        relative_addr: u32,
        width: u32,
    ) {
        DEADLOCK_DETECTOR.record_location_change(
            thread_id,
            AtomicLocation {
                wasm_id,
                relative_addr,
                width: width as u8,
            },
        );
    }

    /// Resets the atomic-wait state for a single target, freeing its wait cells.
    ///
    /// Called from the generated `_reset` body. For each wait cell belonging to
    /// `wasm_id`, this first notifies (wakes) all threads parked on the cell so
    /// they are removed from the runtime's wait queue, then removes the cell from
    /// `WAIT_MAP`, freeing the VFS memory it occupied.
    ///
    /// This prevents a stale wait queue entry from being woken when the target is
    /// re-run after reset: without the notify, a zombie thread parked on a
    /// reused address would be woken instead of the new thread.
    ///
    /// # Safety
    ///
    /// Must only be called when no thread is still blocked on those cells -- i.e.
    /// under the same preconditions as `_reset()`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __vfs_atomic_reset_target(wasm_id: u32) {
        let cells: Vec<(u64, *const u32)> = WAIT_MAP
            .iter()
            .filter(|e| (*e.key() >> 32) as u32 == wasm_id)
            .map(|e| (*e.key(), &**e.value() as *const u32))
            .collect();

        for (_, ptr) in &cells {
            unsafe { __wvl_atomic_notify_vfs(*ptr, u32::MAX) };
        }

        for (key, _) in cells {
            WAIT_MAP.remove(&key);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn reset_target_removes_only_matching_wasm_id_entries() {
            WAIT_MAP.clear();
            let key_a = (1u64 << 32) | 0x100;
            let key_b = (1u64 << 32) | 0x200;
            let key_c = (2u64 << 32) | 0x100;
            WAIT_MAP.insert(key_a, Box::new(0));
            WAIT_MAP.insert(key_b, Box::new(0));
            WAIT_MAP.insert(key_c, Box::new(0));
            assert_eq!(WAIT_MAP.len(), 3);

            unsafe { __vfs_atomic_reset_target(1) };

            assert!(!WAIT_MAP.contains_key(&key_a));
            assert!(!WAIT_MAP.contains_key(&key_b));
            assert!(WAIT_MAP.contains_key(&key_c));
            assert_eq!(WAIT_MAP.len(), 1);

            WAIT_MAP.clear();
        }
    }
}

#[cfg(all(test, feature = "detect-deadlock"))]
mod deadlock_detector_tests {
    use super::deadlock_detector::{AtomicLocation, DeadlockDetector};
    #[cfg(all(target_os = "wasi", feature = "detect-deadlock"))]
    use super::vfs_atomic;

    #[test]
    fn detector_records_wait_edge() {
        let detector = DeadlockDetector::default();
        let location = AtomicLocation {
            wasm_id: 1,
            relative_addr: 4,
            width: 4,
        };

        detector.record_thread_enter(7);
        detector.record_wait_start(7, location, 123);

        assert_eq!(detector.waiting_location(7), Some(location));
        assert_eq!(detector.waiting_expected_low(7), Some(123));
        assert_eq!(detector.waiting_generation_at_wait(7), Some(0));
    }

    #[test]
    fn detector_clears_wait_edge_on_end() {
        let detector = DeadlockDetector::default();
        let location = AtomicLocation {
            wasm_id: 1,
            relative_addr: 4,
            width: 4,
        };

        detector.record_thread_enter(7);
        detector.record_wait_start(7, location, 0);
        detector.record_wait_end(7);

        assert_eq!(detector.waiting_location(7), None);
    }

    #[test]
    fn detector_thread_exit_clears_wait_edge() {
        let detector = DeadlockDetector::default();
        let location = AtomicLocation {
            wasm_id: 1,
            relative_addr: 4,
            width: 4,
        };

        detector.record_thread_enter(7);
        detector.record_wait_start(7, location, 0);
        detector.record_thread_exit(7);

        assert_eq!(detector.waiting_location(7), None);
    }

    #[test]
    fn detector_thread_exit_removes_thread_from_deadlock_check() {
        let detector = DeadlockDetector::default();
        let location = AtomicLocation {
            wasm_id: 1,
            relative_addr: 4,
            width: 4,
        };

        detector.record_wait_start(7, location, 0);
        detector.record_thread_exit(7);

        assert_eq!(detector.check_deadlock(), None);
    }

    #[test]
    #[cfg(all(target_os = "wasi", feature = "detect-deadlock"))]
    fn vfs_exports_thread_lifecycle_hooks() {
        unsafe {
            vfs_atomic::__vfs_deadlock_thread_enter(91);
            vfs_atomic::__vfs_deadlock_thread_exit(91);
        }
    }

    #[test]
    fn location_generation_increments_on_notify_or_write() {
        let detector = DeadlockDetector::default();
        let location = AtomicLocation {
            wasm_id: 1,
            relative_addr: 4,
            width: 4,
        };

        assert_eq!(detector.location_generation(location), 0);
        detector.record_location_change(7, location);

        assert_eq!(detector.location_generation(location), 1);
    }

    #[test]
    fn host_idle_wait_does_not_report_deadlock() {
        let detector = DeadlockDetector::default();
        let location = AtomicLocation {
            wasm_id: 4,
            relative_addr: 1_582_760,
            width: 4,
        };

        detector.record_wait_start(1_000_000, location, u32::MAX as u64);

        assert_eq!(detector.check_deadlock(), None);
    }

    #[test]
    fn host_idle_wait_does_not_suppress_other_wait() {
        let detector = DeadlockDetector::default();
        detector.record_wait_start(
            1_000_000,
            AtomicLocation {
                wasm_id: 4,
                relative_addr: 1_582_760,
                width: 4,
            },
            u32::MAX as u64,
        );
        detector.record_wait_start(
            1_000_001,
            AtomicLocation {
                wasm_id: 1,
                relative_addr: 4,
                width: 4,
            },
            u32::MAX as u64,
        );

        let diagnostic = detector.check_deadlock().expect("deadlock diagnostic");
        assert!(diagnostic.contains("deadlock detected"));
        assert!(diagnostic.contains("thread=1000001"));
    }

    #[test]
    fn lone_non_idle_wait_reports_deadlock() {
        let detector = DeadlockDetector::default();
        let location = AtomicLocation {
            wasm_id: 1,
            relative_addr: 4,
            width: 4,
        };

        detector.record_wait_start(7, location, 0);

        let diagnostic = detector.check_deadlock().expect("deadlock diagnostic");
        assert!(diagnostic.contains("deadlock detected"));
        assert!(diagnostic.contains("thread=7"));
    }

    #[test]
    fn lone_non_idle_max_expected_wait_reports_deadlock() {
        let detector = DeadlockDetector::default();
        let location = AtomicLocation {
            wasm_id: 1,
            relative_addr: 4,
            width: 4,
        };

        detector.record_wait_start(7, location, u32::MAX as u64);

        let diagnostic = detector.check_deadlock().expect("deadlock diagnostic");
        assert!(diagnostic.contains("deadlock detected"));
        assert!(diagnostic.contains("thread=7"));
    }

    #[test]
    fn first_reserved_non_idle_max_expected_wait_reports_deadlock() {
        let detector = DeadlockDetector::default();
        let location = AtomicLocation {
            wasm_id: 1,
            relative_addr: 4,
            width: 4,
        };

        detector.record_wait_start(1_000_000, location, u32::MAX as u64);

        let diagnostic = detector.check_deadlock().expect("deadlock diagnostic");
        assert!(diagnostic.contains("deadlock detected"));
        assert!(diagnostic.contains("thread=1000000"));
    }

    #[test]
    fn closed_wait_component_reports_deadlock() {
        let detector = DeadlockDetector::default();
        let first = AtomicLocation {
            wasm_id: 1,
            relative_addr: 4,
            width: 4,
        };
        let second = AtomicLocation {
            wasm_id: 1,
            relative_addr: 8,
            width: 4,
        };

        detector.record_wait_start(7, first, 0);
        detector.record_wait_start(8, second, 0);

        let diagnostic = detector.check_deadlock().expect("deadlock diagnostic");

        assert!(diagnostic.contains("deadlock detected"));
        assert!(diagnostic.contains("thread=7"));
        assert!(diagnostic.contains("thread=8"));
    }

    #[test]
    fn location_generation_change_prevents_deadlock() {
        let detector = DeadlockDetector::default();
        let location = AtomicLocation {
            wasm_id: 1,
            relative_addr: 4,
            width: 4,
        };

        detector.record_wait_start(7, location, 0);
        detector.record_location_change(0, location);

        assert_eq!(detector.check_deadlock(), None);
    }
}
