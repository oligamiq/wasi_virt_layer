// https://docs.rs/wasmtime-wasi/17.0.3/wasmtime_wasi/struct.WasiCtx.html
// https://docs.rs/wasi-common/17.0.3/wasi_common/table/struct.Table.html

use crate::memory::WasmAccess;

#[cfg(not(target_feature = "atomics"))]
pub mod non_atomic {
    use std::{collections::HashMap, io::Write as _};

    use wasip1::*;

    use super::Wasip1FileSystem;

    pub struct VirtualFileSystemConstState<
        File: Wasip1FileTrait + 'static + Copy,
        const LEN: usize,
        ROOTS: ConstFileSystemRootTy<File, LEN>,
        StdIo: StdIO + 'static,
    > {
        map: HashMap<
            Fd,
            (
                &'static ConstFileSystemDirOrFile<File>,
                VirtualFileSystemConstStateAddInfo,
            ),
        >,
        // add_info: HashMap<usize, VirtualFileSystemConstStateAddInfo>,
        next_fd: Fd,
        __marker: std::marker::PhantomData<(ROOTS, StdIo)>,
    }

    pub struct VirtualFileSystemConstStateAddInfo {
        #[allow(dead_code)]
        cursor: usize,
    }

    impl VirtualFileSystemConstStateAddInfo {
        pub const fn new() -> Self {
            Self { cursor: 0 }
        }

        pub const fn with_cursor(cursor: usize) -> Self {
            Self { cursor }
        }
    }

    // use wasmtime_wasi::WasiCtx;

    impl<
        File: Wasip1FileTrait + 'static + Copy,
        const LEN: usize,
        ROOTS: ConstFileSystemRootTy<File, LEN>,
        StdIo: StdIO + 'static,
    > VirtualFileSystemConstState<File, LEN, ROOTS, StdIo>
    {
        // I want to generate a reference from ROOTS but it doesn't work
        pub fn new(roots: &'static ConstFileSystemRoot<File, LEN>) -> Self {
            let mut map = HashMap::with_capacity(LEN);
            let mut next_fd = 3;

            for i in 0..LEN {
                let fd = next_fd;
                next_fd += 1;
                map.insert(
                    fd,
                    (
                        roots.get_unchecked(i),
                        VirtualFileSystemConstStateAddInfo::new(),
                    ),
                );
            }

            // let add_info = roots
            //     .files
            //     .iter()
            //     .flat_map(|entry| entry.flat_children())
            //     .map(|file| {
            //         (
            //             file as *const _ as usize,
            //             VirtualFileSystemConstStateAddInfo::new(),
            //         )
            //     })
            //     .collect::<HashMap<_, _>>();

            Self {
                map,
                next_fd,
                // add_info,
                __marker: std::marker::PhantomData,
            }
        }

        // pub fn get_add_info(&mut self, fd: Fd) -> Option<&mut VirtualFileSystemConstStateAddInfo> {
        //     let (entry) = self.map.get(&fd)?;

        //     if let ConstFileSystemDirOrFile::File(_, file) = entry {
        //         self.add_info.get_mut(&(file as *const _ as usize))
        //     } else {
        //         None
        //     }
        // }
    }

    impl<
        File: Wasip1FileTrait + 'static + Copy,
        const LEN: usize,
        ROOTS: ConstFileSystemRootTy<File, LEN>,
        StdIo: StdIO + 'static,
    > Wasip1FileSystem for VirtualFileSystemConstState<File, LEN, ROOTS, StdIo>
    {
        fn fd_write(&mut self, fd: Fd, data: &[u8]) -> (Size, wasip1::Errno) {
            match fd {
                wasip1::FD_STDOUT => StdIo::write(&data),
                wasip1::FD_STDERR => StdIo::ewrite(&data),
                _ => {
                    if self.map.contains_key(&fd) {
                        (0, wasip1::ERRNO_PERM)
                    } else {
                        (0, wasip1::ERRNO_BADF)
                    }
                }
            }
        }

        fn path_open(
            &mut self,
            dir_fd: Fd,
            _dir_flags: wasip1::Fdflags,
            path: &str,
            o_flags: wasip1::Oflags,
            fs_rights_base: wasip1::Rights,
            _fs_rights_inheriting: wasip1::Rights,
            fd_flags: wasip1::Fdflags,
        ) -> Result<Fd, wasip1::Errno> {
            let dir = if let Some((dir, _)) = self.map.get(&dir_fd) {
                dir
            } else {
                return Err(wasip1::ERRNO_BADF);
            };

            let dir = match dir {
                ConstFileSystemDirOrFile::Dir(_, dir) => dir,
                ConstFileSystemDirOrFile::File(..) => {
                    return Err(wasip1::ERRNO_NOTDIR);
                }
            };

            let entry = match dir.get_entry_for_path(path) {
                Ok(entry) => {
                    if o_flags & wasip1::OFLAGS_EXCL == wasip1::OFLAGS_EXCL {
                        return Err(wasip1::ERRNO_EXIST);
                    }

                    if o_flags & wasip1::OFLAGS_DIRECTORY == wasip1::OFLAGS_DIRECTORY {
                        if let ConstFileSystemDirOrFile::File(..) = entry {
                            return Err(wasip1::ERRNO_NOTDIR);
                        }
                    }

                    entry
                }
                Err(err) if err == wasip1::ERRNO_NOENT => {
                    if o_flags & wasip1::OFLAGS_CREAT == wasip1::OFLAGS_CREAT {
                        return Err(wasip1::ERRNO_PERM);
                    } else {
                        return Err(err);
                    }
                }
                Err(err) => return Err(err),
            };

            let add_info = if let ConstFileSystemDirOrFile::File(_, file) = entry {
                file.path_open(o_flags, fs_rights_base, fd_flags)?;
                if fd_flags & wasip1::FDFLAGS_APPEND == wasip1::FDFLAGS_APPEND {
                    VirtualFileSystemConstStateAddInfo::with_cursor(file.len())
                } else {
                    VirtualFileSystemConstStateAddInfo::new()
                }
            } else {
                VirtualFileSystemConstStateAddInfo::new()
            };

            let fd = self.next_fd;
            self.next_fd += 1;
            self.map.insert(fd, (entry, add_info));
            Ok(fd)
        }
    }

    #[allow(unused_imports)]
    mod d {
        use const_struct::ConstStruct;

        use super::{ConstFileSystemDirOrFile, Wasip1FileTrait};

        #[derive(ConstStruct)]
        pub struct ConstFileSystemRoot<File: Wasip1FileTrait + 'static + Copy, const LEN: usize> {
            pub files: [ConstFileSystemDirOrFile<File>; LEN],
        }
    }

    pub use d::*;

    impl<File: Wasip1FileTrait + 'static + Copy, const LEN: usize> ConstFileSystemRoot<File, LEN> {
        pub fn new(files: [ConstFileSystemDirOrFile<File>; LEN]) -> Self {
            Self { files }
        }

        pub fn get_unchecked<'a>(&'a self, index: usize) -> &'a ConstFileSystemDirOrFile<File> {
            unsafe { self.files.get_unchecked(index) }
        }
    }

    #[derive(Copy, Clone)]
    pub struct ConstFileSystemDir<File: Wasip1FileTrait + 'static + Copy> {
        pub file_or_directories: &'static [ConstFileSystemDirOrFile<File>],
    }

    impl<File: Wasip1FileTrait + 'static + Copy> ConstFileSystemDir<File> {
        pub const fn new(file_or_directories: &'static [ConstFileSystemDirOrFile<File>]) -> Self {
            Self {
                file_or_directories,
            }
        }

        pub fn flat_children<'a>(&'a self) -> impl Iterator<Item = &'static File> {
            self.file_or_directories
                .iter()
                .flat_map(|child| child.flat_children())
        }
    }

    impl<File: Wasip1FileTrait + 'static + Copy> ConstFileSystemDir<File> {
        pub fn get_entry_for_path<'a>(
            &'a self,
            path: &str,
        ) -> Result<&'a ConstFileSystemDirOrFile<File>, wasip1::Errno> {
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
                        ConstFileSystemDirOrFile::File(name, _) => {
                            if *name == part {
                                ret_file_entry = Some(child);
                                found = true;
                                break;
                            }
                        }
                        ConstFileSystemDirOrFile::Dir(name, dir) => {
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

    #[derive(Copy, Clone)]
    pub enum ConstFileSystemDirOrFile<File: Wasip1FileTrait + 'static + Copy> {
        File(&'static str, File),
        Dir(&'static str, ConstFileSystemDir<File>),
    }

    impl<File: Wasip1FileTrait + 'static + Copy> ConstFileSystemDirOrFile<File> {
        pub fn flat_children(&'static self) -> impl Iterator<Item = &'static File> {
            ConstFileSystemDirOrFileIterator {
                index: 0,
                flat_index: 0,
                dir_or_file: self,
            }
        }
    }

    pub struct ConstFileSystemDirOrFileIterator<File: Wasip1FileTrait + 'static + Copy> {
        index: usize,
        flat_index: usize,
        dir_or_file: &'static ConstFileSystemDirOrFile<File>,
    }

    impl<File: Wasip1FileTrait + 'static + Copy> Iterator for ConstFileSystemDirOrFileIterator<File> {
        type Item = &'static File;

        fn next(&mut self) -> Option<Self::Item> {
            match self.dir_or_file {
                ConstFileSystemDirOrFile::File(_, file) => {
                    self.index += 1;

                    if self.index == 1 { Some(file) } else { None }
                }
                ConstFileSystemDirOrFile::Dir(_, dir) => {
                    if self.index < dir.file_or_directories.len() {
                        let child = &dir.file_or_directories[self.index];
                        self.index += 1;

                        let nth = child.flat_children().nth(self.flat_index);
                        if let Some(file) = nth {
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
            ]
        ) => {
        $crate::wasi::file::non_atomic::ConstFileSystemRoot {
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
            $crate::wasi::file::non_atomic::ConstFileSystemDirOrFile::Dir(
                $name,
                $crate::wasi::file::non_atomic::ConstFileSystemDir {
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
            $crate::wasi::file::non_atomic::ConstFileSystemDirOrFile::File($name, $file)
        };
    }

    #[derive(Copy, Clone)]
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
