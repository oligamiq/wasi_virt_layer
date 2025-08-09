pub mod file {
    use crate::memory::WasmAccess;
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
            ConstRoot: VFSConstNormalFilesTy<File, LEN>,
            File: Wasip1FileTrait + 'static + Copy,
            const LEN: usize,
            StdIo: StdIO + 'static,
        > {
            add_info: ConstBinaryMap<
                'static,
                VFSConstNormalInodeBuilder<File>,
                VFSConstNormalAddInfo,
                LEN,
            >,
            __marker: std::marker::PhantomData<(ConstRoot, File, StdIo)>,
        }
        impl<
            ConstRoot: VFSConstNormalFilesTy<File, LEN>,
            File: Wasip1FileTrait + 'static + Copy,
            const LEN: usize,
            StdIo: StdIO + 'static,
        > VFSConstNormalLFS<ConstRoot, File, LEN, StdIo> {
            pub const fn new() -> Self {
                Self {
                    add_info: ConstBinaryMap::from_key_values(
                        ConstRoot::flat_children_static(),
                        VFSConstNormalAddInfo::new(),
                    ),
                    __marker: std::marker::PhantomData,
                }
            }
        }
        impl<
            ROOT: VFSConstNormalFilesTy<File, LEN>,
            File: Wasip1FileTrait + 'static + Copy,
            const LEN: usize,
            StdIo: StdIO + 'static,
        > Wasip1LFS for VFSConstNormalLFS<ROOT, File, LEN, StdIo> {
            type Inode = &'static VFSConstNormalInodeBuilder<File>;
        }
        pub struct VFSConstNormalAddInfo {
            cursor: usize,
            atime: usize,
        }
        #[automatically_derived]
        impl ::core::marker::Copy for VFSConstNormalAddInfo {}
        #[automatically_derived]
        impl ::core::clone::Clone for VFSConstNormalAddInfo {
            #[inline]
            fn clone(&self) -> VFSConstNormalAddInfo {
                let _: ::core::clone::AssertParamIsClone<usize>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for VFSConstNormalAddInfo {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "VFSConstNormalAddInfo",
                    "cursor",
                    &self.cursor,
                    "atime",
                    &&self.atime,
                )
            }
        }
        impl VFSConstNormalAddInfo {
            pub const fn new() -> Self {
                Self { cursor: 0, atime: 0 }
            }
            pub const fn with_cursor(cursor: usize) -> Self {
                Self { cursor, atime: 0 }
            }
        }
        #[allow(unused_imports)]
        mod d {
            use const_struct::ConstStruct;
            use super::{VFSConstNormalInodeBuilder, Wasip1FileTrait};
            /// A constant file system root that can be used in a WASI component.
            pub struct VFSConstNormalFiles<
                File: Wasip1FileTrait + 'static + Copy,
                const FLAT_LEN: usize,
            > {
                pub files: [VFSConstNormalInodeBuilder<File>; FLAT_LEN],
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl<File, const FLAT_LEN: usize> ::const_struct::keeptype::KeepType<1usize>
            for VFSConstNormalFiles<File, { FLAT_LEN }>
            where
                File: Wasip1FileTrait + 'static + Copy,
            {
                type Type = usize;
            }
            #[automatically_derived]
            pub trait VFSConstNormalFilesTy<
                File,
                const FLAT_LEN: usize,
            >: ::const_struct::PrimitiveTraits<
                    DATATYPE = VFSConstNormalFiles<File, { FLAT_LEN }>,
                >
            where
                File: Wasip1FileTrait + 'static + Copy,
            {
                const FILES: [VFSConstNormalInodeBuilder<File>; FLAT_LEN] = <Self as ::const_struct::PrimitiveTraits>::__DATA
                    .files;
            }
            #[automatically_derived]
            impl<
                PrimitiveType: ::const_struct::PrimitiveTraits<
                        DATATYPE = VFSConstNormalFiles<File, { FLAT_LEN }>,
                    >,
                File,
                const FLAT_LEN: usize,
            > VFSConstNormalFilesTy<File, { FLAT_LEN }> for PrimitiveType
            where
                File: Wasip1FileTrait + 'static + Copy,
            {}
            pub(crate) mod macros {
                #[allow(unused_imports)]
                pub(crate) use VFSConstNormalFiles;
            }
            #[automatically_derived]
            impl<
                File: ::core::fmt::Debug + Wasip1FileTrait + 'static + Copy,
                const FLAT_LEN: usize,
            > ::core::fmt::Debug for VFSConstNormalFiles<File, FLAT_LEN> {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "VFSConstNormalFiles",
                        "files",
                        &&self.files,
                    )
                }
            }
        }
        pub use d::*;
        use crate::{
            binary_map::{ConstBinaryMap, StaticArrayBuilder},
            memory::WasmAccess,
        };
        impl<
            File: Wasip1FileTrait + 'static + Copy,
            const LEN: usize,
        > VFSConstNormalFiles<File, LEN> {
            pub fn new(files: [VFSConstNormalInodeBuilder<File>; LEN]) -> Self {}
            pub fn get_unchecked<'a>(
                &'a self,
                index: usize,
            ) -> &'a VFSConstNormalInodeBuilder<File> {
                unsafe { self.files.get_unchecked(index) }
            }
            pub fn iter<'a>(
                &'a self,
            ) -> impl Iterator<Item = &'a VFSConstNormalInodeBuilder<File>> {
                self.files.iter()
            }
            pub fn flat_children(&'static self) -> impl Iterator<Item = &'static File> {
                self.iter().flat_map(|child| child.flat_children())
            }
        }
        pub struct VFSConstFileSystemDir<File: Wasip1FileTrait + 'static + Copy> {
            pub file_or_directories: &'static [VFSConstNormalInodeBuilder<File>],
        }
        #[automatically_derived]
        impl<
            File: ::core::marker::Copy + Wasip1FileTrait + 'static + Copy,
        > ::core::marker::Copy for VFSConstFileSystemDir<File> {}
        #[automatically_derived]
        impl<
            File: ::core::clone::Clone + Wasip1FileTrait + 'static + Copy,
        > ::core::clone::Clone for VFSConstFileSystemDir<File> {
            #[inline]
            fn clone(&self) -> VFSConstFileSystemDir<File> {
                VFSConstFileSystemDir {
                    file_or_directories: ::core::clone::Clone::clone(
                        &self.file_or_directories,
                    ),
                }
            }
        }
        #[automatically_derived]
        impl<
            File: ::core::fmt::Debug + Wasip1FileTrait + 'static + Copy,
        > ::core::fmt::Debug for VFSConstFileSystemDir<File> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "VFSConstFileSystemDir",
                    "file_or_directories",
                    &&self.file_or_directories,
                )
            }
        }
        impl<File: Wasip1FileTrait + 'static + Copy> VFSConstFileSystemDir<File> {
            pub const fn new(
                file_or_directories: &'static [VFSConstNormalInodeBuilder<File>],
            ) -> Self {
                Self { file_or_directories }
            }
            pub fn flat_children<'a>(&'a self) -> impl Iterator<Item = &'static File> {
                self.file_or_directories.iter().flat_map(|child| child.flat_children())
            }
            pub const fn flat_children_static<'a, const FLAT_LEN: usize>(
                file_or_directories: [VFSConstNormalInodeBuilder<File>; FLAT_LEN],
            ) -> [File; FLAT_LEN] {
                let mut flat_files = StaticArrayBuilder::<File, FLAT_LEN>::new();
                let mut n = 0;
                use const_for::const_for;
                {
                    let _: usize = 1;
                    let mut __ite = (0..file_or_directories.len()).start;
                    let __end = (0..file_or_directories.len()).end;
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
                            if file_or_directories[i]
                                .flat_children_static_inner(&mut flat_files, &mut n)
                            {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Flat files array is too small to hold all files and directories",
                                        ),
                                    );
                                };
                            }
                        }
                    }
                };
                flat_files.build()
            }
            pub(crate) const fn flat_children_static_inner<const FLAT_LEN: usize>(
                &self,
                flat_files: &mut StaticArrayBuilder<File, FLAT_LEN>,
                n: &mut usize,
            ) -> bool {
                use const_for::const_for;
                {
                    let _: usize = 1;
                    let mut __ite = (0..self.file_or_directories.len()).start;
                    let __end = (0..self.file_or_directories.len()).end;
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
                            if self
                                .file_or_directories[i]
                                .flat_children_static_inner(flat_files, n)
                            {
                                return false;
                            }
                        }
                    }
                };
                true
            }
            pub fn iter(
                &self,
            ) -> impl Iterator<Item = &'static VFSConstNormalInodeBuilder<File>> {
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
        pub enum VFSConstNormalInode<File: Wasip1FileTrait + 'static + Copy> {
            File(File),
            Dir(VFSConstFileSystemDir<File>),
        }
        #[automatically_derived]
        impl<
            File: ::core::clone::Clone + Wasip1FileTrait + 'static + Copy,
        > ::core::clone::Clone for VFSConstNormalInode<File> {
            #[inline]
            fn clone(&self) -> VFSConstNormalInode<File> {
                match self {
                    VFSConstNormalInode::File(__self_0) => {
                        VFSConstNormalInode::File(::core::clone::Clone::clone(__self_0))
                    }
                    VFSConstNormalInode::Dir(__self_0) => {
                        VFSConstNormalInode::Dir(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl<
            File: ::core::marker::Copy + Wasip1FileTrait + 'static + Copy,
        > ::core::marker::Copy for VFSConstNormalInode<File> {}
        #[automatically_derived]
        impl<
            File: ::core::fmt::Debug + Wasip1FileTrait + 'static + Copy,
        > ::core::fmt::Debug for VFSConstNormalInode<File> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    VFSConstNormalInode::File(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "File",
                            &__self_0,
                        )
                    }
                    VFSConstNormalInode::Dir(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Dir",
                            &__self_0,
                        )
                    }
                }
            }
        }
        pub enum VFSConstNormalInodeBuilder<File: Wasip1FileTrait + 'static + Copy> {
            File(&'static str, File),
            Dir(&'static str, VFSConstFileSystemDir<File>),
        }
        #[automatically_derived]
        impl<
            File: ::core::clone::Clone + Wasip1FileTrait + 'static + Copy,
        > ::core::clone::Clone for VFSConstNormalInodeBuilder<File> {
            #[inline]
            fn clone(&self) -> VFSConstNormalInodeBuilder<File> {
                match self {
                    VFSConstNormalInodeBuilder::File(__self_0, __self_1) => {
                        VFSConstNormalInodeBuilder::File(
                            ::core::clone::Clone::clone(__self_0),
                            ::core::clone::Clone::clone(__self_1),
                        )
                    }
                    VFSConstNormalInodeBuilder::Dir(__self_0, __self_1) => {
                        VFSConstNormalInodeBuilder::Dir(
                            ::core::clone::Clone::clone(__self_0),
                            ::core::clone::Clone::clone(__self_1),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl<
            File: ::core::marker::Copy + Wasip1FileTrait + 'static + Copy,
        > ::core::marker::Copy for VFSConstNormalInodeBuilder<File> {}
        #[automatically_derived]
        impl<
            File: ::core::fmt::Debug + Wasip1FileTrait + 'static + Copy,
        > ::core::fmt::Debug for VFSConstNormalInodeBuilder<File> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    VFSConstNormalInodeBuilder::File(__self_0, __self_1) => {
                        ::core::fmt::Formatter::debug_tuple_field2_finish(
                            f,
                            "File",
                            __self_0,
                            &__self_1,
                        )
                    }
                    VFSConstNormalInodeBuilder::Dir(__self_0, __self_1) => {
                        ::core::fmt::Formatter::debug_tuple_field2_finish(
                            f,
                            "Dir",
                            __self_0,
                            &__self_1,
                        )
                    }
                }
            }
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
        impl<File: Wasip1FileTrait + 'static + Copy> Iterator
        for VFSConstNormalInodeIterator<File> {
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
                            if let Some(file) = child
                                .flat_children()
                                .nth(self.flat_index)
                            {
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
        pub struct WasiConstFile<File: WasiConstPrimitiveFile> {
            file: File,
        }
        #[automatically_derived]
        impl<File: ::core::marker::Copy + WasiConstPrimitiveFile> ::core::marker::Copy
        for WasiConstFile<File> {}
        #[automatically_derived]
        impl<File: ::core::clone::Clone + WasiConstPrimitiveFile> ::core::clone::Clone
        for WasiConstFile<File> {
            #[inline]
            fn clone(&self) -> WasiConstFile<File> {
                WasiConstFile {
                    file: ::core::clone::Clone::clone(&self.file),
                }
            }
        }
        #[automatically_derived]
        impl<File: ::core::fmt::Debug + WasiConstPrimitiveFile> ::core::fmt::Debug
        for WasiConstFile<File> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "WasiConstFile",
                    "file",
                    &&self.file,
                )
            }
        }
        #[automatically_derived]
        impl<File: WasiConstPrimitiveFile> ::core::marker::StructuralPartialEq
        for WasiConstFile<File> {}
        #[automatically_derived]
        impl<
            File: ::core::cmp::PartialEq + WasiConstPrimitiveFile,
        > ::core::cmp::PartialEq for WasiConstFile<File> {
            #[inline]
            fn eq(&self, other: &WasiConstFile<File>) -> bool {
                self.file == other.file
            }
        }
        #[automatically_derived]
        impl<File: ::core::cmp::Eq + WasiConstPrimitiveFile> ::core::cmp::Eq
        for WasiConstFile<File> {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<File>;
            }
        }
        #[automatically_derived]
        impl<File: ::core::hash::Hash + WasiConstPrimitiveFile> ::core::hash::Hash
        for WasiConstFile<File> {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.file, state)
            }
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
                std::io::stdout().write_all(buf).expect("Failed to write to stdout");
                std::io::stdout().flush().expect("Failed to flush stdout");
                (buf.len() as Size, wasip1::ERRNO_SUCCESS)
            }
            fn ewrite(buf: &[u8]) -> (Size, wasip1::Errno) {
                std::io::stderr().write_all(buf).expect("Failed to write to stderr");
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
                    let iov = unsafe { iovs.add(i).as_ref() }
                        .ok_or(wasip1::ERRNO_FAULT)?;
                    let mut buf = ::alloc::vec::from_elem(0u8, iov.buf_len);
                    let read = self.read(&mut buf)?;
                    if read == 0 {
                        break;
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
}
