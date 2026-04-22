#![cfg(feature = "embedded-fs")]

use crate::__private::wasip1;
use const_struct::ConstStruct;

use crate::{memory::WasmAccess, wasi::file::Wasip1FileTrait};

/// A constant file system root that can be used in a WASI component.
/// A constant file system structure for use in WASM modules.
#[derive(ConstStruct, Debug)]
pub struct StandardEmbeddedFiles<File: Wasip1FileTrait + 'static + Copy, const FLAT_LEN: usize> {
    /// The flattened array of files and directories.
    pub files: [(&'static str, StandardEmbeddedInode<File>); FLAT_LEN],
    /// Indices of pre-opened directories.
    pub pre_open: &'static [usize],
}

impl<File: Wasip1FileTrait + 'static + Copy, const FLAT_LEN: usize>
    StandardEmbeddedFiles<File, FLAT_LEN>
{
    /// Creates a new `StandardEmbeddedFiles`.
    pub const fn new_const(
        files: (
            [(&'static str, StandardEmbeddedInode<File>); FLAT_LEN],
            &'static [usize],
        ),
    ) -> Self {
        Self {
            files: files.0,
            pre_open: files.1,
        }
    }
}

/// An inode in a constant VFS.
#[derive(Clone, Copy, Debug)]
pub enum StandardEmbeddedInode<File: Wasip1FileTrait + 'static + Copy> {
    /// A regular file with its associated file data and parent index.
    File(File, usize),
    /// A directory with its child range and optional parent index.
    Dir((usize, usize), Option<usize>),
}

impl<File: Wasip1FileTrait + 'static + Copy> StandardEmbeddedInode<File> {
    /// Returns the WASI file type of the inode.
    pub const fn filetype(&self) -> wasip1::Filetype {
        match self {
            Self::File(..) => wasip1::FILETYPE_REGULAR_FILE,
            Self::Dir(..) => wasip1::FILETYPE_DIRECTORY,
        }
    }

    /// Returns the size of the inode.
    pub fn size(&self) -> usize {
        match self {
            Self::File(file, _) => file.size(),
            Self::Dir(..) => core::mem::size_of::<((usize, usize), Option<usize>)>(), // directory size is just the size of the inode
        }
    }

    /// Returns the parent inode index, if any.
    pub fn parent(&self) -> Option<usize> {
        match self {
            Self::File(_, parent) => Some(*parent),
            Self::Dir(_, parent) => *parent,
        }
    }
}

/// Macro for defining constant file systems.
#[macro_export]
macro_rules! EmbeddedFiles {
    (
        [
            $(($dir_name:expr, $file_or_dir:tt $(,)?)),* $(,)?
        ] $(,)?
    ) => {
        $crate::__private::inner::fs::StandardEmbeddedFiles::new_const({
            const COUNT: usize = {
                let mut count = 0;

                $(
                    $crate::EmbeddedFiles!(@counter, count, $file_or_dir);
                )*

                count
            };

            struct CheckEqNumberOfFilesAndDirs<const L: usize, const R: usize>;

            #[allow(dead_code)]
            impl<const L: usize, const R: usize> CheckEqNumberOfFilesAndDirs<L, R> {
                #[allow(non_upper_case_globals)]
                const number_of_files_and_dirs_equals_FLAT_LEN_so_you_must_set_StandardEmbeddedFiles_num: usize = (R - L) + (L - R);
            }

            const fn asserter<S: 'static + Copy, const N: usize>(
                _: &[S; N],
            ) {
                #[allow(path_statements)]
                CheckEqNumberOfFilesAndDirs::<COUNT, N>::number_of_files_and_dirs_equals_FLAT_LEN_so_you_must_set_StandardEmbeddedFiles_num;
            }

            use $crate::__private::const_for;

            const fn eq_str(a: &str, b: &str) -> bool {
                let a_bytes = a.as_bytes();
                let b_bytes = b.as_bytes();

                if a_bytes.len() != b_bytes.len() {
                    return false;
                }

                const_for!(i in 0..a_bytes.len() => {
                    if a_bytes[i] != b_bytes[i] {
                        return false;
                    }
                });

                true
            }

            /// if b is a parent of a
            const fn is_parent(a: &str, b: &str) -> bool {
                let a_bytes = a.as_bytes();
                let b_bytes = b.as_bytes();

                if a_bytes.len() < b_bytes.len() {
                    return false;
                }

                const_for!(i in 0..b_bytes.len() => {
                    if a_bytes[i] != b_bytes[i] {
                        return false;
                    }
                });

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

                const_for!(n in i..a_bytes.len() => {
                    if a_bytes[n] == b"/"[0] {
                        return false;
                    }
                });

                true
            }

            const fn get_child_range<S: 'static + Copy, const N: usize>(
                fake_files: [&'static str; N],
                name: &'static str,
                _: &$crate::__private::utils::EmbeddedArrayBuilder<S, N>,
            ) -> (usize, usize) {
                get_child_range_inner(fake_files, name)
            }

            const fn get_child_range_inner<const N: usize>(
                fake_files: [&'static str; N],
                name: &'static str,
            ) -> (usize, usize) {
                let mut first_index = None;
                let mut last_index = None;
                const_for!(i in 0..N => {
                    if is_parent(fake_files[i], name) {
                        if first_index.is_none() {
                            first_index = Some(i);
                        }
                        last_index = Some(i);
                    }
                });

                (first_index.unwrap(), last_index.unwrap() + 1)
            }

            const fn get_parent<S: 'static + Copy, const N: usize>(
                fake_files: [&'static str; N],
                name: &'static str,
                _: &$crate::__private::utils::EmbeddedArrayBuilder<S, N>,
            ) -> Option<usize> {
                const_for!(i in 0..N => {
                    if is_parent(name, fake_files[i]) {
                        return Some(i);
                    }
                });
                None
            }

            const fn get_self<const N: usize>(
                fake_files: [&'static str; N],
                name: &'static str,
            ) -> usize {
                const_for!(i in 0..N => {
                    if eq_str(name, fake_files[i]) {
                        return i;
                    }
                });
                unreachable!()
            }

            const fn custom_sort<T: Copy, const N: usize>(
                mut files: $crate::__private::utils::EmbeddedArrayBuilder<(usize, T), N>,
            ) -> [T; N] {
                let mut sorted = $crate::__private::utils::EmbeddedArrayBuilder::<_, N>::new_const();

                while (files.len() > 0) {
                    let mut depth = None;
                    let mut index = None;
                    const_for!(i in 0..files.len() => {
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
                    });
                    if let Some(index) = index {
                        sorted.push(index.1);
                        files.remove(index.0);
                    }
                }

                sorted.build()
            }

            const EMPTY_ARR: [&'static str; COUNT] = {
                let mut empty_arr = $crate::__private::utils::EmbeddedArrayBuilder::new_const();

                $(
                    $crate::EmbeddedFiles!(@empty, 0, empty_arr, [$dir_name], $file_or_dir);
                )*

                let _ = empty_arr.build();

                custom_sort(empty_arr)
            };

            let flatten = {
                let mut static_array = $crate::__private::utils::EmbeddedArrayBuilder::new_const();

                $(
                    $crate::EmbeddedFiles!(
                        @next,
                        0,
                        static_array,
                        [EMPTY_ARR],
                        [$dir_name],
                        [$dir_name],
                        $file_or_dir
                    );
                )*

                let static_array = custom_sort(static_array);

                let mut file_array = $crate::__private::utils::EmbeddedArrayBuilder::new_const();
                const_for!(i in 0..static_array.len() => {
                    let (_, name, file_or_dir) = static_array[i];
                    file_array.push((
                        name,
                        file_or_dir
                    ));
                });

                let static_array = file_array.build_with_is_check(file_array.check_len());

                let _ = asserter(&static_array);

                static_array
            };

            const PRE_OPEN_COUNT: usize = {
                let mut count = 0;

                $(
                    $crate::EmbeddedFiles!(@pre_open_counter, count, $file_or_dir);
                )*
                count
            };

            const PRE_OPEN: [usize; PRE_OPEN_COUNT] = {
                let mut static_array = $crate::__private::utils::EmbeddedArrayBuilder::new_const();

                $(
                    static_array.push(get_self(EMPTY_ARR, $dir_name));
                )*

                static_array.build()
            };

            (flatten, &PRE_OPEN)
        })
    };

    // failed catch this code
    // [
    //     ("hey", WasiEmbeddedFile::new("Hey!")),
    //     (
    //         "hello",
    //         [
    //             ("world", WasiEmbeddedFile::new("Hello, world!")),
    //             ("everyone", WasiEmbeddedFile::new("Hello, everyone!")),
    //             ("every", WasiEmbeddedFile::new("Hello, every!"))
    //         ]
    //     )
    // ]

    (@counter, $count:ident, [
        $( ($file_or_dir_name:expr, $file_or_dir:tt) $(,)? ),* $(,)?
    ] $(,)?) => {
        $(
            $crate::EmbeddedFiles!(@counter, $count, $file_or_dir);
        )*
        $count += 1;
    };

    (@counter, $count:ident, [
        $( $all:tt ),* $(,)?
    ] $(,)?) => {
        $(
            $crate::EmbeddedFiles!(@counter2, $count, $all);
        )*
        $count += 1;
    };

    (@counter2, $count:ident,
        ($file_or_dir_name:tt, $file_or_dir:tt $(,)?)
    ) => {
        $crate::EmbeddedFiles!(@counter, $count, $file_or_dir);
    };

    (@counter2, $count:ident,
        ($file_or_dir_name:tt, $file_or_dir:stmt $(,)?)
    ) => {
        $crate::EmbeddedFiles!(@counter, $count, { $file_or_dir });
    };

    (@counter, $count:ident, $file:tt) => {
        $count += 1;
    };

    (@pre_open_counter, $count:ident, $file:tt) => {
        $count += 1;
    };

    (@empty, $depth:expr, $empty_arr:ident, [$parent_name:expr], [
        $(($file_or_dir_name:expr, $file_or_dir:tt $(,)?)),* $(,)?
    ] $(,)?) => {
        $(
            $crate::EmbeddedFiles!(@empty, $depth + 1, $empty_arr, [concat!($parent_name, "/", $file_or_dir_name)], $file_or_dir);
        )*
        $empty_arr.push(($depth, $parent_name));
    };

    (@empty, $depth:expr, $empty_arr:ident, [$parent_name:expr], [
        $($all:tt),* $(,)?
    ] $(,)?) => {
        $(
            $crate::EmbeddedFiles!(@empty2, $depth, $empty_arr, [$parent_name], $all);
        )*
        $empty_arr.push(($depth, $parent_name));
    };

    (@empty2, $depth:expr, $empty_arr:ident, [$parent_name:expr],
        ($file_or_dir_name:tt, $file_or_dir:tt $(,)?)
    ) => {
        $crate::EmbeddedFiles!(@empty, $depth + 1, $empty_arr, [concat!($parent_name, "/", $file_or_dir_name)], $file_or_dir);
    };

    // `ident`, `block`, `stmt`, `expr`, `pat`, `ty`, `lifetime`, `literal`, `path`, `meta`, `tt`, `item` and `vis`
    (@empty2, $depth:expr, $empty_arr:ident, [$parent_name:expr],
        ($file_or_dir_name:tt, $file_or_dir:stmt $(,)?)
    ) => {
        $crate::EmbeddedFiles!(@empty, $depth + 1, $empty_arr, [concat!($parent_name, "/", $file_or_dir_name)], { $file_or_dir });
    };

    (@empty, $depth:expr, $empty_arr:ident, [$name:expr], $file:tt) => {
        $empty_arr.push(($depth, $name));
    };

    (@next, $depth:expr, $static_array:ident, [$empty:expr], [$parent_path:expr], [$name:expr], [
        $(($file_or_dir_name:expr, $file_or_dir:tt $(,)?)),* $(,)?
    ] $(,)?) => {
        $(
            $crate::EmbeddedFiles!(@next, $depth + 1, $static_array, [$empty], [concat!($parent_path, "/", $file_or_dir_name)], [$file_or_dir_name], $file_or_dir);
        )*
        $static_array.push(($depth, (
            $parent_path,
            $name,
            $crate::__private::inner::fs::StandardEmbeddedInode::Dir(
                get_child_range(
                    $empty,
                    $parent_path,
                    &$static_array
                ),
                get_parent($empty, $parent_path, &$static_array)
            )
        )));
    };

    (@next, $depth:expr, $static_array:ident, [$empty:expr], [$parent_path:expr], [$name:expr], [
        $($all:tt),* $(,)?
    ] $(,)?) => {
        $(
            $crate::EmbeddedFiles!(@next2, $depth, $static_array, [$empty], [$parent_path], [$name], $all);
        )*
        $static_array.push(($depth, (
            $parent_path,
            $name,
            $crate::__private::inner::fs::StandardEmbeddedInode::Dir(
                get_child_range(
                    $empty,
                    $parent_path,
                    &$static_array
                ),
                get_parent($empty, $parent_path, &$static_array)
            )
        )));
    };

    (@next2, $depth:expr, $static_array:ident, [$empty:expr], [$parent_path:expr], [$name:expr],
        ($file_or_dir_name:tt, $file_or_dir:tt $(,)?)
    ) => {
        $crate::EmbeddedFiles!(@next, $depth + 1, $static_array, [$empty], [concat!($parent_path, "/", $file_or_dir_name)], [$file_or_dir_name], $file_or_dir);
    };

    (@next2, $depth:expr, $static_array:ident, [$empty:expr], [$parent_path:expr], [$name:expr],
        ($file_or_dir_name:tt, $file_or_dir:stmt $(,)?)
    ) => {
        $crate::EmbeddedFiles!(@next, $depth + 1, $static_array, [$empty], [concat!($parent_path, "/", $file_or_dir_name)], [$file_or_dir_name], { $file_or_dir });
    };

    (@next, $depth:expr, $static_array:ident, [$empty:expr], [$path:expr], [$name:expr], $file:tt) => {
        $static_array.push((
            $depth,
            (
                $path,
                $name,
            $crate::__private::inner::fs::StandardEmbeddedInode::File($file, get_parent($empty, $path, &$static_array).unwrap())
        )));
    };
}

/// A wrapper for constant file data.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WasiEmbeddedFile<File: WasiEmbeddedPrimitiveFile> {
    /// The underlying file data.
    pub file: File,
}

impl<File: WasiEmbeddedPrimitiveFile> WasiEmbeddedFile<File> {
    /// Creates a new `WasiEmbeddedFile`.
    pub const fn new_const(file: File) -> Self {
        Self { file }
    }

    pub const fn new(file: File) -> Self {
        Self { file }
    }
}

/// Trait for constant primitive file data.
pub trait WasiEmbeddedPrimitiveFile: core::fmt::Debug {
    /// Returns the length of the file data.
    fn len(&self) -> usize;
    /// Reads raw data from the file.
    fn pread_raw<Wasm: WasmAccess>(
        &self,
        buf_ptr: *mut u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<usize, wasip1::Errno>;
}

impl<'a> WasiEmbeddedPrimitiveFile for &'a str {
    #[inline(always)]
    fn len(&self) -> usize {
        <Self as core::ops::Deref>::deref(self).len()
    }

    #[inline(always)]
    fn pread_raw<Wasm: WasmAccess>(
        &self,
        buf_ptr: *mut u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<usize, wasip1::Errno> {
        let buf_len = core::cmp::min(buf_len, self.len() - offset);
        Wasm::memcpy(buf_ptr, &self.as_bytes()[offset..offset + buf_len]);
        Ok(buf_len)
    }
}

impl<File: WasiEmbeddedPrimitiveFile> Wasip1FileTrait for WasiEmbeddedFile<File> {
    fn size(&self) -> usize {
        self.file.len()
    }

    fn pread_raw<Wasm: WasmAccess>(
        &self,
        buf_ptr: *mut u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<usize, wasip1::Errno> {
        self.file.pread_raw::<Wasm>(buf_ptr, buf_len, offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// If not using `--release`, compilation will fail with: link error
    /// cargo test -r --package wasi_virt_layer --lib -- wasi::file::tests::test_file_flat_iterate --exact --show-output
    #[test]
    fn test_file_flat_iterate() {
        #[allow(dead_code)]
        const EMBEDDED_FILES: StandardEmbeddedFiles<WasiEmbeddedFile<&'static str>, 10> = EmbeddedFiles!([
            (
                "/root",
                [("root.txt", WasiEmbeddedFile::new_const("This is root"))]
            ),
            (
                ".",
                [
                    ("hey", WasiEmbeddedFile::new_const("Hey!")),
                    (
                        "hello",
                        [
                            ("world", WasiEmbeddedFile::new_const("Hello, world!")),
                            ("everyone", WasiEmbeddedFile::new_const("Hello, everyone!")),
                        ]
                    )
                ]
            ),
            (
                "~",
                [
                    ("home", WasiEmbeddedFile::new_const("This is home")),
                    ("user", WasiEmbeddedFile::new_const("This is user")),
                ]
            )
        ]);

        #[cfg(feature = "std")]
        println!("Files: {:#?}", FILES);
    }
}

