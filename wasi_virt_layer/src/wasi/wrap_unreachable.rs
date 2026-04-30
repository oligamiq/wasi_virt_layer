use crate::memory::WasmAccess;

/// Defines callbacks for handling target execution lifecycle when `unreachable` triggers unwinding.
pub trait WrapUnreachable {
    /// Modify the exit code returned by `_main_raw` when unwinding happens.
    fn fix_main_raw_exit_code<Wasm: WasmAccess>(code: i32) -> i32;

    /// Handle specific thread exit logic via unwinding code values.
    fn handle_thread_exit<Wasm: WasmAccess>(code: i32) {
        let _ = code;
        // default empty implementation
    }
}

/// Default implementation of `WrapUnreachable`.
pub struct StandardWrapUnreachable;

impl WrapUnreachable for StandardWrapUnreachable {
    fn fix_main_raw_exit_code<Wasm: WasmAccess>(code: i32) -> i32 {
        code
    }
}

/// A macro to explicitly wrap WebAssembly's unreachable instructions for a given Target module.
/// This generates internal markers interpreted by `wasi_virt_layer-cli` during generation.
#[macro_export]
macro_rules! wrap_unreachable {
    ($handler:ty, $($wasm:ident),+) => {
        const _: () = {
            type __HANDLER = $handler;
        };

        $crate::__private::paste::paste! {
            $(
                #[cfg(target_os = "wasi")]
                #[unsafe(no_mangle)]
                pub extern "C" fn [<__wasip1_virt_layer_ $wasm _wrap_unreachable>]() {}

                #[cfg(target_os = "wasi")]
                #[unsafe(no_mangle)]
                pub extern "C" fn [<__wasip1_virt_layer_ $wasm _fix_main_raw_exit_code>](code: i32) -> i32 {
                    $crate::__as_t!(@as_t, $wasm);
                    <$handler as $crate::wasi::wrap_unreachable::WrapUnreachable>::fix_main_raw_exit_code::<T>(code)
                }

                #[cfg(target_os = "wasi")]
                #[unsafe(no_mangle)]
                pub extern "C" fn [<__wasip1_virt_layer_ $wasm _handle_thread_exit>](code: i32) {
                    $crate::__as_t!(@as_t, $wasm);
                    <$handler as $crate::wasi::wrap_unreachable::WrapUnreachable>::handle_thread_exit::<T>(code)
                }

                #[cfg(target_os = "wasi")]
                #[link(wasm_import_module = "__wasip1_virt_layer")]
                unsafe extern "C" {
                    #[link_name = concat!("__wasip1_virt_layer_", stringify!($wasm), "_get_unreachable_flag")]
                    pub fn [<__wasip1_virt_layer_ $wasm _get_unreachable_flag>]() -> i32;

                    #[link_name = concat!("__wasip1_virt_layer_", stringify!($wasm), "_set_unreachable_flag")]
                    pub fn [<__wasip1_virt_layer_ $wasm _set_unreachable_flag>](val: i32);
                }

                pub struct [<WrapUnreachable $wasm:camel>];

                impl [<WrapUnreachable $wasm:camel>] {
                    pub fn get_flag() -> i32 {
                        #[cfg(target_os = "wasi")]
                        unsafe { [<__wasip1_virt_layer_ $wasm _get_unreachable_flag>]() }

                        #[cfg(not(target_os = "wasi"))]
                        unimplemented!("get_flag is only available on wasm32 targets")
                    }

                    pub fn set_flag(val: i32) {
                        #[cfg(target_os = "wasi")]
                        unsafe { [<__wasip1_virt_layer_ $wasm _set_unreachable_flag>](val) }

                        #[cfg(not(target_os = "wasi"))]
                        unimplemented!("set_flag is only available on wasm32 targets")
                    }

                    pub fn fix_main_raw_exit_code(code: i32) -> i32 {
                        $crate::__as_t!(@as_t, $wasm);
                        <$handler as $crate::wasi::wrap_unreachable::WrapUnreachable>::fix_main_raw_exit_code::<T>(code)
                    }

                    pub fn handle_thread_exit(code: i32) {
                        $crate::__as_t!(@as_t, $wasm);
                        <$handler as $crate::wasi::wrap_unreachable::WrapUnreachable>::handle_thread_exit::<T>(code)
                    }
                }
            )*
        }
    };
    ($($wasm:ident),*) => {
        $crate::__as_t!(@through, $($wasm),* => $crate::wrap_unreachable, @inner);
    };
    (@inner, $($wasm:ident),*) => {
        $crate::wrap_unreachable!($crate::wasi::wrap_unreachable::StandardWrapUnreachable, $($wasm),*);
    };
}
