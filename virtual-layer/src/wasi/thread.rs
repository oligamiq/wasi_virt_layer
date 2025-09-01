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

    fn apply<Wasm: WasmAccess>(&self) -> ThreadRunnerResult {
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

            impl $crate::wasi::thread::ThreadAccess for ThreadPool {
                fn to_correct_memory(&self, ptr: $crate::wasi::thread::ThreadRunner) -> $crate::wasi::thread::ThreadRunnerResult {
                    #[cfg(target_os = "wasi")]
                    {
                        match self {
                            ThreadPool::__wasm1 => {
                                ThreadRunnerResult::new(ptr.main)
                            },
                            ThreadPool::__wasm2 => {
                                ThreadRunnerResult::new(Wasm::memory_director_mut(ptr.main))
                            },
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
}
