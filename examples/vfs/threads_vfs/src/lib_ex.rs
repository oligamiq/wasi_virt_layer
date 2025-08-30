#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use const_struct::const_struct;
use wasip1_virtual_layer::{self, wasi::file::constant::lfs_raw::*, *};
const FILE_COUNT: usize = 5;
type F = WasiConstFile<&'static str>;
type NormalFILES = VFSConstNormalFiles<F, { FILE_COUNT }>;
const FILES: NormalFILES =
    ::wasip1_virtual_layer::wasi::file::constant::lfs_raw::VFSConstNormalFiles::new({
        // const COUNT: usize = {
        //     let mut count = 0;
        //     ConstFiles!(@counter, count, [
        //         ("hey", F::new("Hey!")),
        //         (
        //             "hello",
        //             [
        //                 ("world", F::new("Hello, world!")),
        //                 ("everyone", F::new("Hello, everyone!")),
        //             ],
        //         )
        //     ]);
        //     count
        // };

        // const COUNT: usize = {
        //     let mut count = 0;
        //     ConstFiles!(@counter, count, F::new("Hey!"));
        //     ConstFiles!(@counter, count, [
        //         ("world", F::new("Hello, world!")),
        //         ("everyone", F::new("Hello, everyone!")),
        //     ]);
        //     count += 1;
        //     count
        // };

        // const COUNT: usize = {
        //     let mut count = 0;
        //     count += 1;
        //     ConstFiles!(@counter, count, F::new("Hello, world!"));
        //     ConstFiles!(@counter, count, F::new("Hello, everyone!"));
        //     count += 1;
        //     count += 1;
        //     count
        // };

        // const COUNT: usize = {
        //     let mut count = 0;
        //     ConstFiles!(@counter2, count, ("hey", F::new("Hey!")));
        //     ConstFiles!(@counter2, count, (
        //         "hello",
        //         [
        //             ("world", F::new("Hello, world!")),
        //             ("everyone", F::new("Hello, everyone!")),
        //         ],
        //     ));
        //     count += 1;
        //     count
        // };

        const COUNT: usize = {
            let mut count = 0;
            ConstFiles!(@counter, count, { F::new("Hey!") });
            ConstFiles!(@counter, count, [
                ("world", F::new("Hello, world!")),
                ("everyone", F::new("Hello, everyone!")),
            ]);
            count += 1;
            count
        };

        let mut static_array = ::wasip1_virtual_layer::binary_map::StaticArrayBuilder::new();
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
            _: &::wasip1_virtual_layer::binary_map::StaticArrayBuilder<S, N>,
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
            _: &::wasip1_virtual_layer::binary_map::StaticArrayBuilder<S, N>,
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
            mut files: ::wasip1_virtual_layer::binary_map::StaticArrayBuilder<(usize, T), N>,
        ) -> [T; N] {
            let mut sorted = ::wasip1_virtual_layer::binary_map::StaticArrayBuilder::<_, N>::new();
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
            let mut empty_arr = ::wasip1_virtual_layer::binary_map::StaticArrayBuilder::new();
            empty_arr.push((0 + 1, "./hey"));
            empty_arr.push((0 + 1 + 1, "./hello/world"));
            empty_arr.push((0 + 1 + 1, "./hello/everyone"));
            empty_arr.push((0 + 1, "./hello"));
            empty_arr.push((0, "."));
            let _ = empty_arr.build();
            custom_sort(empty_arr)
        };
        static_array.push((
            0 + 1,
            (
                "./hey",
                "hey",
                ::wasip1_virtual_layer::wasi::file::constant::lfs_raw::VFSConstNormalInode::File(
                    { F::new("Hey!") },
                    get_parent(EMPTY_ARR, "./hey", &static_array).unwrap(),
                ),
            ),
        ));
        static_array.push((
            0 + 1 + 1,
            (
                "./hello/world",
                "world",
                ::wasip1_virtual_layer::wasi::file::constant::lfs_raw::VFSConstNormalInode::File(
                    { F::new("Hello, world!") },
                    get_parent(EMPTY_ARR, "./hello/world", &static_array).unwrap(),
                ),
            ),
        ));
        static_array.push((
            0 + 1 + 1,
            (
                "./hello/everyone",
                "everyone",
                ::wasip1_virtual_layer::wasi::file::constant::lfs_raw::VFSConstNormalInode::File(
                    { F::new("Hello, everyone!") },
                    get_parent(EMPTY_ARR, "./hello/everyone", &static_array).unwrap(),
                ),
            ),
        ));
        static_array.push((
            0 + 1,
            (
                "./hello",
                "hello",
                ::wasip1_virtual_layer::wasi::file::constant::lfs_raw::VFSConstNormalInode::Dir(
                    get_child_range(EMPTY_ARR, "./hello", &static_array),
                    get_parent(EMPTY_ARR, "./hello", &static_array),
                ),
            ),
        ));
        static_array.push((
            0,
            (
                ".",
                ".",
                ::wasip1_virtual_layer::wasi::file::constant::lfs_raw::VFSConstNormalInode::Dir(
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
            let mut static_array = ::wasip1_virtual_layer::binary_map::StaticArrayBuilder::new();
            static_array.push(get_self(EMPTY_ARR, "."));
            static_array.build()
        };
        let static_array = custom_sort(static_array);
        let mut file_array = ::wasip1_virtual_layer::binary_map::StaticArrayBuilder::new();
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
use wasip1_virtual_layer::wasi::file::constant::lfs_raw::VFSConstNormalInode::Dir;
use wasip1_virtual_layer::wasi::file::constant::lfs_raw::VFSConstNormalInode::File;
#[allow(dead_code)]
const FILE_EX: NormalFILES = VFSConstNormalFiles {
    files: [
        (".", Dir((1, 3), None)),
        ("hey", File(F::new("Hey!"), 0)),
        ("hello", Dir((3, 5), Some(0))),
        ("world", File(F::new("Hello, world!"), 2)),
        ("everyone", File(F::new("Hello, everyone!"), 2)),
    ],
    pre_open: &[0],
};
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
            start_line: 84usize,
            start_col: 8usize,
            end_line: 84usize,
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
            ::std::io::_print(format_args!("Files: {0:?}\n", FILES));
        };
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&test_files])
}
