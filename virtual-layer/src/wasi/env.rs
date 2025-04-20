use const_for::const_for;
use const_struct::*;
use wasip1::*;

#[macro_export]
macro_rules! export_env {
    ($ty:ty) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn environ_sizes_get(
            environc: &mut $crate::wasip1::Size,
            environ_buf_size: &mut $crate::wasip1::Size,
        ) -> $crate::wasip1::Errno {
            $crate::wasi::env::environ_sizes_get_const_inner::<$ty>(environc, environ_buf_size)
        }
    };

    ($state:ident) => {
        use ::wasip1::*;

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn environ_sizes_get(
            environc: &mut Size,
            environ_buf_size: &mut Size,
        ) -> Errno {
            environ_sizes_get_inner(&$state, environc, environ_buf_size)
        }
    };
}

#[const_struct]
pub struct VirtualEnvConstState {
    pub environ: &'static [&'static str],
}

#[inline]
pub const fn environ_sizes_get_const_inner<T: PrimitiveTraits<DATATYPE = VirtualEnvConstState>>(
    environc: &mut Size,
    environ_buf_size: &mut Size,
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

    *environ_buf_size = inner::<T>().0;
    *environc = inner::<T>().1;
    ERRNO_SUCCESS
}

// pub trait VirtualEnv {
//     fn get_environ(&self) -> ;
// }
