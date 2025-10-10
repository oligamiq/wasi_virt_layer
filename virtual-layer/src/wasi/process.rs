use crate::memory::WasmAccess;

pub trait ProcessExit {
    fn proc_exit<Wasm: WasmAccess>(code: i32) -> !;
}

pub struct DefaultProcess;

impl ProcessExit for DefaultProcess {
    fn proc_exit<Wasm: WasmAccess>(code: i32) -> ! {
        #[cfg(feature = "std")]
        {
            std::process::exit(code as i32);
        }
        #[cfg(not(feature = "std"))]
        {
            unsafe { wasip1::proc_exit(code) };
            unreachable!();
        }
    }
}

#[macro_export]
macro_rules! plug_process {
    ($($wasm:ident),*) => {
        $crate::__as_t!(@through, $($wasm),* => $crate::plug_process, @inner);
    };
    (@inner, $($wasm:ident),*) => {
        $crate::plug_process!($crate::process::DefaultProcess, $($wasm),*);
    };
    ($ty:ty, $($wasm:ident),*) => {
        $crate::__private::paste::paste! {
            $(
                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _proc_exit>](
                    code: i32
                ) -> ! {
                    <$ty as $crate::process::ProcessExit>::proc_exit::<$wasm>(code)
                }
            )*
        }
    };
}
