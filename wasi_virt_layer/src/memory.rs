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

            impl $crate::memory::WasmAccessRaw for $name {
                const NAME: &'static str = stringify!($name);

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

                $crate::__memory_director_wasm_access!($name);

                #[inline(always)]
                fn _main_raw() -> $crate::__private::wasip1::Errno
                {
                    #[cfg(not(target_os = "wasi"))]
                    unimplemented!("this is not supported on this architecture");

                    #[cfg(target_os = "wasi")]
                    unsafe { [<__wasip1_vfs_ $name ___main_void>]() }
                }

                #[inline(always)]
                fn _reset_raw()
                {
                    #[cfg(not(target_os = "wasi"))]
                    unimplemented!("this is not supported on this architecture");

                    #[cfg(target_os = "wasi")]
                    unsafe { [<__wasip1_vfs_ $name _reset>]() };
                }

                #[inline(always)]
                fn _start_raw()
                {
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
    ($name:ident) => {
        $crate::__private::paste::paste! {
            #[inline(always)]
            fn memory_director_raw(ptr: isize) -> isize {
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
    ($_:ident) => {};
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
        unsafe { core::ptr::write(self.ptr, value) };
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

pub trait WasmAccessRaw: core::fmt::Debug {
    const NAME: &'static str;

    /// Copies a slice of data into WASM memory starting at the given offset.
    fn memcpy_raw(offset: *mut u8, src: *const u8, len: usize);

    /// Copies data from the source pointer into the provided mutable slice of WASM memory.
    fn memcpy_to_raw(offset: *mut u8, src: *const u8, len: usize);

    #[cfg(not(feature = "multi_memory"))]
    fn memory_director_raw(ptr: isize) -> isize;

    fn _main_raw() -> wasip1::Errno;

    fn _reset_raw();

    fn _start_raw();
}

impl<T: WasmAccessRaw> WasmAccess for T {
    const NAME: &'static str = Self::NAME;

    fn memcpy<U>(offset: *mut U, data: &[U]) {
        Self::memcpy_raw(
            offset as *mut u8,
            data.as_ptr() as *const u8,
            core::mem::size_of::<U>() * data.len(),
        );
    }

    fn memcpy_to<U>(offset: &mut [U], src: *const U) {
        Self::memcpy_to_raw(
            offset.as_mut_ptr() as *mut u8,
            src as *const u8,
            core::mem::size_of::<U>() * offset.len(),
        );
    }

    fn store_le<U>(offset: *mut U, value: U) {
        Self::memcpy_raw(
            offset as *mut u8,
            &value as *const U as *const u8,
            core::mem::size_of::<U>(),
        );
    }

    fn load_le<U: core::fmt::Debug + Copy>(offset: *const U) -> U {
        let mut value = core::mem::MaybeUninit::<U>::uninit();
        Self::memcpy_to_raw(
            value.as_mut_ptr() as *mut u8,
            offset as *const u8,
            core::mem::size_of::<U>(),
        );
        unsafe { core::ptr::read(value.as_ptr() as *const U) }
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
    const NAME: &'static str;

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
#[derive(Debug, PartialEq)]
pub struct WasmPathAccess<'a, Wasm: WasmAccess> {
    path: WasmArrayAccess<'a, u8, Wasm>,
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

/// A component of a WASM path.
#[derive(Debug, Clone, Copy, PartialEq)]
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

impl WasmAccessRaw for WasmAccessFaker {
    const NAME: &'static str = "WasmAccessFaker";

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

    #[inline(always)]
    fn _main_raw() -> wasip1::Errno {
        wasip1::ERRNO_SUCCESS
    }

    #[inline(always)]
    fn _reset_raw() {}

    #[inline(always)]
    fn _start_raw() {}

    #[cfg(not(feature = "multi_memory"))]
    fn memory_director_raw(ptr: isize) -> isize {
        ptr
    }
}
