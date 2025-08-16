mod ex {
    use crate::{
        ConstFiles, wasi::file::non_atomic::{VFSConstNormalFiles, WasiConstFile},
    };
    const FILES: VFSConstNormalFiles<WasiConstFile<&'static str>, 9> = crate::wasi::file::non_atomic::VFSConstNormalFiles::new({
        const COUNT: usize = {
            let mut count = 0;
            count += 1;
            count += 1;
            count += 1;
            count += 1;
            count += 1;
            count += 1;
            count += 1;
            count += 1;
            count += 1;
            count += 1;
            count
        };
        let mut static_array = crate::binary_map::StaticArrayBuilder::new();
        struct CheckEqNumberOfFilesAndDirs<const L: usize, const R: usize>;
        #[allow(dead_code)]
        impl<const L: usize, const R: usize> CheckEqNumberOfFilesAndDirs<L, R> {
            #[allow(non_upper_case_globals)]
            const number_of_files_and_dirs_equals_FLAT_LEN_so_you_must_set_VFSConstNormalFiles_num: usize = (R
                - L) + (L - R);
        }
        const fn asserter<S: 'static + Copy, const N: usize>(
            _: &crate::binary_map::StaticArrayBuilder<S, N>,
        ) {
            #[allow(path_statements)]
            CheckEqNumberOfFilesAndDirs::<
                COUNT,
                N,
            >::number_of_files_and_dirs_equals_FLAT_LEN_so_you_must_set_VFSConstNormalFiles_num;
        }
        asserter(&static_array);
        use crate::__private::const_for;
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
            _: &crate::binary_map::StaticArrayBuilder<S, N>,
        ) -> (usize, usize) {
            let mut first_index = None;
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
                        if first_index.is_none() && is_parent(fake_files[i], name) {
                            first_index = Some(i);
                        }
                        if eq_str(fake_files[i], name) {
                            return (first_index.unwrap(), i);
                        }
                    }
                }
            };
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        const fn get_parent<S: 'static + Copy, const N: usize>(
            fake_files: [&'static str; N],
            name: &'static str,
            _: &crate::binary_map::StaticArrayBuilder<S, N>,
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
                        if is_parent(name, fake_files[i]) {
                            return i;
                        }
                    }
                }
            };
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        let empty_arr = {
            let mut empty_arr = crate::binary_map::StaticArrayBuilder::new();
            empty_arr.push("/root/root.txt");
            empty_arr.push("/root");
            empty_arr.push("./hey");
            empty_arr.push("./hello/world");
            empty_arr.push("./hello/everyone");
            empty_arr.push("./hello");
            empty_arr.push(".");
            empty_arr.push("~/home");
            empty_arr.push("~/user");
            empty_arr.push("~");
            empty_arr.build()
        };
        static_array
            .push((
                "/root/root.txt",
                crate::wasi::file::non_atomic::VFSConstNormalInode::File((
                    { WasiConstFile::new("This is root") },
                    get_parent(empty_arr, "/root/root.txt", &static_array),
                )),
            ));
        static_array
            .push((
                "/root",
                crate::wasi::file::non_atomic::VFSConstNormalInode::Dir(
                    get_child_range(empty_arr, "/root", &static_array),
                ),
            ));
        static_array
            .push((
                "./hey",
                crate::wasi::file::non_atomic::VFSConstNormalInode::File((
                    { WasiConstFile::new("Hey!") },
                    get_parent(empty_arr, "./hey", &static_array),
                )),
            ));
        static_array
            .push((
                "./hello/world",
                crate::wasi::file::non_atomic::VFSConstNormalInode::File((
                    { WasiConstFile::new("Hello, world!") },
                    get_parent(empty_arr, "./hello/world", &static_array),
                )),
            ));
        static_array
            .push((
                "./hello/everyone",
                crate::wasi::file::non_atomic::VFSConstNormalInode::File((
                    { WasiConstFile::new("Hello, everyone!") },
                    get_parent(empty_arr, "./hello/everyone", &static_array),
                )),
            ));
        static_array
            .push((
                "./hello",
                crate::wasi::file::non_atomic::VFSConstNormalInode::Dir(
                    get_child_range(empty_arr, "./hello", &static_array),
                ),
            ));
        static_array
            .push((
                ".",
                crate::wasi::file::non_atomic::VFSConstNormalInode::Dir(
                    get_child_range(empty_arr, ".", &static_array),
                ),
            ));
        static_array
            .push((
                "~/home",
                crate::wasi::file::non_atomic::VFSConstNormalInode::File((
                    { WasiConstFile::new("This is home") },
                    get_parent(empty_arr, "~/home", &static_array),
                )),
            ));
        static_array
            .push((
                "~/user",
                crate::wasi::file::non_atomic::VFSConstNormalInode::File((
                    { WasiConstFile::new("This is user") },
                    get_parent(empty_arr, "~/user", &static_array),
                )),
            ));
        static_array
            .push((
                "~",
                crate::wasi::file::non_atomic::VFSConstNormalInode::Dir(
                    get_child_range(empty_arr, "~", &static_array),
                ),
            ));
        static_array.build()
    });
}
