use crate::{__private::wasip1::*, memory::WasmAccessName};
#[cfg(target_os = "wasi")]
use const_for::const_for;
use const_struct::*;

use crate::memory::WasmAccess;

/// @embedded or @dynamic
/// Whether to import JavaScript runtime args from vfs,
/// args is automatically imported even if you are not using it,
/// so that you can block it
///
/// @embedded or @dynamic
/// Whether to use embedded or dynamic args.
/// @embedded if using embedded args.
/// @dynamic if using dynamic args.
/// @embedded is faster and small than @dynamic.
///
/// ```rust
/// // @embedded
/// import_wasm!(test_wasm);
///
/// use const_struct::*;
/// use wasi_virt_layer::prelude::*;
/// #[const_struct]
/// const VIRTUAL_ARGS: VirtualArgsEmbeddedState = VirtualArgsEmbeddedState {
///     args: &["command", "arg1", "arg2"],
/// };
/// plug_args!(@embedded, VirtualArgsTy, test_wasm);
/// ```
///
/// ```rust
/// // @dynamic
/// import_wasm!(test_wasm);
///
/// use std::sync::{LazyLock, Mutex};
/// use wasi_virt_layer::prelude::*;
///
/// struct VirtualArgsState {
///    args: Vec<String>,
/// }
/// impl<'a> VirtualArgs<'a> for VirtualArgsState {
///    type Str = String;
///
///   fn get_args(&mut self) -> &[Self::Str] {
///       &self.args
///   }
/// }
/// static VIRTUAL_ARGS: LazyLock<Mutex<VirtualArgsState>> = LazyLock::new(|| {
///    let mut args = Vec::<String>::new();
///   args.push("command".into());
///   args.push("arg1".into());
///   Mutex::new(VirtualArgsState { args })
/// });
/// plug_args!(@dynamic, &mut VIRTUAL_ARGS.lock().unwrap(), test_wasm);
/// ```
/// Plugs the command-line arguments ecosystem.
#[macro_export]
macro_rules! plug_args {
    (@embedded, $ty:ty, $($wasm:ident),* $(,)?) => {
        $crate::__as_t!(@through, $($wasm),* => $crate::plug_args, @inner, @embedded, $ty);
    };

    (@dynamic, $state:expr, $($wasm:ident),* $(,)?) => {
        $crate::__as_t!(@through, $($wasm),* => $crate::plug_args, @inner, @dynamic, $state);
    };

    (@inner, @embedded, $ty:ty, $($wasm:ident),*) => {
        const _: () = {
            type __TYPE = $ty;
        };

        $crate::__private::paste::paste! {
            $(
                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _args_sizes_get>](
                    __args_count: *mut $crate::__private::wasip1::Size,
                    __args_buf_size: *mut $crate::__private::wasip1::Size,
                ) -> $crate::__private::wasip1::Errno {
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::__private::inner::args::args_sizes_get_embedded_inner::<$ty, T>(__args_count, __args_buf_size)
                }

                #[cfg(target_os = "wasi")]
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _args_get>](
                    __args: *mut *const u8,
                    __args_buf: *mut u8,
                ) -> $crate::__private::wasip1::Errno {
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::__private::inner::args::args_get_embedded_inner::<$ty, T>(__args, __args_buf)
                }
            )*
        }
    };

    (@inner, @dynamic, $state:expr, $($wasm:ident),*) => {
        const _: () = {
            const _ = || {
                let _ = $state;
            };
        };

        $crate::__private::paste::paste! {
            $(
                #[cfg(target_os = "wasi")]
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _args_sizes_get>](
                    args_count: *mut $crate::__private::wasip1::Size,
                    args_buf_size: *mut $crate::__private::wasip1::Size,
                ) -> $crate::__private::wasip1::Errno {
                    $crate::__as_t!(@as_t, $wasm);
                    let state = $state;
                    $crate::__private::inner::args::args_sizes_get_inner::<T>(state, args_count, args_buf_size)
                }

                #[cfg(target_os = "wasi")]
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _args_get>](
                    args: *mut *const u8,
                    args_buf: *mut u8,
                ) -> $crate::__private::wasip1::Errno {
                    $crate::__as_t!(@as_t, $wasm);
                    let state = $state;
                    $crate::__private::inner::args::args_get_inner::<T>(state, args, args_buf)
                }

                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _args_sizes_get>](
                    __args_count: *mut $crate::__private::wasip1::Size,
                    __args_buf_size: *mut $crate::__private::wasip1::Size,
                ) -> $crate::__private::wasip1::Errno {
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::__private::inner::args::args_sizes_get_const_inner::<$ty, T>(__args_count, __args_buf_size)
                }

                #[cfg(target_os = "wasi")]
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _args_get>](
                    __args: *mut *const u8,
                    __args_buf: *mut u8,
                ) -> $crate::__private::wasip1::Errno {
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::__private::inner::args::args_get_const_inner::<$ty, T>(__args, __args_buf)
                }
            )*
        }
    };
}

/// A structure holding constant command-line arguments.
#[const_struct]
pub struct VirtualArgsEmbeddedState {
    /// The command-line arguments.
    pub args: &'static [&'static str],
}

/// Inner function for retrieving constant command-line argument sizes.
#[inline]
#[cfg(target_os = "wasi")]
pub fn args_sizes_get_embedded_inner<
    T: PrimitiveTraits<DATATYPE = VirtualArgsEmbeddedState>,
    Wasm: WasmAccess,
>(
    args_count: *mut Size,
    args_buf_size: *mut Size,
) -> Errno {
    const fn inner<T: PrimitiveTraits<DATATYPE = VirtualArgsEmbeddedState>>() -> (Size, Size) {
        let mut size = 0;
        let mut count = 0;
        const_for!(i in 0..T::__DATA.args.len() => {
            let len = T::__DATA.args[i].len() + 1; // +1 for null terminator
            size += len;
            count += 1;
        });

        (size, count)
    }

    Wasm::store_le(args_buf_size, inner::<T>().0);
    Wasm::store_le(args_count, inner::<T>().1);
    ERRNO_SUCCESS
}

/// Inner function for retrieving constant command-line arguments.
#[inline]
#[cfg(target_os = "wasi")]
pub fn args_get_embedded_inner<
    T: PrimitiveTraits<DATATYPE = VirtualArgsEmbeddedState>,
    Wasm: WasmAccess,
>(
    args: *mut *const u8,
    args_buf: *mut u8,
) -> Errno {
    let mut args = args;
    let mut args_buf = args_buf;

    const_for!(i in 0..T::__DATA.args.len() => {
        Wasm::store_le(args, args_buf as *const u8);

        Wasm::memcpy(args_buf, T::__DATA.args[i].as_bytes());
        Wasm::store_le(unsafe { args_buf.add(T::__DATA.args[i].len()) }, 0u8);

        args = unsafe { args.add(1) };
        args_buf = unsafe { args_buf.add(T::__DATA.args[i].len() + 1) };
    });

    ERRNO_SUCCESS
}

/// Inner function for retrieving constant command-line argument sizes.
#[inline]
#[cfg(target_os = "wasi")]
pub fn args_sizes_get_const_inner<
    T: PrimitiveTraits<DATATYPE = VirtualArgsEmbeddedState>,
    Wasm: WasmAccess,
>(
    args_count: *mut Size,
    args_buf_size: *mut Size,
) -> Errno {
    const fn inner<T: PrimitiveTraits<DATATYPE = VirtualArgsEmbeddedState>>() -> (Size, Size) {
        let mut size = 0;
        let mut count = 0;
        const_for!(i in 0..T::__DATA.args.len() => {
            let len = T::__DATA.args[i].len() + 1; // +1 for null terminator
            size += len;
            count += 1;
        });

        (size, count)
    }

    Wasm::store_le(args_buf_size, inner::<T>().0);
    Wasm::store_le(args_count, inner::<T>().1);
    ERRNO_SUCCESS
}

/// Inner function for retrieving constant command-line arguments.
#[inline]
#[cfg(target_os = "wasi")]
pub fn args_get_const_inner<
    T: PrimitiveTraits<DATATYPE = VirtualArgsEmbeddedState>,
    Wasm: WasmAccess,
>(
    args: *mut *const u8,
    args_buf: *mut u8,
) -> Errno {
    let mut args = args;
    let mut args_buf = args_buf;

    const_for!(i in 0..T::__DATA.args.len() => {
        Wasm::store_le(args, args_buf as *const u8);

        Wasm::memcpy(args_buf, T::__DATA.args[i].as_bytes());
        Wasm::store_le(unsafe { args_buf.add(T::__DATA.args[i].len()) }, 0u8);

        args = unsafe { args.add(1) };
        args_buf = unsafe { args_buf.add(T::__DATA.args[i].len() + 1) };
    });

    ERRNO_SUCCESS
}

/// Trait for a virtual command-line arguments implementation.
pub trait VirtualArgs<'a> {
    /// The type used for argument strings.
    type Str: AsRef<str>;

    /// Returns the command-line arguments.
    fn get_args(&'a mut self) -> &'a [Self::Str];

    /// Returns the sizes of the command-line arguments.
    fn args_sizes_get(&'a mut self) -> (Size, Size) {
        let args = self.get_args();
        let mut size = 0;
        let mut count = 0;
        for arg in args {
            let len = arg.as_ref().len() + 1; // +1 for null terminator
            size += len;
            count += 1;
        }

        (size, count)
    }

    /// Copies the command-line arguments into the provided buffers.
    fn args_get<Wasm: WasmAccess + WasmAccessName + 'static>(
        &'a mut self,
        args: *mut *const u8,
        args_buf: *mut u8,
    ) -> Errno {
        let mut args = args;
        let mut args_buf = args_buf;

        for arg in self.get_args() {
            Wasm::store_le(args, args_buf as *const u8);

            Wasm::memcpy(args_buf, arg.as_ref().as_bytes());
            Wasm::store_le(unsafe { args_buf.add(arg.as_ref().len()) as *mut u8 }, 0u8);

            args = unsafe { args.add(1) };
            args_buf = unsafe { args_buf.add(arg.as_ref().len() + 1) };
        }

        ERRNO_SUCCESS
    }
}

impl<'a, T: core::ops::DerefMut<Target = U>, U: VirtualArgs<'a> + 'a> VirtualArgs<'a> for T {
    type Str = U::Str;

    fn get_args(&'a mut self) -> &'a [Self::Str] {
        self.deref_mut().get_args()
    }

    fn args_sizes_get(&'a mut self) -> (Size, Size) {
        self.deref_mut().args_sizes_get()
    }
}

/// Inner function for retrieving dynamic command-line argument sizes.
#[cfg(target_os = "wasi")]
pub fn args_sizes_get_inner<'a, Wasm: WasmAccess + WasmAccessName + 'static>(
    state: &'a mut impl VirtualArgs<'a>,
    args_count: *mut Size,
    args_buf_size: *mut Size,
) -> Errno {
    let (size, count) = state.args_sizes_get();

    Wasm::store_le(args_buf_size, size);
    Wasm::store_le(args_count, count);

    ERRNO_SUCCESS
}

/// Inner function for retrieving dynamic command-line arguments.
#[inline]
#[cfg(target_os = "wasi")]
pub fn args_get_inner<'a, Wasm: WasmAccess + WasmAccessName + 'static>(
    state: &'a mut impl VirtualArgs<'a>,
    args: *mut *const u8,
    args_buf: *mut u8,
) -> Errno {
    state.args_get::<Wasm>(args, args_buf)
}
