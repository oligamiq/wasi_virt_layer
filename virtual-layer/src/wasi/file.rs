// https://docs.rs/wasmtime-wasi/17.0.3/wasmtime_wasi/struct.WasiCtx.html
// https://docs.rs/wasi-common/17.0.3/wasi_common/table/struct.Table.html

use crate::memory::WasmAccess;

// no implementing dcache

#[cfg(not(target_feature = "atomics"))]
pub mod non_atomic {
    use dashmap::DashMap;
    use std::{collections::HashMap, hash::Hash, io::Write as _};
    use wasip1::*;

    /// small posix like virtual file system
    /// but inode has some metadata
    pub struct Wasip1VFS<LFS: Wasip1LFS> {
        lfs: DashMap<Device, LFS>,
        map: DashMap<Fd, LFS::Inode>,
    }

    /// small posix like local file system
    pub trait Wasip1LFS {
        type Inode;
    }

    pub struct VFSConstNormalLFS<
        ConstRoot: VFSConstNormalFilesTy<File, FLAT_LEN>,
        File: Wasip1FileTrait + 'static + Copy,
        const FLAT_LEN: usize,
        StdIo: StdIO + 'static,
    > {
        add_info: [VFSConstNormalAddInfo; FLAT_LEN],
        __marker: std::marker::PhantomData<(ConstRoot, File, StdIo)>,
    }

    impl<
        ConstRoot: VFSConstNormalFilesTy<File, FLAT_LEN>,
        File: Wasip1FileTrait + 'static + Copy,
        const FLAT_LEN: usize,
        StdIo: StdIO + 'static,
    > VFSConstNormalLFS<ConstRoot, File, FLAT_LEN, StdIo>
    {
        pub const fn new() -> Self {
            Self {
                add_info: [VFSConstNormalAddInfo::new(); FLAT_LEN],
                __marker: std::marker::PhantomData,
            }
        }
    }

    impl<
        ROOT: VFSConstNormalFilesTy<File, FLAT_LEN>,
        File: Wasip1FileTrait + 'static + Copy,
        const FLAT_LEN: usize,
        StdIo: StdIO + 'static,
    > Wasip1LFS for VFSConstNormalLFS<ROOT, File, FLAT_LEN, StdIo>
    {
        type Inode = &'static VFSConstNormalInodeBuilder<File>;
    }

    #[derive(Copy, Clone, Debug)]
    pub struct VFSConstNormalAddInfo {
        cursor: usize,
        atime: usize,
    }

    impl VFSConstNormalAddInfo {
        pub const fn new() -> Self {
            Self {
                cursor: 0,
                atime: 0,
            }
        }

        pub const fn with_cursor(cursor: usize) -> Self {
            Self { cursor, atime: 0 }
        }
    }

    // use wasmtime_wasi::WasiCtx;

    // impl<File: Wasip1FileTrait + 'static + Copy, const LEN: usize, StdIo: StdIO + 'static>
    //     VFSConstVFS<File, LEN, ROOT, StdIo>
    // {
    //     // I want to generate a reference from ROOTS but it doesn't work
    //     pub fn new(roots: &'static VFSConstNormalFiles<File, LEN>) -> Self {
    //         let mut map = HashMap::with_capacity(LEN);
    //         let mut next_fd = 3;

    //         for i in 0..LEN {
    //             let fd = next_fd;
    //             next_fd += 1;
    //             map.insert(fd, (roots.get_unchecked(i), VFSConstVFSAddInfo::new()));
    //         }

    //         // let add_info = roots
    //         //     .files
    //         //     .iter()
    //         //     .flat_map(|entry| entry.flat_children())
    //         //     .map(|file| {
    //         //         (
    //         //             file as *const _ as usize,
    //         //             VFSConstVFSAddInfo::new(),
    //         //         )
    //         //     })
    //         //     .collect::<HashMap<_, _>>();

    //         Self {
    //             map,
    //             next_fd,
    //             // add_info,
    //             __marker: std::marker::PhantomData,
    //         }
    //     }

    //     // pub fn get_add_info(&mut self, fd: Fd) -> Option<&mut VFSConstVFSAddInfo> {
    //     //     let (entry) = self.map.get(&fd)?;

    //     //     if let VFSConstNormalInodeBuilder::File(_, file) = entry {
    //     //         self.add_info.get_mut(&(file as *const _ as usize))
    //     //     } else {
    //     //         None
    //     //     }
    //     // }
    // }

    // impl<
    //     File: Wasip1FileTrait + 'static + Copy,
    //     const LEN: usize,
    //     ROOTS: VFSConstNormalFiles<File, LEN>,
    //     StdIo: StdIO + 'static,
    // > Wasip1FileSystem for VFSConstVFS<File, LEN, ROOTS, StdIo>
    // {
    //     fn fd_write(&mut self, fd: Fd, data: &[u8]) -> (Size, wasip1::Errno) {
    //         match fd {
    //             wasip1::FD_STDOUT => StdIo::write(&data),
    //             wasip1::FD_STDERR => StdIo::ewrite(&data),
    //             _ => {
    //                 if self.map.contains_key(&fd) {
    //                     (0, wasip1::ERRNO_PERM)
    //                 } else {
    //                     (0, wasip1::ERRNO_BADF)
    //                 }
    //             }
    //         }
    //     }

    //     fn path_open(
    //         &mut self,
    //         dir_fd: Fd,
    //         _dir_flags: wasip1::Fdflags,
    //         path: &str,
    //         o_flags: wasip1::Oflags,
    //         fs_rights_base: wasip1::Rights,
    //         _fs_rights_inheriting: wasip1::Rights,
    //         fd_flags: wasip1::Fdflags,
    //     ) -> Result<Fd, wasip1::Errno> {
    //         let dir = if let Some((dir, _)) = self.map.get(&dir_fd) {
    //             dir
    //         } else {
    //             return Err(wasip1::ERRNO_BADF);
    //         };

    //         let dir = match dir {
    //             VFSConstNormalInodeBuilder::Dir(_, dir) => dir,
    //             VFSConstNormalInodeBuilder::File(..) => {
    //                 return Err(wasip1::ERRNO_NOTDIR);
    //             }
    //         };

    //         let entry = match dir.get_entry_for_path(path) {
    //             Ok(entry) => {
    //                 if o_flags & wasip1::OFLAGS_EXCL == wasip1::OFLAGS_EXCL {
    //                     return Err(wasip1::ERRNO_EXIST);
    //                 }

    //                 if o_flags & wasip1::OFLAGS_DIRECTORY == wasip1::OFLAGS_DIRECTORY {
    //                     if let VFSConstNormalInodeBuilder::File(..) = entry {
    //                         return Err(wasip1::ERRNO_NOTDIR);
    //                     }
    //                 }

    //                 entry
    //             }
    //             Err(err) if err == wasip1::ERRNO_NOENT => {
    //                 if o_flags & wasip1::OFLAGS_CREAT == wasip1::OFLAGS_CREAT {
    //                     return Err(wasip1::ERRNO_PERM);
    //                 } else {
    //                     return Err(err);
    //                 }
    //             }
    //             Err(err) => return Err(err),
    //         };

    //         let add_info = if let VFSConstNormalInodeBuilder::File(_, file) = entry {
    //             file.path_open(o_flags, fs_rights_base, fd_flags)?;
    //             if fd_flags & wasip1::FDFLAGS_APPEND == wasip1::FDFLAGS_APPEND {
    //                 VFSConstVFSAddInfo::with_cursor(file.len())
    //             } else {
    //                 VFSConstVFSAddInfo::new()
    //             }
    //         } else {
    //             VFSConstVFSAddInfo::new()
    //         };

    //         let fd = self.next_fd;
    //         self.next_fd += 1;
    //         self.map.insert(fd, (entry, add_info));
    //         Ok(fd)
    //     }
    // }

    use const_struct::ConstStruct;

    /// A constant file system root that can be used in a WASI component.
    #[derive(ConstStruct, Debug)]
    pub struct VFSConstNormalFiles<File: Wasip1FileTrait + 'static + Copy, const FLAT_LEN: usize> {
        pub files: [VFSConstNormalInode<File>; FLAT_LEN],
    }

    use crate::{
        binary_map::{ConstBinaryMap, StaticArrayBuilder},
        memory::WasmAccess,
    };

    impl<File: Wasip1FileTrait + 'static + Copy, const LEN: usize> VFSConstNormalFiles<File, LEN> {
        pub fn new(files: [VFSConstNormalInodeBuilder<File>; LEN]) -> Self {}

        pub fn get_unchecked<'a>(&'a self, index: usize) -> &'a VFSConstNormalInodeBuilder<File> {
            unsafe { self.files.get_unchecked(index) }
        }

        pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a VFSConstNormalInodeBuilder<File>> {
            self.files.iter()
        }

        pub fn flat_children(&'static self) -> impl Iterator<Item = &'static File> {
            self.iter().flat_map(|child| child.flat_children())
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct VFSConstFileSystemDir<File: Wasip1FileTrait + 'static + Copy> {
        pub file_or_directories: &'static [VFSConstNormalInodeBuilder<File>],
    }

    impl<File: Wasip1FileTrait + 'static + Copy> VFSConstFileSystemDir<File> {
        pub const fn new(file_or_directories: &'static [VFSConstNormalInodeBuilder<File>]) -> Self {
            Self {
                file_or_directories,
            }
        }

        pub fn flat_children<'a>(&'a self) -> impl Iterator<Item = &'static File> {
            self.file_or_directories
                .iter()
                .flat_map(|child| child.flat_children())
        }

        pub const fn flat_children_static<'a, const FLAT_LEN: usize>(
            file_or_directories: [VFSConstNormalInodeBuilder<File>; FLAT_LEN],
        ) -> [File; FLAT_LEN] {
            let mut flat_files = StaticArrayBuilder::<File, FLAT_LEN>::new();
            let mut n = 0;

            use const_for::const_for;
            const_for!(i in 0..file_or_directories.len() => {
                if file_or_directories[i].flat_children_static_inner(&mut flat_files, &mut n) {
                    panic!("Flat files array is too small to hold all files and directories");
                }
            });

            flat_files.build()
        }

        pub(crate) const fn flat_children_static_inner<const FLAT_LEN: usize>(
            &self,
            flat_files: &mut StaticArrayBuilder<File, FLAT_LEN>,
            n: &mut usize,
        ) -> bool {
            use const_for::const_for;

            const_for!(i in 0..self.file_or_directories.len() => {
                if self.file_or_directories[i].flat_children_static_inner(flat_files, n) {
                    return false;
                }
            });
            true
        }

        pub fn iter(&self) -> impl Iterator<Item = &'static VFSConstNormalInodeBuilder<File>> {
            self.file_or_directories.iter()
        }
    }

    impl<File: Wasip1FileTrait + 'static + Copy> VFSConstFileSystemDir<File> {
        pub fn get_entry_for_path<'a>(
            &'a self,
            path: &str,
        ) -> Result<&'a VFSConstNormalInodeBuilder<File>, wasip1::Errno> {
            let mut parts = path.split('/');

            let mut entry = self.file_or_directories;

            let mut ret_file_entry = None;
            let mut ret_dir_entry = None;
            while let Some(part) = parts.next() {
                if ret_file_entry.is_some() {
                    return Err(wasip1::ERRNO_NOTDIR);
                }

                let mut found = false;
                for child in entry.iter() {
                    match child {
                        VFSConstNormalInodeBuilder::File(name, _) => {
                            if *name == part {
                                ret_file_entry = Some(child);
                                found = true;
                                break;
                            }
                        }
                        VFSConstNormalInodeBuilder::Dir(name, dir) => {
                            if *name == part {
                                ret_dir_entry = Some(child);
                                entry = dir.file_or_directories;
                                found = true;
                                break;
                            }
                        }
                    }
                }

                if !found {
                    return Err(wasip1::ERRNO_NOENT);
                }
            }

            if let Some(entry) = ret_file_entry {
                Ok(entry)
            } else if let Some(entry) = ret_dir_entry {
                if path.ends_with("/") {
                    return Ok(entry);
                } else {
                    return Err(wasip1::ERRNO_NOTDIR);
                }
            } else {
                return Err(wasip1::ERRNO_NOENT);
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub enum VFSConstNormalInode<File: Wasip1FileTrait + 'static + Copy> {
        File(File),
        Dir(&'static [usize]),
    }

    #[derive(Clone, Copy, Debug)]
    pub enum VFSConstNormalInodeBuilder<File: Wasip1FileTrait + 'static + Copy> {
        File(&'static str, File),
        Dir(&'static str, VFSConstFileSystemDir<File>),
    }

    impl<File: Wasip1FileTrait + 'static + Copy> VFSConstNormalInodeBuilder<File> {
        pub fn flat_children(&'static self) -> impl Iterator<Item = &'static File> {
            VFSConstNormalInodeIterator {
                index: 0,
                flat_index: 0,
                dir_or_file: self,
            }
        }

        pub(crate) const fn flat_children_static_inner<const FLAT_LEN: usize>(
            &self,
            flat_files: &mut StaticArrayBuilder<File, FLAT_LEN>,
            n: &mut usize,
        ) -> bool {
            match self {
                VFSConstNormalInodeBuilder::File(_, file) => {
                    *n += 1;
                    if flat_files.push(*file).is_some() {
                        return false;
                    }
                }
                VFSConstNormalInodeBuilder::Dir(_, dir) => {
                    dir.flat_children_static_inner(flat_files, n);
                }
            }
            true
        }
    }

    pub struct VFSConstNormalInodeIterator<File: Wasip1FileTrait + 'static + Copy> {
        index: usize,
        flat_index: usize,
        dir_or_file: &'static VFSConstNormalInodeBuilder<File>,
    }

    impl<File: Wasip1FileTrait + 'static + Copy> Iterator for VFSConstNormalInodeIterator<File> {
        type Item = &'static File;

        fn next(&mut self) -> Option<Self::Item> {
            match self.dir_or_file {
                VFSConstNormalInodeBuilder::File(_, file) => {
                    self.index += 1;

                    if self.index == 1 { Some(file) } else { None }
                }
                VFSConstNormalInodeBuilder::Dir(_, dir) => {
                    if self.index < dir.file_or_directories.len() {
                        let child = &dir.file_or_directories[self.index];

                        if let Some(file) = child.flat_children().nth(self.flat_index) {
                            self.flat_index += 1;
                            Some(file)
                        } else {
                            self.flat_index = 0;
                            self.index += 1;
                            self.next()
                        }
                    } else {
                        None
                    }
                }
            }
        }
    }

    #[macro_export]
    macro_rules! ConstFiles {
        (
            [
                $(($file_or_dir_name:expr, $file_or_dir:tt)),* $(,)?
            ] $(,)?
        ) => {
        $crate::wasi::file::non_atomic::VFSConstNormalFiles {
            files: [
                $(
                    $crate::ConstFiles!(@inner, $file_or_dir_name, $file_or_dir),
                )*
                ],
            }
        };

        (@inner, $name:expr, [
            $(($file_or_dir_name:expr, $file_or_dir:tt)),* $(,)?
        ]) => {
            $crate::wasi::file::non_atomic::VFSConstNormalInodeBuilder::Dir(
                $name,
                $crate::wasi::file::non_atomic::VFSConstFileSystemDir {
                    file_or_directories: &[
                        $(
                            $crate::ConstFiles!(@inner, $file_or_dir_name, $file_or_dir),
                        )*
                    ],
                },
            )
        };

        (@inner, $name:expr, $file:expr) => {
            #[allow(unused_braces)]
            $crate::wasi::file::non_atomic::VFSConstNormalInodeBuilder::File($name, $file)
        };
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub struct WasiConstFile<File: WasiConstPrimitiveFile> {
        file: File,
    }

    impl<File: WasiConstPrimitiveFile> WasiConstFile<File> {
        pub const fn new(file: File) -> Self {
            Self { file }
        }
    }

    pub trait WasiConstPrimitiveFile {
        fn len(&self) -> usize;
    }

    impl<'a> WasiConstPrimitiveFile for &'a str {
        #[inline(always)]
        fn len(&self) -> usize {
            <Self as std::ops::Deref>::deref(self).len()
        }
    }

    impl<File: WasiConstPrimitiveFile> Wasip1FileTrait for WasiConstFile<File> {
        fn path_open(
            &self,
            o_flags: wasip1::Oflags,
            fs_rights_base: wasip1::Rights,
            _fd_flags: wasip1::Fdflags,
        ) -> Result<(), wasip1::Errno> {
            if fs_rights_base & wasip1::RIGHTS_FD_WRITE == wasip1::RIGHTS_FD_WRITE {
                return Err(wasip1::ERRNO_PERM);
            }

            if o_flags & wasip1::OFLAGS_TRUNC == wasip1::OFLAGS_TRUNC {
                return Err(wasip1::ERRNO_PERM);
            }

            Ok(())
        }

        #[inline(always)]
        fn len(&self) -> usize {
            self.file.len()
        }
    }

    pub struct DefaultStdIO;

    impl StdIO for DefaultStdIO {}

    pub trait StdIO {
        fn write(buf: &[u8]) -> (Size, wasip1::Errno) {
            std::io::stdout()
                .write_all(buf)
                .expect("Failed to write to stdout");

            std::io::stdout().flush().expect("Failed to flush stdout");

            (buf.len() as Size, wasip1::ERRNO_SUCCESS)
        }

        fn ewrite(buf: &[u8]) -> (Size, wasip1::Errno) {
            std::io::stderr()
                .write_all(buf)
                .expect("Failed to write to stderr");

            std::io::stderr().flush().expect("Failed to flush stderr");

            (buf.len() as Size, wasip1::ERRNO_SUCCESS)
        }
    }

    pub trait Wasip1FileTrait {
        fn path_open(
            &self,
            o_flags: wasip1::Oflags,
            fs_rights_base: wasip1::Rights,
            fd_flags: wasip1::Fdflags,
        ) -> Result<(), wasip1::Errno>;

        fn len(&self) -> usize;

        /// Reads data from the file into the provided buffer.
        /// Returns the number of bytes read.
        /// This function is called by the `fd_read` function.
        /// Implementing this function directly is more efficient,
        /// but it is recommended to implement `fn read`!
        fn read_iovs<Wasm: WasmAccess>(
            &self,
            iovs: *const wasip1::Ciovec,
            iovs_len: usize,
        ) -> Result<usize, wasip1::Errno> {
            let mut total_read = 0;

            for i in 0..iovs_len {
                let iov = unsafe { iovs.add(i).as_ref() }.ok_or(wasip1::ERRNO_FAULT)?;
                let mut buf = vec![0u8; iov.buf_len];
                let read = self.read(&mut buf)?;
                if read == 0 {
                    break; // EOF
                }
                total_read += read;
                Wasm::memcpy(iov.buf as *mut _, &buf[..read]);
            }

            Ok(total_read)
        }

        /// Reads data from the file into the provided buffer.
        /// Returns the number of bytes read.
        fn read(&self, _buf: &mut [u8]) -> Result<usize, wasip1::Errno> {
            return Err(wasip1::ERRNO_NOSYS);
        }
    }
}

pub trait Wasip1FileSystem {
    fn fd_write(&mut self, fd: Fd, data: &[u8]) -> (Size, wasip1::Errno);

    fn path_open(
        &mut self,
        dir_fd: Fd,
        dir_flags: wasip1::Fdflags,
        path: &str,
        o_flags: wasip1::Oflags,
        fs_rights_base: wasip1::Rights,
        fs_rights_inheriting: wasip1::Rights,
        fd_flags: wasip1::Fdflags,
    ) -> Result<Fd, wasip1::Errno>;
}

use wasip1::*;

#[inline]
pub fn fd_write_inner<Wasm: WasmAccess>(
    state: &mut impl Wasip1FileSystem,
    fd: Fd,
    iovs_ptr: *const Ciovec,
    iovs_len: usize,
    nwritten: *mut Size,
) -> Errno {
    let mut iovs_vec: Vec<Ciovec> = Vec::with_capacity(iovs_len);
    unsafe { iovs_vec.set_len(iovs_len) };

    Wasm::memcpy_to(&mut iovs_vec, iovs_ptr);

    let len = iovs_vec.iter().map(|iovs| iovs.buf_len).sum::<usize>();

    let mut space = Vec::<u8>::with_capacity(len);
    unsafe { space.set_len(len) };

    let mut offset = 0;
    for iovs in iovs_vec {
        let buf_len = iovs.buf_len;
        let buf_ptr = iovs.buf;

        Wasm::memcpy_to(&mut space[offset..offset + buf_len], buf_ptr);

        offset += buf_len;
    }

    let (written, ret) = state.fd_write(fd, &space);
    Wasm::store_le(nwritten, written);
    ret
}

// #[inline]
// pub fn path_open_inner<Wasm: WasmAccess>(
//     state: &mut impl Wasip1FileSystem,
//     dir_fd: Fd,
//     dir_flags: Fdflags,
//     path_ptr: *const u8,
//     path_len: usize,
//     o_flags: Oflags,
//     fs_rights_base: Rights,
//     fs_rights_inheriting: Rights,
//     fd_flags: Fdflags,
//     fd_ret: *mut Fd,
// ) -> Errno {
// }

#[macro_export]
macro_rules! export_fs {
    (@const, $state:expr, $wasm:ty) => {
        $crate::paste::paste! {
            #[cfg(target_arch = "wasm32")]
            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_write>](
                fd: $crate::wasip1::Fd,
                iovs_ptr: *const $crate::wasip1::Ciovec,
                iovs_len: usize,
                nwritten: *mut usize,
            ) -> $crate::wasip1::Errno {
                let state = $state;
                $crate::wasi::file::fd_write_inner::<$wasm>(state, fd, iovs_ptr, iovs_len, nwritten)
            }

            // #[unsafe(no_mangle)]
            // #[cfg(target_arch = "wasm32")]
            // pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _path_open>](
            //     fd: $crate::wasip1::Fd,
            //     dir_flags: $crate::wasip1::Fdflags,
            //     path_ptr: *const u8,
            //     path_len: usize,
            //     o_flags: $crate::wasip1::Oflags,
            //     fs_rights_base: $crate::wasip1::Rights,
            //     fs_rights_inheriting: $crate::wasip1::Rights,
            //     fd_flags: $crate::wasip1::Fdflags,
            //     fd_ret: *mut $crate::wasip1::Fd,
            // ) -> $crate::wasip1::Errno {

            // }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{
        ConstFiles,
        wasi::file::non_atomic::{VFSConstNormalFiles, WasiConstFile},
    };

    /// If not using `--release`, compilation will fail with: link error
    /// cargo test -r --package wasip1-virtual-layer --lib -- wasi::file::tests::test_file_flat_iterate --exact --show-output
    #[test]
    fn test_file_flat_iterate() {
        const FILES: VFSConstNormalFiles<WasiConstFile<&'static str>> = ConstFiles!([
            ("/", [("root", WasiConstFile::new("This is root"))]),
            (
                ".",
                [
                    ("hey", { WasiConstFile::new("Hey!") }),
                    (
                        "hello",
                        [
                            ("world", { WasiConstFile::new("Hello, world!") }),
                            ("everyone", { WasiConstFile::new("Hello, everyone!") }),
                        ]
                    )
                ]
            ),
            (
                "~",
                [
                    ("home", { WasiConstFile::new("This is home") }),
                    ("user", { WasiConstFile::new("This is user") }),
                ]
            )
        ]);

        println!("{:#?}", FILES);

        let flat_files = FILES.flat_children().collect::<Vec<_>>();

        assert_eq!(flat_files[0], &WasiConstFile::new("This is root"));
        assert_eq!(flat_files[1], &WasiConstFile::new("Hey!"));
        assert_eq!(flat_files[2], &WasiConstFile::new("Hello, world!"));
        assert_eq!(flat_files[3], &WasiConstFile::new("Hello, everyone!"));
        assert_eq!(flat_files[4], &WasiConstFile::new("This is home"));
        assert_eq!(flat_files[5], &WasiConstFile::new("This is user"));
    }
}
