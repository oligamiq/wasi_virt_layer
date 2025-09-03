#[cfg(target_os = "wasi")]
use crate::memory::WasmAccess;

pub trait VirtualThread {
    // fn
}

/// ref ~/.rustup/toolchains/stable-x86_64-pc-windows-msvc/lib/rustlib/src/rust/library/std/src/sys/pal/wasi/thread.rs
/// this type is *mut Box<dyn FnOnce()>
/// but we can't use it directly, because ABI was not designed with this in mind
#[repr(transparent)]
pub struct ThreadRunner {
    main: *mut Box<dyn FnOnce()>,
}

impl ThreadRunner {
    fn new(main: *mut Box<dyn FnOnce()>) -> Self {
        ThreadRunner { main }
    }

    pub fn apply<Wasm: WasmAccess>(&self) -> ThreadRunnerResult {
        #[cfg(target_os = "wasi")]
        {
            #[cfg(feature = "multi_memory")]
            {
                ThreadRunnerResult::new(self.main)
            }

            #[cfg(not(feature = "multi_memory"))]
            {
                ThreadRunnerResult::new(Wasm::memory_director_mut(self.main))
            }
        }

        #[cfg(not(target_os = "wasi"))]
        {
            panic!("This function is only available on WASI");
        }
    }
}
#[repr(transparent)]
pub struct ThreadRunnerResult {
    main: *mut Box<dyn FnOnce()>,
}

impl ThreadRunnerResult {
    fn new(main: *mut Box<dyn FnOnce()>) -> Self {
        ThreadRunnerResult { main }
    }
}

pub trait ThreadAccess {
    fn to_correct_memory(&self, ptr: ThreadRunner) -> ThreadRunnerResult;
}

pub struct VirtualThreadPool<ThreadAccessor: ThreadAccess, const N: usize> {
    accessor: [ThreadAccessor; N],
}

pub struct DirectThreadPool<const N: usize> {
    threads: [ThreadRunner; N],
}

#[macro_export]
macro_rules! export_thread {
    ($($wasm:tt),*) => {
        $crate::__private::paste::paste! {
            #[allow(non_camel_case_types)]
            enum ThreadPool {
                $(
                    [<__ $wasm>],
                )*
            }

            impl $crate::thread::ThreadAccess for ThreadPool {
                fn to_correct_memory(&self, ptr: $crate::thread::ThreadRunner) -> $crate::thread::ThreadRunnerResult {
                    #[cfg(target_os = "wasi")]
                    {
                        match self {
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
            }
        }
    };

    (@filter, $ptr:ident, self) => {
        $ptr.apply::<$crate::__private::__self>()
    };

    (@filter, $ptr:ident, $wasm:ident) => {
        $ptr.apply::<$wasm>()
    };
}
