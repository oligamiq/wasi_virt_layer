use core::{
    num::NonZero,
    sync::atomic::{AtomicU32, Ordering},
};

#[cfg(target_os = "wasi")]
use crate::memory::WasmAccess;

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
        }
    }
}

/// ref ~/.rustup/toolchains/stable-x86_64-pc-windows-msvc/lib/rustlib/src/rust/library/std/src/sys/pal/wasi/thread.rs
/// this type is *mut Box<dyn FnOnce()>
/// but we can't use it directly, because ABI was not designed with this in mind
#[repr(transparent)]
pub struct ThreadRunnerBase {
    main: *mut Box<dyn FnOnce()>,
}

impl ThreadRunnerBase {
    #[cfg(target_os = "wasi")]
    pub fn apply<Wasm: WasmAccess>(&self) -> ThreadRunner {
        #[cfg(feature = "multi_memory")]
        {
            ThreadRunner::new(self.main)
        }

        #[cfg(not(feature = "multi_memory"))]
        {
            ThreadRunner::new(Wasm::memory_director_mut(self.main))
        }
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThreadRunner {
    main: *mut Box<dyn FnOnce()>,
}

unsafe impl Send for ThreadRunner {}

impl ThreadRunner {
    fn new(main: *mut Box<dyn FnOnce()>) -> Self {
        ThreadRunner { main }
    }

    pub const fn inner(self) -> *mut Box<dyn FnOnce()> {
        self.main
    }
}

pub trait ThreadAccess: Send + 'static {
    fn to_correct_memory(&self, ptr: ThreadRunnerBase) -> ThreadRunner;
    fn call_wasi_thread_start(&self, ptr: ThreadRunner, thread_id: Option<NonZero<u32>>);
    fn as_name(&self) -> &'static str;
}

pub struct VirtualThreadPool<ThreadAccessor: ThreadAccess, const N: usize> {
    accessor: [ThreadAccessor; N],
}

pub struct DirectThreadPool;

mod spawn {
    use core::cell::UnsafeCell;

    // It is safe as it releases immediately.
    thread_local! {
        static IS_ROOT_THREAD: UnsafeCell<bool> = UnsafeCell::new(false);
    }

    #[cfg(feature = "debug")]
    #[cfg(target_os = "wasi")]
    #[unsafe(no_mangle)]
    extern "C" fn root_spawn_debug() {
        root_spawn(|| {
            unreachable!();
        });
    }

    #[cfg(feature = "debug")]
    #[cfg(target_os = "wasi")]
    #[unsafe(no_mangle)]
    extern "C" fn root_spawn_debug2() {
        let b = std::thread::Builder::new();
        unsafe {
            b.spawn_unchecked::<_, ()>(|| {
                todo!();
            })
            .unwrap();
        };
    }

    /// Spawn a new thread.
    /// If you call `std::thread::spawn` in ThreadPool, it will be looped.
    /// So, you should use `root_spawn` instead.
    pub fn root_spawn<F, T>(f: F) -> std::thread::JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
        // println!("Backtrace at root_spawn:\n{:?}", *crate::debug::BACKTRACE);

        println!("Spawning a root thread");
        IS_ROOT_THREAD.with(|flag| {
            unsafe { flag.get().write(true) };
        });
        println!("Root thread flag set to true");

        // let s = std::thread::spawn(f);
        let b = std::thread::Builder::new();

        let b = b.stack_size(1024 * 1024 * 2); // 2MB stack
        let b = b.name("wasip1-root-thread".to_string());

        let fmt = format!("Root thread builder: {b:?}");
        crate::debug::out(fmt.as_bytes());
        crate::debug::out(b"\n");

        // // let s = {
        // let thread_id = {
        //     use core::sync::atomic::{AtomicU64, Ordering};
        //     static COUNTER: AtomicU64 = AtomicU64::new(0);

        //     #[cold]
        //     fn exhausted() -> ! {
        //         panic!("failed to generate unique thread ID: bitspace exhausted")
        //     }

        //     let mut last = COUNTER.load(Ordering::Relaxed);
        //     loop {
        //         let Some(id) = last.checked_add(1) else {
        //             exhausted();
        //         };

        //         match COUNTER.compare_exchange_weak(last, id, Ordering::Relaxed, Ordering::Relaxed)
        //         {
        //             Ok(_) => break core::num::NonZero::new(id).unwrap(),
        //             Err(id) => last = id,
        //         }
        //     }
        // };

        // println!("Generated unique thread ID: {thread_id}");

        // let thread = {
        //     use alloc::sync::Arc;

        //     #[derive(Debug)]
        //     struct Inner {
        //         name: Option<CString>,
        //         thread_id: core::num::NonZero<u64>,
        //     }

        //     let name = Some("wasip1-root-thread").map(|s| CString::new(s).unwrap());

        //     // We have to use `unsafe` here to construct the `Parker` in-place,
        //     // which is required for the UNIX implementation.
        //     //
        //     // SAFETY: We pin the Arc immediately after creation, so its address never
        //     // changes.
        //     let inner = unsafe {
        //         let inner = Inner { name, thread_id };
        //         core::pin::Pin::new_unchecked(Arc::new(inner))
        //     };

        //     inner
        // };
        // println!("Inner thread data created: {thread:?}");

        // let my_thread = std::thread::Thread::new(id, name);

        // let hooks = spawnhook::run_spawn_hooks(&my_thread);
        // };

        let s = unsafe { b.spawn_unchecked(f) };

        println!("Root thread spawned");

        let s = s.expect("failed to spawn thread");

        println!("Root thread handle created");

        s
    }

    #[cfg(target_os = "wasi")]
    #[unsafe(no_mangle)]
    /// When calling thread_spawn, first branch based on the result of this function.
    extern "C" fn __wasip1_vfs_is_root_spawn() -> bool {
        eprintln!("Checking if current thread is root thread");
        // get and turn off the flag
        IS_ROOT_THREAD.with(|flag| unsafe { flag.get().replace(false) })
    }
}
pub use spawn::root_spawn;

impl VirtualThread for DirectThreadPool {
    // new thread start function call by other wasm
    fn new_thread(
        &mut self,
        accessor: impl ThreadAccess,
        runner: ThreadRunner,
    ) -> Option<NonZero<u32>> {
        println!("Spawning a new thread for {}", accessor.as_name());

        static THREAD_COUNT: AtomicU32 = AtomicU32::new(1);

        let thread_id = THREAD_COUNT.fetch_add(1, Ordering::SeqCst);

        println!("Assigned thread ID: {}", thread_id);

        root_spawn(move || {
            println!("In new thread with ID: {}", thread_id);
            accessor.call_wasi_thread_start(runner, NonZero::new(thread_id));
        });

        println!("Root thread spawned");
        NonZero::new(thread_id as u32)
    }
}

#[macro_export]
macro_rules! export_thread {
    ($pool:tt, $($wasm:tt),*) => {
        $crate::__private::paste::paste! {
            #[allow(non_camel_case_types)]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub(crate) enum ThreadAccessor {
                $(
                    [<__ $wasm>],
                )*
            }

            impl $crate::thread::ThreadAccess for ThreadAccessor {
                fn to_correct_memory(&self, ptr: $crate::__private::inner::thread::ThreadRunnerBase) -> $crate::thread::ThreadRunner {
                    println!("Converting to correct memory for {}", self.as_name());

                    #[cfg(target_os = "wasi")]
                    {
                        match *self {
                            $(
                                Self::[<__ $wasm>] => {
                                    $crate::export_thread!(@filter, ptr, $wasm)
                                }
                            )*
                        }
                    }

                    #[cfg(not(target_os = "wasi"))]
                    {
                        panic!("This function is only available on WASI");
                    }
                }

                fn call_wasi_thread_start(&self, ptr: $crate::thread::ThreadRunner, thread_id: Option<core::num::NonZero<u32>>) {
                    println!("Calling wasi_thread_start for {}", self.as_name());

                    #[cfg(target_os = "wasi")]
                    {
                        match *self {
                            $(
                                Self::[<__ $wasm>] => {
                                    todo!();

                                    unsafe { [<__wasip1_vfs_ $wasm _wasi_thread_start>](
                                        match thread_id {
                                            Some(id) => u32::from(id) as i32,
                                            None => -1,
                                        },
                                        ptr.inner() as i32,
                                    ) }

                                    todo!();
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
                            Self::[<__ $wasm>] => $crate::export_thread!(@as_name, $wasm),
                        )*
                    }
                }
            }

            $(
                #[cfg(target_os = "wasi")]
                #[doc(hidden)]
                #[link(wasm_import_module = "wasip1-vfs")]
                unsafe extern "C" {
                    #[unsafe(no_mangle)]
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
                    [<__wasip1_vfs_ $wasm _wasi_thread_start>](thread_id, ptr);
                }

                #[cfg(target_os = "wasi")]
                #[unsafe(no_mangle)]
                unsafe extern "C" fn [<__wasip1_vfs_wasi_thread_spawn_ $wasm>](
                    data_ptr: $crate::__private::inner::thread::ThreadRunnerBase,
                ) -> i32 {
                    use $crate::thread::{VirtualThread, ThreadAccess};
                    const ACCESSOR: ThreadAccessor = ThreadAccessor::[<__ $wasm>];

                    println!("Spawning a new thread for {}", $crate::export_thread!(@as_name, $wasm));

                    #[allow(unused_mut)]
                    let mut pool = $pool;

                    println!("Thread pool obtained");

                    let correct_memory = ACCESSOR.to_correct_memory(data_ptr);

                    println!("Correct memory obtained for {}: {:?}", ACCESSOR.as_name(), correct_memory);

                    match pool.new_thread(ACCESSOR, correct_memory) {
                        Some(thread_id) => {
                            println!("New thread created with ID {thread_id}");
                            return u32::from(thread_id) as i32;
                        },
                        None => {
                            panic!("Failed to create a new thread");
                        }
                    }
                }

                $crate::export_thread!(@sched_yield, $pool, $wasm);
            )*
        }
    };

    (@filter, $ptr:ident, self) => {
        $ptr.apply::<$crate::__private::__self>()
    };

    (@filter, $ptr:ident, $wasm:ident) => {
        $ptr.apply::<$wasm>()
    };

    (@as_name, self) => {
        <$crate::__private::__self as $crate::memory::WasmAccess>::NAME
    };

    (@as_name, $wasm:ident) => {
        <$wasm as $crate::memory::WasmAccess>::NAME
    };

    (@sched_yield, $pool:tt, self) => {
        $crate::__private::paste::paste! {
            #[unsafe(no_mangle)]
            #[cfg(target_os = "wasi")]
            pub unsafe extern "C" fn __wasip1_vfs_self_sched_yield(
            ) -> $crate::__private::wasip1::Errno {
                use $crate::thread::VirtualThread;

                #[allow(unused_mut)]
                let mut pool = $pool;
                pool.sched_yield::<$crate::__private::__self>()
            }
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
                pool.sched_yield::<$wasm>()
            }
        }
    };
}
