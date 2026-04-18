use crate::__private::wasip1;

/// By entering the names of the files to be combined, a bridge for the combination is created.
/// You need to prepare as many Wasip1 instances on the virtual file system as the number of files to be combined.
/// ```
/// use wasi_virt_layer::prelude::*;
///
/// import_wasm!(my_wasm);
/// ```
/// Macro for importing a WebAssembly module and generating its memory access bridge.
#[macro_export]
macro_rules! import_wasm {
    (anonymous) => {
        compile_error!("This name is reserved for internal use. Please choose another name for your import.");
    };
    (<anonymous>) => {
        import_wasm!(@inner, anonymous);
    };
    ($name:ident) => {
        import_wasm!(@inner, $name);
    };
    (@inner, $name:ident) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy)]
        struct $name;

        $crate::__private::paste::paste! {
            #[doc(hidden)]
            #[cfg(target_os = "wasi")]
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
                pub fn [<__wasip1_vfs_ $name ___main_void>]() -> $crate::__private::wasip1::Errno;

                #[unsafe(no_mangle)]
                pub fn [<__wasip1_vfs_ $name __start>]();

                #[unsafe(no_mangle)]
                pub fn [<__wasip1_vfs_ $name _reset>]();
            }

            #[cfg(target_os = "wasi")]
            #[unsafe(no_mangle)]
            unsafe extern "C" fn [<__wasip1_vfs_ $name __start_anchor>]() {
                unsafe { [<__wasip1_vfs_ $name __start>]() };
            }

            $crate::__memory_director_import_etc!($name);

            impl $crate::memory::WasmAccessName for $name {
                const NAME: &'static str = stringify!($name);
            }

            impl $crate::memory::WasmAccessDynCompatibleRaw for $name {
                #[inline(always)]
                fn memcpy_raw(&self, offset: *mut u8, data: *const u8, len: usize)
                {
                    #[cfg(not(target_os = "wasi"))]
                    unimplemented!("this is not supported on this architecture");

                    #[cfg(target_os = "wasi")]
                    unsafe { [<__wasip1_vfs_ $name _memory_copy_from>](
                        offset,
                        data,
                        len,
                    ) };
                }

                #[inline(always)]
                fn memcpy_to_raw(&self, offset: *mut u8, src: *const u8, len: usize)
                {
                    #[cfg(not(target_os = "wasi"))]
                    unimplemented!("this is not supported on this architecture");

                    #[cfg(target_os = "wasi")]
                    unsafe { [<__wasip1_vfs_ $name _memory_copy_to>](
                        offset,
                        src,
                        len,
                    ) };
                }

                $crate::__memory_director_wasm_access!(@dyn, $name);

                #[inline(always)]
                fn _main_raw(&self) -> $crate::__private::wasip1::Errno
                {
                    #[cfg(not(target_os = "wasi"))]
                    unimplemented!("this is not supported on this architecture");

                    #[cfg(target_os = "wasi")]
                    unsafe { [<__wasip1_vfs_ $name ___main_void>]() }
                }

                #[inline(always)]
                fn _reset_raw(&self)
                {
                    #[cfg(not(target_os = "wasi"))]
                    unimplemented!("this is not supported on this architecture");

                    #[cfg(target_os = "wasi")]
                    unsafe { [<__wasip1_vfs_ $name _reset>]() };
                }

                #[inline(always)]
                fn _start_raw(&self)
                {
                    #[cfg(not(target_os = "wasi"))]
                    unimplemented!("this is not supported on this architecture");

                    #[cfg(target_os = "wasi")]
                    unsafe { [<__wasip1_vfs_ $name __start>]() };
                }
            }

            impl $crate::memory::WasmAccessRaw for $name {
                #[inline(always)]
                fn memcpy_raw(offset: *mut u8, data: *const u8, len: usize)
                {
                    #[cfg(not(target_os = "wasi"))]
                    unimplemented!("this is not supported on this architecture");

                    #[cfg(target_os = "wasi")]
                    unsafe { [<__wasip1_vfs_ $name _memory_copy_from>](
                        offset,
                        data,
                        len,
                    ) };
                }

                #[inline(always)]
                fn memcpy_to_raw(offset: *mut u8, src: *const u8, len: usize)
                {
                    #[cfg(not(target_os = "wasi"))]
                    unimplemented!("this is not supported on this architecture");

                    #[cfg(target_os = "wasi")]
                    unsafe { [<__wasip1_vfs_ $name _memory_copy_to>](
                        offset,
                        src,
                        len,
                    ) };
                }

                $crate::__memory_director_wasm_access!(@const, $name);

                #[inline(always)]
                fn _main_raw() -> $crate::__private::wasip1::Errno
                {
                    #[cfg(not(target_os = "wasi"))]
                    unimplemented!("this is not supported on this architecture");

                    #[cfg(target_os = "wasi")]
                    unsafe { [<__wasip1_vfs_ $name ___main_void>]() }
                }

                #[inline(always)]
                fn _reset_raw() {
                    #[cfg(not(target_os = "wasi"))]
                    unimplemented!("this is not supported on this architecture");

                    #[cfg(target_os = "wasi")]
                    unsafe { [<__wasip1_vfs_ $name _reset>]() };
                }

                #[inline(always)]
                fn _start_raw() {
                    #[cfg(not(target_os = "wasi"))]
                    unimplemented!("this is not supported on this architecture");

                    #[cfg(target_os = "wasi")]
                    unsafe { [<__wasip1_vfs_ $name __start>]() };
                }
            }
        }
    };
}

#[cfg(not(feature = "multi_memory"))]
#[macro_export]
macro_rules! __memory_director_wasm_access {
    (@dyn, $name:ident) => {
        $crate::__private::paste::paste! {
            #[inline(always)]
            fn memory_director_raw(&self, ptr: isize) -> isize {
                #[cfg(not(target_os = "wasi"))]
                unimplemented!("this is not supported on this architecture");

                #[cfg(target_os = "wasi")]
                unsafe { [<__wasip1_vfs_ $name _memory_director>](
                    ptr,
                ) }
            }
        }
    };

    (@const, $name:ident) => {
        $crate::__private::paste::paste! {
            #[inline(always)]
            const fn memory_director_raw(ptr: isize) -> isize {
                #[cfg(not(target_os = "wasi"))]
                unimplemented!("this is not supported on this architecture");

                #[cfg(target_os = "wasi")]
                unsafe { [<__wasip1_vfs_ $name _memory_director>](
                    ptr,
                ) }
            }
        }
    };
}

#[cfg(feature = "multi_memory")]
#[macro_export]
macro_rules! __memory_director_wasm_access {
    (@dyn, $_:ident) => {};

    (@const, $_:ident) => {};
}

#[cfg(not(feature = "multi_memory"))]
#[macro_export]
macro_rules! __memory_director_import_etc {
    ($name:ident) => {
        $crate::__private::paste::paste! {
            #[cfg(target_os = "wasi")]
            #[unsafe(no_mangle)]
            unsafe extern "C" fn [<__wasip1_vfs_ $name _memory_trap_anchor>](
                _ptr: isize,
            ) -> isize {
                unsafe { [<__wasip1_vfs_ $name _memory_trap>](
                    _ptr,
                ) }
            }

            #[doc(hidden)]
            #[cfg(target_os = "wasi")]
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
#[cfg(target_os = "wasi")]
#[cfg(feature = "multi_memory")]
#[doc(hidden)]
unsafe extern "C" fn __wasip1_vfs_flag_vfs_multi_memory() {}

#[unsafe(no_mangle)]
#[cfg(target_os = "wasi")]
#[cfg(not(feature = "multi_memory"))]
#[doc(hidden)]
unsafe extern "C" fn __wasip1_vfs_flag_vfs_single_memory() {}

#[unsafe(no_mangle)]
#[cfg(target_os = "wasi")]
#[doc(hidden)]
unsafe extern "C" fn __wasip1_vfs_flag_vfs_memory(ptr: *mut u8, src: *mut u8) {
    unsafe { core::ptr::copy_nonoverlapping(src, ptr, 1) };
}

/// Provides access to an array in WASM memory.
#[derive(Debug)]
pub struct WasmArrayAccess<'a, T: core::fmt::Debug + Copy, Wasm: WasmAccess> {
    ptr: *const T,
    len: usize,
    __marker: core::marker::PhantomData<&'a ()>,
    __marker_wasm: core::marker::PhantomData<Wasm>,
}

impl<'a, T: core::fmt::Debug + Copy, Wasm: WasmAccess> Clone for WasmArrayAccess<'a, T, Wasm> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T: core::fmt::Debug + Copy, Wasm: WasmAccess> Copy for WasmArrayAccess<'a, T, Wasm> {}

impl<'a, T: core::fmt::Debug + Copy, Wasm: WasmAccess> WasmArrayAccess<'a, T, Wasm> {
    /// Creates a new `WasmArrayAccess`.
    #[inline(always)]
    pub fn new(ptr: *const T, len: usize) -> Self {
        {
            Self {
                ptr,
                len,
                __marker: core::marker::PhantomData,
                __marker_wasm: core::marker::PhantomData,
            }
        }
    }

    /// Retrieves the element at the given index.
    #[inline(always)]
    pub fn get(&self, index: usize) -> T {
        {
            let ptr = unsafe { self.ptr.add(index) };
            Wasm::load_le(ptr)
        }
    }

    /// Returns an iterator over the array elements.
    #[inline(always)]
    pub fn iter(&self) -> WasmArrayAccessIterator<T, Wasm> {
        WasmArrayAccessIterator::new(self.ptr, self.len)
    }

    /// Returns the number of elements in the array.
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.len
    }
}

impl<'a, T: core::fmt::Debug + Copy + PartialEq, Wasm: WasmAccess> PartialEq
    for WasmArrayAccess<'a, T, Wasm>
{
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && (0..self.len).all(|i| self.get(i) == other.get(i))
    }
}

impl<'a, T: core::fmt::Debug + Copy, Wasm: WasmAccess> IntoIterator
    for WasmArrayAccess<'a, T, Wasm>
{
    type Item = T;
    type IntoIter = WasmArrayAccessIterator<T, Wasm>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator over elements in WASM memory.
pub struct WasmArrayAccessIterator<T: core::fmt::Debug + Copy, Wasm: WasmAccess> {
    ptr: *const T,
    len: usize,
    __marker: core::marker::PhantomData<Wasm>,
}

impl<T: core::fmt::Debug + Copy, Wasm: WasmAccess> WasmArrayAccessIterator<T, Wasm> {
    /// Creates a new `WasmArrayAccessIterator`.
    pub fn new(ptr: *const T, len: usize) -> Self {
        Self {
            ptr,
            len,
            __marker: core::marker::PhantomData,
        }
    }
}

impl<T: core::fmt::Debug + Copy, Wasm: WasmAccess> Iterator for WasmArrayAccessIterator<T, Wasm> {
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

/// A mutable iterator over elements in WASM memory.
pub struct WasmArrayAccessMutIterator<T: core::fmt::Debug + Copy, Wasm: WasmAccess> {
    ptr: *mut T,
    len: usize,
    __marker: core::marker::PhantomData<Wasm>,
}

impl<T: core::fmt::Debug + Copy, Wasm: WasmAccess> WasmArrayAccessMutIterator<T, Wasm> {
    /// Creates a new `WasmArrayAccessMutIterator`.
    pub fn new(ptr: *mut T, len: usize) -> Self {
        Self {
            ptr,
            len,
            __marker: core::marker::PhantomData,
        }
    }
}

/// A component representing a single mutable element in WASM memory.
pub struct WasmArrayAccessMutIteratorComponent<T: core::fmt::Debug + Copy, Wasm: WasmAccess> {
    ptr: *mut T,
    __marker: core::marker::PhantomData<Wasm>,
}

impl<T: core::fmt::Debug + Copy, Wasm: WasmAccess> WasmArrayAccessMutIteratorComponent<T, Wasm> {
    /// Creates a new `WasmArrayAccessMutIteratorComponent`.
    pub fn new(ptr: *mut T) -> Self {
        Self {
            ptr,
            __marker: core::marker::PhantomData,
        }
    }

    /// Sets the value at the current position.
    pub fn set(&self, value: T) {
        Wasm::store_le(self.ptr, value);
    }
}

impl<T: core::fmt::Debug + Copy, Wasm: WasmAccess> Iterator
    for WasmArrayAccessMutIterator<T, Wasm>
{
    type Item = WasmArrayAccessMutIteratorComponent<T, Wasm>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        let component = WasmArrayAccessMutIteratorComponent::new(self.ptr);
        self.ptr = unsafe { self.ptr.add(1) };
        self.len -= 1;
        Some(component)
    }
}

pub trait WasmAccessName: core::fmt::Debug {
    const NAME: &'static str;
}

pub trait WasmAccessNameDynCompatible: core::fmt::Debug {
    fn name(&self) -> &'static str;
}

impl<T: WasmAccessName> WasmAccessNameDynCompatible for T {
    fn name(&self) -> &'static str {
        Self::NAME
    }
}

pub trait WasmAccessDynCompatibleRaw: core::fmt::Debug {
    /// Copies a slice of data into WASM memory starting at the given offset.
    fn memcpy_raw(&self, offset: *mut u8, src: *const u8, len: usize);

    /// Copies data from the source pointer into the provided mutable slice of WASM memory.
    fn memcpy_to_raw(&self, offset: *mut u8, src: *const u8, len: usize);

    #[cfg(not(feature = "multi_memory"))]
    fn memory_director_raw(&self, ptr: isize) -> isize;

    fn _main_raw(&self) -> wasip1::Errno;

    fn _reset_raw(&self);

    fn _start_raw(&self);
}

pub trait WasmAccessDynCompatible: WasmAccessDynCompatibleRaw {
    fn memcpy_with<T>(&self, offset: *mut T, data: &[T]);

    fn memcpy_to_with<T>(&self, offset: &mut [T], src: *const T);

    fn store_le_with<T>(&self, offset: *mut T, value: T);

    fn load_le_with<T: core::fmt::Debug + Copy>(&self, offset: *const T) -> T;

    fn as_array_with<T: core::fmt::Debug + Copy>(&self, ptr: *const T, len: usize) -> WasmArrayAccessDynCompatible<T, Self>
    where Self: Sized
    {
        WasmArrayAccessDynCompatible::new(self, ptr, len)
    }

    #[cfg(feature = "alloc")]
    fn get_array_with<T: core::fmt::Debug + Copy>(&self, ptr: *const T, len: usize) -> alloc::boxed::Box<[T]>
    where Self: Sized
    {
        use crate::utils::alloc_buff;

        let (buff, _) = unsafe {
            alloc_buff(len, |b| {
                self.memcpy_to_with(b, ptr);
            })
        };

        buff
    }

    #[cfg(not(feature = "multi_memory"))]
    fn memory_director_with<T>(&self, ptr: *const T) -> *const T;

    #[cfg(not(feature = "multi_memory"))]
    fn memory_director_mut_with<T>(&self, ptr: *mut T) -> *mut T;

    fn _main_with(&self) -> wasip1::Errno;

    fn _reset_with(&self);

    fn _start_with(&self);
}

impl<T: AsRef<dyn WasmAccessDynCompatibleRaw> + core::fmt::Debug> WasmAccessDynCompatibleRaw for T {
    fn memcpy_raw(&self, offset: *mut u8, src: *const u8, len: usize) {
        self.as_ref().memcpy_raw(offset, src, len);
    }

    fn memcpy_to_raw(&self, offset: *mut u8, src: *const u8, len: usize) {
        self.as_ref().memcpy_to_raw(offset, src, len);
    }

    #[cfg(not(feature = "multi_memory"))]
    fn memory_director_raw(&self, ptr: isize) -> isize {
        self.as_ref().memory_director_raw(ptr)
    }

    fn _main_raw(&self) -> wasip1::Errno {
        self.as_ref()._main_raw()
    }

    fn _reset_raw(&self) {
        self.as_ref()._reset_raw();
    }

    fn _start_raw(&self) {
        self.as_ref()._start_raw();
    }
}

impl<T: WasmAccessDynCompatibleRaw> WasmAccessDynCompatible for T {
    fn memcpy_with<U>(&self, offset: *mut U, data: &[U]) {
        Self::memcpy_raw.memcpy_upper_with(self, offset, data);
    }

    fn memcpy_to_with<U>(&self, offset: &mut [U], src: *const U) {
        Self::memcpy_to_raw.memcpy_to_upper_with(self, offset, src);
    }

    fn store_le_with<U>(&self, offset: *mut U, value: U) {
        Self::memcpy_raw.store_le_upper_with(self, offset, value);
    }

    fn load_le_with<U: core::fmt::Debug + Copy>(&self, offset: *const U) -> U {
        Self::memcpy_to_raw.load_le_upper_with(self, offset)
    }

    #[cfg(not(feature = "multi_memory"))]
    fn memory_director_with<U>(&self, ptr: *const U) -> *const U {
        Self::memory_director_raw(self, ptr as isize) as *const U
    }

    #[cfg(not(feature = "multi_memory"))]
    fn memory_director_mut_with<U>(&self, ptr: *mut U) -> *mut U {
        Self::memory_director_raw(self, ptr as isize) as *mut U
    }

    fn _main_with(&self) -> wasip1::Errno {
        Self::_main_raw(self)
    }

    fn _reset_with(&self) {
        Self::_reset_raw(self);
    }

    fn _start_with(&self) {
        Self::_start_raw(self);
    }
}

pub trait WasmAccessRaw: core::fmt::Debug {
    fn memcpy_raw(offset: *mut u8, src: *const u8, len: usize);

    fn memcpy_to_raw(offset: *mut u8, src: *const u8, len: usize);

    #[cfg(not(feature = "multi_memory"))]
    fn memory_director_raw(ptr: isize) -> isize;

    fn _main_raw() -> wasip1::Errno;

    fn _reset_raw();

    fn _start_raw();
}

pub trait WasmAccessMemoryUtilUpper {
    /// Require `memcpy_raw` to be implemented.
    fn memcpy_upper<U>(self, offset: *mut U, data: &[U])
    where
        Self: FnOnce(*mut u8, *const u8, usize) + Sized,
    {
        (|_: (), offset, data, len| {
            self(offset, data, len);
        })
        .memcpy_upper_with_inline((), offset, data);
    }

    /// Require `memcpy_raw` to be implemented.
    fn memcpy_upper_with<U, T>(self, t: T, offset: *mut U, data: &[U])
    where
        Self: FnOnce(T, *mut u8, *const u8, usize) + Sized,
    {
        self.memcpy_upper_with_inline(t, offset, data);
    }

    #[inline(always)]
    fn memcpy_upper_with_inline<U, T>(self, t: T, offset: *mut U, data: &[U])
    where
        Self: FnOnce(T, *mut u8, *const u8, usize) + Sized,
    {
        self(
            t,
            offset as *mut u8,
            data.as_ptr() as *const u8,
            core::mem::size_of::<U>() * data.len(),
        );
    }

    /// Require `memcpy_to_raw` to be implemented.
    fn memcpy_to_upper<U>(self, offset: &mut [U], src: *const U)
    where
        Self: FnOnce(*mut u8, *const u8, usize) + Sized,
    {
        (|_: (), offset, src, len| {
            self(offset, src, len);
        })
        .memcpy_to_upper_with_inline((), offset, src);
    }

    /// Require `memcpy_to_raw` to be implemented.
    fn memcpy_to_upper_with<U, T>(self, t: T, offset: &mut [U], src: *const U)
    where
        Self: FnOnce(T, *mut u8, *const u8, usize) + Sized,
    {
        self.memcpy_to_upper_with_inline(t, offset, src);
    }

    #[inline(always)]
    fn memcpy_to_upper_with_inline<U, T>(self, t: T, offset: &mut [U], src: *const U)
    where
        Self: FnOnce(T, *mut u8, *const u8, usize) + Sized,
    {
        self(
            t,
            offset.as_mut_ptr() as *mut u8,
            src as *const u8,
            core::mem::size_of::<U>() * offset.len(),
        );
    }

    /// Require `memcpy_raw` to be implemented.
    fn store_le_upper<U>(self, offset: *mut U, value: U)
    where
        Self: FnOnce(*mut u8, *const u8, usize) + Sized,
    {
        (|_: (), offset, value, len| {
            self(offset, value, len);
        })
        .store_le_upper_with_inline((), offset, value);
    }

    /// Require `memcpy_raw` to be implemented.
    fn store_le_upper_with<U, T>(self, t: T, offset: *mut U, value: U)
    where
        Self: FnOnce(T, *mut u8, *const u8, usize) + Sized,
    {
        self.store_le_upper_with_inline(t, offset, value);
    }

    #[inline(always)]
    fn store_le_upper_with_inline<U, T>(self, t: T, offset: *mut U, value: U)
    where
        Self: FnOnce(T, *mut u8, *const u8, usize) + Sized,
    {
        self(
            t,
            offset as *mut u8,
            &value as *const U as *const u8,
            core::mem::size_of::<U>(),
        );
    }

    /// Require `memcpy_to_raw` to be implemented.
    fn load_le_upper<U: core::fmt::Debug + Copy>(self, offset: *const U) -> U
    where
        Self: FnOnce(*mut u8, *const u8, usize) + Sized,
    {
        (|_: (), offset, src, len| self(offset, src, len)).load_le_upper_with_inline((), offset)
    }

    /// Require `memcpy_to_raw` to be implemented.
    fn load_le_upper_with<U: core::fmt::Debug + Copy, T>(self, t: T, offset: *const U) -> U
    where
        Self: FnOnce(T, *mut u8, *const u8, usize) + Sized,
    {
        self.load_le_upper_with_inline(t, offset)
    }

    #[inline(always)]
    fn load_le_upper_with_inline<U: core::fmt::Debug + Copy, T>(self, t: T, offset: *const U) -> U
    where
        Self: FnOnce(T, *mut u8, *const u8, usize) + Sized,
    {
        let mut value = core::mem::MaybeUninit::<U>::uninit();
        self(
            t,
            value.as_mut_ptr() as *mut u8,
            offset as *const u8,
            core::mem::size_of::<U>(),
        );
        unsafe { core::ptr::read(value.as_ptr() as *const U) }
    }
}

impl<T> WasmAccessMemoryUtilUpper for T {}

impl<T: WasmAccessRaw> WasmAccess for T {
    fn memcpy<U>(offset: *mut U, data: &[U]) {
        Self::memcpy_raw.memcpy_upper(offset, data);
    }

    fn memcpy_to<U>(offset: &mut [U], src: *const U) {
        Self::memcpy_to_raw.memcpy_to_upper(offset, src);
    }

    fn store_le<U>(offset: *mut U, value: U) {
        Self::memcpy_raw.store_le_upper(offset, value);
    }

    fn load_le<U: core::fmt::Debug + Copy>(offset: *const U) -> U {
        Self::memcpy_to_raw.load_le_upper(offset)
    }

    #[cfg(not(feature = "multi_memory"))]
    fn memory_director<U>(ptr: *const U) -> *const U {
        Self::memory_director_raw(ptr as isize) as *const U
    }

    #[cfg(not(feature = "multi_memory"))]
    fn memory_director_mut<U>(ptr: *mut U) -> *mut U {
        Self::memory_director_raw(ptr as isize) as *mut U
    }

    fn _main() -> wasip1::Errno {
        Self::_main_raw()
    }

    fn _reset() {
        Self::_reset_raw();
    }

    fn _start() {
        Self::_start_raw();
    }
}

pub trait WasmAccess: WasmAccessRaw {
    /// Copies a slice of data into WASM memory starting at the given offset.
    fn memcpy<T>(offset: *mut T, data: &[T]);

    /// Copies data from the source pointer into the provided mutable slice of WASM memory.
    fn memcpy_to<T>(offset: &mut [T], src: *const T);
    /// Stores a value in WASM memory at the given offset using little-endian encoding.
    fn store_le<T>(offset: *mut T, value: T);
    /// Loads a value from WASM memory at the given offset using little-endian encoding.
    fn load_le<T: core::fmt::Debug + Copy>(offset: *const T) -> T;

    /// Helper method to create a `WasmArrayAccess` for the given pointer and length.
    fn as_array<'a, T: core::fmt::Debug + Copy>(
        ptr: *const T,
        len: usize,
    ) -> WasmArrayAccess<'a, T, Self>
    where
        Self: Sized,
    {
        WasmArrayAccess::new(ptr, len)
    }

    /// Returns a box containing the data from the WASM array.
    #[cfg(feature = "alloc")]
    fn get_array<T: core::fmt::Debug>(ptr: *const T, len: usize) -> alloc::boxed::Box<[T]>
    where
        Self: Sized,
    {
        use crate::utils::alloc_buff;

        let (buff, _) = unsafe {
            alloc_buff(len, |b| {
                Self::memcpy_to(b, ptr);
            })
        };
        buff
    }

    /// Directs a pointer to its mapped address in a single-memory model.
    #[cfg(not(feature = "multi_memory"))]
    fn memory_director<T>(ptr: *const T) -> *const T;

    /// Directs a mutable pointer to its mapped address in a single-memory model.
    #[cfg(not(feature = "multi_memory"))]
    fn memory_director_mut<T>(ptr: *mut T) -> *mut T;

    /// wrapping wasm's _start function
    /// By default in Rust code, when _start is called,
    /// the main function is executed.
    /// If you wish to call it again,
    /// you must use the __main_void function.
    /// When you write code that explicitly calls this function,
    /// the command-line tool (wasi_virt_layer-cli) detects it,
    /// and as a result,
    /// __main_void will no longer be invoked from within _start.
    /// Instead, it can only be called through the function
    /// that wraps __main_void.
    /// This does not apply if it's used as a library.
    ///
    /// Using this and plug_env,
    /// it is possible to override arguments, for example, to call
    /// If this function name is main, same name with rust generated main on test and error
    fn _main() -> wasip1::Errno;

    /// memory reset to memory which instantiate
    /// function's roll
    /// - memory fill zeroed
    /// - reset global variables
    /// - memory copied from data-segment
    /// if you call this function,
    /// virtual file system's memory isn't changed
    fn _reset();

    /// Calls the initialization function provided.
    /// If you are using the _main function of the same TRAIT,
    /// RUST's main function will not be automatically executed during initialization.
    ///
    /// Examples include:
    /// ```ignore
    /// import_wasm!(my_wasm);
    ///
    /// fn main() {
    ///   my_wasm::_reset();
    ///   my_wasm::_start();
    ///   my_wasm::_main();
    /// }
    /// ```
    /// Other:
    /// ```ignore
    /// import_wasm!(my_wasm);
    ///
    /// fn init() {
    ///   my_wasm::_reset();
    ///   my_wasm::_start();
    /// }
    /// fn main() {
    ///   my_wasm::_main();
    /// }
    fn _start();
}

/// Provides access to a file path in WASM memory.
#[derive(Debug)]
pub struct WasmPathAccess<'a, Wasm: WasmAccess> {
    path: WasmArrayAccess<'a, u8, Wasm>,
}

impl<'a, Wasm: WasmAccess> PartialEq for WasmPathAccess<'a, Wasm> {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl<'a, Wasm: WasmAccess> Clone for WasmPathAccess<'a, Wasm> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, Wasm: WasmAccess> Copy for WasmPathAccess<'a, Wasm> {}

impl<'a, Wasm: WasmAccess> WasmPathAccess<'a, Wasm> {
    /// Creates a new `WasmPathAccess`.
    #[inline(always)]
    pub fn new(ptr: *const u8, len: usize) -> Self {
        Self {
            path: WasmArrayAccess::new(ptr, len),
        }
    }

    /// Returns an iterator over the components of the path.
    #[inline(always)]
    pub fn components(&self) -> WasmPathComponents<'a, Wasm> {
        let path = self.path;
        WasmPathComponents { path }
    }
}

/// An iterator over the components of a WASM path.
pub struct WasmPathComponents<'a, Wasm: WasmAccess> {
    path: WasmArrayAccess<'a, u8, Wasm>,
}

pub trait WasmPathComponentCommon: core::fmt::Debug + Copy + PartialEq {
    fn as_root_dir(&self) -> bool;

    fn as_cur_dir(&self) -> bool;

    fn as_parent_dir(&self) -> bool;

    fn as_normal(&self) -> Option<impl IntoIterator<Item = u8> + Clone + '_>;
}

pub trait WasmPathAccessCommon: core::fmt::Debug {
    fn components_common<'a>(&'a self) -> impl Iterator<Item = impl WasmPathComponentCommon + 'a> + 'a;
}

impl<'a, Wasm: WasmAccess> WasmPathComponentCommon for WasmPathComponent<'a, Wasm> {
    fn as_root_dir(&self) -> bool {
        matches!(self, WasmPathComponent::RootDir)
    }

    fn as_cur_dir(&self) -> bool {
        matches!(self, WasmPathComponent::CurDir)
    }

    fn as_parent_dir(&self) -> bool {
        matches!(self, WasmPathComponent::ParentDir)
    }

    fn as_normal(&self) -> Option<impl IntoIterator<Item = u8> + Clone + '_> {
        match self {
            WasmPathComponent::Normal(access) => Some(*access),
            _ => None,
        }
    }
}

impl<'a, Wasm: WasmAccess> WasmPathAccessCommon for WasmPathAccess<'a, Wasm> {
    fn components_common<'b>(&'b self) -> impl Iterator<Item = impl WasmPathComponentCommon + 'b> + 'b {
        self.components()
    }
}

/// A component of a WASM path.
#[derive(Debug)]
pub enum WasmPathComponent<'a, Wasm: WasmAccess> {
    /// The root directory, `/`.
    RootDir,

    /// A reference to the current directory, `.`.
    CurDir,

    /// A reference to the parent directory, `..`.
    ParentDir,

    /// A normal file or directory name.
    Normal(WasmArrayAccess<'a, u8, Wasm>),
}

impl<'a, Wasm: WasmAccess> Clone for WasmPathComponent<'a, Wasm> {
    fn clone(&self) -> Self {
        match self {
            WasmPathComponent::RootDir => WasmPathComponent::RootDir,
            WasmPathComponent::CurDir => WasmPathComponent::CurDir,
            WasmPathComponent::ParentDir => WasmPathComponent::ParentDir,
            WasmPathComponent::Normal(access) => WasmPathComponent::Normal(*access),
        }
    }
}

impl<'a, Wasm: WasmAccess> Copy for WasmPathComponent<'a, Wasm> {}

impl<'a, Wasm: WasmAccess> PartialEq for WasmPathComponent<'a, Wasm> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (WasmPathComponent::RootDir, WasmPathComponent::RootDir) => true,
            (WasmPathComponent::CurDir, WasmPathComponent::CurDir) => true,
            (WasmPathComponent::ParentDir, WasmPathComponent::ParentDir) => true,
            (WasmPathComponent::Normal(a), WasmPathComponent::Normal(b)) => a == b,
            _ => false,
        }
    }
}

impl<'a, Wasm: WasmAccess> WasmPathComponent<'a, Wasm> {
    /// Compares the component with a string.
    pub fn eq_str(&self, other: &str) -> bool {
        match self {
            WasmPathComponent::RootDir => other == "/",
            WasmPathComponent::CurDir => other == ".",
            WasmPathComponent::ParentDir => other == "..",
            WasmPathComponent::Normal(access) => {
                access.len == other.len()
                    && (0..access.len).all(|i| access.get(i) == other.as_bytes()[i])
            }
        }
    }
}

impl<'a, Wasm: WasmAccess> Iterator for WasmPathComponents<'a, Wasm> {
    type Item = WasmPathComponent<'a, Wasm>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.path.len == 0 {
            return None;
        }

        if self.path.get(0) == b'/' {
            let mut index = 1;
            while index < self.path.len && self.path.get(index) == b'/' {
                index += 1;
            }

            self.path.ptr = unsafe { self.path.ptr.add(index) };
            self.path.len -= index;
            return Some(WasmPathComponent::RootDir);
        }

        if self.path.get(0) == b'.' {
            if self.path.len == 1 {
                self.path.len = 0;
                return Some(WasmPathComponent::CurDir);
            }

            let second = self.path.get(1);

            if second == b'/' {
                let mut index = 2;
                while index < self.path.len && self.path.get(index) == b'/' {
                    index += 1;
                }
                self.path.ptr = unsafe { self.path.ptr.add(index) };
                self.path.len -= index;
                return Some(WasmPathComponent::CurDir);
            } else if second == b'.' {
                if self.path.len == 2 {
                    self.path.len = 0;
                    return Some(WasmPathComponent::ParentDir);
                }

                let third = self.path.get(2);
                if third == b'/' {
                    let mut index = 3;
                    while index < self.path.len && self.path.get(index) == b'/' {
                        index += 1;
                    }
                    self.path.ptr = unsafe { self.path.ptr.add(index) };
                    self.path.len -= index;
                    return Some(WasmPathComponent::ParentDir);
                }

                let mut end = 3;
                while end < self.path.len && self.path.get(end) != b'/' {
                    end += 1;
                }

                let component = WasmArrayAccess::new(self.path.ptr, end);

                while end < self.path.len && self.path.get(end) == b'/' {
                    end += 1;
                }
                self.path.ptr = unsafe { self.path.ptr.add(end) };
                self.path.len -= end;
                return Some(WasmPathComponent::Normal(component));
            }
        } else {
            let mut end = 0;
            while end < self.path.len && self.path.get(end) != b'/' {
                end += 1;
            }

            let component = WasmArrayAccess::new(self.path.ptr, end);

            while end < self.path.len && self.path.get(end) == b'/' {
                end += 1;
            }
            self.path.ptr = unsafe { self.path.ptr.add(end) };
            self.path.len -= end;
            return Some(WasmPathComponent::Normal(component));
        }

        None
    }
}

/// Provides access to an array in WASM memory.
#[derive(Debug)]
pub struct WasmArrayAccessDynCompatible<'a, 'b, T: core::fmt::Debug + Copy, Wasm: WasmAccessDynCompatible> {
    access: &'b Wasm,
    ptr: *const T,
    len: usize,
    __marker: core::marker::PhantomData<&'a ()>,
}

impl<'a, 'b, T: core::fmt::Debug + Copy, Wasm: WasmAccessDynCompatible> Clone for WasmArrayAccessDynCompatible<'a, 'b, T, Wasm> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, 'b, T: core::fmt::Debug + Copy, Wasm: WasmAccessDynCompatible> Copy for WasmArrayAccessDynCompatible<'a, 'b, T, Wasm> {}

impl<'a, 'b, T: core::fmt::Debug + Copy, Wasm: WasmAccessDynCompatible> WasmArrayAccessDynCompatible<'a, 'b, T, Wasm> {
    /// Creates a new `WasmArrayAccessDynCompatible`.
    #[inline(always)]
    pub fn new(access: &'b Wasm, ptr: *const T, len: usize) -> Self {
        Self {
            access,
            ptr,
            len,
            __marker: core::marker::PhantomData,
        }
    }

    /// Retrieves the element at the given index.
    #[inline(always)]
    pub fn get(&self, index: usize) -> T {
        let offset = unsafe { self.ptr.add(index) };
        self.access.load_le_with(offset)
    }

    /// Returns an iterator over the array elements.
    #[inline(always)]
    pub fn iter(&self) -> WasmArrayAccessDynCompatibleIterator<'b, T, Wasm> {
        WasmArrayAccessDynCompatibleIterator::new(self.access, self.ptr, self.len)
    }

    /// Returns the number of elements in the array.
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.len
    }
}

impl<'a, 'b, T: core::fmt::Debug + Copy + PartialEq, Wasm: WasmAccessDynCompatible> PartialEq
    for WasmArrayAccessDynCompatible<'a, 'b, T, Wasm>
{
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && (0..self.len).all(|i| self.get(i) == other.get(i))
    }
}

impl<'a, 'b, T: core::fmt::Debug + Copy, Wasm: WasmAccessDynCompatible> IntoIterator
    for WasmArrayAccessDynCompatible<'a, 'b, T, Wasm>
{
    type Item = T;
    type IntoIter = WasmArrayAccessDynCompatibleIterator<'b, T, Wasm>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct WasmArrayAccessDynCompatibleIterator<'c, T: core::fmt::Debug + Copy, Wasm: WasmAccessDynCompatible> {
    access: &'c Wasm,
    ptr: *const T,
    len: usize,
}

impl<'c, T: core::fmt::Debug + Copy, Wasm: WasmAccessDynCompatible> WasmArrayAccessDynCompatibleIterator<'c, T, Wasm> {
    /// Creates a new `WasmArrayAccessDynCompatibleIterator`.
    pub fn new(wasm: &'c Wasm, ptr: *const T, len: usize) -> Self {
        Self {
            access: wasm,
            ptr,
            len,
        }
    }
}

impl<'c, T: core::fmt::Debug + Copy, Wasm: WasmAccessDynCompatible> Iterator for WasmArrayAccessDynCompatibleIterator<'c, T, Wasm> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        let item = self.access.load_le_with(self.ptr);
        self.ptr = unsafe { self.ptr.add(1) };
        self.len -= 1;
        Some(item)
    }
}

pub struct WasmArrayAccessDynCompatibleMutIterator<'c, T: core::fmt::Debug + Copy, Wasm: WasmAccessDynCompatible> {
    access: &'c Wasm,
    ptr: *mut T,
    len: usize,
}

impl<'c, T: core::fmt::Debug + Copy, Wasm: WasmAccessDynCompatible> WasmArrayAccessDynCompatibleMutIterator<'c, T, Wasm> {
    /// Creates a new `WasmArrayAccessDynCompatibleMutIterator`.
    pub fn new(wasm: &'c Wasm, ptr: *mut T, len: usize) -> Self {
        Self {
            access: wasm,
            ptr,
            len,
        }
    }
}

pub struct WasmArrayAccessDynCompatibleMutIteratorComponent<'c, T: core::fmt::Debug + Copy, Wasm: WasmAccessDynCompatible> {
    access: &'c Wasm,
    ptr: *mut T,
}

impl<'c, T: core::fmt::Debug + Copy, Wasm: WasmAccessDynCompatible> WasmArrayAccessDynCompatibleMutIteratorComponent<'c, T, Wasm> {
    /// Creates a new `WasmArrayAccessDynCompatibleMutIteratorComponent`.
    pub fn new(wasm: &'c Wasm, ptr: *mut T) -> Self {
        Self {
            access: wasm,
            ptr,
        }
    }

    /// Sets the value at the current position.
    pub fn set(&self, value: T) {
        self.access.store_le_with(self.ptr, value);
    }
}

impl<'c, T: core::fmt::Debug + Copy, Wasm: WasmAccessDynCompatible> Iterator
    for WasmArrayAccessDynCompatibleMutIterator<'c, T, Wasm>
{
    type Item = WasmArrayAccessDynCompatibleMutIteratorComponent<'c, T, Wasm>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        let component = WasmArrayAccessDynCompatibleMutIteratorComponent::new(self.access, self.ptr);
        self.ptr = unsafe { self.ptr.add(1) };
        self.len -= 1;
        Some(component)
    }
}

#[derive(Debug)]
pub struct WasmPathAccessDynCompatible<'a, 'b, Wasm: WasmAccessDynCompatible> {
    path: WasmArrayAccessDynCompatible<'a, 'b, u8, Wasm>,
}

impl<'a, 'b, Wasm: WasmAccessDynCompatible> PartialEq for WasmPathAccessDynCompatible<'a, 'b, Wasm> {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl<'a, 'b, Wasm: WasmAccessDynCompatible> Clone for WasmPathAccessDynCompatible<'a, 'b, Wasm> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, 'b, Wasm: WasmAccessDynCompatible> Copy for WasmPathAccessDynCompatible<'a, 'b, Wasm> {}

impl<'a, 'b, Wasm: WasmAccessDynCompatible> WasmPathAccessDynCompatible<'a, 'b, Wasm> {
    /// Creates a new `WasmPathAccessDynCompatible`.
    #[inline(always)]
    pub fn new(access: &'b Wasm, ptr: *const u8, len: usize) -> Self {
        Self {
            path: WasmArrayAccessDynCompatible::new(access, ptr, len),
        }
    }

    /// Returns an iterator over the components of the path.
    #[inline(always)]
    pub fn components(&self) -> WasmPathComponentsDynCompatible<'a, 'b, Wasm> {
        let path = self.path;
        WasmPathComponentsDynCompatible { path }
    }
}

impl<'a, 'b, Wasm: WasmAccessDynCompatible> WasmPathAccessCommon for WasmPathAccessDynCompatible<'a, 'b, Wasm> {
    fn components_common<'c>(&'c self) -> impl Iterator<Item = impl WasmPathComponentCommon + 'c> + 'c {
        self.components()
    }
}

pub struct WasmPathComponentsDynCompatible<'a, 'b, Wasm: WasmAccessDynCompatible> {
    path: WasmArrayAccessDynCompatible<'a, 'b, u8, Wasm>,
}

#[derive(Debug)]
pub enum WasmPathComponentDynCompatible<'a, 'b, Wasm: WasmAccessDynCompatible> {
    RootDir,
    CurDir,
    ParentDir,
    Normal(WasmArrayAccessDynCompatible<'a, 'b, u8, Wasm>),
}

impl<'a, 'b, Wasm: WasmAccessDynCompatible> Clone for WasmPathComponentDynCompatible<'a, 'b, Wasm> {
    fn clone(&self) -> Self {
        match self {
            WasmPathComponentDynCompatible::RootDir => WasmPathComponentDynCompatible::RootDir,
            WasmPathComponentDynCompatible::CurDir => WasmPathComponentDynCompatible::CurDir,
            WasmPathComponentDynCompatible::ParentDir => WasmPathComponentDynCompatible::ParentDir,
            WasmPathComponentDynCompatible::Normal(access) => {
                WasmPathComponentDynCompatible::Normal(*access)
            }
        }
    }
}

impl<'a, 'b, Wasm: WasmAccessDynCompatible> Copy for WasmPathComponentDynCompatible<'a, 'b, Wasm> {}

impl<'a, 'b, Wasm: WasmAccessDynCompatible> PartialEq for WasmPathComponentDynCompatible<'a, 'b, Wasm> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (WasmPathComponentDynCompatible::RootDir, WasmPathComponentDynCompatible::RootDir) => true,
            (WasmPathComponentDynCompatible::CurDir, WasmPathComponentDynCompatible::CurDir) => true,
            (WasmPathComponentDynCompatible::ParentDir, WasmPathComponentDynCompatible::ParentDir) => true,
            (WasmPathComponentDynCompatible::Normal(a), WasmPathComponentDynCompatible::Normal(b)) => a == b,
            _ => false,
        }
    }
}

impl<'a, 'b, Wasm: WasmAccessDynCompatible> WasmPathComponentDynCompatible<'a, 'b, Wasm> {
    pub fn eq_str(&self, other: &str) -> bool {
        match self {
            WasmPathComponentDynCompatible::RootDir => other == "/",
            WasmPathComponentDynCompatible::CurDir => other == ".",
            WasmPathComponentDynCompatible::ParentDir => other == "..",
            WasmPathComponentDynCompatible::Normal(access) => {
                access.len() == other.len()
                    && (0..access.len()).all(|i| access.get(i) == other.as_bytes()[i])
            }
        }
    }
}

impl<'a, 'b, Wasm: WasmAccessDynCompatible> Iterator for WasmPathComponentsDynCompatible<'a, 'b, Wasm> {
    type Item = WasmPathComponentDynCompatible<'a, 'b, Wasm>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.path.len() == 0 {
            return None;
        }

        if self.path.get(0) == b'/' {
            let mut index = 1;
            while index < self.path.len() && self.path.get(index) == b'/' {
                index += 1;
            }

            self.path.ptr = unsafe { self.path.ptr.add(index) };
            self.path.len -= index;
            return Some(WasmPathComponentDynCompatible::RootDir);
        }

        if self.path.get(0) == b'.' {
            if self.path.len() == 1 {
                self.path.len = 0;
                return Some(WasmPathComponentDynCompatible::CurDir);
            }

            let second = self.path.get(1);

            if second == b'/' {
                let mut index = 2;
                while index < self.path.len() && self.path.get(index) == b'/' {
                    index += 1;
                }
                self.path.ptr = unsafe { self.path.ptr.add(index) };
                self.path.len -= index;
                return Some(WasmPathComponentDynCompatible::CurDir);
            } else if second == b'.' {
                if self.path.len() == 2 {
                    self.path.len = 0;
                    return Some(WasmPathComponentDynCompatible::ParentDir);
                }

                let third = self.path.get(2);
                if third == b'/' {
                    let mut index = 3;
                    while index < self.path.len() && self.path.get(index) == b'/' {
                        index += 1;
                    }
                    self.path.ptr = unsafe { self.path.ptr.add(index) };
                    self.path.len -= index;
                    return Some(WasmPathComponentDynCompatible::ParentDir);
                }

                let mut end = 3;
                while end < self.path.len() && self.path.get(end) != b'/' {
                    end += 1;
                }

                let component =
                    WasmArrayAccessDynCompatible::new(self.path.access, self.path.ptr, end);

                while end < self.path.len() && self.path.get(end) == b'/' {
                    end += 1;
                }
                self.path.ptr = unsafe { self.path.ptr.add(end) };
                self.path.len -= end;
                return Some(WasmPathComponentDynCompatible::Normal(component));
            }
        } else {
            let mut end = 0;
            while end < self.path.len() && self.path.get(end) != b'/' {
                end += 1;
            }

            let component = WasmArrayAccessDynCompatible::new(self.path.access, self.path.ptr, end);

            while end < self.path.len() && self.path.get(end) == b'/' {
                end += 1;
            }
            self.path.ptr = unsafe { self.path.ptr.add(end) };
            self.path.len -= end;
            return Some(WasmPathComponentDynCompatible::Normal(component));
        }

        None
    }
}

impl<'a, 'b, Wasm: WasmAccessDynCompatible> WasmPathComponentCommon for WasmPathComponentDynCompatible<'a, 'b, Wasm> {
    fn as_root_dir(&self) -> bool {
        matches!(self, WasmPathComponentDynCompatible::RootDir)
    }

    fn as_cur_dir(&self) -> bool {
        matches!(self, WasmPathComponentDynCompatible::CurDir)
    }

    fn as_parent_dir(&self) -> bool {
        matches!(self, WasmPathComponentDynCompatible::ParentDir)
    }

    fn as_normal(&self) -> Option<impl IntoIterator<Item = u8> + Clone + '_> {
        match self {
            WasmPathComponentDynCompatible::Normal(access) => Some(*access),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_path_components() {
        let path = WasmPathAccess::<WasmAccessFaker>::new(b"a/b//c".as_ptr(), 6);
        let components = path.components();
        let mut iter = components.into_iter();

        assert!(iter.next().unwrap().eq_str("a"));
        assert!(iter.next().unwrap().eq_str("b"));
        assert!(iter.next().unwrap().eq_str("c"));
        assert!(iter.next().is_none());

        let path =
            WasmPathAccess::<WasmAccessFaker>::new(b"virtual-layer/src/wasi/file.rs".as_ptr(), 30);
        let components = path.components();
        let mut iter = components.into_iter();

        assert!(iter.next().unwrap().eq_str("virtual-layer"));
        assert!(iter.next().unwrap().eq_str("src"));
        assert!(iter.next().unwrap().eq_str("wasi"));
        assert!(iter.next().unwrap().eq_str("file.rs"));
        assert!(iter.next().is_none());

        let path = WasmPathAccess::<WasmAccessFaker>::new(b"//bin/lsd.exe".as_ptr(), 13);
        let components = path.components();
        let mut iter = components.into_iter();

        assert!(iter.next().unwrap().eq_str("/"));
        assert!(iter.next().unwrap().eq_str("bin"));
        assert!(iter.next().unwrap().eq_str("lsd.exe"));
        assert!(iter.next().is_none());

        let path =
            WasmPathAccess::<WasmAccessFaker>::new(b"/bin////../bin/explorer.exe".as_ptr(), 27);
        let components = path.components();
        let mut iter = components.into_iter();

        assert!(iter.next().unwrap().eq_str("/"));
        assert!(iter.next().unwrap().eq_str("bin"));
        assert!(iter.next().unwrap().eq_str(".."));
        assert!(iter.next().unwrap().eq_str("bin"));
        assert!(iter.next().unwrap().eq_str("explorer.exe"));
        assert!(iter.next().is_none());
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct WasmAccessFaker;

impl WasmAccessName for WasmAccessFaker {
    const NAME: &'static str = "WasmAccessFaker";
}

impl WasmAccessDynCompatibleRaw for WasmAccessFaker {
    fn memcpy_raw(&self, offset: *mut u8, src: *const u8, len: usize) {
        unsafe {
            core::ptr::copy_nonoverlapping(src, offset, len);
        }
    }

    fn memcpy_to_raw(&self, offset: *mut u8, src: *const u8, len: usize) {
        unsafe {
            core::ptr::copy_nonoverlapping(src, offset, len);
        }
    }

    #[inline(always)]
    fn _main_raw(&self) -> wasip1::Errno {
        wasip1::ERRNO_SUCCESS
    }

    #[inline(always)]
    fn _reset_raw(&self) {}

    #[inline(always)]
    fn _start_raw(&self) {}

    #[cfg(not(feature = "multi_memory"))]
    fn memory_director_raw(&self, ptr: isize) -> isize {
        ptr
    }
}

impl WasmAccessRaw for WasmAccessFaker {
    fn memcpy_raw(offset: *mut u8, src: *const u8, len: usize) {
        unsafe {
            core::ptr::copy_nonoverlapping(src, offset, len);
        }
    }

    fn memcpy_to_raw(offset: *mut u8, src: *const u8, len: usize) {
        unsafe {
            core::ptr::copy_nonoverlapping(src, offset, len);
        }
    }

    fn _main_raw() -> wasip1::Errno {
        wasip1::ERRNO_SUCCESS
    }

    fn _reset_raw() {}

    fn _start_raw() {}

    #[cfg(not(feature = "multi_memory"))]
    fn memory_director_raw(ptr: isize) -> isize {
        ptr
    }
}
