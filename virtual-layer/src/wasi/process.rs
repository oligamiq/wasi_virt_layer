use crate::{__private::wasip1, memory::WasmAccess};

pub trait ProcessExit {
    fn proc_exit<Wasm: WasmAccess>(code: i32) -> wasip1::Errno;
}

pub struct DefaultProcess;

impl ProcessExit for DefaultProcess {
    fn proc_exit<Wasm: WasmAccess>(code: i32) -> wasip1::Errno {
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
macro_rules! export_process {
    ($ty:ty) => {
        #[unsafe(no_mangle)]
        #[cfg(target_os = "wasi")]
        pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _proc_exit>](
            code: i32
        ) -> $crate::__private::wasip1::Errno {
            <$ty as $crate::process::ProcessExit>::proc_exit::<Wasm>(code)
        }
    }
}
