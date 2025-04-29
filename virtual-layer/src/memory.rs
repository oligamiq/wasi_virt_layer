/// By entering the names of the files to be combined, a bridge for the combination is created.
/// You need to prepare as many Wasip1 instances on the virtual file system as the number of files to be combined.
#[macro_export]
macro_rules! import_wasm {
    ($name:ident) => {
        $crate::paste::paste! {
            #[allow(non_camel_case_types)]
            struct $name;

            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "wasip1-vfs")]
            unsafe extern "C" {
                /// https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Copy
                #[unsafe(no_mangle)]
                pub fn [<__wasip1_vfs_ $name _memory_copy_from>](
                    offset: *mut u8,
                    src: *const u8,
                    len: usize,
                );

                #[unsafe(no_mangle)]
                pub fn [<__wasip1_vfs_ $name _memory_copy_to>](
                    offset: *mut u8,
                    src: *const u8,
                    len: usize,
                );

                #[unsafe(no_mangle)]
                pub fn [<__wasip1_vfs_ $name ___main_void>]();

                #[unsafe(no_mangle)]
                pub fn [<__wasip1_vfs_ $name __start>]();

                #[unsafe(no_mangle)]
                pub fn [<__wasip1_vfs_ $name _reset>]();
            }

            impl $crate::memory::WasmAccess for $name {
                #[inline(always)]
                fn memcpy<T>(offset: *mut T, data: &[T])
                {
                    #[cfg(not(target_arch = "wasm32"))]
                    unimplemented!("this is not supported on this architecture");

                    #[cfg(target_arch = "wasm32")]
                    unsafe { [<__wasip1_vfs_ $name _memory_copy_from>](
                        offset as *mut u8,
                        data.as_ptr() as *const u8,
                        core::mem::size_of::<T>() * data.len(),
                    ) };
                }

                #[inline(always)]
                fn memcpy_to<T>(offset: &mut [T], src: *const T)
                {
                    #[cfg(not(target_arch = "wasm32"))]
                    unimplemented!("this is not supported on this architecture");

                    #[cfg(target_arch = "wasm32")]
                    unsafe { [<__wasip1_vfs_ $name _memory_copy_to>](
                        offset.as_mut_ptr() as *mut u8,
                        src as *const u8,
                        core::mem::size_of::<T>() * offset.len(),
                    ) };
                }

                #[inline(always)]
                fn store_le<T>(offset: *mut T, value: T)
                {
                    #[cfg(not(target_arch = "wasm32"))]
                    unimplemented!("this is not supported on this architecture");

                    #[cfg(target_arch = "wasm32")]
                    unsafe { [<__wasip1_vfs_ $name _memory_copy_from>](
                        offset as *mut u8,
                        &value as *const T as *const u8,
                        core::mem::size_of::<T>(),
                    ) };
                }

                #[inline(always)]
                fn load_le<T: core::fmt::Debug>(offset: *const T) -> T
                {
                    #[cfg(not(target_arch = "wasm32"))]
                    unimplemented!("this is not supported on this architecture");

                    #[cfg(target_arch = "wasm32")]
                    unsafe {
                        let mut value = core::mem::MaybeUninit::uninit();
                        [<__wasip1_vfs_ $name _memory_copy_to>](
                            value.as_mut_ptr() as *mut u8,
                            offset as *const u8,
                            core::mem::size_of::<T>(),
                        );
                        value
                            .assume_init()
                    }
                }

                #[inline(always)]
                fn main()
                {
                    #[cfg(not(target_arch = "wasm32"))]
                    unimplemented!("this is not supported on this architecture");

                    #[cfg(target_arch = "wasm32")]
                    unsafe { [<__wasip1_vfs_ $name ___main_void>]() };
                }

                #[inline(always)]
                fn reset()
                {
                    #[cfg(not(target_arch = "wasm32"))]
                    unimplemented!("this is not supported on this architecture");

                    #[cfg(target_arch = "wasm32")]
                    unsafe { [<__wasip1_vfs_ $name _reset>]() };
                }

                #[inline(always)]
                fn _start()
                {
                    #[cfg(not(target_arch = "wasm32"))]
                    unimplemented!("this is not supported on this architecture");

                    #[cfg(target_arch = "wasm32")]
                    unsafe { [<__wasip1_vfs_ $name __start>]() };
                }
            }
        }
    };
}

#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
unsafe extern "C" fn __wasip1_vfs_flag_vfs_memory(ptr: *mut u8, src: *mut u8) {
    unsafe { core::ptr::copy_nonoverlapping(src, ptr, 1) };
}

pub trait WasmAccess {
    fn memcpy<T>(offset: *mut T, data: &[T]);
    fn memcpy_to<T>(offset: &mut [T], src: *const T);
    fn store_le<T>(offset: *mut T, value: T);
    fn load_le<T: core::fmt::Debug>(offset: *const T) -> T;

    /// wrapping wasm's _start function
    /// By default in Rust code, when _start is called,
    /// the main function is executed.
    /// If you wish to call it again,
    /// you must use the __main_void function.
    /// When you write code that explicitly calls this function,
    /// the command-line tool (virtual-layer-cli) detects it,
    /// and as a result,
    /// __main_void will no longer be invoked from within _start.
    /// Instead, it can only be called through the function
    /// that wraps __main_void.
    /// This does not apply if it's used as a library.
    ///
    /// Using this and export_env,
    /// it is possible to override arguments, for example, to call
    fn main();

    /// memory reset to memory which instantiate
    /// function's roll
    /// - other memory fill zeroed
    /// - reset global variables
    /// - memory copied from data-segment
    /// if you call this function,
    /// virtual file system's memory isn't changed
    /// _start is not called
    ///
    /// to call this function after _start
    fn reset();

    /// Calls the initialization function provided.
    /// If you are using the main function of the same TRAIT,
    /// RUST's main function will not be automatically executed during initialization.
    ///
    /// if you want to use reset, call this function first otherwise
    fn _start();
}
