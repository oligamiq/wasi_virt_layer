use core::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    num::NonZero,
    ptr::NonNull,
    slice::SliceIndex,
    sync::atomic::{AtomicBool, AtomicU8, AtomicU32, Ordering},
};
use std::{sync::Arc, thread::JoinHandle};

#[allow(unused_imports)]
use crate::{memory::WasmAccess, wasip1};

pub trait VirtualThread {
    fn new_thread(
        &mut self,
        accessor: impl ThreadAccess,
        runner: ThreadRunner,
    ) -> Option<NonZero<u32>>;

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
#[derive(Debug)]
pub struct ThreadRunner {
    main: NonNull<Box<dyn FnOnce()>>,
}

unsafe impl Send for ThreadRunner {}

impl ThreadRunner {
    pub fn __new(ptr: *mut Box<dyn FnOnce()>) -> Self {
        ThreadRunner {
            main: NonNull::new(ptr).unwrap(),
        }
    }

    pub const fn inner(self) -> *mut Box<dyn FnOnce()> {
        self.main.as_ptr()
    }
}

/// Thread Util on each wasm
pub trait ThreadAccess: Send + 'static {
    /// If creation is failed, thread_id return None
    /// Run given function(ThreadRunner) and wait
    fn call_wasi_thread_start(&self, ptr: ThreadRunner, thread_id: Option<NonZero<u32>>);
    /// Get wasm name on which create thread
    fn as_name(&self) -> &'static str;
}

use parking_lot::{Mutex, RwLock};

pub struct ThreadWorkerCondition(core::sync::atomic::AtomicU8);

impl ThreadWorkerCondition {
    /// This worker is not doing anything.
    /// It is waiting for a task.
    const BLANK: u8 = 1;
    /// A task is being executed on this worker.
    const RUNNING: u8 = 2;
    /// A task is being sent to this worker.
    const SENDING_TASK: u8 = 3;
    /// A task has been sent to this worker.
    const SENDED_TASK: u8 = 4;
    /// This worker is receiving a task.
    const RECEIVING_TASK: u8 = 5;

    pub const fn blank() -> Self {
        ThreadWorkerCondition(core::sync::atomic::AtomicU8::new(Self::BLANK))
    }
}

struct UnsafeSharedPlace<T> {
    data: UnsafeCell<Option<T>>,
}

unsafe impl<T> Send for UnsafeSharedPlace<T> where T: Send {}
unsafe impl<T> Sync for UnsafeSharedPlace<T> where T: Send {}

impl<T> UnsafeSharedPlace<T> {
    pub const fn new() -> Self {
        UnsafeSharedPlace {
            data: UnsafeCell::new(None),
        }
    }

    pub unsafe fn replace(&self, value: T) {
        unsafe { &mut *self.data.get() }.replace(value);
    }
}

pub enum WaitThreadJoin {
    None,
    WaitWithFn(Box<dyn FnOnce() -> () + Send>),
}

impl WaitThreadJoin {
    pub fn join(self) {
        match self {
            WaitThreadJoin::None => {}
            WaitThreadJoin::WaitWithFn(f) => {
                f();
            }
        }
    }
}

pub struct ThreadWorker {
    condition: ThreadWorkerCondition,
    runner: UnsafeSharedPlace<ThreadRunner>,
    queue: flume::Receiver<ThreadRunner>,
    thread_handle: JoinHandle<()>,
}

impl ThreadWorker {
    /// When b is no longer present, access from within the thread must be stopped.
    pub(crate) unsafe fn spawn_on_new_thread<
        'b,
        I: Iterator<Item = core::pin::Pin<&'b mut MaybeUninit<Self>>> + Send + 'b,
    >(
        mut sl: I,
        count: usize,
    ) -> WaitThreadJoin {
        debug_assert!(count >= 1);

        let is_waiting = Arc::new(AtomicU8::new(0));
        let is_waiting_clone = is_waiting.clone();
        let thread_id = std::thread::current();

        let first_area = UnsafeCell::new(sl.next().unwrap());
        let first_area_ptr = first_area.get();

        let handle = root_spawn_unchecked(std::thread::Builder::new(), move || {
            for mut sl in sl.take(count - 1) {
                unsafe {
                    Self::each_nested_spawn(&mut sl, |condition, runner| {
                        let thread_handle = root_spawn(std::thread::Builder::new(), || {
                            Self::listener_loop(condition, runner);
                        })
                        .unwrap();
                        return Some(thread_handle);
                    })
                };
            }

            if is_waiting_clone.load(Ordering::SeqCst) == 1 {
                thread_id.unpark();
                is_waiting_clone.store(2, Ordering::SeqCst);
            }

            unsafe {
                Self::each_nested_spawn(&mut *first_area.get(), |condition, runner| {
                    Self::listener_loop(condition, runner);
                    return None;
                })
            };
        })
        .unwrap();

        unsafe { Self::set_thread_handle(&mut *first_area_ptr, handle) };

        let wait = move || {
            if is_waiting.load(Ordering::SeqCst) == 2 {
                return;
            }
            is_waiting.store(1, Ordering::SeqCst);
            std::thread::park();
        };

        WaitThreadJoin::WaitWithFn(Box::new(wait))
    }

    /// When b is no longer present, access from within the thread must be stopped in f.
    unsafe fn each_nested_spawn<'a, 'b>(
        sl: &'a mut core::pin::Pin<&'b mut MaybeUninit<Self>>,
        f: impl FnOnce(
            &'static ThreadWorkerCondition,
            &'static UnsafeSharedPlace<ThreadRunner>,
        ) -> Option<JoinHandle<()>>,
    ) {
        let sl_ptr = sl.as_mut_ptr();
        let sl: &'a mut ThreadWorker = unsafe { &mut *sl_ptr };

        unsafe { core::ptr::write(&mut sl.condition, ThreadWorkerCondition::blank()) };
        unsafe { core::ptr::write(&mut sl.runner, UnsafeSharedPlace::new()) };
        let condition: &'static _ =
            unsafe { core::mem::transmute::<&'a _, &'static _>(&sl.condition) };
        let runner: &'static _ = unsafe { core::mem::transmute::<&'a _, &'static _>(&sl.runner) };

        if let Some(thread_handle) = f(condition, runner) {
            unsafe { core::ptr::write(&mut sl.thread_handle, thread_handle) };
        }
    }

    /// If you don't give a handle on each_nested_spawn, you can set it later.
    unsafe fn set_thread_handle<'a, 'b>(
        sl: &'a mut core::pin::Pin<&'b mut MaybeUninit<Self>>,
        handle: JoinHandle<()>,
    ) {
        let sl_ptr = sl.as_mut_ptr();
        let sl: &'a mut ThreadWorker = unsafe { &mut *sl_ptr };

        unsafe { core::ptr::write(&mut sl.thread_handle, handle) };
    }

    fn send(&self, data: ThreadRunner) -> bool {
        let condition = &self.condition;
        let shared_place = &self.runner;

        let current = condition.0.compare_exchange(
            ThreadWorkerCondition::BLANK,
            ThreadWorkerCondition::SENDING_TASK,
            Ordering::SeqCst,
            Ordering::SeqCst,
        );
        if current.is_ok() {
            unsafe { shared_place.replace(data) };

            condition
                .0
                .store(ThreadWorkerCondition::SENDED_TASK, Ordering::SeqCst);

            self.thread_handle.thread().unpark();

            return true;
        } else {
            let current = condition.0.compare_exchange(
                ThreadWorkerCondition::RUNNING,
                ThreadWorkerCondition::SENDING_TASK,
                Ordering::SeqCst,
                Ordering::SeqCst,
            );
            match current {
                Ok(_) | Err(ThreadWorkerCondition::BLANK) => {
                    unsafe { shared_place.replace(data) };

                    condition
                        .0
                        .store(ThreadWorkerCondition::SENDED_TASK, Ordering::SeqCst);

                    if current.is_err() {
                        self.thread_handle.thread().unpark();
                    }

                    return true;
                }
                Err(_) => {
                    return false;
                }
            }
        }
    }

    fn recv(
        condition: &ThreadWorkerCondition,
        runner: &mut UnsafeCell<Option<ThreadRunner>>,
    ) -> ThreadRunner {
        let old = condition.0.compare_exchange(
            ThreadWorkerCondition::SENDED_TASK,
            ThreadWorkerCondition::RECEIVING_TASK,
            Ordering::SeqCst,
            Ordering::SeqCst,
        );

        match old {
            Ok(_) => {
                std::thread::park();

                let old = condition.0.compare_exchange(
                    ThreadWorkerCondition::SENDED_TASK,
                    ThreadWorkerCondition::RUNNING,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                );
                if old.is_err() {
                    panic!("ThreadWorker condition corrupted");
                }

                let data = unsafe { &mut *runner.get() }.take().unwrap();
                data
            }
            Err(v) if v == ThreadWorkerCondition::SENDED_TASK => {
                let old = condition.0.compare_exchange(
                    ThreadWorkerCondition::SENDED_TASK,
                    ThreadWorkerCondition::RUNNING,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                );
                if old.is_err() {
                    panic!("ThreadWorker condition corrupted");
                }

                let data = unsafe { &mut *runner.get() }.take().unwrap();
                data
            }
        }
    }

    fn listener_loop(condition: &ThreadWorkerCondition, runner: &UnsafeSharedPlace<ThreadRunner>) {
        loop {
            // wait for condition
            // if received, run the runner
            // after finish, set condition to blank
        }
    }
}

pub struct VirtualThreadPool {
    max_threads: AtomicU32,
    kept_workers_pool: RwLock<Vec<ThreadWorker>>,
}

impl VirtualThreadPool {
    pub const fn new(max_threads: u32) -> Self {
        VirtualThreadPool {
            max_threads: AtomicU32::new(max_threads),
            kept_workers_pool: RwLock::new(Vec::new()),
        }
    }

    pub fn search_blank_thread_worker(&self) -> Option<usize> {
        let pool = self.kept_workers_pool.read();

        for (index, worker) in pool.iter().enumerate() {
            let condition_value = worker.condition.0.load(Ordering::SeqCst);
            if condition_value == ThreadWorkerCondition::BLANK {
                return Some(index);
            }
        }

        None
    }

    pub fn set_capacity<'a>(&'a self) -> WaitThreadJoin {
        let max_threads = self.max_threads.load(Ordering::SeqCst);

        let current_len = self.kept_workers_pool.read().len() as u32;

        if current_len == max_threads {
            // no change
            return WaitThreadJoin::None;
        }

        let mut pool = self.kept_workers_pool.write();

        if current_len < max_threads {
            let mut new_workers = Vec::with_capacity(max_threads as usize);

            // This pool contents must no longer be used.
            for worker_count in 0..pool.len() {
                let worker = unsafe { core::ptr::read(&pool[worker_count]) };
                new_workers.push(worker);
            }

            let _ = core::mem::replace::<Vec<ThreadWorker>>(&mut pool, new_workers);

            unsafe { pool.set_len(current_len as usize) };

            unsafe fn pin_vec<'holder, 'a, T: Unpin + 'holder, S: 'holder>(
                vec: &'a mut Vec<T>,
                range: core::ops::Range<S>,
            ) -> impl Iterator<Item = core::pin::Pin<&'holder mut MaybeUninit<T>>> + 'holder
            where
                std::ops::Range<S>: core::slice::SliceIndex<[MaybeUninit<T>]>,
                &'holder mut <std::ops::Range<S> as core::slice::SliceIndex<[MaybeUninit<T>]>>::Output:
                    IntoIterator<Item = &'holder mut MaybeUninit<T>>,
                <std::ops::Range<S> as core::slice::SliceIndex<[MaybeUninit<T>]>>::Output: 'holder,
                'holder: 'a,
            {
                let vec = unsafe {
                    core::mem::transmute::<&'a mut Vec<T>, &'holder mut Vec<MaybeUninit<T>>>(vec)
                };
                (&mut vec[range])
                    .into_iter()
                    .map(move |item| core::pin::Pin::new(item))
            }

            let pin =
                unsafe { pin_vec::<'a, '_>(&mut pool, current_len as usize..max_threads as usize) };

            if let Some(index) = self.search_blank_thread_worker() {
                panic!("ThreadWorker at index {} is still running", index);
            }

            // use new_workers
            unsafe { ThreadWorker::spawn_on_new_thread(pin, (max_threads - current_len) as usize) }
        } else {
            pool.truncate(max_threads as usize);

            return WaitThreadJoin::None;
        }
    }
}

unsafe impl Send for VirtualThreadPool {}
unsafe impl Sync for VirtualThreadPool {}
pub struct DirectThreadPool;

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

impl VirtualThread for DirectThreadPool {
    // new thread start function call by other wasm
    fn new_thread(
        &mut self,
        accessor: impl ThreadAccess,
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

#[macro_export]
macro_rules! plug_thread {
    ($pool:tt, $($wasm:ident),*) => {
        $crate::__as_t!(@through, $($wasm),* => $crate::plug_thread, @inner, $pool);
    };

    (@inner, $pool:tt, $($wasm:ident),*) => {
        $crate::__private::paste::paste! {
            #[allow(non_camel_case_types)]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub(crate) enum ThreadAccessor {
                $(
                    $wasm,
                )*
            }

            impl $crate::thread::ThreadAccess for ThreadAccessor {
                fn call_wasi_thread_start(&self, ptr: $crate::thread::ThreadRunner, thread_id: Option<core::num::NonZero<u32>>) {
                    #[cfg(target_os = "wasi")]
                    {
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
                                <T as $crate::memory::WasmAccess>::NAME
                            }
                        )*
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

                    // println!("Spawning a new thread in {}", ACCESSOR.as_name());
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
