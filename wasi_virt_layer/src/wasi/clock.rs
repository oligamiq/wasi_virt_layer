use crate::__private::wasip1::*;
use crate::memory::WasmAccess;

/// Trait for handling clock operations including time retrieval and resolution queries.
pub trait Clock {
    /// Gets the current time for the specified clock.
    ///
    /// # Parameters
    /// - `clock_id`: The identifier of the clock to query
    /// - `precision`: The maximum amount of nanoseconds of imprecision the caller can tolerate
    /// - `time_ptr`: Pointer to write the resulting timestamp to (in nanoseconds)
    ///
    /// # Returns
    /// `ERRNO_SUCCESS` on success, or an error code on failure
    fn clock_time_get<Wasm: WasmAccess>(
        clock_id: Clockid,
        precision: Timestamp,
        time_ptr: *mut Timestamp,
    ) -> Errno;

    /// Gets the resolution of the specified clock.
    ///
    /// # Parameters
    /// - `clock_id`: The identifier of the clock to query
    /// - `resolution_ptr`: Pointer to write the resolution to (in nanoseconds)
    ///
    /// # Returns
    /// `ERRNO_SUCCESS` on success, or an error code on failure
    fn clock_res_get<Wasm: WasmAccess>(
        clock_id: Clockid,
        resolution_ptr: *mut Timestamp,
    ) -> Errno;
}

/// Default implementation of `Clock` that delegates to the underlying WASI runtime.
pub struct StandardClock;

impl Clock for StandardClock {
    fn clock_time_get<Wasm: WasmAccess>(
        clock_id: Clockid,
        precision: Timestamp,
        time_ptr: *mut Timestamp,
    ) -> Errno {
        #[cfg(target_os = "wasi")]
        {
            use crate::transporter::non_recursive_clock_time_get;

            #[cfg(not(feature = "multi_memory"))]
            let adjusted_ptr = unsafe { Wasm::memory_director_mut(time_ptr as *mut u8) as *mut Timestamp };
            #[cfg(feature = "multi_memory")]
            let adjusted_ptr = time_ptr;

            match unsafe { non_recursive_clock_time_get(clock_id, precision) } {
                Ok(timestamp) => {
                    unsafe { core::ptr::write(adjusted_ptr, timestamp) };
                    ERRNO_SUCCESS
                }
                Err(e) => e,
            }
        }
        #[cfg(not(target_os = "wasi"))]
        {
            let _ = clock_id;
            let _ = precision;
            let _ = time_ptr;
            ERRNO_NOSYS
        }
    }

    fn clock_res_get<Wasm: WasmAccess>(
        clock_id: Clockid,
        resolution_ptr: *mut Timestamp,
    ) -> Errno {
        #[cfg(target_os = "wasi")]
        {
            use crate::transporter::non_recursive_clock_res_get;

            #[cfg(not(feature = "multi_memory"))]
            let adjusted_ptr = unsafe { Wasm::memory_director_mut(resolution_ptr as *mut u8) as *mut Timestamp };
            #[cfg(feature = "multi_memory")]
            let adjusted_ptr = resolution_ptr;

            match unsafe { non_recursive_clock_res_get(clock_id) } {
                Ok(resolution) => {
                    unsafe { core::ptr::write(adjusted_ptr, resolution) };
                    ERRNO_SUCCESS
                }
                Err(e) => e,
            }
        }
        #[cfg(not(target_os = "wasi"))]
        {
            let _ = clock_id;
            let _ = resolution_ptr;
            ERRNO_NOSYS
        }
    }
}

/// Plugs the clock ecosystem by defining necessary handlers for clock operations.
///
/// # Examples
/// ```no_run
/// use wasi_virt_layer::prelude::*;
///
/// // Using the default StandardClock implementation
/// plug_clock!(self);
///
/// // Using a custom Clock implementation
/// struct CustomClock;
/// impl Clock for CustomClock {
///     fn clock_time_get<Wasm: WasmAccess>(
///         clock_id: wasi_virt_layer::__private::wasip1::Clockid,
///         precision: wasi_virt_layer::__private::wasip1::Timestamp,
///         time_ptr: *mut wasi_virt_layer::__private::wasip1::Timestamp,
///     ) -> wasi_virt_layer::__private::wasip1::Errno {
///         // Custom implementation here
///         StandardClock::clock_time_get::<Wasm>(clock_id, precision, time_ptr)
///     }
///
///     fn clock_res_get<Wasm: WasmAccess>(
///         clock_id: wasi_virt_layer::__private::wasip1::Clockid,
///         resolution_ptr: *mut wasi_virt_layer::__private::wasip1::Timestamp,
///     ) -> wasi_virt_layer::__private::wasip1::Errno {
///         // Custom implementation here
///         StandardClock::clock_res_get::<Wasm>(clock_id, resolution_ptr)
///     }
/// }
/// plug_clock!(CustomClock, self);
/// ```
#[macro_export]
macro_rules! plug_clock {
    ($ty:ty, $($wasm:ident),+) => {
        const _ : () = {
            type __TYPE = $ty;
        };

        $crate::__private::paste::paste! {
            $(
                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _clock_time_get>](
                    clock_id: $crate::__private::wasip1::Clockid,
                    precision: $crate::__private::wasip1::Timestamp,
                    time_ptr: *mut $crate::__private::wasip1::Timestamp,
                ) -> $crate::__private::wasip1::Errno {
                    $crate::__as_t!(@as_t, $wasm);
                    <$ty as $crate::clock::Clock>::clock_time_get::<T>(clock_id, precision, time_ptr)
                }

                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _clock_res_get>](
                    clock_id: $crate::__private::wasip1::Clockid,
                    resolution_ptr: *mut $crate::__private::wasip1::Timestamp,
                ) -> $crate::__private::wasip1::Errno {
                    $crate::__as_t!(@as_t, $wasm);
                    <$ty as $crate::clock::Clock>::clock_res_get::<T>(clock_id, resolution_ptr)
                }
            )*
        }
    };
    ($($wasm:ident),*) => {
        $crate::__as_t!(@through, $($wasm),* => $crate::plug_clock, @inner);
    };
    (@inner, $($wasm:ident),*) => {
        $crate::plug_clock!($crate::clock::StandardClock, $($wasm),*);
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::WasmAccessRaw;

    #[derive(Debug)]
    struct MockWasm;

    impl WasmAccessRaw for MockWasm {
        fn memcpy_raw(offset: *mut u8, data: *const u8, len: usize) {
            unsafe { core::ptr::copy_nonoverlapping(data, offset, len) };
        }

        fn memcpy_to_raw(offset: *mut u8, src: *const u8, len: usize) {
            unsafe { core::ptr::copy_nonoverlapping(src, offset, len) };
        }

        #[cfg(not(feature = "multi_memory"))]
        fn memory_director_raw(ptr: isize) -> isize {
            ptr
        }

        fn _main_raw() -> Errno {
            ERRNO_SUCCESS
        }

        fn _reset_raw() {}

        fn _start_raw() {}
    }

    #[test]
    #[cfg(not(target_os = "wasi"))]
    fn test_standard_clock_non_wasi() {
        let mut time_buf = 0u64;
        let result = StandardClock::clock_time_get::<MockWasm>(
            CLOCKID_REALTIME,
            1000,
            &mut time_buf as *mut u64,
        );
        // On non-WASI targets, should return ERRNO_NOSYS
        assert_eq!(result, ERRNO_NOSYS);
    }

    #[test]
    #[cfg(not(target_os = "wasi"))]
    fn test_standard_clock_res_get_non_wasi() {
        let mut resolution_buf = 0u64;
        let result = StandardClock::clock_res_get::<MockWasm>(
            CLOCKID_REALTIME,
            &mut resolution_buf as *mut u64,
        );
        // On non-WASI targets, should return ERRNO_NOSYS
        assert_eq!(result, ERRNO_NOSYS);
    }
}
