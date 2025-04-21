use const_for::const_for;
use const_struct::*;
use wasip1::*;

use crate::memory::{MemoryAccess, MemoryAccessTypes};

/// @block or @through
/// Whether to import JavaScript runtime env from vfs,
/// @through if retrieving from JavaScript runtime.
#[macro_export]
macro_rules! export_env {
    (@inner, @const, $ty:ty, $wasm:ident) => {
        $crate::paste::paste! {
            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn [<$wasm _environ_sizes_get>](
                environ_count: *mut $crate::wasip1::Size,
                environ_buf_size: *mut $crate::wasip1::Size,
            ) -> $crate::wasip1::Errno {
                $crate::wasi::env::environ_sizes_get_const_inner::<$ty, $wasm>(environ_count, environ_buf_size)
            }

            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn [<$wasm _environ_get>](
                environ: *mut *const u8,
                environ_buf: *mut u8,
            ) -> $crate::wasip1::Errno {
                $crate::wasi::env::environ_get_const_inner::<$ty, $wasm>(environ, environ_buf)
            }
        }
    };

    (@inner, @static, $state:expr, $wasm:ident) => {
        $crate::paste::paste! {
            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn [<$wasm _environ_sizes_get>](
                environ_count: *mut $crate::wasip1::Size,
                environ_buf_size: *mut $crate::wasip1::Size,
            ) -> $crate::wasip1::Errno {
                let state = $state;
                $crate::wasi::env::environ_sizes_get_inner::<$wasm>(state, environ_count, environ_buf_size)
            }

            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn [<$wasm _environ_get>](
                environ: *mut *const u8,
                environ_buf: *mut u8,
            ) -> $crate::wasip1::Errno {
                let state = $state;
                $crate::wasi::env::environ_get_inner::<$wasm>(state, environ, environ_buf)
            }
        }
    };

    (@block, @const, $ty:ty, $wasm:ident) => {
        pub unsafe extern "C" fn __wasip1_vfs_block_environ() {}
        $crate::export_env!(@inner, @const, $ty, $wasm);
    };

    (@block, @static, $state:expr, $wasm:ident) => {
        pub unsafe extern "C" fn __wasip1_vfs_block_environ() {}
        $crate::export_env!(@inner, @static, $state, $wasm);
    };

    (@through, @const, $ty:ty, $wasm:ident) => {
        $crate::export_env!(@inner, @const, $ty, $wasm);
    };

    (@through, @static, $state:expr, $wasm:ident) => {
        $crate::export_env!(@inner, @static, $state, $wasm);
    };
}

#[const_struct]
pub struct VirtualEnvConstState {
    pub environ: &'static [&'static str],
}

#[inline]
pub fn environ_sizes_get_const_inner<
    T: PrimitiveTraits<DATATYPE = VirtualEnvConstState>,
    Wasm: MemoryAccess,
>(
    environ_count: *mut Size,
    environ_buf_size: *mut Size,
) -> Errno
where
    Size: MemoryAccessTypes<Wasm>,
{
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
pub fn environ_get_const_inner<
    T: PrimitiveTraits<DATATYPE = VirtualEnvConstState>,
    Wasm: MemoryAccess,
>(
    environ: *mut *const u8,
    environ_buf: *mut u8,
) -> Errno
where
    *const u8: MemoryAccessTypes<Wasm>,
    u8: MemoryAccessTypes<Wasm>,
{
    let mut environ = environ;
    let mut environ_buf = environ_buf;

    const_for!(i in 0..T::__DATA.environ.len() => {
        Wasm::store_le(environ, environ_buf);

        Wasm::memcpy(environ_buf, T::__DATA.environ[i].as_bytes());
        Wasm::store_le(unsafe { environ_buf.add(T::__DATA.environ[i].len()) }, 0);

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

    fn environ_get<Wasm: MemoryAccess>(
        &'a mut self,
        environ: *mut *const u8,
        environ_buf: *mut u8,
    ) -> Errno
    where
        *const u8: MemoryAccessTypes<Wasm>,
        u8: MemoryAccessTypes<Wasm>,
    {
        let mut environ = environ;
        let mut environ_buf = environ_buf;

        for env in self.get_environ() {
            Wasm::store_le(environ, environ_buf);

            Wasm::memcpy(environ_buf, env.as_ref().as_bytes());
            Wasm::store_le(unsafe { environ_buf.add(env.as_ref().len()) }, 0);

            environ = unsafe { environ.add(1) };
            environ_buf = unsafe { environ_buf.add(env.as_ref().len() + 1) };
        }

        ERRNO_SUCCESS
    }
}

impl<'a, T: std::ops::DerefMut<Target = U>, U: VirtualEnv<'a> + 'a> VirtualEnv<'a> for T {
    type Str = U::Str;

    fn get_environ(&'a mut self) -> &'a [Self::Str] {
        self.deref_mut().get_environ()
    }

    fn environ_sizes_get(&'a mut self) -> (Size, Size) {
        self.deref_mut().environ_sizes_get()
    }
}

pub fn environ_sizes_get_inner<'a, Wasm: MemoryAccess>(
    state: &'a mut impl VirtualEnv<'a>,
    environ_count: *mut Size,
    environ_buf_size: *mut Size,
) -> Errno
where
    Size: MemoryAccessTypes<Wasm>,
{
    let (size, count) = state.environ_sizes_get();

    Wasm::store_le(environ_buf_size, size);
    Wasm::store_le(environ_count, count);

    ERRNO_SUCCESS
}

#[inline]
pub fn environ_get_inner<'a, Wasm: MemoryAccess>(
    state: &'a mut impl VirtualEnv<'a>,
    environ: *mut *const u8,
    environ_buf: *mut u8,
) -> Errno
where
    *const u8: MemoryAccessTypes<Wasm>,
    u8: MemoryAccessTypes<Wasm>,
{
    state.environ_get(environ, environ_buf)
}
