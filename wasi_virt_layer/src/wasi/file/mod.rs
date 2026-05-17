// https://docs.rs/wasmtime-wasi/17.0.3/wasmtime_wasi/struct.WasiCtx.html
// https://docs.rs/wasi-common/17.0.3/wasi_common/table/struct.Table.html

#[cfg(any(feature = "embedded-fs", feature = "dynamic-fs"))]
use core::any::Any;
#[cfg(any(feature = "embedded-fs", feature = "dynamic-fs"))]
use core::borrow::Borrow;
#[cfg(any(feature = "embedded-fs", feature = "dynamic-fs"))]
use core::ops::Deref;

#[cfg(any(feature = "embedded-fs", feature = "dynamic-fs"))]
use smallstr::SmallString;

#[cfg(feature = "alloc")]
use crate::memory::WasmAccessDynCompatible as _;
use crate::memory::{WasmAccess, WasmAccessDynCompatibleRaw, WasmAccessName};
#[cfg(feature = "dynamic-fs")]
pub mod dynamic;
#[cfg(feature = "embedded-fs")]
pub mod embedded;
#[cfg(feature = "multiple-fs")]
pub mod multiple;
#[cfg(any(feature = "embedded-fs", feature = "dynamic-fs"))]
pub mod stdio;
#[cfg(any(
    feature = "embedded-fs",
    feature = "dynamic-fs",
    feature = "multiple-fs"
))]
pub(crate) mod trace;
pub(crate) mod types;
use crate::__private::wasip1;

pub use self::types::{BoxedInode, InodeIdCommon, OpenFdInfo, OpenFdInfoWithInode};

// no implementing dcache

use crate::__private::wasip1::*;

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

pub use const_struct::ConstDefault;

/// Additional mutable info for an inode.
pub trait WasiAddInfo: core::fmt::Debug + Clone + Copy + ConstDefault {
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

#[cfg(any(feature = "embedded-fs", feature = "dynamic-fs"))]
#[allow(dead_code)]
pub(crate) trait Wasip1LFSBaseWrapper: Wasip1LFSBase {
    fn downcast_inode(inode: &dyn InodeIdCommon) -> &<Self as Wasip1LFSBase>::Inode;
}

#[cfg(any(feature = "embedded-fs", feature = "dynamic-fs"))]
impl<T: Wasip1LFSBase + ?Sized> Wasip1LFSBaseWrapper for T
where
    <Self as Wasip1LFSBase>::Inode: 'static,
{
    fn downcast_inode(base_inode: &dyn InodeIdCommon) -> &<Self as Wasip1LFSBase>::Inode {
        let inode = base_inode as &dyn Any;

        #[cfg(feature = "trace")]
        {
            inode
                .downcast_ref::<<Self as Wasip1LFSBase>::Inode>()
                .unwrap_or_else(|| {
                    panic!(
                        "Failed to downcast inode. Expected type: {}, but got a different type. {:?}",
                        core::any::type_name::<<Self as Wasip1LFSBase>::Inode>(),
                        base_inode
                    )
                })
        }

        #[cfg(not(feature = "trace"))]
        {
            inode
                .downcast_ref::<<Self as Wasip1LFSBase>::Inode>()
                .unwrap()
        }
    }
}

/// small posix like local file system
/// Trait for a local file system implementation.
pub trait Wasip1LFSBase: core::fmt::Debug {
    /// The type used for inodes.
    /// Pre-opened inodes.
    type Inode: InodeIdCommon;

    /// Writes raw data to a file.
    fn fd_write_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno>;

    /// Writes raw data to stdout.
    fn fd_write_stdout_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno>;

    /// Writes raw data to stderr.
    fn fd_write_stderr_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        data: *const u8,
        data_len: usize,
    ) -> Result<Size, wasip1::Errno>;

    /// Returns whether the inode is a directory.
    fn is_dir(&self, inode: &Self::Inode) -> bool;

    /// Reads directory entries.
    fn fd_readdir_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
        buf: *mut u8,
        buf_len: usize,
        cookie: Dircookie,
    ) -> Result<(Size, Dircookie), wasip1::Errno>;

    /// Retrieves file statistics for a path.
    fn path_filestat_get_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno>;

    /// Retrieves pre-open statistics.
    fn fd_prestat_get_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
    ) -> Result<wasip1::Prestat, wasip1::Errno>;

    /// Retrieves the name of a pre-opened directory.
    fn fd_prestat_dir_name_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> Result<(), wasip1::Errno>;

    /// Retrieves file statistics for a file descriptor.
    fn fd_filestat_get_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno>;

    /// Reads data from a file descriptor into a buffer at a given offset.
    fn fd_pread_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
        buf: *mut u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<Size, wasip1::Errno>;

    /// Writes data to a file descriptor from a buffer at a given offset.
    fn fd_pwrite_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
        buf: *const u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<Size, wasip1::Errno>;

    /// Provides advice on how a file will be accessed.
    fn fd_advise_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
        offset: u64,
        len: u64,
        advice: wasip1::Advice,
    ) -> Result<(), wasip1::Errno>;

    /// Allocates space for a file.
    fn fd_allocate_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
        offset: u64,
        len: u64,
    ) -> Result<(), wasip1::Errno>;

    /// Synchronizes the data of a file to disk.
    fn fd_datasync_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
    ) -> Result<(), wasip1::Errno>;

    /// Synchronizes the data and metadata of a file to disk.
    fn fd_sync_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
    ) -> Result<(), wasip1::Errno>;

    /// Sets the size of a file.
    fn fd_filestat_set_size_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
        size: u64,
    ) -> Result<(), wasip1::Errno>;

    /// Sets the timestamps of a file.
    fn fd_filestat_set_times_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
        atim: wasip1::Timestamp,
        mtim: wasip1::Timestamp,
        fst_flags: wasip1::Fstflags,
    ) -> Result<(), wasip1::Errno>;

    /// Sets the timestamps of a file or directory at a path.
    fn path_filestat_set_times_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
        atim: wasip1::Timestamp,
        mtim: wasip1::Timestamp,
        fst_flags: wasip1::Fstflags,
    ) -> Result<(), wasip1::Errno>;

    /// Creates a symbolic link.
    fn path_symlink_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
        old_path_ptr: *const u8,
        old_path_len: usize,
        new_path_ptr: *const u8,
        new_path_len: usize,
    ) -> Result<(), wasip1::Errno>;

    /// Reads data from stdin.
    fn fd_read_stdin_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        buf: *mut u8,
        buf_len: usize,
    ) -> Result<Size, wasip1::Errno>;

    /// Opens a path.
    fn path_open_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
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

    /// Reads the contents of a symbolic link.
    fn path_readlink_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
        path_ptr: *const u8,
        path_len: usize,
        buf: *mut u8,
        buf_len: usize,
    ) -> Result<Size, wasip1::Errno>;

    /// Creates a directory.
    fn path_create_directory_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        dir_ino: &Self::Inode,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<(), wasip1::Errno>;

    /// Creates a hard link.
    fn path_link_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        old_dir_ino: &Self::Inode,
        old_flags: wasip1::Lookupflags,
        old_path_ptr: *const u8,
        old_path_len: usize,
        new_dir_ino: &Self::Inode,
        new_path_ptr: *const u8,
        new_path_len: usize,
    ) -> Result<(), wasip1::Errno>;

    /// Removes a directory.
    fn path_remove_directory_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        dir_ino: &Self::Inode,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<(), wasip1::Errno>;

    /// Renames a file or directory.
    fn path_rename_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        old_dir_ino: &Self::Inode,
        old_path_ptr: *const u8,
        old_path_len: usize,
        new_dir_ino: &Self::Inode,
        new_path_ptr: *const u8,
        new_path_len: usize,
    ) -> Result<(), wasip1::Errno>;

    /// Unlinks a file.
    fn path_unlink_file_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        dir_ino: &Self::Inode,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<(), wasip1::Errno>;
}

/// Trait for a static or constant local file system implementation.
#[cfg(feature = "embedded-fs")]
pub trait EmbeddedLFS: Wasip1LFSBase
where
    Self::Inode: 'static,
{
    const PRE_OPEN: &'static [Self::Inode];
}

#[cfg(any(feature = "embedded-fs", feature = "dynamic-fs"))]
#[allow(dead_code)]
pub trait DynamicLFS: Wasip1LFSBase {
    fn pre_open_inodes(&self) -> impl IntoIterator<Item = (Self::Inode, impl DerefToStrCustom)>;
}

#[cfg(any(feature = "embedded-fs", feature = "dynamic-fs"))]
#[allow(dead_code)]
pub trait DerefToStrCustom {
    fn deref_to_str<'a>(&'a self) -> &'a str;
}

#[cfg(all(
    any(feature = "embedded-fs", feature = "dynamic-fs"),
    feature = "alloc",
    not(feature = "std")
))]
impl DerefToStrCustom for alloc::string::String {
    fn deref_to_str<'a>(&'a self) -> &'a str {
        self.as_str()
    }
}

#[cfg(all(
    any(feature = "embedded-fs", feature = "dynamic-fs"),
    feature = "alloc",
    not(feature = "std")
))]
impl DerefToStrCustom for &alloc::string::String {
    fn deref_to_str<'a>(&'a self) -> &'a str {
        self.as_str()
    }
}

#[cfg(all(any(feature = "embedded-fs", feature = "dynamic-fs"), feature = "std"))]
impl DerefToStrCustom for std::string::String {
    fn deref_to_str<'a>(&'a self) -> &'a str {
        self.as_str()
    }
}

#[cfg(all(any(feature = "embedded-fs", feature = "dynamic-fs"), feature = "std"))]
impl DerefToStrCustom for &std::string::String {
    fn deref_to_str<'a>(&'a self) -> &'a str {
        self.as_str()
    }
}

#[cfg(any(feature = "embedded-fs", feature = "dynamic-fs"))]
impl DerefToStrCustom for str {
    fn deref_to_str<'a>(&'a self) -> &'a str {
        self
    }
}

#[cfg(any(feature = "embedded-fs", feature = "dynamic-fs"))]
impl DerefToStrCustom for &str {
    fn deref_to_str<'a>(&'a self) -> &'a str {
        self
    }
}

#[cfg(any(feature = "embedded-fs", feature = "dynamic-fs"))]
impl<const N: usize> DerefToStrCustom for SmallString<[u8; N]> {
    fn deref_to_str<'a>(&'a self) -> &'a str {
        self.as_str()
    }
}

#[cfg(any(feature = "embedded-fs", feature = "dynamic-fs"))]
impl<const N: usize> DerefToStrCustom for &SmallString<[u8; N]> {
    fn deref_to_str<'a>(&'a self) -> &'a str {
        self.as_str()
    }
}

#[cfg(any(feature = "embedded-fs", feature = "dynamic-fs"))]
impl<T> DerefToStrCustom for (T,)
where
    T: Deref,
    T::Target: Borrow<str>,
{
    fn deref_to_str<'a>(&'a self) -> &'a str {
        self.0.deref().borrow()
    }
}

#[cfg(any(feature = "embedded-fs", feature = "dynamic-fs"))]
impl<T, U> DerefToStrCustom for (T, U)
where
    T: Deref,
    T::Target: Borrow<str>,
{
    fn deref_to_str<'a>(&'a self) -> &'a str {
        self.0.deref().borrow()
    }
}

#[cfg(any(feature = "embedded-fs", feature = "dynamic-fs"))]
#[allow(dead_code)]
pub trait Wasip1DynCompatibleLFSSlice: core::fmt::Debug {
    /// return inode, and the name of the pre-opened directory
    fn index(
        &self,
        index: usize,
        f: &mut dyn for<'a, 'b> FnMut(Option<(&'a dyn InodeIdCommon, &'b dyn DerefToStrCustom)>),
    );
}

/// Trait for a dynamically compatible local file system.
#[cfg(any(feature = "embedded-fs", feature = "dynamic-fs"))]
#[allow(dead_code)]
pub trait Wasip1DynCompatibleLFS<B: BoxedInode>: core::fmt::Debug {
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
    ) -> Result<B, wasip1::Errno>;

    fn path_readlink_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        path_ptr: *const u8,
        path_len: usize,
        buf: *mut u8,
        buf_len: usize,
    ) -> Result<Size, wasip1::Errno>;

    fn path_create_directory_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        dir_inode: &dyn InodeIdCommon,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<(), wasip1::Errno>;

    fn path_link_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        old_dir_inode: &dyn InodeIdCommon,
        old_flags: wasip1::Lookupflags,
        old_path_ptr: *const u8,
        old_path_len: usize,
        new_dir_inode: &dyn InodeIdCommon,
        new_path_ptr: *const u8,
        new_path_len: usize,
    ) -> Result<(), wasip1::Errno>;

    fn path_remove_directory_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        dir_inode: &dyn InodeIdCommon,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<(), wasip1::Errno>;

    fn path_rename_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        old_dir_inode: &dyn InodeIdCommon,
        old_path_ptr: *const u8,
        old_path_len: usize,
        new_dir_inode: &dyn InodeIdCommon,
        new_path_ptr: *const u8,
        new_path_len: usize,
    ) -> Result<(), wasip1::Errno>;

    fn path_unlink_file_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        dir_inode: &dyn InodeIdCommon,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<(), wasip1::Errno>;

    fn fd_pwrite_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        data: *const u8,
        data_len: usize,
        offset: usize,
    ) -> Result<Size, wasip1::Errno>;

    fn fd_advise_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        offset: u64,
        len: u64,
        advice: wasip1::Advice,
    ) -> Result<(), wasip1::Errno>;

    fn fd_allocate_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        offset: u64,
        len: u64,
    ) -> Result<(), wasip1::Errno>;

    fn fd_datasync_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
    ) -> Result<(), wasip1::Errno>;

    fn fd_sync_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
    ) -> Result<(), wasip1::Errno>;

    fn fd_filestat_set_size_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        size: u64,
    ) -> Result<(), wasip1::Errno>;

    fn fd_filestat_set_times_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        atim: wasip1::Timestamp,
        mtim: wasip1::Timestamp,
        fst_flags: wasip1::Fstflags,
    ) -> Result<(), wasip1::Errno>;

    fn path_filestat_set_times_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
        atim: wasip1::Timestamp,
        mtim: wasip1::Timestamp,
        fst_flags: wasip1::Fstflags,
    ) -> Result<(), wasip1::Errno>;

    fn path_symlink_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        old_path_ptr: *const u8,
        old_path_len: usize,
        new_path_ptr: *const u8,
        new_path_len: usize,
    ) -> Result<(), wasip1::Errno>;
}

/// Trait for a virtual file implementation.
pub trait Wasip1FileTrait: core::fmt::Debug {
    /// Returns the size of the file.
    fn size(&self) -> usize;

    /// Writes data to the file from the provided buffer at a given offset.
    fn pwrite(&self, buf: &[u8], offset: usize) -> Result<usize, wasip1::Errno>;

    /// Writes data to the file from the provided buffer at a given offset.
    fn pwrite_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        buf_ptr: *const u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<usize, wasip1::Errno>;

    /// Writes data to the file from the provided buffer at a given offset compatible with dynamic dispatch.
    fn pwrite_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        buf_ptr: *const u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<usize, wasip1::Errno>;

    /// Provides advice on how a file will be accessed.
    fn advise(&self, offset: u64, len: u64, advice: wasip1::Advice) -> Result<(), wasip1::Errno>;

    /// Allocates space for a file.
    fn allocate(&self, offset: u64, len: u64) -> Result<(), wasip1::Errno>;

    /// Synchronizes the data of a file to disk.
    fn datasync(&self) -> Result<(), wasip1::Errno>;

    /// Synchronizes the data and metadata of a file to disk.
    fn sync(&self) -> Result<(), wasip1::Errno>;

    /// Sets the size of a file.
    fn filestat_set_size(&self, size: u64) -> Result<(), wasip1::Errno>;

    /// Sets the timestamps of a file.
    fn filestat_set_times(
        &self,
        atim: wasip1::Timestamp,
        mtim: wasip1::Timestamp,
        fst_flags: wasip1::Fstflags,
    ) -> Result<(), wasip1::Errno>;

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
    fn pread_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
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

    /// Reads data from the file into the provided buffer at a given offset compatible with dynamic dispatch.
    #[allow(unused_variables)]
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
    fn fd_write_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        iovs_ptr: *const Ciovec,
        iovs_len: usize,
        nwritten: *mut Size,
    ) -> wasip1::Errno;

    /// Writes data to a file descriptor from a buffer at a given offset.
    fn fd_pwrite_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        iovs_ptr: *const Ciovec,
        iovs_len: usize,
        offset: u64,
        nwritten: *mut Size,
    ) -> wasip1::Errno;

    /// Provides advice on how a file will be accessed.
    fn fd_advise_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        offset: u64,
        len: u64,
        advice: wasip1::Advice,
    ) -> wasip1::Errno;

    /// Allocates space for a file.
    fn fd_allocate_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        offset: u64,
        len: u64,
    ) -> wasip1::Errno;

    /// Synchronizes the data of a file to disk.
    fn fd_datasync_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
    ) -> wasip1::Errno;

    /// Synchronizes the data and metadata of a file to disk.
    fn fd_sync_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
    ) -> wasip1::Errno;

    /// Gets the current offset of a file descriptor.
    fn fd_tell_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        offset_ret: *mut u64,
    ) -> wasip1::Errno;

    /// Sets the flags of a file descriptor.
    fn fd_fdstat_set_flags_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        flags: wasip1::Fdflags,
    ) -> wasip1::Errno;

    /// Sets the rights of a file descriptor.
    fn fd_fdstat_set_rights_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        fs_rights_base: wasip1::Rights,
        fs_rights_inheriting: wasip1::Rights,
    ) -> wasip1::Errno;

    /// Sets the size of a file.
    fn fd_filestat_set_size_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        size: u64,
    ) -> wasip1::Errno;

    /// Sets the timestamps of a file.
    fn fd_filestat_set_times_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        atim: wasip1::Timestamp,
        mtim: wasip1::Timestamp,
        fst_flags: wasip1::Fstflags,
    ) -> wasip1::Errno;

    /// Sets the timestamps of a file or directory at a path.
    fn path_filestat_set_times_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
        atim: wasip1::Timestamp,
        mtim: wasip1::Timestamp,
        fst_flags: wasip1::Fstflags,
    ) -> wasip1::Errno;

    /// Creates a symbolic link.
    fn path_symlink_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        old_path_ptr: *const u8,
        old_path_len: usize,
        fd: Fd,
        new_path_ptr: *const u8,
        new_path_len: usize,
    ) -> wasip1::Errno;

    /// Renumbers a file descriptor.
    fn fd_renumber_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        to: Fd,
    ) -> wasip1::Errno;

    /// Reads directory entries from a file descriptor.
    fn fd_readdir_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        buf: *mut u8,
        buf_len: usize,
        cookie: Dircookie,
        nread: *mut Size,
    ) -> wasip1::Errno;

    /// Retrieves file statistics for a path relative to a file descriptor.
    fn path_filestat_get_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
        filestat: *mut wasip1::Filestat,
    ) -> wasip1::Errno;

    /// Retrieves pre-open statistics for a file descriptor.
    fn fd_prestat_get_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        prestat: *mut wasip1::Prestat,
    ) -> wasip1::Errno;

    /// Retrieves the name of a pre-opened directory for a file descriptor.
    fn fd_prestat_dir_name_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> wasip1::Errno;

    /// Closes a file descriptor.
    fn fd_close_raw<Wasm: WasmAccess + WasmAccessName + 'static>(&self, fd: Fd) -> wasip1::Errno;

    /// Retrieves file statistics for a file descriptor.
    fn fd_filestat_get_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        filestat: *mut wasip1::Filestat,
    ) -> wasip1::Errno;

    /// Retrieves file descriptor statistics.
    fn fd_fdstat_get_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        fdstat: *mut wasip1::Fdstat,
    ) -> wasip1::Errno;

    /// Reads data from a file descriptor into buffers.
    fn fd_read_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        iovs_ptr: *const Ciovec,
        iovs_len: usize,
        nread: *mut Size,
    ) -> wasip1::Errno;

    /// Reads data from a file descriptor into buffers at a given offset.
    fn fd_pread_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        iovs_ptr: *const Ciovec,
        iovs_len: usize,
        offset: u64,
        nread: *mut Size,
    ) -> wasip1::Errno;

    /// Seeks to a position in a file descriptor.
    fn fd_seek_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        offset: i64,
        whence: wasip1::Whence,
        new_offset_ptr: *mut i64,
    ) -> wasip1::Errno;

    /// Opens a path relative to a file descriptor.
    fn path_open_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
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

    /// Reads the contents of a symbolic link relative to a file descriptor.
    fn path_readlink_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        path_ptr: *const u8,
        path_len: usize,
        buf: *mut u8,
        buf_len: usize,
        buf_nread: *mut Size,
    ) -> wasip1::Errno;

    /// Creates a directory.
    fn path_create_directory_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        path_ptr: *const u8,
        path_len: usize,
    ) -> wasip1::Errno;

    /// Creates a hard link.
    fn path_link_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        old_fd: Fd,
        old_flags: wasip1::Lookupflags,
        old_path_ptr: *const u8,
        old_path_len: usize,
        new_fd: Fd,
        new_path_ptr: *const u8,
        new_path_len: usize,
    ) -> wasip1::Errno;

    /// Removes a directory.
    fn path_remove_directory_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        path_ptr: *const u8,
        path_len: usize,
    ) -> wasip1::Errno;

    /// Renames a file or directory.
    fn path_rename_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        old_fd: Fd,
        old_path_ptr: *const u8,
        old_path_len: usize,
        new_fd: Fd,
        new_path_ptr: *const u8,
        new_path_len: usize,
    ) -> wasip1::Errno;

    /// Unlinks a file.
    fn path_unlink_file_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        fd: Fd,
        path_ptr: *const u8,
        path_len: usize,
    ) -> wasip1::Errno;
}

/// Plugs the file system ecosystem by defining necessary handlers.
/// It ensures that the provided Wasm uses the given virtual file system.
/// If self is passed, file operations performed within the VFS will also use the virtual file system.
/// If used properly, it can even be wrapped as a component without imports.
/// On the other hand, if plug_fs is not applied to self, file access in the VFS will use the WASI p1 ABI to access the external file system.
///
/// ```rust,no_run
/// use wasi_virt_layer::prelude::*;
///
/// import_wasm!(test_wasm);
///
/// // Example: plug a virtual file system to `test_wasm`
/// // assuming `vfs` implements `Wasip1FileSystem`
/// // plug_fs!(&vfs, test_wasm, self);
/// ```
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
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_seek>](
                    fd: $crate::__private::wasip1::Fd,
                    offset: i64,
                    whence: i8,
                    new_offset_ptr: *mut i64,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    let whence_const = match whence as u8 {
                        0 => $crate::__private::wasip1::WHENCE_SET,
                        1 => $crate::__private::wasip1::WHENCE_CUR,
                        2 => $crate::__private::wasip1::WHENCE_END,
                        _ => {
                            // Invalid whence value - let fd_seek_raw handle the error
                            return $crate::__private::wasip1::ERRNO_INVAL;
                        }
                    };
                    $crate::file::Wasip1FileSystem::fd_seek_raw::<T>(state, fd, offset, whence_const, new_offset_ptr)
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

                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _path_readlink>](
                    fd: $crate::__private::wasip1::Fd,
                    path_ptr: *const u8,
                    path_len: usize,
                    buf: *mut u8,
                    buf_len: usize,
                    buf_nread: *mut $crate::__private::wasip1::Size,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::path_readlink_raw::<T>(state, fd, path_ptr, path_len, buf, buf_len, buf_nread)
                }

                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _path_create_directory>](
                    fd: $crate::__private::wasip1::Fd,
                    path_ptr: *const u8,
                    path_len: usize,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::path_create_directory_raw::<T>(state, fd, path_ptr, path_len)
                }

                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _path_link>](
                    old_fd: $crate::__private::wasip1::Fd,
                    old_flags: $crate::__private::wasip1::Lookupflags,
                    old_path_ptr: *const u8,
                    old_path_len: usize,
                    new_fd: $crate::__private::wasip1::Fd,
                    new_path_ptr: *const u8,
                    new_path_len: usize,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::path_link_raw::<T>(state, old_fd, old_flags, old_path_ptr, old_path_len, new_fd, new_path_ptr, new_path_len)
                }

                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _path_remove_directory>](
                    fd: $crate::__private::wasip1::Fd,
                    path_ptr: *const u8,
                    path_len: usize,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::path_remove_directory_raw::<T>(state, fd, path_ptr, path_len)
                }

                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _path_rename>](
                    old_fd: $crate::__private::wasip1::Fd,
                    old_path_ptr: *const u8,
                    old_path_len: usize,
                    new_fd: $crate::__private::wasip1::Fd,
                    new_path_ptr: *const u8,
                    new_path_len: usize,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::path_rename_raw::<T>(state, old_fd, old_path_ptr, old_path_len, new_fd, new_path_ptr, new_path_len)
                }

                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _path_unlink_file>](
                    fd: $crate::__private::wasip1::Fd,
                    path_ptr: *const u8,
                    path_len: usize,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::path_unlink_file_raw::<T>(state, fd, path_ptr, path_len)
                }

                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_pread>](
                    fd: $crate::__private::wasip1::Fd,
                    iovs_ptr: *const $crate::__private::wasip1::Ciovec,
                    iovs_len: usize,
                    offset: u64,
                    nread_ret: *mut $crate::__private::wasip1::Size,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::fd_pread_raw::<T>(state, fd, iovs_ptr, iovs_len, offset, nread_ret)
                }

                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_pwrite>](
                    fd: $crate::__private::wasip1::Fd,
                    iovs_ptr: *const $crate::__private::wasip1::Ciovec,
                    iovs_len: usize,
                    offset: u64,
                    nwritten_ret: *mut $crate::__private::wasip1::Size,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::fd_pwrite_raw::<T>(state, fd, iovs_ptr, iovs_len, offset, nwritten_ret)
                }

                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_advise>](
                    fd: $crate::__private::wasip1::Fd,
                    offset: u64,
                    len: u64,
                    advice: $crate::__private::wasip1::Advice,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::fd_advise_raw::<T>(state, fd, offset, len, advice)
                }

                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_allocate>](
                    fd: $crate::__private::wasip1::Fd,
                    offset: u64,
                    len: u64,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::fd_allocate_raw::<T>(state, fd, offset, len)
                }

                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_datasync>](
                    fd: $crate::__private::wasip1::Fd,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::fd_datasync_raw::<T>(state, fd)
                }

                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_sync>](
                    fd: $crate::__private::wasip1::Fd,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::fd_sync_raw::<T>(state, fd)
                }

                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_tell>](
                    fd: $crate::__private::wasip1::Fd,
                    offset_ret: *mut u64,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::fd_tell_raw::<T>(state, fd, offset_ret)
                }

                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_fdstat_set_flags>](
                    fd: $crate::__private::wasip1::Fd,
                    flags: $crate::__private::wasip1::Fdflags,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::fd_fdstat_set_flags_raw::<T>(state, fd, flags)
                }

                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_fdstat_set_rights>](
                    fd: $crate::__private::wasip1::Fd,
                    fs_rights_base: $crate::__private::wasip1::Rights,
                    fs_rights_inheriting: $crate::__private::wasip1::Rights,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::fd_fdstat_set_rights_raw::<T>(state, fd, fs_rights_base, fs_rights_inheriting)
                }

                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_filestat_set_size>](
                    fd: $crate::__private::wasip1::Fd,
                    size: u64,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::fd_filestat_set_size_raw::<T>(state, fd, size)
                }

                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_filestat_set_times>](
                    fd: $crate::__private::wasip1::Fd,
                    atim: $crate::__private::wasip1::Timestamp,
                    mtim: $crate::__private::wasip1::Timestamp,
                    fst_flags: $crate::__private::wasip1::Fstflags,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::fd_filestat_set_times_raw::<T>(state, fd, atim, mtim, fst_flags)
                }

                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _path_filestat_set_times>](
                    fd: $crate::__private::wasip1::Fd,
                    flags: $crate::__private::wasip1::Lookupflags,
                    path_ptr: *const u8,
                    path_len: usize,
                    atim: $crate::__private::wasip1::Timestamp,
                    mtim: $crate::__private::wasip1::Timestamp,
                    fst_flags: $crate::__private::wasip1::Fstflags,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::path_filestat_set_times_raw::<T>(state, fd, flags, path_ptr, path_len, atim, mtim, fst_flags)
                }

                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _path_symlink>](
                    old_path_ptr: *const u8,
                    old_path_len: usize,
                    fd: $crate::__private::wasip1::Fd,
                    new_path_ptr: *const u8,
                    new_path_len: usize,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::path_symlink_raw::<T>(state, old_path_ptr, old_path_len, fd, new_path_ptr, new_path_len)
                }

                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _fd_renumber>](
                    fd: $crate::__private::wasip1::Fd,
                    to: $crate::__private::wasip1::Fd,
                ) -> $crate::__private::wasip1::Errno {
                    let state = $state;
                    $crate::__as_t!(@as_t, $wasm);
                    $crate::file::Wasip1FileSystem::fd_renumber_raw::<T>(state, fd, to)
                }
            )*
        }
    };
}
