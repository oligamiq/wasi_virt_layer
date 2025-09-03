use core::{
    num::NonZero,
    ptr::NonNull,
    sync::atomic::{AtomicU32, AtomicUsize, Ordering},
};

#[cfg(target_os = "wasi")]
use crate::memory::WasmAccess;

pub trait VirtualThread {
    fn new_thread(
        &mut self,
        accessor: impl ThreadAccess,
        runner: ThreadRunner,
    ) -> Option<NonZero<u32>>;
}

/// ref ~/.rustup/toolchains/stable-x86_64-pc-windows-msvc/lib/rustlib/src/rust/library/std/src/sys/pal/wasi/thread.rs
/// this type is *mut Box<dyn FnOnce()>
/// but we can't use it directly, because ABI was not designed with this in mind
#[repr(transparent)]
pub struct ThreadRunnerBase {
    main: *mut Box<dyn FnOnce()>,
}

impl ThreadRunnerBase {
    const fn new(main: *mut Box<dyn FnOnce()>) -> Self {
        ThreadRunnerBase { main }
    }

    #[cfg(target_os = "wasi")]
    pub fn apply<Wasm: WasmAccess>(&self) -> ThreadRunner {
        #[cfg(target_os = "wasi")]
        {
            #[cfg(feature = "multi_memory")]
            {
                ThreadRunner::new(self.main)
            }

            #[cfg(not(feature = "multi_memory"))]
            {
                ThreadRunner::new(Wasm::memory_director_mut(self.main))
            }
        }

        #[cfg(not(target_os = "wasi"))]
        {
            panic!("This function is only available on WASI");
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

pub trait ThreadAccess: Send + Sync + 'static {
    fn to_correct_memory(&self, ptr: ThreadRunnerBase) -> ThreadRunner;
    fn call_wasi_thread_start(&self, ptr: ThreadRunner, thread_id: Option<NonZero<u32>>);
}

pub struct VirtualThreadPool<ThreadAccessor: ThreadAccess, const N: usize> {
    accessor: [ThreadAccessor; N],
}

pub struct DirectThreadPool;

impl VirtualThread for DirectThreadPool {
    // new thread start function call by other wasm
    fn new_thread(
        &mut self,
        accessor: impl ThreadAccess,
        runner: ThreadRunner,
    ) -> Option<NonZero<u32>> {
        static THREAD_COUNT: AtomicU32 = AtomicU32::new(1);

        let thread_id = THREAD_COUNT.fetch_add(1, Ordering::SeqCst);

        std::thread::spawn(move || {
            accessor.call_wasi_thread_start(runner, NonZero::new(thread_id));
        });

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
                    #[cfg(target_os = "wasi")]
                    {
                        match *self {
                            $(
                                [<__ $wasm>] => {
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
                    #[cfg(target_os = "wasi")]
                    {
                        match *self {
                            $(
                                [<__ $wasm>] => {
                                    #[doc(hidden)]
                                    #[cfg(target_os = "wasi")]
                                    #[link(wasm_import_module = "wasip1-vfs")]
                                    unsafe extern "C" {
                                        #[unsafe(no_mangle)]
                                        pub fn [<__wasip1_vfs_ $wasm _wasi_thread_start>](
                                            thread_id: i32,
                                            ptr: i32,
                                        );
                                    }

                                    #[cfg(target_os = "wasi")]
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
            }

            $(
                #[cfg(target_os = "wasi")]
                #[unsafe(no_mangle)]
                unsafe extern "C" fn [<__wasip1_vfs_wasi_thread_start_ $wasm>](
                    data_ptr: $crate::__private::inner::thread::ThreadRunnerBase,
                ) {
                    use $crate::thread::{VirtualThread, ThreadAccess};

                    #[allow(unused_mut)]
                    let mut pool = $pool;
                    const ACCESSOR: ThreadAccessor = ThreadAccessor::[<__ $wasm>];
                    pool.new_thread(ACCESSOR, ACCESSOR.to_correct_memory(data_ptr));
                }
            )*
        }
    };
    (@filter, $ptr:ident, self) => {
        $ptr.apply::<$crate::__private::__self>()
    };

    (@filter, $ptr:ident, $wasm:ident) => {
        $ptr.apply::<$wasm>()
    };
}
