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

    fn apply<Wasm: WasmAccess>(&self) -> *mut Box<dyn FnOnce()> {
        #[cfg(target_os = "wasi")]
        {
            #[cfg(feature = "multi_memory")]
            {
                self.main
            }

            #[cfg(not(feature = "multi_memory"))]
            {
                Wasm::memory_director_mut(self.main)
            }
        }

        #[cfg(not(target_os = "wasi"))]
        {
            panic!("This function is only available on WASI");
        }
    }
}

pub trait ThreadAccess {
    fn to_correct_memory(&self, ptr: ThreadRunner) -> ThreadRunner;
}

pub struct VirtualThreadPool<ThreadAccessor: ThreadAccess, const N: usize> {
    accessor: [ThreadAccessor; N],
}

pub struct DirectThreadPool<const N: usize> {
    threads: [ThreadRunner; N],
}

#[macro_export]
macro_rules! export_thread {
    ($($wasm:ty),*) => {
        enum ThreadPool {
            $(
                $crate::export_thread!(@filter, $wasm),
            )*
        }
    };

    (@filter, self) => {
        __self,
    };

    (@filter, $other:ident) => {
        $other,
    };
}
