use crate::memory::{WasmAccess, WasmAccessName};

/// Trait for handling process exit.
pub trait ProcessExit {
    /// Exits the current process with the given code.
    fn proc_exit<Wasm: WasmAccess + WasmAccessName + 'static>(code: i32);
}

/// Default implementation of `ProcessExit`.
pub struct StandardProcess;

impl ProcessExit for StandardProcess {
    fn proc_exit<Wasm: WasmAccess + WasmAccessName + 'static>(code: i32) {
        #[cfg(feature = "std")]
        {
            std::process::exit(code as i32);
        }
        #[cfg(not(feature = "std"))]
        {
            use crate::transporter::Wasip1Transporter;

            Wasip1Transporter::process_abort(code as u32);
        }
    }
}

/// Plugs the process exit ecosystem by defining necessary handlers.
///
/// ```rust,no_run
/// use wasi_virt_layer::prelude::*;
///
/// import_wasm!(test_wasm);
///
/// plug_process!(StandardProcess, test_wasm);
/// ```
#[macro_export]
macro_rules! plug_process {
    ($ty:ty, $($wasm:ident),+) => {
        const _ : () = {
            #[allow(unused)]
            type __TYPE = $ty;
        };

        $crate::__private::paste::paste! {
            $(
                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _proc_exit>](
                    code: i32
                ) {
                    $crate::__as_t!(@as_t, $wasm);
                    <$ty as $crate::process::ProcessExit>::proc_exit::<T>(code)
                }
            )*
        }
    };
    ($($wasm:ident),*) => {
        $crate::__as_t!(@through, $($wasm),* => $crate::plug_process, @inner);
    };
    (@inner, $($wasm:ident),*) => {
        $crate::plug_process!($crate::process::StandardProcess, $($wasm),*);
    };
}
