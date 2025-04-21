/// By entering the names of the files to be combined, a bridge for the combination is created.
/// You need to prepare as many Wasip1 instances on the virtual file system as the number of files to be combined.
#[macro_export]
macro_rules! import_wasm {
    ($name:ident) => {
        $crate::paste::paste! {
            #[allow(non_camel_case_types)]
            struct $name;

            $crate::import_wasm!(@store, $name, u8, usize);

            impl $crate::memory::MemoryAccess for $name {
                #[inline(always)]
                fn memcpy<T>(offset: *mut T, data: &[T])
                where
                    T: $crate::memory::MemoryAccessTypes<Self>,
                {
                    <T as $crate::memory::MemoryAccessTypes<Self>>::memcpy(offset, data);
                }

                #[inline(always)]
                fn store_le<T: $crate::memory::MemoryAccessTypes<Self>>(offset: *mut T, value: T)
                where
                    T: $crate::memory::MemoryAccessTypes<Self>,
                {
                    <T as $crate::memory::MemoryAccessTypes<Self>>::store_le(offset, value);
                }
            }
        }
    };

    (@store_inner, $name:ident, $ty:ty, $normal_ty:ty, $middle:tt) => {
        $crate::paste::paste! {
            impl $crate::memory::MemoryAccessTypes<$name> for $ty {
                #[inline(always)]
                fn memcpy(offset: *mut Self, data: &[Self]) {
                    unsafe { [<__wasip1_vfs_ $name _memory_copy $middle $normal_ty>](offset, data.as_ptr(), data.len()) };
                }

                #[inline(always)]
                fn store_le(offset: *mut Self, value: Self) {
                    unsafe { [<__wasip1_vfs_ $name _memory_store_le $middle $normal_ty>](offset, value) };
                }
            }

            #[doc(hidden)]
            #[cfg_attr(target_arch = "wasm32", link(wasm_import_module = "wasip1-vfs"))]
            unsafe extern "C" {
                /// https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Copy
                #[unsafe(no_mangle)]
                pub fn [<__wasip1_vfs_ $name _memory_copy $middle $normal_ty>](
                    offset: *mut $ty,
                    src: *const $ty,
                    len: usize,
                );

                #[unsafe(no_mangle)]
                pub fn [<__wasip1_vfs_ $name _memory_store_le $middle $normal_ty>](
                    offset: *mut $ty,
                    value: $ty,
                );
            }
        }
    };

    (@store, $name:ident, $($ty:ty),*) => {
        $(
            $crate::import_wasm!(@store_inner, $name, $ty, $ty, _);
            $crate::import_wasm!(@store_inner, $name, *const $ty, $ty, _const_ptr_);
            $crate::import_wasm!(@store_inner, $name, *mut $ty, $ty, _mut_ptr_);
        )*
    };
}

#[unsafe(no_mangle)]
#[doc(hidden)]
unsafe extern "C" fn __wasip1_vfs_flag_vfs_memory(ptr: *mut u8, src: *mut u8) {
    unsafe { core::ptr::copy_nonoverlapping(src, ptr, 1) };
}

pub trait MemoryAccess: Sized {
    fn memcpy<T: MemoryAccessTypes<Self>>(offset: *mut T, data: &[T]);
    fn store_le<T: MemoryAccessTypes<Self>>(offset: *mut T, value: T);
}

pub trait MemoryAccessTypes<T>: Sized {
    fn memcpy(offset: *mut Self, data: &[Self]);
    fn store_le(offset: *mut Self, value: Self);
}
