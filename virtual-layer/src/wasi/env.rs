use crate::__private::wasip1::*;
#[cfg(target_os = "wasi")]
use const_for::const_for;
use const_struct::*;

use crate::memory::WasmAccess;

/// @block or @through
/// Whether to import JavaScript runtime env from vfs,
/// env is automatically imported even if you are not using it,
/// so that you can block it
/// @through if retrieving from JavaScript runtime.
///
/// @const or @static
/// Whether to use const or static env.
/// @const if using const env.
/// @static if using static env.
/// @const is faster and small than @static.
///
/// ```rust
/// // @const
/// import_wasm!(test_wasm);
///
/// use const_struct::*;
/// use wasip1_virtual_layer::prelude::*;
/// #[const_struct]
/// const VIRTUAL_ENV: VirtualEnvConstState = VirtualEnvConstState {
///     environ: &["RUST_MIN_STACK=16777216", "HOME=~/"],
/// };
/// export_env!(@block, @const, VirtualEnvTy, test_wasm);
/// ```
///
/// ```rust
/// // @static
/// import_wasm!(test_wasm);
///
/// use std::sync::{LazyLock, Mutex};
/// use wasip1_virtual_layer::prelude::*;
///
/// struct VirtualEnvState {
///    environ: Vec<String>,
/// }
/// impl<'a> VirtualEnv<'a> for VirtualEnvState {
///    type Str = String;
///
///   fn get_environ(&mut self) -> &[Self::Str] {
///       &self.environ
///   }
/// }
/// static VIRTUAL_ENV: LazyLock<Mutex<VirtualEnvState>> = LazyLock::new(|| {
///    let mut environ = Vec::<String>::new();
///   environ.push("RUST_MIN_STACK=16777216".into());
///   environ.push("HOME=~/".into());
///   Mutex::new(VirtualEnvState { environ })
/// });
/// export_env!(@through, @static, &mut VIRTUAL_ENV.lock().unwrap(), test_wasm);
/// ```
#[macro_export]
macro_rules! export_env {
    (@inner, @const, $ty:ty, $wasm:ty) => {
        $crate::__private::paste::paste! {
            #[unsafe(no_mangle)]
            #[cfg(target_os = "wasi")]
            pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _environ_sizes_get>](
                environ_count: *mut $crate::__private::wasip1::Size,
                environ_buf_size: *mut $crate::__private::wasip1::Size,
            ) -> $crate::__private::wasip1::Errno {
                $crate::__private::inner::env::environ_sizes_get_const_inner::<$ty, $wasm>(environ_count, environ_buf_size)
            }

            #[cfg(target_os = "wasi")]
            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _environ_get>](
                environ: *mut *const u8,
                environ_buf: *mut u8,
            ) -> $crate::__private::wasip1::Errno {
                $crate::__private::inner::env::environ_get_const_inner::<$ty, $wasm>(environ, environ_buf)
            }
        }
    };

    (@inner, @static, $state:expr, $wasm:ty) => {
        $crate::__private::paste::paste! {
            #[cfg(target_os = "wasi")]
            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _environ_sizes_get>](
                environ_count: *mut $crate::__private::wasip1::Size,
                environ_buf_size: *mut $crate::__private::wasip1::Size,
            ) -> $crate::__private::wasip1::Errno {
                let state = $state;
                $crate::__private::inner::env::environ_sizes_get_inner::<$wasm>(state, environ_count, environ_buf_size)
            }

            #[cfg(target_os = "wasi")]
            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _environ_get>](
                environ: *mut *const u8,
                environ_buf: *mut u8,
            ) -> $crate::__private::wasip1::Errno {
                let state = $state;
                $crate::__private::inner::env::environ_get_inner::<$wasm>(state, environ, environ_buf)
            }
        }
    };

    (@block_inner) => {
        #[cfg(target_os = "wasi")]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn __wasip1_vfs_environ_sizes_get(
            environ_count: *mut $crate::__private::wasip1::Size,
            environ_buf_size: *mut $crate::__private::wasip1::Size,
        ) -> $crate::__private::wasip1::Errno {
            unsafe { *environ_count = 0 };
            unsafe { *environ_buf_size = 0 };
            $crate::__private::wasip1::ERRNO_SUCCESS
        }

        #[cfg(target_os = "wasi")]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn __wasip1_vfs_environ_get(
            environ: *mut *const u8,
            environ_buf: *mut u8,
        ) -> $crate::__private::wasip1::Errno {
            $crate::__private::wasip1::ERRNO_SUCCESS
        }
    };

    (@block, @const, $ty:ty, $wasm:ty) => {
        $crate::export_env!(@block_inner);
        $crate::export_env!(@inner, @const, $ty, $wasm);
    };

    (@block, @static, $state:expr, $wasm:ty) => {
        $crate::export_env!(@block_inner);
        $crate::export_env!(@inner, @static, $state, $wasm);
    };

    (@through, @const, $ty:ty, $wasm:ty) => {
        $crate::export_env!(@inner, @const, $ty, $wasm);
    };

    (@through, @static, $state:expr, $wasm:ty) => {
        $crate::export_env!(@inner, @static, $state, $wasm);
    };
}

#[const_struct]
pub struct VirtualEnvConstState {
    pub environ: &'static [&'static str],
}

#[inline]
#[cfg(target_os = "wasi")]
pub fn environ_sizes_get_const_inner<
    T: PrimitiveTraits<DATATYPE = VirtualEnvConstState>,
    Wasm: WasmAccess,
>(
    environ_count: *mut Size,
    environ_buf_size: *mut Size,
) -> Errno {
    const fn inner<T: PrimitiveTraits<DATATYPE = VirtualEnvConstState>>() -> (Size, Size) {
        let mut size = 0;
        let mut count = 0;
        const_for!(i in 0..T::__DATA.environ.len() => {
            let len = T::__DATA.environ[i].len() + 1; // +1 for null terminator
            size += len;
            count += 1;
        });

        (size, count)
    }

    Wasm::store_le(environ_buf_size, inner::<T>().0);
    Wasm::store_le(environ_count, inner::<T>().1);
    ERRNO_SUCCESS
}

#[inline]
#[cfg(target_os = "wasi")]
pub fn environ_get_const_inner<
    T: PrimitiveTraits<DATATYPE = VirtualEnvConstState>,
    Wasm: WasmAccess,
>(
    environ: *mut *const u8,
    environ_buf: *mut u8,
) -> Errno {
    let mut environ = environ;
    let mut environ_buf = environ_buf;

    const_for!(i in 0..T::__DATA.environ.len() => {
        Wasm::store_le(environ, environ_buf as *const u8);

        Wasm::memcpy(environ_buf, T::__DATA.environ[i].as_bytes());
        Wasm::store_le(unsafe { environ_buf.add(T::__DATA.environ[i].len()) }, 0u8);

        environ = unsafe { environ.add(1) };
        environ_buf = unsafe { environ_buf.add(T::__DATA.environ[i].len() + 1) };
    });

    ERRNO_SUCCESS
}

pub trait VirtualEnv<'a> {
    type Str: AsRef<str>;

    fn get_environ(&'a mut self) -> &'a [Self::Str];
    fn environ_sizes_get(&'a mut self) -> (Size, Size) {
        let environ = self.get_environ();
        let mut size = 0;
        let mut count = 0;
        for env in environ {
            let len = env.as_ref().len() + 1; // +1 for null terminator
            size += len;
            count += 1;
        }

        (size, count)
    }

    fn environ_get<Wasm: WasmAccess>(
        &'a mut self,
        environ: *mut *const u8,
        environ_buf: *mut u8,
    ) -> Errno {
        let mut environ = environ;
        let mut environ_buf = environ_buf;

        for env in self.get_environ() {
            Wasm::store_le(environ, environ_buf as *const u8);

            Wasm::memcpy(environ_buf, env.as_ref().as_bytes());
            Wasm::store_le(
                unsafe { environ_buf.add(env.as_ref().len()) as *mut u8 },
                0u8,
            );

            environ = unsafe { environ.add(1) };
            environ_buf = unsafe { environ_buf.add(env.as_ref().len() + 1) };
        }

        ERRNO_SUCCESS
    }
}

impl<'a, T: core::ops::DerefMut<Target = U>, U: VirtualEnv<'a> + 'a> VirtualEnv<'a> for T {
    type Str = U::Str;

    fn get_environ(&'a mut self) -> &'a [Self::Str] {
        self.deref_mut().get_environ()
    }

    fn environ_sizes_get(&'a mut self) -> (Size, Size) {
        self.deref_mut().environ_sizes_get()
    }
}

#[cfg(target_os = "wasi")]
pub fn environ_sizes_get_inner<'a, Wasm: WasmAccess>(
    state: &'a mut impl VirtualEnv<'a>,
    environ_count: *mut Size,
    environ_buf_size: *mut Size,
) -> Errno {
    let (size, count) = state.environ_sizes_get();

    Wasm::store_le(environ_buf_size, size);
    Wasm::store_le(environ_count, count);

    ERRNO_SUCCESS
}

#[inline]
#[cfg(target_os = "wasi")]
pub fn environ_get_inner<'a, Wasm: WasmAccess>(
    state: &'a mut impl VirtualEnv<'a>,
    environ: *mut *const u8,
    environ_buf: *mut u8,
) -> Errno {
    state.environ_get::<Wasm>(environ, environ_buf)
}
