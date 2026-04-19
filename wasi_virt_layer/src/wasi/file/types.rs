use crate::__private::wasip1;

/// Common inode identity bound shared across dynamic file-system backends.
pub trait InodeIdCommon: core::fmt::Debug + core::any::Any + 'static {}

impl<T: core::fmt::Debug + 'static> InodeIdCommon for T {}

/// Type-erased inode wrapper used by dynamic backend dispatch.
pub trait BoxedInode: core::fmt::Debug + Send + Sync {
    fn from_inode(inode: impl InodeIdCommon + 'static) -> Self;
    fn from_inode_with_ty<T: crate::wasi::file::Wasip1LFSBase + 'static>(
        inode: <T as crate::wasi::file::Wasip1LFSBase>::Inode,
    ) -> Self;
    fn as_inode(&self) -> &dyn InodeIdCommon;
}

/// Per-file-descriptor state used by virtual file systems.
pub trait OpenFdInfo: core::fmt::Debug + Sized {
    fn cursor(&self) -> usize;
    fn set_cursor(&mut self, cursor: usize);

    fn base_rights(&self) -> wasip1::Rights {
        !0
    }
    fn set_base_rights(&mut self, _base_rights: wasip1::Rights) {}

    fn inheriting_rights(&self) -> wasip1::Rights {
        !0
    }
    fn set_inheriting_rights(&mut self, _inheriting_rights: wasip1::Rights) {}

    fn fd_flags(&self) -> wasip1::Fdflags {
        0
    }
    fn set_fd_flags(&mut self, _fd_flags: wasip1::Fdflags) {}
}

/// Open file descriptor information that carries an inode handle.
pub trait OpenFdInfoWithInode: OpenFdInfo {
    type InodeId: InodeIdCommon;

    fn from_inode_id(inode_id: Self::InodeId) -> Self;
    fn inode_id(&self) -> &Self::InodeId;
    fn set_inode_id(&mut self, inode_id: Self::InodeId);
}
