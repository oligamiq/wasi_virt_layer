use crate::__private::wasip1;

/// Common inode identity bound shared across dynamic file-system backends.
pub trait InodeIdCommon: core::fmt::Debug + core::any::Any + 'static {}

impl<T: core::fmt::Debug + 'static> InodeIdCommon for T {}

/// Type-erased inode wrapper used by dynamic backend dispatch.
pub trait BoxedInode: core::fmt::Debug + Send + Sync {
    /// Creates a boxed inode from a generic inode implementing `InodeIdCommon`.
    fn from_inode(inode: impl InodeIdCommon + 'static) -> Self;
    /// Creates a boxed inode from a strongly-typed inode associated with a specific LFS backend.
    fn from_inode_with_ty<T: crate::wasi::file::Wasip1LFSBase + 'static>(
        inode: <T as crate::wasi::file::Wasip1LFSBase>::Inode,
    ) -> Self;
    /// Returns the underlying type-erased inode.
    fn as_inode(&self) -> &dyn InodeIdCommon;
}

/// Per-file-descriptor state used by virtual file systems.
pub trait OpenFdInfo: core::fmt::Debug + Sized {
    /// Returns the current read/write cursor for the file descriptor.
    fn cursor(&self) -> usize;
    /// Sets the read/write cursor for the file descriptor.
    fn set_cursor(&mut self, cursor: usize);

    /// Returns the base capabilities (rights) associated with the file descriptor.
    fn base_rights(&self) -> wasip1::Rights {
        !0
    }
    /// Sets the base capabilities (rights) associated with the file descriptor.
    fn set_base_rights(&mut self, _base_rights: wasip1::Rights) {}

    /// Returns the capabilities (rights) that new file descriptors inherit from this one.
    fn inheriting_rights(&self) -> wasip1::Rights {
        !0
    }
    /// Sets the capabilities (rights) that new file descriptors inherit from this one.
    fn set_inheriting_rights(&mut self, _inheriting_rights: wasip1::Rights) {}

    /// Returns the file descriptor flags (e.g. append, non-blocking).
    fn fd_flags(&self) -> wasip1::Fdflags {
        0
    }
    /// Sets the file descriptor flags (e.g. append, non-blocking).
    fn set_fd_flags(&mut self, _fd_flags: wasip1::Fdflags) {}
}

/// Open file descriptor information that carries an inode handle.
pub trait OpenFdInfoWithInode: OpenFdInfo {
    /// The associated inode type.
    type InodeId: InodeIdCommon;

    /// Creates an instance from the given inode identifier.
    fn from_inode_id(inode_id: Self::InodeId) -> Self;
    /// Returns a reference to the associated inode identifier.
    fn inode_id(&self) -> &Self::InodeId;
    /// Sets the associated inode identifier.
    fn set_inode_id(&mut self, inode_id: Self::InodeId);
}
