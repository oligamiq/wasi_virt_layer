// https://docs.rs/wasmtime-wasi/17.0.3/wasmtime_wasi/struct.WasiCtx.html
// https://docs.rs/wasi-common/17.0.3/wasi_common/table/struct.Table.html

use crate::memory::WasmAccess;

// no implementing dcache

#[cfg(not(target_feature = "atomics"))]
pub mod non_atomic {
    use wasip1::*;

    /// small posix like virtual file system
    /// but inode has some metadata
    pub struct Wasip1ConstVFS<LFS: Wasip1LFS + Sync, const FLAT_LEN: usize> {
        lfs: LFS,
        map: heapless::Vec<LFS::Inode, FLAT_LEN>,
    }

    impl<LFS: Wasip1LFS + Sync, const FLAT_LEN: usize> Wasip1ConstVFS<LFS, FLAT_LEN> {
        pub const fn new(lfs: LFS) -> Self {
            let map = heapless::Vec::new();
            Self { lfs, map }
        }

        #[inline]
        pub fn get_inode(&self, fd: Fd) -> Option<&LFS::Inode> {
            self.map.get(fd as usize)
        }

        #[inline]
        pub fn get_inode_and_lfs(&mut self, fd: Fd) -> Option<(&LFS::Inode, &mut LFS)> {
            let inode = self.map.get(fd as usize)?;
            Some((inode, &mut self.lfs))
        }

        pub(crate) fn fd_readdir_raw<Wasm: WasmAccess>(
            &mut self,
            fd: Fd,
            mut buf: *mut u8,
            mut buf_len: usize,
            mut cookie: Dircookie,
        ) -> Result<Size, wasip1::Errno> {
            let (inode, lfs) = self.get_inode_and_lfs(fd).ok_or(wasip1::ERRNO_BADF)?;

            // check is this a directory
            if !lfs.is_dir(inode) {
                return Err(wasip1::ERRNO_NOTDIR);
            }

            let mut read = 0;

            loop {
                let (n, next_cookie) = lfs.fd_readdir_raw::<Wasm>(inode, buf, buf_len, cookie)?;
                if n == 0 {
                    return Ok(read);
                }
                read += n;
                buf = unsafe { buf.add(n) };
                buf_len -= n;
                cookie = next_cookie;
            }
        }

        pub(crate) fn fd_write_raw<Wasm: WasmAccess>(
            &mut self,
            fd: Fd,
            iovs_ptr: *const Ciovec,
            iovs_len: usize,
        ) -> Result<Size, wasip1::Errno> {
            let (inode, lfs) = self.get_inode_and_lfs(fd).ok_or(wasip1::ERRNO_BADF)?;

            let iovs_vec = Wasm::as_array(iovs_ptr, iovs_len);

            let mut written = 0;

            for iovs in iovs_vec {
                let buf_len = iovs.buf_len;
                let buf_ptr = iovs.buf;

                written += lfs.fd_write_raw::<Wasm>(inode, buf_ptr, buf_len)?;
            }

            Ok(written)
        }
    }

    impl<LFS: Wasip1LFS + Sync, const FLAT_LEN: usize> Wasip1FileSystem
        for Wasip1ConstVFS<LFS, FLAT_LEN>
    {
        fn fd_write_raw<Wasm: WasmAccess>(
            &mut self,
            fd: Fd,
            iovs_ptr: *const Ciovec,
            iovs_len: usize,
            nwritten: *mut Size,
        ) -> wasip1::Errno {
            match self.fd_write_raw::<Wasm>(fd, iovs_ptr, iovs_len) {
                Ok(n) => {
                    Wasm::store_le(nwritten, n);
                    wasip1::ERRNO_SUCCESS
                }
                Err(e) => e,
            }
        }

        fn fd_readdir_raw<Wasm: WasmAccess>(
            &mut self,
            fd: Fd,
            buf: *mut u8,
            buf_len: usize,
            cookie: Dircookie,
            nread: *mut Size,
        ) -> wasip1::Errno {
            match self.fd_readdir_raw::<Wasm>(fd, buf, buf_len, cookie) {
                Ok(n) => {
                    Wasm::store_le(nread, n);
                    wasip1::ERRNO_SUCCESS
                }
                Err(e) => e,
            }
        }

        fn path_open(
            &mut self,
            dir_fd: Fd,
            dir_flags: wasip1::Fdflags,
            path: &str,
            o_flags: wasip1::Oflags,
            fs_rights_base: wasip1::Rights,
            fs_rights_inheriting: wasip1::Rights,
            fd_flags: wasip1::Fdflags,
        ) -> Result<Fd, wasip1::Errno> {
            todo!()
        }
    }

    /// small posix like local file system
    pub trait Wasip1LFS {
        type Inode;

        fn fd_write_raw<Wasm: WasmAccess>(
            &mut self,
            inode: &Self::Inode,
            data: *const u8,
            data_len: usize,
        ) -> Result<Size, wasip1::Errno>;

        fn is_dir(&self, inode: &Self::Inode) -> bool;

        fn fd_readdir_raw<Wasm: WasmAccess>(
            &mut self,
            inode: &Self::Inode,
            buf: *mut u8,
            buf_len: usize,
            cookie: Dircookie,
        ) -> Result<(Size, Dircookie), wasip1::Errno>;
    }

    pub struct VFSConstNormalLFS<
        ConstRoot: VFSConstNormalFilesTy<File, FLAT_LEN>,
        File: Wasip1FileTrait + 'static + Copy,
        const FLAT_LEN: usize,
        StdIo: StdIO + 'static,
    > {
        add_info: [VFSConstNormalAddInfo; FLAT_LEN],
        __marker: core::marker::PhantomData<(ConstRoot, File, StdIo)>,
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
                __marker: core::marker::PhantomData,
            }
        }

        #[inline]
        pub fn update_access_time(&mut self, inode: &usize, atime: usize) {
            let add_info = &mut self.add_info[*inode];
            add_info.atime = atime;
        }

        #[inline]
        pub fn is_dir(&self, inode: &usize) -> bool {
            let (_, file_or_dir) = ConstRoot::FILES[*inode];
            match file_or_dir {
                VFSConstNormalInode::Dir(..) => true,
                VFSConstNormalInode::File(..) => false,
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
        type Inode = usize;

        fn fd_write_raw<Wasm: WasmAccess>(
            &mut self,
            fd: &Self::Inode,
            data: *const u8,
            len: usize,
        ) -> Result<Size, wasip1::Errno> {
            match fd {
                1 => {
                    // stdout
                    #[cfg(not(feature = "multi_memory"))]
                    {
                        StdIo::write_direct::<Wasm>(data, len)
                    }
                    #[cfg(feature = "multi_memory")]
                    {
                        let mut buf = Vec::with_capacity(len);
                        unsafe { buf.set_len(len) };
                        Wasm::memcpy_to(&mut buf, data);
                        StdIo::write(&buf)
                    }
                }
                2 => {
                    // stderr
                    #[cfg(not(feature = "multi_memory"))]
                    {
                        StdIo::write_direct::<Wasm>(data, len)
                    }
                    #[cfg(feature = "multi_memory")]
                    {
                        let mut buf = Vec::with_capacity(len);
                        unsafe { buf.set_len(len) };
                        Wasm::memcpy_to(&mut buf, data);
                        StdIo::write(&buf)
                    }
                }
                _ => Err(wasip1::ERRNO_ROFS),
            }
        }

        fn is_dir(&self, inode: &Self::Inode) -> bool {
            self.is_dir(inode)
        }

        fn fd_readdir_raw<Wasm: WasmAccess>(
            &mut self,
            inode: &Self::Inode,
            buf: *mut u8,
            buf_len: usize,
            cookie: Dircookie,
        ) -> Result<(Size, Dircookie), wasip1::Errno> {
            let (_, dir) = ROOT::FILES[*inode];
            let (start, end) = match dir {
                VFSConstNormalInode::Dir(range) => range,
                _ => unreachable!(),
            };

            let index = start + cookie as usize;
            if index >= end {
                return Ok((0, cookie)); // No more entries
            }

            let (name, file_or_dir) = ROOT::FILES[index];

            let next_cookie = cookie + 1;

            let entry = wasip1::Dirent {
                d_next: next_cookie,
                d_ino: index as _,
                d_namlen: name.len() as _,
                d_type: match file_or_dir {
                    VFSConstNormalInode::File(..) => wasip1::FILETYPE_REGULAR_FILE,
                    VFSConstNormalInode::Dir(..) => wasip1::FILETYPE_DIRECTORY,
                },
            };
            let entry_buf = unsafe {
                core::slice::from_raw_parts(
                    &entry as *const _ as *const u8,
                    core::cmp::min(core::mem::size_of::<wasip1::Dirent>(), buf_len),
                )
            };

            Wasm::memcpy(buf, entry_buf);

            if buf_len < core::mem::size_of::<wasip1::Dirent>() {
                return Ok((buf_len, cookie));
            }

            let name_bytes = unsafe {
                core::slice::from_raw_parts(
                    name.as_ptr(),
                    core::cmp::min(name.len(), buf_len - core::mem::size_of::<wasip1::Dirent>()),
                )
            };

            Wasm::memcpy(
                unsafe { buf.add(core::mem::size_of::<wasip1::Dirent>()) },
                name_bytes,
            );

            Ok((
                core::mem::size_of::<wasip1::Dirent>() + name_bytes.len(),
                next_cookie,
            ))
        }
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

    use const_struct::ConstStruct;

    use crate::{memory::WasmAccess, transporter::Wasip1Transporter, wasi::file::Wasip1FileSystem};

    /// A constant file system root that can be used in a WASI component.
    #[derive(ConstStruct, Debug)]
    pub struct VFSConstNormalFiles<File: Wasip1FileTrait + 'static + Copy, const FLAT_LEN: usize> {
        pub files: [(&'static str, VFSConstNormalInode<File>); FLAT_LEN],
    }

    impl<File: Wasip1FileTrait + 'static + Copy, const FLAT_LEN: usize>
        VFSConstNormalFiles<File, FLAT_LEN>
    {
        pub const fn new(files: [(&'static str, VFSConstNormalInode<File>); FLAT_LEN]) -> Self {
            Self { files }
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub enum VFSConstNormalInode<File: Wasip1FileTrait + 'static + Copy> {
        File(File),
        /// (first index..last index)
        Dir((usize, usize)),
    }

    #[macro_export]
    macro_rules! ConstFiles {
        (
            [
                $(($file_or_dir_name:expr, $file_or_dir:tt)),* $(,)?
            ] $(,)?
        ) => {
            $crate::wasi::file::non_atomic::VFSConstNormalFiles::new({
                const COUNT: usize = {
                    let mut count = 0;

                    $(
                        $crate::ConstFiles!(@counter, count, $file_or_dir);
                    )*

                    count
                };

                let mut static_array = $crate::binary_map::StaticArrayBuilder::new();

                struct CheckEqNumberOfFilesAndDirs<const L: usize, const R: usize>;

                #[allow(dead_code)]
                impl<const L: usize, const R: usize> CheckEqNumberOfFilesAndDirs<L, R> {
                    #[allow(non_upper_case_globals)]
                    const number_of_files_and_dirs_equals_FLAT_LEN_so_you_must_set_VFSConstNormalFiles_num: usize = (R - L) + (L - R);
                }

                const fn asserter<S: 'static + Copy, const N: usize>(
                    _: &$crate::binary_map::StaticArrayBuilder<S, N>,
                ) {
                    #[allow(path_statements)]
                    CheckEqNumberOfFilesAndDirs::<COUNT, N>::number_of_files_and_dirs_equals_FLAT_LEN_so_you_must_set_VFSConstNormalFiles_num;
                }

                asserter(&static_array);

                const fn vfs_const_macro_fn<S: 'static + Copy, const N: usize>(
                    fake_files: [&'static str; N],
                    name: &'static str,
                    _: &$crate::binary_map::StaticArrayBuilder<S, N>,
                ) -> (usize, usize) {
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

                    const fn starts_with_str(a: &str, b: &str) -> bool {
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

                        true
                    }

                    let mut first_index = None;
                    const_for!(i in 0..N => {
                        if first_index.is_none() && starts_with_str(fake_files[i], name) {
                            first_index = Some(i);
                        }

                        if eq_str(fake_files[i], name) {
                            return (first_index.unwrap(), i);
                        }
                    });

                    unreachable!()
                }

                let empty_arr = {
                    let mut empty_arr = $crate::binary_map::StaticArrayBuilder::new();

                    $(
                        $crate::ConstFiles!(@empty, empty_arr, [$file_or_dir_name], $file_or_dir);
                    )*

                    empty_arr.build()
                };

                $(
                    $crate::ConstFiles!(
                        @next,
                        static_array,
                        [empty_arr],
                        [$file_or_dir_name],
                        $file_or_dir
                    );
                )*

                static_array.build()
            })
        };

        (@counter, $count:ident, [
            $(($file_or_dir_name:expr, $file_or_dir:tt)),* $(,)?
        ]) => {
            $(
                $crate::ConstFiles!(@counter, $count, $file_or_dir);
            )*
            $count += 1;
        };

        (@counter, $count:ident, $file:tt) => {
            $count += 1;
        };

        (@empty, $empty_arr:ident, [$parent_name:expr], [
            $(($file_or_dir_name:expr, $file_or_dir:tt)),* $(,)?
        ]) => {
            $(
                $crate::ConstFiles!(@empty, $empty_arr, [concat!($parent_name, "/", $file_or_dir_name)], $file_or_dir);
            )*
            $empty_arr.push($parent_name);
        };

        (@empty, $empty_arr:ident, [$name:expr], $file:tt) => {
            $empty_arr.push($name);
        };

        (@next, $static_array:ident, [$empty:expr], [$name:expr], [
            $(($file_or_dir_name:expr, $file_or_dir:tt)),* $(,)?
        ]) => {
            $(
                $crate::ConstFiles!(@next, $static_array, [$empty], [concat!($name, "/", $file_or_dir_name)], $file_or_dir);
            )*
            $static_array.push((
                $name,
                $crate::wasi::file::non_atomic::VFSConstNormalInode::Dir(
                    vfs_const_macro_fn(
                        $empty,
                        $name,
                        &$static_array
                    )
                )
            ));
        };

        (@next, $static_array:ident, [$empty:expr], [$name:expr], $file:expr) => {
            $static_array.push((
                $name,
                $crate::wasi::file::non_atomic::VFSConstNormalInode::File($file)
            ));
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
            <Self as core::ops::Deref>::deref(self).len()
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
                return Err(wasip1::ERRNO_ROFS);
            }

            if o_flags & wasip1::OFLAGS_TRUNC == wasip1::OFLAGS_TRUNC {
                return Err(wasip1::ERRNO_ROFS);
            }

            Ok(())
        }

        #[inline(always)]
        fn len(&self) -> usize {
            self.file.len()
        }
    }

    pub struct DefaultStdIO;

    impl StdIO for DefaultStdIO {
        fn write(buf: &[u8]) -> Result<Size, wasip1::Errno> {
            Wasip1Transporter::write_to_stdout(buf)
        }

        #[cfg(not(feature = "multi_memory"))]
        fn write_direct<Wasm: WasmAccess>(
            buf: *const u8,
            len: usize,
        ) -> Result<Size, wasip1::Errno> {
            Wasip1Transporter::write_to_stdout_direct::<Wasm>(buf, len)
        }

        fn ewrite(buf: &[u8]) -> Result<Size, wasip1::Errno> {
            Wasip1Transporter::write_to_stderr(buf)
        }

        #[cfg(not(feature = "multi_memory"))]
        fn ewrite_direct<Wasm: WasmAccess>(
            buf: *const u8,
            len: usize,
        ) -> Result<Size, wasip1::Errno> {
            Wasip1Transporter::write_to_stderr_direct::<Wasm>(buf, len)
        }
    }

    pub trait StdIO {
        #[allow(unused_variables)]
        fn write(buf: &[u8]) -> Result<Size, wasip1::Errno> {
            Err(wasip1::ERRNO_NOSYS)
        }

        #[cfg(not(feature = "multi_memory"))]
        #[allow(unused_variables)]
        fn write_direct<Wasm: WasmAccess>(
            buf: *const u8,
            len: usize,
        ) -> Result<Size, wasip1::Errno> {
            #[cfg(feature = "alloc")]
            {
                Self::write(&Wasm::get_array(buf, len))
            }

            #[cfg(not(feature = "alloc"))]
            {
                // Stub implementation for non-std environments
                Err(wasip1::ERRNO_NOSYS)
            }
        }

        #[allow(unused_variables)]
        fn ewrite(buf: &[u8]) -> Result<Size, wasip1::Errno> {
            Err(wasip1::ERRNO_NOSYS)
        }

        #[cfg(not(feature = "multi_memory"))]
        #[allow(unused_variables)]
        fn ewrite_direct<Wasm: WasmAccess>(
            buf: *const u8,
            len: usize,
        ) -> Result<Size, wasip1::Errno> {
            #[cfg(feature = "alloc")]
            {
                Self::ewrite(&Wasm::get_array(buf, len))
            }

            #[cfg(not(feature = "alloc"))]
            {
                // Stub implementation for non-std environments
                Err(wasip1::ERRNO_NOSYS)
            }
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
        #[cfg_attr(not(feature = "alloc"), allow(unused_variables))]
        fn read_iovs<Wasm: WasmAccess>(
            &self,
            iovs: *const wasip1::Ciovec,
            iovs_len: usize,
        ) -> Result<usize, wasip1::Errno> {
            #[cfg(feature = "alloc")]
            {
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

            #[cfg(not(feature = "alloc"))]
            {
                // Stub implementation for non-std environments
                Err(wasip1::ERRNO_NOSYS)
            }
        }

        /// Reads data from the file into the provided buffer.
        /// Returns the number of bytes read.
        fn read(&self, _buf: &mut [u8]) -> Result<usize, wasip1::Errno> {
            return Err(wasip1::ERRNO_NOSYS);
        }
    }
}

pub trait Wasip1FileSystem {
    fn fd_write_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
        iovs_ptr: *const Ciovec,
        iovs_len: usize,
        nwritten: *mut Size,
    ) -> wasip1::Errno;

    fn fd_readdir_raw<Wasm: WasmAccess>(
        &mut self,
        fd: Fd,
        buf: *mut u8,
        buf_len: usize,
        cookie: Dircookie,
        nread: *mut Size,
    ) -> wasip1::Errno;

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
        $crate::__private::paste::paste! {
            #[cfg(target_arch = "wasm32")]
            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_write>](
                fd: $crate::wasip1::Fd,
                iovs_ptr: *const $crate::wasip1::Ciovec,
                iovs_len: usize,
                nwritten: *mut usize,
            ) -> $crate::wasip1::Errno {
                let state = $state;
                $crate::wasi::file::Wasip1FileSystem::fd_write_raw::<$wasm>(state, fd, iovs_ptr, iovs_len, nwritten)
            }

            #[cfg(target_arch = "wasm32")]
            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_readdir>](
                fd: $crate::wasip1::Fd,
                buf: *mut u8,
                buf_len: usize,
                cookie: $crate::wasip1::Dircookie,
                nread: *mut $crate::wasip1::Size,
            ) -> $crate::wasip1::Errno {
                let state = $state;
                $crate::wasi::file::Wasip1FileSystem::fd_readdir_raw::<$wasm>(state, fd, buf, buf_len, cookie, nread)
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
        const _: VFSConstNormalFiles<WasiConstFile<&'static str>, 10> = ConstFiles!([
            (
                "/root",
                [("root.txt", { WasiConstFile::new("This is root") })]
            ),
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
    }
}
