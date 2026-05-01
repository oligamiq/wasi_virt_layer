use crate::__private::wasip1::*;
use crate::memory::{WasmAccess, WasmAccessName};

/// Trait for handling the `sched_yield` WASI function.
pub trait SchedYield {
    /// Yields the execution of the current thread.
    fn sched_yield<Wasm: WasmAccess + WasmAccessName + 'static>() -> Errno;
}

/// Default implementation of `SchedYield`.
pub struct DefaultSched;

impl SchedYield for DefaultSched {
    fn sched_yield<Wasm: WasmAccess + WasmAccessName + 'static>() -> Errno {
        unsafe {
            crate::transporter::non_recursive_sched_yield();
        }
        ERRNO_SUCCESS
    }
}

/// Plugs the sched ecosystem by defining necessary handlers.
#[macro_export]
macro_rules! plug_sched {
    ($ty:ty, $($wasm:ident),+) => {
        const _: () = {
            type __TYPE = $ty;
        };

        $crate::__private::paste::paste! {
            $(
                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _sched_yield>](
                ) -> $crate::__private::wasip1::Errno {
                    $crate::__as_t!(@as_t, $wasm);
                    <$ty as $crate::sched::SchedYield>::sched_yield::<T>()
                }
            )*
        }
    };
    ($($wasm:ident),*) => {
        $crate::__as_t!(@through, $($wasm),* => $crate::plug_sched, @inner);
    };
    (@inner, $($wasm:ident),*) => {
        $crate::plug_sched!($crate::sched::DefaultSched, $($wasm),*);
    };
}
