use core::{
    num::NonZero,
    ptr::NonNull,
    sync::atomic::{AtomicU32, AtomicUsize, Ordering},
};
use std::{sync::Arc, thread::JoinHandle};

#[allow(unused_imports)]
use crate::{__private::wasip1, memory::WasmAccess};

/// Trait for a virtual thread implementation.
pub trait VirtualThread<ThreadAccessor: ThreadAccess> {
    /// Creates a new thread and returns its ID.
    fn new_thread(
        &mut self,
        accessor: ThreadAccessor,
        runner: ThreadRunner,
    ) -> Option<NonZero<u32>>;

    /// Yields the execution of the current thread.
    #[inline(always)]
    fn sched_yield<Wasm: WasmAccess>(&mut self) -> wasip1::Errno {
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
    pool: Option<Arc<parking_lot::Mutex<Vec<JoinHandle<()>>>>>,
}

impl JoinPoolHandle {
    pub fn lock(&self) -> parking_lot::MutexGuard<'_, Vec<JoinHandle<()>>> {
        self.pool.as_ref().unwrap().lock()
    }

    pub fn extend<I: IntoIterator<Item = JoinHandle<()>>>(&self, iter: I) {
        let mut guard = self.pool.as_ref().unwrap().lock();
        guard.extend(iter);
    }

    pub const fn const_new() -> Self {
        JoinPoolHandle { pool: None }
    }

    pub fn init(&mut self) -> bool {
        if self.pool.is_none() {
            self.pool = Some(Arc::new(parking_lot::Mutex::new(Vec::new())));
            true
        } else {
            false
        }
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
                println!("Waiting for thread pool flush to complete...");
                println!("Thread ID: {:?}", std::thread::current().id());
                let s = recv.recv();
                println!("Thread pool flush completed: {:?}", s);
            }
            WaitThreadJoin::RecvN(recv, n) => {
                println!(
                    "Waiting for thread pool flush to complete for {} threads...",
                    n
                );
                for _ in 0..n {
                    let s = recv.recv();
                    println!("Thread pool flush completed for one thread: {:?}", s);
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
    ),
    AddThread(usize, std::sync::mpsc::SyncSender<()>, JoinPoolHandle),
    Terminate(flume::Sender<()>, JoinPoolHandle),
}

impl<ThreadAccessor: ThreadAccess> VirtualThreadPoolMessage<ThreadAccessor> {
    pub fn use_(self, queue: &flume::Receiver<VirtualThreadPoolMessage<ThreadAccessor>>) -> bool {
        match self {
            VirtualThreadPoolMessage::Run(runner, accessor_wrapper, thread_id) => {
                let accessor = accessor_wrapper.as_accessor();
                accessor.call_wasi_thread_start(runner, Some(thread_id));
            }
            VirtualThreadPoolMessage::AddThread(count, ref sender, ref kept_workers_pool) => {
                // Passing an iterator to kept_workers_pool causes the lock to hold for too long.
                let threads = self.create_thread(count, &queue).collect::<Vec<_>>();
                kept_workers_pool.extend(threads);
                let s = sender.try_send(());
                println!("Sent add thread completion signal: {:?}", s);
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
        while queue.recv().unwrap().use_(queue) {}
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
        println!("Creating {count} threads in the thread pool...");
        core::iter::repeat_n(queue.clone(), count).map(move |queue| {
            let thread = root_spawn(std::thread::Builder::new(), move || {
                Self::listen(&queue);
            })
            .unwrap();
            thread
        })
    }
}

/// A thread pool implementation for virtual threads.
pub struct VirtualThreadPool<ThreadAccessor: ThreadAccess> {
    max_threads: AtomicUsize,
    read_kept_workers_pool_size: AtomicUsize,
    queue: parking_lot::Mutex<Option<flume::Sender<VirtualThreadPoolMessage<ThreadAccessor>>>>,
    queue_receiver: Option<flume::Receiver<VirtualThreadPoolMessage<ThreadAccessor>>>,
    kept_workers_pool: JoinPoolHandle,
}

impl<ThreadAccessor: ThreadAccess> VirtualThreadPool<ThreadAccessor> {
    /// Creates a new `VirtualThreadPool` without initialization.
    pub const unsafe fn const_new(max_threads: usize) -> Self {
        VirtualThreadPool {
            max_threads: AtomicUsize::new(max_threads),
            kept_workers_pool: JoinPoolHandle::const_new(),
            queue: parking_lot::Mutex::new(None),
            queue_receiver: None,
            read_kept_workers_pool_size: AtomicUsize::new(0),
        }
    }

    /// Initializes the thread pool. This must be called before use.
    pub fn init(&mut self) {
        if self.kept_workers_pool.init() {
            let (sender, receiver) = flume::unbounded();
            *self.queue.lock() = Some(sender);
            self.queue_receiver = Some(receiver);
        }
    }

    /// Sets the maximum capacity of the thread pool.
    pub fn set_capacity(&self, max_threads: usize) {
        self.max_threads.store(max_threads, Ordering::SeqCst);
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
            println!("[] Increasing thread pool size from {current_len} to {max_threads}");

            match {
                self.add_queue_with(|sender| {
                    println!(
                        "[] Requesting addition of {} threads to the thread pool...",
                        max_threads - current_len
                    );

                    let (send, recv) = std::sync::mpsc::sync_channel(1);
                    println!("sender.receiver_count(): {}", sender.receiver_count());
                    println!("sender.len(): {}", sender.len());
                    if !sender.is_empty() || sender.receiver_count() <= 1 {
                        println!("[] Another thread is handling the addition. Skipping...");
                        return None;
                    }
                    println!("[] Sending add thread request...");
                    Some((
                        VirtualThreadPoolMessage::AddThread(
                            max_threads - current_len,
                            send,
                            self.kept_workers_pool.clone(),
                        ),
                        recv,
                    ))
                })
            } {
                Some(recv) => {
                    return WaitThreadJoin::Recv(recv);
                }
                None => {
                    let count = max_threads - current_len;
                    let (send, recv) = std::sync::mpsc::sync_channel(count);
                    println!("[] count {count}");
                    let msg = VirtualThreadPoolMessage::<ThreadAccessor>::AddThread(
                        count - 1,
                        send,
                        self.kept_workers_pool.clone(),
                    );

                    let queue_receiver = self.queue_receiver.clone();

                    println!("[] queue_receiver is_some(): {}", queue_receiver.is_some());

                    let handle = root_spawn(std::thread::Builder::new(), move || {
                        println!("[] Thread pool addition thread started.");

                        VirtualThreadPoolMessage::listen_with(
                            queue_receiver.as_ref().unwrap(),
                            msg,
                        );
                    })
                    .unwrap();

                    pool.push(handle);
                    return WaitThreadJoin::Recv(recv);
                }
            }
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
    pub fn run(&self, accessor: ThreadAccessor, runner: ThreadRunner, thread_id: NonZero<u32>) {
        self.queue
            .lock()
            .as_mut()
            .unwrap()
            .send(VirtualThreadPoolMessage::Run(
                runner,
                ThreadAccessorWrapper::new(accessor),
                thread_id,
            ))
            .unwrap();
    }
}

impl<ThreadAccessor: ThreadAccess> VirtualThread<ThreadAccessor>
    for VirtualThreadPool<ThreadAccessor>
{
    fn new_thread(
        &mut self,
        accessor: ThreadAccessor,
        runner: ThreadRunner,
    ) -> Option<NonZero<u32>> {
        static THREAD_COUNT: AtomicU32 = AtomicU32::new(1);

        let thread_id = THREAD_COUNT.fetch_add(1, Ordering::SeqCst);

        let thread_id_nz = NonZero::new(thread_id as u32)?;

        self.run(accessor, runner, thread_id_nz);

        Some(thread_id_nz)
    }
}

unsafe impl<ThreadAccessor: ThreadAccess> Send for VirtualThreadPool<ThreadAccessor> {}
unsafe impl<ThreadAccessor: ThreadAccess> Sync for VirtualThreadPool<ThreadAccessor> {}
/// A "pool" that spawns native threads directly for each request.
pub struct DirectThreadPool<ThreadAccessor: ThreadAccess>(
    core::marker::PhantomData<ThreadAccessor>,
);

impl<ThreadAccessor: ThreadAccess> DirectThreadPool<ThreadAccessor> {
    /// Creates a new `DirectThreadPool`.
    pub const fn const_new() -> Self {
        DirectThreadPool(core::marker::PhantomData)
    }
}

mod spawn {
    use core::cell::UnsafeCell;

    // It is safe as it releases immediately.
    thread_local! {
        static IS_ROOT_THREAD: UnsafeCell<bool> = UnsafeCell::new(false);
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
        IS_ROOT_THREAD.with(|flag| {
            unsafe { flag.get().write(true) };
        });

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
        IS_ROOT_THREAD.with(|flag| {
            unsafe { flag.get().write(true) };
        });

        unsafe { builder.spawn_unchecked(f) }
    }

    #[cfg(target_os = "wasi")]
    #[unsafe(no_mangle)]
    /// When calling thread_spawn, first branch based on the result of this function.
    extern "C" fn __wasip1_vfs_is_root_spawn() -> bool {
        // get and turn off the flag
        IS_ROOT_THREAD.with(|flag| unsafe { flag.get().replace(false) })
    }
}

pub use spawn::{root_spawn, root_spawn_unchecked};

impl<ThreadAccessor: ThreadAccess> VirtualThread<ThreadAccessor>
    for DirectThreadPool<ThreadAccessor>
{
    // new thread start function call by other wasm
    fn new_thread(
        &mut self,
        accessor: ThreadAccessor,
        runner: ThreadRunner,
    ) -> Option<NonZero<u32>> {
        static THREAD_COUNT: AtomicU32 = AtomicU32::new(1);

        let thread_id = THREAD_COUNT.fetch_add(1, Ordering::SeqCst);

        let builder = std::thread::Builder::new();

        root_spawn(builder, move || {
            accessor.call_wasi_thread_start(runner, NonZero::new(thread_id));
        })
        .ok()?;

        NonZero::new(thread_id as u32)
    }
}

/// Plugs the thread ecosystem by defining necessary accessor enums and hooks.
#[macro_export]
macro_rules! plug_thread {
    ($pool:tt, $($wasm:ident),* $(,)?) => {
        $crate::__as_t!(@through, $($wasm),* => $crate::plug_thread, @inner, $pool);
    };

    (@inner, $pool:tt, $($wasm:ident),* $(,)?) => {
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
                        println!("$$$ Calling wasi_thread_start in {}", self.as_name());
                        match *self {
                            $(
                                Self::$wasm => {
                                    // println!("Calling wasi_thread_start in {}", self.as_name());
                                    // println!("  thread_id: {:?}", thread_id);
                                    // println!("  data_ptr: {:?}", ptr);
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
                    const ACCESSOR: ThreadAccessor = ThreadAccessor::$wasm;
                    println!("$$$ Spawning a new thread in {}", ACCESSOR.as_name());
                    // println!("  data_ptr: {:?}", data_ptr);

                    #[allow(unused_mut)]
                    let mut pool = $pool;

                    match pool.new_thread(ACCESSOR, $crate::thread::ThreadRunner::__new(data_ptr)) {
                        Some(thread_id) => {
                            return u32::from(thread_id) as i32;
                        },
                        None => {
                            panic!("Failed to create a new thread");
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

    static INIT: InitOnce = InitOnce::new();

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
