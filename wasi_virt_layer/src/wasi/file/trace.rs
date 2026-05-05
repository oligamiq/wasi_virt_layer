#[cfg(feature = "trace-fs")]
macro_rules! trace_fs {
    (
        $vfs:expr,
        $wasm:ty;
        $($arg:tt)*
    ) => {
        {
            #[cfg(feature = "std")]
            let msg = {
                format!($($arg)*)
            };

            #[cfg(not(feature = "std"))]
            let msg = {
                stringify!($($arg)*)
            };

            let b = msg.as_bytes();

            $crate::simple_debug::simple_debug_print(b);
        }
    };
}

#[cfg(not(feature = "trace-fs"))]
macro_rules! trace_fs {
    (
        $vfs:expr,
        $wasm:ty;
        $($arg:tt)*
    ) => {};
}

pub(crate) use trace_fs;
