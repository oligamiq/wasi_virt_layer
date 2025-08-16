/// By entering the names of the files to be combined, a bridge for the combination is created.
/// You need to prepare as many Wasip1 instances on the virtual file system as the number of files to be combined.
#[macro_export]
macro_rules! import_wasm {
    ($name:ident) => {
        $crate::__private::paste::paste! {
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

            #[cfg(target_arch = "wasm32")]
            #[unsafe(no_mangle)]
            unsafe extern "C" fn [<__wasip1_vfs_ $name __start_wrap>]() {
                unsafe { [<__wasip1_vfs_ $name __start>]() };
            }

            $crate::__memory_director_import_etc!($name);

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
                        core::ptr::read(value.as_ptr() as *const T)
                    }
                }

                $crate::__memory_director_wasm_access!($name);

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

#[cfg(not(feature = "multi_memory"))]
#[macro_export]
macro_rules! __memory_director_wasm_access {
    ($name:ident) => {
        $crate::__private::paste::paste! {
            #[inline(always)]
            fn memory_director<T>(ptr: *const T) -> *const T {
                #[cfg(not(target_arch = "wasm32"))]
                unimplemented!("this is not supported on this architecture");

                #[cfg(target_arch = "wasm32")]
                unsafe { [<__wasip1_vfs_ $name _memory_director>](
                    ptr as isize,
                ) as *const T }
            }

            #[inline(always)]
            fn memory_director_mut<T>(ptr: *mut T) -> *mut T {
                #[cfg(not(target_arch = "wasm32"))]
                unimplemented!("this is not supported on this architecture");

                #[cfg(target_arch = "wasm32")]
                unsafe { [<__wasip1_vfs_ $name _memory_director>](
                    ptr as isize,
                ) as *mut T }
            }
        }
    };
}

#[cfg(feature = "multi_memory")]
#[macro_export]
macro_rules! __memory_director_wasm_access {
    ($_:ident) => {};
}

#[cfg(not(feature = "multi_memory"))]
#[macro_export]
macro_rules! __memory_director_import_etc {
    ($name:ident) => {
        $crate::__private::paste::paste! {
            #[cfg(target_arch = "wasm32")]
            #[unsafe(no_mangle)]
            unsafe extern "C" fn [<__wasip1_vfs_ $name _memory_trap_wrap>](
                _ptr: isize,
            ) -> isize {
                unsafe { [<__wasip1_vfs_ $name _memory_trap>](
                    _ptr,
                ) }
            }

            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "wasip1-vfs")]
            unsafe extern "C" {
                #[unsafe(no_mangle)]
                pub fn [<__wasip1_vfs_ $name _memory_trap>](
                    _ptr: isize,
                ) -> isize;

                #[unsafe(no_mangle)]
                pub fn [<__wasip1_vfs_ $name _memory_director>](ptr: isize) -> isize;
            }
        }
    };
}

#[cfg(feature = "multi_memory")]
#[macro_export]
macro_rules! __memory_director_import_etc {
    ($_:ident) => {};
}

#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
#[cfg(feature = "multi_memory")]
#[doc(hidden)]
unsafe extern "C" fn __wasip1_vfs_flag_vfs_multi_memory() {}

#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
#[cfg(not(feature = "multi_memory"))]
#[doc(hidden)]
unsafe extern "C" fn __wasip1_vfs_flag_vfs_single_memory() {}

#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
unsafe extern "C" fn __wasip1_vfs_flag_vfs_memory(ptr: *mut u8, src: *mut u8) {
    unsafe { core::ptr::copy_nonoverlapping(src, ptr, 1) };
}

pub struct WasmArrayAccess<T: core::fmt::Debug, Wasm: WasmAccess> {
    ptr: *const T,
    len: usize,
    __marker: core::marker::PhantomData<Wasm>,
}

impl<T: core::fmt::Debug, Wasm: WasmAccess> WasmArrayAccess<T, Wasm> {
    #[inline(always)]
    pub fn new(ptr: *const T, len: usize) -> Self {
        Self {
            ptr,
            len,
            __marker: core::marker::PhantomData,
        }
    }

    #[inline(always)]
    pub fn get(&self, index: usize) -> T {
        let ptr = unsafe { self.ptr.add(index) };
        Wasm::load_le(ptr)
    }

    #[inline(always)]
    pub fn iter(&self) -> WasmArrayAccessIterator<T, Wasm> {
        WasmArrayAccessIterator::new(self.ptr, self.len)
    }
}

impl<T: core::fmt::Debug, Wasm: WasmAccess> IntoIterator for WasmArrayAccess<T, Wasm> {
    type Item = T;
    type IntoIter = WasmArrayAccessIterator<T, Wasm>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct WasmArrayAccessIterator<T: core::fmt::Debug, Wasm: WasmAccess> {
    ptr: *const T,
    len: usize,
    __marker: core::marker::PhantomData<Wasm>,
}

impl<T: core::fmt::Debug, Wasm: WasmAccess> WasmArrayAccessIterator<T, Wasm> {
    pub fn new(ptr: *const T, len: usize) -> Self {
        Self {
            ptr,
            len,
            __marker: core::marker::PhantomData,
        }
    }
}

impl<T: core::fmt::Debug, Wasm: WasmAccess> Iterator for WasmArrayAccessIterator<T, Wasm> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        let item = Wasm::load_le(self.ptr);
        self.ptr = unsafe { self.ptr.add(1) };
        self.len -= 1;
        Some(item)
    }
}

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

pub trait WasmAccess {
    /// Copies data from the source pointer to the offset.
    fn memcpy<T>(offset: *mut T, data: &[T]);

    /// Copies data from the source pointer to the offset.
    fn memcpy_to<T>(offset: &mut [T], src: *const T);
    fn store_le<T>(offset: *mut T, value: T);
    fn load_le<T: core::fmt::Debug>(offset: *const T) -> T;

    /// utility internal
    fn as_array<T: core::fmt::Debug>(ptr: *const T, len: usize) -> WasmArrayAccess<T, Self>
    where
        Self: Sized,
    {
        WasmArrayAccess::new(ptr, len)
    }

    /// utility internal
    #[cfg(feature = "alloc")]
    fn get_array<T: core::fmt::Debug>(ptr: *const T, len: usize) -> Vec<T>
    where
        Self: Sized,
    {
        let mut vec = Vec::<T>::with_capacity(len);
        unsafe { vec.set_len(len) };
        Self::memcpy_to(&mut vec, ptr);
        vec
    }

    #[cfg(not(feature = "multi_memory"))]
    fn memory_director<T>(ptr: *const T) -> *const T;

    #[cfg(not(feature = "multi_memory"))]
    fn memory_director_mut<T>(ptr: *mut T) -> *mut T;

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
    /// to call this function after first _start.
    /// After this, _start must be called before main is called.
    fn reset();

    /// Calls the initialization function provided.
    /// If you are using the main function of the same TRAIT,
    /// RUST's main function will not be automatically executed during initialization.
    ///
    /// if you want to use reset, call this function first otherwise
    fn _start();
}
