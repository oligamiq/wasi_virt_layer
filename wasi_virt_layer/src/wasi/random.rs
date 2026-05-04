use crate::__private::wasip1;
use crate::memory::{WasmAccess, WasmAccessName};

/// Trait for handling random number generation.
pub trait Random {
    /// Fills the given buffer with random bytes.
    fn random_get<Wasm: WasmAccess + WasmAccessName + 'static>(
        buf: *mut u8,
        buf_len: usize,
    ) -> wasip1::Errno;
}

/// Default implementation of `Random` using host randomness.
pub struct StandardRandom;

impl Random for StandardRandom {
    fn random_get<Wasm: WasmAccess + WasmAccessName + 'static>(
        buf: *mut u8,
        buf_len: usize,
    ) -> wasip1::Errno {
        #[cfg(target_os = "wasi")]
        {
            use crate::transporter::non_recursive_random_get;
            #[cfg(not(feature = "multi_memory"))]
            let ptr = Wasm::memory_director_mut(buf);
            #[cfg(feature = "multi_memory")]
            let ptr = buf;

            match unsafe { non_recursive_random_get(ptr, buf_len) } {
                Ok(_) => wasip1::ERRNO_SUCCESS,
                Err(e) => e,
            }
        }
        #[cfg(not(target_os = "wasi"))]
        {
            let _ = buf;
            let _ = buf_len;
            wasip1::ERRNO_NOSYS
        }
    }
}

/// Implementation of `Random` using a pseudo-random number generator.
pub struct PseudoRandom;

#[cfg(feature = "threads")]
static PSEUDO_RANDOM_STATE: parking_lot::Mutex<u64> = parking_lot::const_mutex(0x123456789ABCDEF0);

#[cfg(not(feature = "threads"))]
static mut PSEUDO_RANDOM_STATE: u64 = 0x123456789ABCDEF0;

fn next_u64() -> u64 {
    #[cfg(feature = "threads")]
    {
        let mut x = PSEUDO_RANDOM_STATE.lock();
        *x ^= *x << 13;
        *x ^= *x >> 7;
        *x ^= *x << 17;
        *x
    }
    #[cfg(not(feature = "threads"))]
    {
        unsafe {
            PSEUDO_RANDOM_STATE ^= PSEUDO_RANDOM_STATE << 13;
            PSEUDO_RANDOM_STATE ^= PSEUDO_RANDOM_STATE >> 7;
            PSEUDO_RANDOM_STATE ^= PSEUDO_RANDOM_STATE << 17;
            PSEUDO_RANDOM_STATE
        }
    }
}

impl Random for PseudoRandom {
    fn random_get<Wasm: WasmAccess + WasmAccessName + 'static>(
        buf: *mut u8,
        buf_len: usize,
    ) -> wasip1::Errno {
        let mut i = 0;
        while i < buf_len {
            let random_val = next_u64();
            let bytes = random_val.to_ne_bytes();
            let to_copy = core::cmp::min(8, buf_len - i);
            Wasm::memcpy(unsafe { buf.add(i) }, &bytes[..to_copy]);
            i += to_copy;
        }
        wasip1::ERRNO_SUCCESS
    }
}

/// Plugs the random ecosystem by defining necessary handlers.
///
/// ```rust,no_run
/// use wasi_virt_layer::prelude::*;
///
/// import_wasm!(test_wasm);
///
/// plug_random!(StandardRandom, test_wasm);
/// ```
#[macro_export]
macro_rules! plug_random {
    ($ty:ty, $($wasm:ident),+) => {
        const _ : () = {
            type __TYPE = $ty;
        };

        $crate::__private::paste::paste! {
            $(
                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _random_get>](
                    buf: *mut u8,
                    buf_len: $crate::__private::wasip1::Size,
                ) -> $crate::__private::wasip1::Errno {
                    $crate::__as_t!(@as_t, $wasm);
                    <$ty as $crate::random::Random>::random_get::<T>(buf, buf_len)
                }
            )*
        }
    };
    ($($wasm:ident),*) => {
        $crate::__as_t!(@through, $($wasm),* => $crate::plug_random, @inner);
    };
    (@inner, $($wasm:ident),*) => {
        $crate::plug_random!($crate::random::StandardRandom, $($wasm),*);
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::WasmAccessFaker;

    #[test]
    fn test_pseudo_random_deterministic() {
        let mut buf1 = [0u8; 16];
        let mut buf2 = [0u8; 16];

        // Reset state for test determinism
        #[cfg(feature = "threads")]
        {
            *PSEUDO_RANDOM_STATE.lock() = 0x123456789ABCDEF0;
        }
        #[cfg(not(feature = "threads"))]
        {
            unsafe { PSEUDO_RANDOM_STATE = 0x123456789ABCDEF0 };
        }

        PseudoRandom::random_get::<WasmAccessFaker>(buf1.as_mut_ptr(), 16);

        #[cfg(feature = "threads")]
        {
            *PSEUDO_RANDOM_STATE.lock() = 0x123456789ABCDEF0;
        }
        #[cfg(not(feature = "threads"))]
        {
            unsafe { PSEUDO_RANDOM_STATE = 0x123456789ABCDEF0 };
        }

        PseudoRandom::random_get::<WasmAccessFaker>(buf2.as_mut_ptr(), 16);

        assert_eq!(buf1, buf2);
        assert_ne!(buf1, [0u8; 16]);
    }
}
