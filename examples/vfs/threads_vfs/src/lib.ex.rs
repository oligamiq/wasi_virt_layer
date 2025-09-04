#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use const_struct::const_struct;
use wasip1_virtual_layer::{file::*, prelude::*, thread::DirectThreadPool};
#[doc(hidden)]
#[allow(non_snake_case, unused_unsafe)]
pub unsafe fn _export_init_cabi<T: Guest>() {
    unsafe {
        _rt::run_ctors_once();
        { T::init() };
    }
}
#[doc(hidden)]
#[allow(non_snake_case, unused_unsafe)]
pub unsafe fn _export_start_cabi<T: Guest>() {
    unsafe {
        _rt::run_ctors_once();
        { T::start() };
    }
}
pub trait Guest {
    #[allow(async_fn_in_trait)]
    fn init() -> ();
    #[allow(async_fn_in_trait)]
    fn start() -> ();
}
#[doc(hidden)]
pub(crate) use __export_world_init_cabi;
mod _rt {
    #![allow(dead_code, clippy::all)]
    pub fn run_ctors_once() {
        wit_bindgen::rt::run_ctors_once();
    }
}
#[doc(inline)]
pub(crate) use __export_init_impl as export;
#[unsafe(
    link_section = "component-type:wit-bindgen:0.43.0:hello:host:init:encoded world"
)]
#[doc(hidden)]
#[allow(clippy::octal_escapes)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 172] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x072\x01A\x02\x01A\x03\x01\
@\0\x01\0\x04\0\x04init\x01\0\x04\0\x05start\x01\0\x04\0\x0fhello:host/init\x04\0\
\x0b\x0a\x01\0\x04init\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-co\
mponent\x070.235.0\x10wit-bindgen-rust\x060.43.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen::rt::maybe_link_cabi_realloc();
}
const _: &[u8] = b"// wit is only kebab-case\npackage hello:host;\n\nworld init {\n  export init: func();\n  export start: func();\n}\n";
struct Starter;
impl Guest for Starter {
    fn init() -> () {}
    fn start() -> () {
        {
            ::std::io::_print(format_args!("Files: {0:?}\n", FILES));
        };
        ::core::panicking::panic("not yet implemented")
    }
}
const _: () = {
    #[unsafe(export_name = "init")]
    unsafe extern "C" fn export_init() {
        unsafe { self::_export_init_cabi::<Starter>() }
    }
    #[unsafe(export_name = "start")]
    unsafe extern "C" fn export_start() {
        unsafe { self::_export_start_cabi::<Starter>() }
    }
};
#[allow(non_camel_case_types)]
struct test_threads;
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::fmt::Debug for test_threads {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "test_threads")
    }
}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::clone::Clone for test_threads {
    #[inline]
    fn clone(&self) -> test_threads {
        *self
    }
}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::marker::Copy for test_threads {}
#[doc(hidden)]
#[link(wasm_import_module = "wasip1-vfs")]
unsafe extern "C" {
    /// https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Copy
    #[unsafe(no_mangle)]
    pub fn __wasip1_vfs_test_threads_memory_copy_from(
        offset: *mut u8,
        src: *const u8,
        len: usize,
    );
    #[unsafe(no_mangle)]
    pub fn __wasip1_vfs_test_threads_memory_copy_to(
        offset: *mut u8,
        src: *const u8,
        len: usize,
    );
    #[unsafe(no_mangle)]
    pub fn __wasip1_vfs_test_threads___main_void();
    #[unsafe(no_mangle)]
    pub fn __wasip1_vfs_test_threads__start();
    #[unsafe(no_mangle)]
    pub fn __wasip1_vfs_test_threads_reset();
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __wasip1_vfs_test_threads__start_wrap() {
    unsafe { __wasip1_vfs_test_threads__start() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __wasip1_vfs_test_threads_memory_trap_wrap(_ptr: isize) -> isize {
    unsafe { __wasip1_vfs_test_threads_memory_trap(_ptr) }
}
#[doc(hidden)]
#[link(wasm_import_module = "wasip1-vfs")]
unsafe extern "C" {
    #[unsafe(no_mangle)]
    pub fn __wasip1_vfs_test_threads_memory_trap(_ptr: isize) -> isize;
    #[unsafe(no_mangle)]
    pub fn __wasip1_vfs_test_threads_memory_director(ptr: isize) -> isize;
}
#[doc(hidden)]
#[link(wasm_import_module = "wasip1-vfs")]
unsafe extern "C" {
    #[unsafe(no_mangle)]
    pub fn __wasip1_vfs_test_threads_wasi_thread_start(thread_id: isize, ptr: isize);
}
impl ::wasip1_virtual_layer::memory::WasmAccess for test_threads {
    #[inline(always)]
    fn memcpy<T>(offset: *mut T, data: &[T]) {
        unsafe {
            __wasip1_vfs_test_threads_memory_copy_from(
                offset as *mut u8,
                data.as_ptr() as *const u8,
                core::mem::size_of::<T>() * data.len(),
            )
        };
    }
    #[inline(always)]
    fn memcpy_to<T>(offset: &mut [T], src: *const T) {
        unsafe {
            __wasip1_vfs_test_threads_memory_copy_to(
                offset.as_mut_ptr() as *mut u8,
                src as *const u8,
                core::mem::size_of::<T>() * offset.len(),
            )
        };
    }
    #[inline(always)]
    fn store_le<T>(offset: *mut T, value: T) {
        unsafe {
            __wasip1_vfs_test_threads_memory_copy_from(
                offset as *mut u8,
                &value as *const T as *const u8,
                core::mem::size_of::<T>(),
            )
        };
    }
    #[inline(always)]
    fn load_le<T: core::fmt::Debug>(offset: *const T) -> T {
        unsafe {
            let mut value = core::mem::MaybeUninit::uninit();
            __wasip1_vfs_test_threads_memory_copy_to(
                value.as_mut_ptr() as *mut u8,
                offset as *const u8,
                core::mem::size_of::<T>(),
            );
            core::ptr::read(value.as_ptr() as *const T)
        }
    }
    #[inline(always)]
    fn memory_director<T>(ptr: *const T) -> *const T {
        unsafe { __wasip1_vfs_test_threads_memory_director(ptr as isize) as *const T }
    }
    #[inline(always)]
    fn memory_director_mut<T>(ptr: *mut T) -> *mut T {
        unsafe { __wasip1_vfs_test_threads_memory_director(ptr as isize) as *mut T }
    }
    #[inline(always)]
    fn _wasi_thread_start(thread_id: isize, ptr: isize) {
        unsafe { __wasip1_vfs_test_threads_wasi_thread_start(thread_id, ptr) }
    }
    #[inline(always)]
    fn _main() {
        unsafe { __wasip1_vfs_test_threads___main_void() };
    }
    #[inline(always)]
    fn reset() {
        unsafe { __wasip1_vfs_test_threads_reset() };
    }
    #[inline(always)]
    fn _start() {
        unsafe { __wasip1_vfs_test_threads__start() };
    }
}
const FILE_COUNT: usize = 5;
type F = WasiConstFile<&'static str>;
type NormalFILES = VFSConstNormalFiles<F, { FILE_COUNT }>;
const FILES: NormalFILES = ::wasip1_virtual_layer::__private::inner::fs::VFSConstNormalFiles::new({
    const COUNT: usize = {
        let mut count = 0;
        count += 1;
        count += 1;
        count += 1;
        count += 1;
        count += 1;
        count
    };
    let mut static_array = ::wasip1_virtual_layer::__private::utils::StaticArrayBuilder::new();
    struct CheckEqNumberOfFilesAndDirs<const L: usize, const R: usize>;
    #[allow(dead_code)]
    impl<const L: usize, const R: usize> CheckEqNumberOfFilesAndDirs<L, R> {
        #[allow(non_upper_case_globals)]
        const number_of_files_and_dirs_equals_FLAT_LEN_so_you_must_set_VFSConstNormalFiles_num: usize = (R
            - L) + (L - R);
    }
    const fn asserter<S: 'static + Copy, const N: usize>(_: &[S; N]) {
        #[allow(path_statements)]
        CheckEqNumberOfFilesAndDirs::<
            COUNT,
            N,
        >::number_of_files_and_dirs_equals_FLAT_LEN_so_you_must_set_VFSConstNormalFiles_num;
    }
    use ::wasip1_virtual_layer::__private::const_for;
    const fn eq_str(a: &str, b: &str) -> bool {
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();
        if a_bytes.len() != b_bytes.len() {
            return false;
        }
        {
            let _: usize = 1;
            let mut __ite = (0..a_bytes.len()).start;
            let __end = (0..a_bytes.len()).end;
            let mut __is_first = true;
            let __step = 1;
            loop {
                if !__is_first {
                    __ite += __step;
                }
                __is_first = false;
                let i = __ite;
                if __ite >= __end {
                    break;
                }
                {
                    if a_bytes[i] != b_bytes[i] {
                        return false;
                    }
                }
            }
        };
        true
    }
    /// if b is a parent of a
    const fn is_parent(a: &str, b: &str) -> bool {
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();
        if a_bytes.len() < b_bytes.len() {
            return false;
        }
        {
            let _: usize = 1;
            let mut __ite = (0..b_bytes.len()).start;
            let __end = (0..b_bytes.len()).end;
            let mut __is_first = true;
            let __step = 1;
            loop {
                if !__is_first {
                    __ite += __step;
                }
                __is_first = false;
                let i = __ite;
                if __ite >= __end {
                    break;
                }
                {
                    if a_bytes[i] != b_bytes[i] {
                        return false;
                    }
                }
            }
        };
        let mut i = b_bytes.len();
        while i < a_bytes.len() && a_bytes[i] == b"/"[0] {
            i += 1;
        }
        if i == a_bytes.len() {
            return false;
        }
        if i == b_bytes.len() {
            return false;
        }
        {
            let _: usize = 1;
            let mut __ite = (i..a_bytes.len()).start;
            let __end = (i..a_bytes.len()).end;
            let mut __is_first = true;
            let __step = 1;
            loop {
                if !__is_first {
                    __ite += __step;
                }
                __is_first = false;
                let n = __ite;
                if __ite >= __end {
                    break;
                }
                {
                    if a_bytes[n] == b"/"[0] {
                        return false;
                    }
                }
            }
        };
        true
    }
    const fn get_child_range<S: 'static + Copy, const N: usize>(
        fake_files: [&'static str; N],
        name: &'static str,
        _: &::wasip1_virtual_layer::__private::utils::StaticArrayBuilder<S, N>,
    ) -> (usize, usize) {
        get_child_range_inner(fake_files, name)
    }
    const fn get_child_range_inner<const N: usize>(
        fake_files: [&'static str; N],
        name: &'static str,
    ) -> (usize, usize) {
        let mut first_index = None;
        let mut last_index = None;
        {
            let _: usize = 1;
            let mut __ite = (0..N).start;
            let __end = (0..N).end;
            let mut __is_first = true;
            let __step = 1;
            loop {
                if !__is_first {
                    __ite += __step;
                }
                __is_first = false;
                let i = __ite;
                if __ite >= __end {
                    break;
                }
                {
                    if is_parent(fake_files[i], name) {
                        if first_index.is_none() {
                            first_index = Some(i);
                        }
                        last_index = Some(i);
                    }
                }
            }
        };
        (first_index.unwrap(), last_index.unwrap() + 1)
    }
    const fn get_parent<S: 'static + Copy, const N: usize>(
        fake_files: [&'static str; N],
        name: &'static str,
        _: &::wasip1_virtual_layer::__private::utils::StaticArrayBuilder<S, N>,
    ) -> Option<usize> {
        {
            let _: usize = 1;
            let mut __ite = (0..N).start;
            let __end = (0..N).end;
            let mut __is_first = true;
            let __step = 1;
            loop {
                if !__is_first {
                    __ite += __step;
                }
                __is_first = false;
                let i = __ite;
                if __ite >= __end {
                    break;
                }
                {
                    if is_parent(name, fake_files[i]) {
                        return Some(i);
                    }
                }
            }
        };
        None
    }
    const fn get_self<const N: usize>(
        fake_files: [&'static str; N],
        name: &'static str,
    ) -> usize {
        {
            let _: usize = 1;
            let mut __ite = (0..N).start;
            let __end = (0..N).end;
            let mut __is_first = true;
            let __step = 1;
            loop {
                if !__is_first {
                    __ite += __step;
                }
                __is_first = false;
                let i = __ite;
                if __ite >= __end {
                    break;
                }
                {
                    if eq_str(name, fake_files[i]) {
                        return i;
                    }
                }
            }
        };
        ::core::panicking::panic("internal error: entered unreachable code")
    }
    const fn custom_sort<T: Copy, const N: usize>(
        mut files: ::wasip1_virtual_layer::__private::utils::StaticArrayBuilder<
            (usize, T),
            N,
        >,
    ) -> [T; N] {
        let mut sorted = ::wasip1_virtual_layer::__private::utils::StaticArrayBuilder::<
            _,
            N,
        >::new();
        while (files.len() > 0) {
            let mut depth = None;
            let mut index = None;
            {
                let _: usize = 1;
                let mut __ite = (0..files.len()).start;
                let __end = (0..files.len()).end;
                let mut __is_first = true;
                let __step = 1;
                loop {
                    if !__is_first {
                        __ite += __step;
                    }
                    __is_first = false;
                    let i = __ite;
                    if __ite >= __end {
                        break;
                    }
                    {
                        let file = files.get(i).unwrap();
                        if let Some(d) = depth {
                            if file.0 < d {
                                depth = Some(file.0);
                                index = Some((i, file.1));
                            }
                        } else {
                            depth = Some(file.0);
                            index = Some((i, file.1));
                        }
                    }
                }
            };
            if let Some(index) = index {
                sorted.push(index.1);
                files.remove(index.0);
            }
        }
        sorted.build()
    }
    const EMPTY_ARR: [&'static str; COUNT] = {
        let mut empty_arr = ::wasip1_virtual_layer::__private::utils::StaticArrayBuilder::new();
        empty_arr.push((0 + 1, "./hey"));
        empty_arr.push((0 + 1 + 1, "./hello/world"));
        empty_arr.push((0 + 1 + 1, "./hello/everyone"));
        empty_arr.push((0 + 1, "./hello"));
        empty_arr.push((0, "."));
        let _ = empty_arr.build();
        custom_sort(empty_arr)
    };
    static_array
        .push((
            0 + 1,
            (
                "./hey",
                "hey",
                ::wasip1_virtual_layer::__private::inner::fs::VFSConstNormalInode::File(
                    { F::new("Hey!") },
                    get_parent(EMPTY_ARR, "./hey", &static_array).unwrap(),
                ),
            ),
        ));
    static_array
        .push((
            0 + 1 + 1,
            (
                "./hello/world",
                "world",
                ::wasip1_virtual_layer::__private::inner::fs::VFSConstNormalInode::File(
                    { F::new("Hello, world!") },
                    get_parent(EMPTY_ARR, "./hello/world", &static_array).unwrap(),
                ),
            ),
        ));
    static_array
        .push((
            0 + 1 + 1,
            (
                "./hello/everyone",
                "everyone",
                ::wasip1_virtual_layer::__private::inner::fs::VFSConstNormalInode::File(
                    { F::new("Hello, everyone!") },
                    get_parent(EMPTY_ARR, "./hello/everyone", &static_array).unwrap(),
                ),
            ),
        ));
    static_array
        .push((
            0 + 1,
            (
                "./hello",
                "hello",
                ::wasip1_virtual_layer::__private::inner::fs::VFSConstNormalInode::Dir(
                    get_child_range(EMPTY_ARR, "./hello", &static_array),
                    get_parent(EMPTY_ARR, "./hello", &static_array),
                ),
            ),
        ));
    static_array
        .push((
            0,
            (
                ".",
                ".",
                ::wasip1_virtual_layer::__private::inner::fs::VFSConstNormalInode::Dir(
                    get_child_range(EMPTY_ARR, ".", &static_array),
                    get_parent(EMPTY_ARR, ".", &static_array),
                ),
            ),
        ));
    const PRE_OPEN_COUNT: usize = {
        let mut count = 0;
        count += 1;
        count
    };
    const PRE_OPEN: [usize; PRE_OPEN_COUNT] = {
        let mut static_array = ::wasip1_virtual_layer::__private::utils::StaticArrayBuilder::new();
        static_array.push(get_self(EMPTY_ARR, "."));
        static_array.build()
    };
    let static_array = custom_sort(static_array);
    let mut file_array = ::wasip1_virtual_layer::__private::utils::StaticArrayBuilder::new();
    {
        let _: usize = 1;
        let mut __ite = (0..static_array.len()).start;
        let __end = (0..static_array.len()).end;
        let mut __is_first = true;
        let __step = 1;
        loop {
            if !__is_first {
                __ite += __step;
            }
            __is_first = false;
            let i = __ite;
            if __ite >= __end {
                break;
            }
            {
                let (_, name, file_or_dir) = static_array[i];
                file_array.push((name, file_or_dir));
            }
        }
    };
    let static_array = file_array.build_with_is_check(file_array.check_len());
    let _ = asserter(&static_array);
    (static_array, &PRE_OPEN)
});
#[automatically_derived]
#[allow(dead_code)]
pub struct FilesTy;
#[automatically_derived]
impl ::const_struct::PrimitiveTraits for FilesTy {
    type DATATYPE = NormalFILES;
    const __DATA: <Self as ::const_struct::PrimitiveTraits>::DATATYPE = FILES;
}
mod thread {
    use super::*;
    #[allow(non_camel_case_types)]
    pub(crate) enum ThreadAccessor {
        __self,
        __test_threads,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::fmt::Debug for ThreadAccessor {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ThreadAccessor::__self => "__self",
                    ThreadAccessor::__test_threads => "__test_threads",
                },
            )
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::clone::Clone for ThreadAccessor {
        #[inline]
        fn clone(&self) -> ThreadAccessor {
            *self
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::marker::Copy for ThreadAccessor {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::marker::StructuralPartialEq for ThreadAccessor {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::cmp::PartialEq for ThreadAccessor {
        #[inline]
        fn eq(&self, other: &ThreadAccessor) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::cmp::Eq for ThreadAccessor {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::hash::Hash for ThreadAccessor {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    impl ::wasip1_virtual_layer::thread::ThreadAccess for ThreadAccessor {
        fn to_correct_memory(
            &self,
            ptr: ::wasip1_virtual_layer::__private::inner::thread::ThreadRunnerBase,
        ) -> ::wasip1_virtual_layer::thread::ThreadRunner {
            {
                match *self {
                    __self => ptr.apply::<::wasip1_virtual_layer::__private::__self>(),
                    __test_threads => ptr.apply::<test_threads>(),
                }
            }
        }
        fn call_wasi_thread_start(
            &self,
            ptr: ::wasip1_virtual_layer::thread::ThreadRunner,
            thread_id: Option<core::num::NonZero<u32>>,
        ) {
            {
                match *self {
                    Self::__self => {
                        unsafe {
                            __wasip1_vfs_self_wasi_thread_start(
                                match thread_id {
                                    Some(id) => u32::from(id) as i32,
                                    None => -1,
                                },
                                ptr.inner() as i32,
                            )
                        }
                    }
                    Self::__test_threads => {
                        unsafe {
                            __wasip1_vfs_test_threads_wasi_thread_start(
                                match thread_id {
                                    Some(id) => u32::from(id) as i32,
                                    None => -1,
                                },
                                ptr.inner() as i32,
                            )
                        }
                    }
                }
            }
        }
        fn as_name(&self) -> &'static str {
            match *self {
                __self => "self",
                __test_threads => "test_threads",
            }
        }
    }
    #[doc(hidden)]
    #[link(wasm_import_module = "wasip1-vfs")]
    unsafe extern "C" {
        #[unsafe(no_mangle)]
        pub fn __wasip1_vfs_self_wasi_thread_start(thread_id: i32, ptr: i32);
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __wasip1_vfs_wasi_thread_start_self(
        data_ptr: ::wasip1_virtual_layer::__private::inner::thread::ThreadRunnerBase,
    ) {
        use ::wasip1_virtual_layer::thread::{VirtualThread, ThreadAccess};
        #[allow(unused_mut)]
        let mut pool = DirectThreadPool;
        const ACCESSOR: ThreadAccessor = ThreadAccessor::__self;
        pool.new_thread(ACCESSOR, ACCESSOR.to_correct_memory(data_ptr));
    }
    #[doc(hidden)]
    #[link(wasm_import_module = "wasip1-vfs")]
    unsafe extern "C" {
        #[unsafe(no_mangle)]
        pub fn __wasip1_vfs_test_threads_wasi_thread_start(thread_id: i32, ptr: i32);
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __wasip1_vfs_wasi_thread_start_test_threads(
        data_ptr: ::wasip1_virtual_layer::__private::inner::thread::ThreadRunnerBase,
    ) {
        use ::wasip1_virtual_layer::thread::{VirtualThread, ThreadAccess};
        #[allow(unused_mut)]
        let mut pool = DirectThreadPool;
        const ACCESSOR: ThreadAccessor = ThreadAccessor::__test_threads;
        pool.new_thread(ACCESSOR, ACCESSOR.to_correct_memory(data_ptr));
    }
}
mod tests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "tests::test_files"]
    #[doc(hidden)]
    pub const test_files: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::test_files"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "examples\\vfs\\threads_vfs\\src\\lib.rs",
            start_line: 55usize,
            start_col: 8usize,
            end_line: 55usize,
            end_col: 18usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_files()),
        ),
    };
    fn test_files() {
        {
            ::std::io::_print(format_args!("Files: {0:#?}\n", FILES));
        };
    }
}
mod fs {
    use super::*;
    type LFS = VFSConstNormalLFS<FilesTy, F, FILE_COUNT, DefaultStdIO>;
    static mut VIRTUAL_FILE_SYSTEM: Wasip1ConstVFS<LFS, FILE_COUNT> = Wasip1ConstVFS::new(
        VFSConstNormalLFS::new(),
    );
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __wasip1_vfs_test_threads_fd_write(
        fd: ::wasip1_virtual_layer::__private::wasip1::Fd,
        iovs_ptr: *const ::wasip1_virtual_layer::__private::wasip1::Ciovec,
        iovs_len: usize,
        nwritten: *mut usize,
    ) -> ::wasip1_virtual_layer::__private::wasip1::Errno {
        let state = { #[allow(static_mut_refs)] unsafe { &mut VIRTUAL_FILE_SYSTEM } };
        ::wasip1_virtual_layer::file::Wasip1FileSystem::fd_write_raw::<
            test_threads,
        >(state, fd, iovs_ptr, iovs_len, nwritten)
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __wasip1_vfs_test_threads_fd_readdir(
        fd: ::wasip1_virtual_layer::__private::wasip1::Fd,
        buf: *mut u8,
        buf_len: usize,
        cookie: ::wasip1_virtual_layer::__private::wasip1::Dircookie,
        nread: *mut ::wasip1_virtual_layer::__private::wasip1::Size,
    ) -> ::wasip1_virtual_layer::__private::wasip1::Errno {
        let state = { #[allow(static_mut_refs)] unsafe { &mut VIRTUAL_FILE_SYSTEM } };
        ::wasip1_virtual_layer::file::Wasip1FileSystem::fd_readdir_raw::<
            test_threads,
        >(state, fd, buf, buf_len, cookie, nread)
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __wasip1_vfs_test_threads_path_filestat_get(
        fd: ::wasip1_virtual_layer::__private::wasip1::Fd,
        flags: ::wasip1_virtual_layer::__private::wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
        filestat: *mut ::wasip1_virtual_layer::__private::wasip1::Filestat,
    ) -> ::wasip1_virtual_layer::__private::wasip1::Errno {
        let state = { #[allow(static_mut_refs)] unsafe { &mut VIRTUAL_FILE_SYSTEM } };
        ::wasip1_virtual_layer::file::Wasip1FileSystem::path_filestat_get_raw::<
            test_threads,
        >(state, fd, flags, path_ptr, path_len, filestat)
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __wasip1_vfs_test_threads_fd_prestat_get(
        fd: ::wasip1_virtual_layer::__private::wasip1::Fd,
        prestat: *mut ::wasip1_virtual_layer::__private::wasip1::Prestat,
    ) -> ::wasip1_virtual_layer::__private::wasip1::Errno {
        let state = { #[allow(static_mut_refs)] unsafe { &mut VIRTUAL_FILE_SYSTEM } };
        ::wasip1_virtual_layer::file::Wasip1FileSystem::fd_prestat_get_raw::<
            test_threads,
        >(state, fd, prestat)
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __wasip1_vfs_test_threads_fd_prestat_dir_name(
        fd: ::wasip1_virtual_layer::__private::wasip1::Fd,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> ::wasip1_virtual_layer::__private::wasip1::Errno {
        let state = { #[allow(static_mut_refs)] unsafe { &mut VIRTUAL_FILE_SYSTEM } };
        ::wasip1_virtual_layer::file::Wasip1FileSystem::fd_prestat_dir_name_raw::<
            test_threads,
        >(state, fd, dir_path_ptr, dir_path_len)
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __wasip1_vfs_test_threads_fd_close(
        fd: ::wasip1_virtual_layer::__private::wasip1::Fd,
    ) -> ::wasip1_virtual_layer::__private::wasip1::Errno {
        let state = { #[allow(static_mut_refs)] unsafe { &mut VIRTUAL_FILE_SYSTEM } };
        ::wasip1_virtual_layer::file::Wasip1FileSystem::fd_close_raw::<
            test_threads,
        >(state, fd)
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __wasip1_vfs_test_threads_path_open(
        fd: ::wasip1_virtual_layer::__private::wasip1::Fd,
        dir_flags: ::wasip1_virtual_layer::__private::wasip1::Fdflags,
        path_ptr: *const u8,
        path_len: usize,
        o_flags: ::wasip1_virtual_layer::__private::wasip1::Oflags,
        fs_rights_base: ::wasip1_virtual_layer::__private::wasip1::Rights,
        fs_rights_inheriting: ::wasip1_virtual_layer::__private::wasip1::Rights,
        fd_flags: ::wasip1_virtual_layer::__private::wasip1::Fdflags,
        fd_ret: *mut ::wasip1_virtual_layer::__private::wasip1::Fd,
    ) -> ::wasip1_virtual_layer::__private::wasip1::Errno {
        let state = { #[allow(static_mut_refs)] unsafe { &mut VIRTUAL_FILE_SYSTEM } };
        ::wasip1_virtual_layer::file::Wasip1FileSystem::path_open_raw::<
            test_threads,
        >(
            state,
            fd,
            dir_flags,
            path_ptr,
            path_len,
            o_flags,
            fs_rights_base,
            fs_rights_inheriting,
            fd_flags,
            fd_ret,
        )
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __wasip1_vfs_test_threads_fd_read(
        fd: ::wasip1_virtual_layer::__private::wasip1::Fd,
        iovs_ptr: *const ::wasip1_virtual_layer::__private::wasip1::Ciovec,
        iovs_len: usize,
        nread_ret: *mut ::wasip1_virtual_layer::__private::wasip1::Size,
    ) -> ::wasip1_virtual_layer::__private::wasip1::Errno {
        let state = { #[allow(static_mut_refs)] unsafe { &mut VIRTUAL_FILE_SYSTEM } };
        ::wasip1_virtual_layer::file::Wasip1FileSystem::fd_read_raw::<
            test_threads,
        >(state, fd, iovs_ptr, iovs_len, nread_ret)
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __wasip1_vfs_test_threads_fd_filestat_get(
        fd: ::wasip1_virtual_layer::__private::wasip1::Fd,
        filestat: *mut ::wasip1_virtual_layer::__private::wasip1::Filestat,
    ) -> ::wasip1_virtual_layer::__private::wasip1::Errno {
        let state = { #[allow(static_mut_refs)] unsafe { &mut VIRTUAL_FILE_SYSTEM } };
        ::wasip1_virtual_layer::file::Wasip1FileSystem::fd_filestat_get_raw::<
            test_threads,
        >(state, fd, filestat)
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&test_files])
}
