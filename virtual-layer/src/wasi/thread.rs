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
#[derive(Debug)]
pub struct ThreadRunner {
    main: *mut Box<dyn FnOnce()>,
}

unsafe impl Send for ThreadRunner {}

impl ThreadRunner {
    pub const fn inner(self) -> *mut Box<dyn FnOnce()> {
        self.main
    }
}

pub trait ThreadAccess: Send + 'static {
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

    #[cfg(target_os = "wasi")]
    #[unsafe(no_mangle)]
    /// When calling thread_spawn, first branch based on the result of this function.
    extern "C" fn __wasip1_vfs_is_root_spawn() -> bool {
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
                fn call_wasi_thread_start(&self, ptr: $crate::thread::ThreadRunner, thread_id: Option<core::num::NonZero<u32>>) {
                    #[cfg(target_os = "wasi")]
                    {
                        match *self {
                            $(
                                Self::[<__ $wasm>] => {
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
                            Self::[<__ $wasm>] => {
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
                    data_ptr: $crate::__private::inner::thread::ThreadRunner,
                ) -> i32 {
                    use $crate::thread::{VirtualThread, ThreadAccess};
                    const ACCESSOR: ThreadAccessor = ThreadAccessor::[<__ $wasm>];

                    // println!("Spawning a new thread in {}", ACCESSOR.as_name());
                    // println!("  data_ptr: {:?}", data_ptr);

                    #[allow(unused_mut)]
                    let mut pool = $pool;

                    match pool.new_thread(ACCESSOR, data_ptr) {
                        Some(thread_id) => {
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
