/// Defines callbacks for handling target execution lifecycle when `unreachable` triggers unwinding.
pub trait WrapUnreachable {
    /// Modify the exit code returned by `_main_raw` when unwinding happens.
    fn fix_main_raw_exit_code(code: i32) -> i32;

    /// Handle specific thread exit logic via unwinding code values.
    fn handle_thread_exit(code: i32) {
        let _ = code;
        // default empty implementation
    }
}

/// A macro to explicitly wrap WebAssembly's unreachable instructions for a given Target module.
/// This generates internal markers interpreted by `wasi_virt_layer-cli` during generation.
#[macro_export]
macro_rules! wrap_unreachable {
    ($target:ident, $handler:ty) => {
        const _: () = {
            type __HANDLER = $handler;
        };

        $crate::__private::paste::paste! {
            #[cfg(target_arch = "wasm32")]
            #[unsafe(no_mangle)]
            pub extern "C" fn [<__wasip1_virt_layer_ $target _wrap_unreachable>]() {}

            #[cfg(target_arch = "wasm32")]
            #[unsafe(no_mangle)]
            pub extern "C" fn [<__wasip1_virt_layer_ $target _fix_main_raw_exit_code>](code: i32) -> i32 {
                <$handler as $crate::wasi::wrap_unreachable::WrapUnreachable>::fix_main_raw_exit_code(code)
            }

            #[cfg(target_arch = "wasm32")]
            #[unsafe(no_mangle)]
            pub extern "C" fn [<__wasip1_virt_layer_ $target _handle_thread_exit>](code: i32) {
                <$handler as $crate::wasi::wrap_unreachable::WrapUnreachable>::handle_thread_exit(code)
            }

            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "__wasip1_virt_layer")]
            unsafe extern "C" {
                #[link_name = concat!("__wasip1_virt_layer_", stringify!($target), "_get_unreachable_flag")]
                pub fn [<__wasip1_virt_layer_ $target _get_unreachable_flag>]() -> i32;

                #[link_name = concat!("__wasip1_virt_layer_", stringify!($target), "_set_unreachable_flag")]
                pub fn [<__wasip1_virt_layer_ $target _set_unreachable_flag>](val: i32);
            }

            pub struct [<WrapUnreachable $target:camel>];

            impl [<WrapUnreachable $target:camel>] {
                pub fn get_flag() -> i32 {
                    #[cfg(target_arch = "wasm32")]
                    unsafe { [<__wasip1_virt_layer_ $target _get_unreachable_flag>]() }

                    #[cfg(not(target_arch = "wasm32"))]
                    unimplemented!("get_flag is only available on wasm32 targets")
                }

                pub fn set_flag(val: i32) {
                    #[cfg(target_arch = "wasm32")]
                    unsafe { [<__wasip1_virt_layer_ $target _set_unreachable_flag>](val) }

                    #[cfg(not(target_arch = "wasm32"))]
                    unimplemented!("set_flag is only available on wasm32 targets")
                }

                pub fn fix_main_raw_exit_code(code: i32) -> i32 {
                    <$handler as $crate::wasi::wrap_unreachable::WrapUnreachable>::fix_main_raw_exit_code(code)
                }

                pub fn handle_thread_exit(code: i32) {
                    <$handler as $crate::wasi::wrap_unreachable::WrapUnreachable>::handle_thread_exit(code)
                }
            }
        }
    };
}
