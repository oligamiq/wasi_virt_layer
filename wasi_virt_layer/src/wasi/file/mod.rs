// https://docs.rs/wasmtime-wasi/17.0.3/wasmtime_wasi/struct.WasiCtx.html
// https://docs.rs/wasi-common/17.0.3/wasi_common/table/struct.Table.html

use core::any::Any;
use core::borrow::Borrow;
use core::ops::Deref;

use smallstr::SmallString;

use crate::memory::{
    WasmAccess, WasmAccessDynCompatible as _, WasmAccessDynCompatibleRaw, WasmAccessName,
};
#[cfg(feature = "alloc")]
pub mod changeable;
pub mod constant;
#[cfg(feature = "multiple_lfs")]
pub mod multiple;
pub mod stdio;
use crate::__private::wasip1;

// no implementing dcache

use crate::__private::wasip1::*;
use crate::wasi::file::changeable::inode::InodeIdCommon;
use crate::wasi::file::multiple::inode::BoxedInodeCommon;

/// File statistics excluding the device ID.
pub struct FilestatWithoutDevice {
    /// File serial number.
    pub ino: Inode,
    /// File type.
    pub filetype: Filetype,
    /// Number of hard links to the file.
    pub nlink: Linkcount,
    /// For regular files, the file size in bytes. For symbolic links, the length in bytes of the pathname contained in the symbolic link.
    pub size: Filesize,
    /// Last data access timestamp.
    pub atim: Timestamp,
    /// Last data modification timestamp.
    pub mtim: Timestamp,
    /// Last file status change timestamp.
    pub ctim: Timestamp,
}

pub(crate) trait ConstDefault: core::fmt::Debug {
    const DEFAULT: Self;
}

/// Additional mutable info for an inode.
pub trait WasiAddInfo: core::fmt::Debug + Clone + Copy {
    /// Returns the access time.
    fn access_time(&self) -> Timestamp {
        0
    }
    /// Sets the access time.
    fn set_access_time(&mut self, _atime: Timestamp) {}

    /// Returns the modification time.
    fn modification_time(&self) -> Timestamp {
        0
    }
    /// Sets the modification time.
    fn set_modification_time(&mut self, _mtime: Timestamp) {}

    /// Returns the creation time.
    fn creation_time(&self) -> Timestamp {
        0
    }
    /// Sets the creation time.
    fn set_creation_time(&mut self, _ctime: Timestamp) {}
}

/// An empty implementation of WasiAddInfo for read-only or stateless inodes.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoAddInfo;

impl WasiAddInfo for NoAddInfo {}

impl ConstDefault for NoAddInfo {
    const DEFAULT: Self = Self;
}

/// A default implementation of WasiAddInfo storing all timestamps.
#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultAddInfo {
    /// Access time
    pub atim: Timestamp,
    /// Modification time
    pub mtim: Timestamp,
    /// Creation time
    pub ctim: Timestamp,
}

impl ConstDefault for DefaultAddInfo {
    const DEFAULT: Self = Self {
        atim: 0,
        mtim: 0,
        ctim: 0,
    };
}

impl WasiAddInfo for DefaultAddInfo {
    fn access_time(&self) -> Timestamp {
        self.atim
    }
    fn set_access_time(&mut self, atime: Timestamp) {
        self.atim = atime;
    }

    fn modification_time(&self) -> Timestamp {
        self.mtim
    }
    fn set_modification_time(&mut self, mtime: Timestamp) {
        self.mtim = mtime;
    }

    fn creation_time(&self) -> Timestamp {
        self.ctim
    }
    fn set_creation_time(&mut self, ctime: Timestamp) {
        self.ctim = ctime;
    }
}

pub(crate) trait Wasip1LFSBaseWrapper: Wasip1LFSBase {
    fn downcast_inode(inode: &dyn InodeIdCommon) -> &<Self as Wasip1LFSBase>::Inode;
}

impl<T: Wasip1LFSBase + ?Sized> Wasip1LFSBaseWrapper for T
where
    <Self as Wasip1LFSBase>::Inode: 'static,
{
    fn downcast_inode(inode: &dyn InodeIdCommon) -> &<Self as Wasip1LFSBase>::Inode {
        let inode = inode as &dyn Any;

        inode
            .downcast_ref::<<Self as Wasip1LFSBase>::Inode>()
            .unwrap()
    }
}

/// small posix like local file system
/// Trait for a local file system implementation.
pub trait Wasip1LFSBase: core::fmt::Debug {
    /// The type used for inodes.
    /// Pre-opened inodes.
    type Inode: InodeIdCommon;

    /// Writes raw data to a file.
    fn fd_write_raw<Wasm: WasmAccess>(
        &self,
        inode: &Self::Inode,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno>;

    /// Writes raw data to stdout.
    fn fd_write_stdout_raw<Wasm: WasmAccess>(
        &self,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno>;

    /// Writes raw data to stderr.
    fn fd_write_stderr_raw<Wasm: WasmAccess>(
        &self,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno>;

    /// Returns whether the inode is a directory.
    fn is_dir(&self, inode: &Self::Inode) -> bool;

    /// Reads directory entries.
    fn fd_readdir_raw<Wasm: WasmAccess>(
        &self,
        inode: &Self::Inode,
        buf: *mut u8,
        buf_len: usize,
        cookie: Dircookie,
    ) -> Result<(Size, Dircookie), wasip1::Errno>;

    /// Retrieves file statistics for a path.
    fn path_filestat_get_raw<Wasm: WasmAccess>(
        &self,
        inode: &Self::Inode,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno>;

    /// Retrieves pre-open statistics.
    fn fd_prestat_get_raw<Wasm: WasmAccess>(
        &self,
        inode: &Self::Inode,
    ) -> Result<wasip1::Prestat, wasip1::Errno>;

    /// Retrieves the name of a pre-opened directory.
    fn fd_prestat_dir_name_raw<Wasm: WasmAccess>(
        &self,
        inode: &Self::Inode,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> Result<(), wasip1::Errno>;

    /// Retrieves file statistics for a file descriptor.
    fn fd_filestat_get_raw<Wasm: WasmAccess>(
        &self,
        inode: &Self::Inode,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno>;

    /// Reads data from a file descriptor into a buffer at a given offset.
    fn fd_pread_raw<Wasm: WasmAccess>(
        &self,
        inode: &Self::Inode,
        buf: *mut u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<Size, wasip1::Errno>;

    /// Reads data from stdin.
    fn fd_read_stdin_raw<Wasm: WasmAccess>(
        &self,
        buf: *mut u8,
        buf_len: usize,
    ) -> Result<Size, wasip1::Errno>;

    /// Opens a path.
    fn path_open_raw<Wasm: WasmAccess>(
        &self,
        dir_ino: &Self::Inode,
        dir_flags: wasip1::Fdflags,
        path_ptr: *const u8,
        path_len: usize,
        o_flags: wasip1::Oflags,
        fs_rights_base: wasip1::Rights,
        fs_rights_inheriting: wasip1::Rights,
        fd_flags: wasip1::Fdflags,
    ) -> Result<Self::Inode, wasip1::Errno>;
}

pub trait Wasip1ConstLFS: Wasip1LFSBase
where
    Self::Inode: 'static,
{
    const PRE_OPEN: &'static [Self::Inode];
}

pub trait Wasip1DynamicLFS: Wasip1LFSBase {
    fn pre_open_inodes(&self) -> impl IntoIterator<Item = (Self::Inode, impl DerefToStrCustom)>;
}

pub trait DerefToStrCustom {
    fn deref_to_str<'a>(&'a self) -> &'a str;
}

#[cfg(all(feature = "alloc", not(feature = "std")))]
impl DerefToStrCustom for alloc::string::String {
    fn deref_to_str<'a>(&'a self) -> &'a str {
        self.as_str()
    }
}

#[cfg(all(feature = "alloc", not(feature = "std")))]
impl DerefToStrCustom for &alloc::string::String {
    fn deref_to_str<'a>(&'a self) -> &'a str {
        self.as_str()
    }
}

#[cfg(feature = "std")]
impl DerefToStrCustom for std::string::String {
    fn deref_to_str<'a>(&'a self) -> &'a str {
        self.as_str()
    }
}

#[cfg(feature = "std")]
impl DerefToStrCustom for &std::string::String {
    fn deref_to_str<'a>(&'a self) -> &'a str {
        self.as_str()
    }
}

impl DerefToStrCustom for str {
    fn deref_to_str<'a>(&'a self) -> &'a str {
        self
    }
}

impl DerefToStrCustom for &str {
    fn deref_to_str<'a>(&'a self) -> &'a str {
        self
    }
}

impl<const N: usize> DerefToStrCustom for SmallString<[u8; N]> {
    fn deref_to_str<'a>(&'a self) -> &'a str {
        self.as_str()
    }
}

impl<const N: usize> DerefToStrCustom for &SmallString<[u8; N]> {
    fn deref_to_str<'a>(&'a self) -> &'a str {
        self.as_str()
    }
}

impl<T> DerefToStrCustom for (T,)
where
    T: Deref,
    T::Target: Borrow<str>,
{
    fn deref_to_str<'a>(&'a self) -> &'a str {
        self.0.deref().borrow()
    }
}

impl<T, U> DerefToStrCustom for (T, U)
where
    T: Deref,
    T::Target: Borrow<str>,
{
    fn deref_to_str<'a>(&'a self) -> &'a str {
        self.0.deref().borrow()
    }
}

pub trait Wasip1DynCompatibleLFSSlice: core::fmt::Debug {
    /// return inode, and the name of the pre-opened directory
    fn index(
        &self,
        index: usize,
        f: &mut dyn for<'a, 'b> FnMut(Option<(&'a dyn InodeIdCommon, &'b dyn DerefToStrCustom)>),
    );
}

pub trait Wasip1DynCompatibleLFS: core::fmt::Debug {
    fn pre_open_inodes(&self, f: &mut dyn for<'a> FnMut(&'a dyn Wasip1DynCompatibleLFSSlice));

    fn fd_write_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno>;

    fn fd_write_stdout_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno>;

    fn fd_write_stderr_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno>;

    fn fd_readdir_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        buf: *mut u8,
        buf_len: usize,
        cookie: Dircookie,
    ) -> Result<(Size, Dircookie), wasip1::Errno>;

    fn path_filestat_get_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno>;

    fn fd_prestat_get_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
    ) -> Result<wasip1::Prestat, wasip1::Errno>;

    fn fd_prestat_dir_name_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> Result<(), wasip1::Errno>;

    fn fd_filestat_get_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno>;

    fn fd_pread_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        buf: *mut u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<Size, wasip1::Errno>;

    fn fd_read_stdin_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        buf: *mut u8,
        buf_len: usize,
    ) -> Result<Size, wasip1::Errno>;

    fn path_open_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        dir_ino: &dyn InodeIdCommon,
        dir_flags: wasip1::Fdflags,
        path_ptr: *const u8,
        path_len: usize,
        o_flags: wasip1::Oflags,
        fs_rights_base: wasip1::Rights,
        fs_rights_inheriting: wasip1::Rights,
        fd_flags: wasip1::Fdflags,
    ) -> Result<BoxedInodeCommon, wasip1::Errno>;
}

/// Trait for a virtual file implementation.
pub trait Wasip1FileTrait: core::fmt::Debug {
    /// Returns the size of the file.
    fn size(&self) -> usize;

    /// Reads data from the file into the provided buffer.
    /// Returns the number of bytes read.
    #[allow(unused_variables)]
    fn pread(&self, buf: &mut [u8], offset: usize) -> Result<usize, wasip1::Errno> {
        return Err(wasip1::ERRNO_NOSYS);
    }

    /// This function is called,
    /// but if the read function is implemented
    /// and the alloc feature is ON,
    /// this function is automatically implemented.
    /// Reads data from the file into the provided buffer at a given offset.
    #[allow(unused_variables)]
    fn pread_raw<Wasm: WasmAccess>(
        &self,
        buf_ptr: *mut u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<usize, wasip1::Errno> {
        #[cfg(feature = "alloc")]
        {
            use crate::utils::alloc_buff;

            let (_, nread) = unsafe {
                alloc_buff(buf_len, |b| {
                    let nread = self.pread(b, offset)?;
                    Wasm::memcpy(buf_ptr, &b[..nread]);
                    Ok(nread)
                })
            };

            nread
        }

        #[cfg(not(feature = "alloc"))]
        {
            // Stub implementation for non-std environments
            Err(wasip1::ERRNO_NOSYS)
        }
    }

    fn pread_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        buf_ptr: *mut u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<usize, wasip1::Errno> {
        #[cfg(feature = "alloc")]
        {
            use crate::utils::alloc_buff;

            let (_, nread) = unsafe {
                alloc_buff(buf_len, |b| {
                    let nread = self.pread(b, offset)?;
                    access.memcpy_with(buf_ptr, &b[..nread]);
                    Ok(nread)
                })
            };

            nread
        }

        #[cfg(not(feature = "alloc"))]
        {
            // Stub implementation for non-std environments
            Err(wasip1::ERRNO_NOSYS)
        }
    }
}

/// Trait for a virtual file system implementation.
pub trait Wasip1FileSystem: core::fmt::Debug {
    /// Writes data to a file descriptor.
    fn fd_write_raw<Wasm: WasmAccess + WasmAccessName>(
        &self,
        fd: Fd,
        iovs_ptr: *const Ciovec,
        iovs_len: usize,
        nwritten: *mut Size,
    ) -> wasip1::Errno;

    /// Reads directory entries from a file descriptor.
    fn fd_readdir_raw<Wasm: WasmAccess + WasmAccessName>(
        &self,
        fd: Fd,
        buf: *mut u8,
        buf_len: usize,
        cookie: Dircookie,
        nread: *mut Size,
    ) -> wasip1::Errno;

    /// Retrieves file statistics for a path relative to a file descriptor.
    fn path_filestat_get_raw<Wasm: WasmAccess + WasmAccessName>(
        &self,
        fd: Fd,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
        filestat: *mut wasip1::Filestat,
    ) -> wasip1::Errno;

    /// Retrieves pre-open statistics for a file descriptor.
    fn fd_prestat_get_raw<Wasm: WasmAccess + WasmAccessName>(
        &self,
        fd: Fd,
        prestat: *mut wasip1::Prestat,
    ) -> wasip1::Errno;

    /// Retrieves the name of a pre-opened directory for a file descriptor.
    fn fd_prestat_dir_name_raw<Wasm: WasmAccess + WasmAccessName>(
        &self,
        fd: Fd,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> wasip1::Errno;

    /// Closes a file descriptor.
    fn fd_close_raw<Wasm: WasmAccess + WasmAccessName>(&self, fd: Fd) -> wasip1::Errno;

    /// Retrieves file statistics for a file descriptor.
    fn fd_filestat_get_raw<Wasm: WasmAccess + WasmAccessName>(
        &self,
        fd: Fd,
        filestat: *mut wasip1::Filestat,
    ) -> wasip1::Errno;

    /// Retrieves file descriptor statistics.
    fn fd_fdstat_get_raw<Wasm: WasmAccess + WasmAccessName>(
        &self,
        fd: Fd,
        fdstat: *mut wasip1::Fdstat,
    ) -> wasip1::Errno;

    /// Reads data from a file descriptor into buffers.
    fn fd_read_raw<Wasm: WasmAccess + WasmAccessName>(
        &self,
        fd: Fd,
        iovs_ptr: *const Ciovec,
        iovs_len: usize,
        nread: *mut Size,
    ) -> wasip1::Errno;

    /// Opens a path relative to a file descriptor.
    fn path_open_raw<Wasm: WasmAccess + WasmAccessName>(
        &self,
        dir_fd: Fd,
        dir_flags: wasip1::Fdflags,
        path_ptr: *const u8,
        path_len: usize,
        o_flags: wasip1::Oflags,
        fs_rights_base: wasip1::Rights,
        fs_rights_inheriting: wasip1::Rights,
        fd_flags: wasip1::Fdflags,
        fd_ret: *mut wasip1::Fd,
    ) -> wasip1::Errno;
}

/// Plugs the file system ecosystem by defining necessary handlers.
#[macro_export]
macro_rules! plug_fs {
    ($state:expr, $($wasm:ident),* $(,)?) => {
        $crate::__as_t!(@through, $($wasm),* => $crate::plug_fs, @inner, $state);

        // To prevent unused errors from occurring
        const _: () = {
            let _ = || { $state };
        };
    };

    (@inner, $state:expr, $($wasm:ident),* $(,)?) => {
        $crate::__private::paste::paste! {
            $(
                #[cfg(target_os = "wasi")]
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_write>](
                    fd: $crate::__private::wasip1::Fd,
                    iovs_ptr: *const $crate::__private::wasip1::Ciovec,
                    iovs_len: usize,
                    nwritten: *mut usize,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::fd_write_raw::<T>(state, fd, iovs_ptr, iovs_len, nwritten)
                }

                #[cfg(target_os = "wasi")]
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_readdir>](
                    fd: $crate::__private::wasip1::Fd,
                    buf: *mut u8,
                    buf_len: usize,
                    cookie: $crate::__private::wasip1::Dircookie,
                    nread: *mut $crate::__private::wasip1::Size,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::fd_readdir_raw::<T>(state, fd, buf, buf_len, cookie, nread)
                }

                #[cfg(target_os = "wasi")]
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _path_filestat_get>](
                    fd: $crate::__private::wasip1::Fd,
                    flags: $crate::__private::wasip1::Lookupflags,
                    path_ptr: *const u8,
                    path_len: usize,
                    filestat: *mut $crate::__private::wasip1::Filestat,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::path_filestat_get_raw::<T>(state, fd, flags, path_ptr, path_len, filestat)
                }

                #[cfg(target_os = "wasi")]
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_prestat_get>](
                    fd: $crate::__private::wasip1::Fd,
                    prestat: *mut $crate::__private::wasip1::Prestat,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::fd_prestat_get_raw::<T>(state, fd, prestat)
                }

                #[cfg(target_os = "wasi")]
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_prestat_dir_name>](
                    fd: $crate::__private::wasip1::Fd,
                    dir_path_ptr: *mut u8,
                    dir_path_len: usize,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::fd_prestat_dir_name_raw::<T>(state, fd, dir_path_ptr, dir_path_len)
                }

                #[cfg(target_os = "wasi")]
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_close>](
                    fd: $crate::__private::wasip1::Fd,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::fd_close_raw::<T>(state, fd)
                }

                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _path_open>](
                    fd: $crate::__private::wasip1::Fd,
                    dir_flags: $crate::__private::wasip1::Fdflags,
                    path_ptr: *const u8,
                    path_len: usize,
                    o_flags: $crate::__private::wasip1::Oflags,
                    fs_rights_base: $crate::__private::wasip1::Rights,
                    fs_rights_inheriting: $crate::__private::wasip1::Rights,
                    fd_flags: $crate::__private::wasip1::Fdflags,
                    fd_ret: *mut $crate::__private::wasip1::Fd,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::path_open_raw::<T>(state, fd, dir_flags, path_ptr, path_len, o_flags, fs_rights_base, fs_rights_inheriting, fd_flags, fd_ret)
                }

                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_read>](
                    fd: $crate::__private::wasip1::Fd,
                    iovs_ptr: *const $crate::__private::wasip1::Ciovec,
                    iovs_len: usize,
                    nread_ret: *mut $crate::__private::wasip1::Size,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::fd_read_raw::<T>(state, fd, iovs_ptr, iovs_len, nread_ret)
                }

                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_filestat_get>](
                    fd: $crate::__private::wasip1::Fd,
                    filestat: *mut $crate::__private::wasip1::Filestat,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::fd_filestat_get_raw::<T>(state, fd, filestat)
                }

                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_fdstat_get>](
                    fd: $crate::__private::wasip1::Fd,
                    fdstat: *mut $crate::__private::wasip1::Fdstat,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::fd_fdstat_get_raw::<T>(state, fd, fdstat)
                }
            )*
        }
    };
}
