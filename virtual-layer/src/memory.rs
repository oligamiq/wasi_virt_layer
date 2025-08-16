/// By entering the names of the files to be combined, a bridge for the combination is created.
/// You need to prepare as many Wasip1 instances on the virtual file system as the number of files to be combined.
#[macro_export]
macro_rules! import_wasm {
    ($name:ident) => {
        $crate::__private::paste::paste! {
            #[allow(non_camel_case_types)]
            #[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub struct WasmArrayAccess<'a, T: core::fmt::Debug + Copy, Wasm: WasmAccess> {
    ptr: *const T,
    len: usize,
    __marker: core::marker::PhantomData<&'a Wasm>,
}

impl<'a, T: core::fmt::Debug + Copy, Wasm: WasmAccess> WasmArrayAccess<'a, T, Wasm> {
    #[inline(always)]
    pub fn new(ptr: *const T, len: usize) -> Self {
        {
            Self {
                ptr,
                len,
                __marker: core::marker::PhantomData,
            }
        }
    }

    #[inline(always)]
    pub fn get(&self, index: usize) -> T {
        {
            let ptr = unsafe { self.ptr.add(index) };
            Wasm::load_le(ptr)
        }
    }

    #[inline(always)]
    pub fn iter(&self) -> WasmArrayAccessIterator<T, Wasm> {
        WasmArrayAccessIterator::new(self.ptr, self.len)
    }

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

pub struct WasmArrayAccessIterator<T: core::fmt::Debug + Copy, Wasm: WasmAccess> {
    ptr: *const T,
    len: usize,
    __marker: core::marker::PhantomData<Wasm>,
}

impl<T: core::fmt::Debug + Copy, Wasm: WasmAccess> WasmArrayAccessIterator<T, Wasm> {
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

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

pub trait WasmAccess: Copy {
    /// Copies data from the source pointer to the offset.
    fn memcpy<T>(offset: *mut T, data: &[T]);

    /// Copies data from the source pointer to the offset.
    fn memcpy_to<T>(offset: &mut [T], src: *const T);
    fn store_le<T>(offset: *mut T, value: T);
    fn load_le<T: core::fmt::Debug>(offset: *const T) -> T;

    /// utility internal
    fn as_array<'a, T: core::fmt::Debug + Copy>(
        ptr: *const T,
        len: usize,
    ) -> WasmArrayAccess<'a, T, Self>
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WasmPathAccess<'a, Wasm: WasmAccess> {
    path: WasmArrayAccess<'a, u8, Wasm>,
}

impl<'a, Wasm: WasmAccess> WasmPathAccess<'a, Wasm> {
    #[inline(always)]
    pub fn new(ptr: *const u8, len: usize) -> Self {
        Self {
            path: WasmArrayAccess::new(ptr, len),
        }
    }

    #[inline(always)]
    pub fn components(&self) -> WasmPathComponents<'a, Wasm> {
        let path = self.path;
        WasmPathComponents { path }
    }
}

pub struct WasmPathComponents<'a, Wasm: WasmAccess> {
    path: WasmArrayAccess<'a, u8, Wasm>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WasmPathComponent<'a, Wasm: WasmAccess> {
    /// If wasi, root only "/"
    RootDir,

    /// A reference to the current directory, i.e., `.`.
    CurDir,

    /// A reference to the parent directory, i.e., `..`.
    ParentDir,

    /// A normal component, e.g., `a` and `b` in `a/b`.
    ///
    /// This variant is the most common one, it represents references to files
    /// or directories.
    Normal(WasmArrayAccess<'a, u8, Wasm>),
}

impl<'a, Wasm: WasmAccess> WasmPathComponent<'a, Wasm> {
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

impl WasmAccess for WasmAccessFaker {
    fn memcpy<T>(offset: *mut T, data: &[T]) {
        unsafe {
            core::ptr::copy_nonoverlapping(data.as_ptr(), offset, data.len());
        }
    }

    fn store_le<T>(offset: *mut T, value: T) {
        unsafe { core::ptr::write(offset, value) };
    }

    fn load_le<T: core::fmt::Debug>(offset: *const T) -> T {
        unsafe { core::ptr::read(offset) }
    }

    #[inline(always)]
    fn main() {}

    #[inline(always)]
    fn reset() {}

    #[inline(always)]
    fn _start() {}

    #[cfg(not(feature = "multi_memory"))]
    fn memory_director<T>(ptr: *const T) -> *const T {
        ptr
    }

    #[cfg(not(feature = "multi_memory"))]
    fn memory_director_mut<T>(ptr: *mut T) -> *mut T {
        ptr
    }

    fn memcpy_to<T>(offset: &mut [T], src: *const T) {
        unsafe {
            core::ptr::copy_nonoverlapping(src, offset.as_mut_ptr(), offset.len());
        }
    }
}
