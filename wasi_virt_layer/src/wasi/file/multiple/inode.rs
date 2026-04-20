#![cfg(feature = "multiple-fs")]

use smallbox::SmallBox;

use crate::{
    __private::wasip1,
    wasi::file::{BoxedInode, InodeIdCommon, OpenFdInfo, OpenFdInfoWithInode, Wasip1LFSBase},
};

/// A normal boxed inode used for multiple file systems.
#[derive(Debug)]
pub struct BoxedInodeNormal(SmallBox<dyn InodeIdCommon, [usize; 4]>);

impl BoxedInode for BoxedInodeNormal {
    fn from_inode(inode: impl InodeIdCommon + 'static) -> Self {
        let boxed: SmallBox<dyn InodeIdCommon, [usize; 4]> = smallbox::smallbox!(inode);
        Self(boxed)
    }

    fn from_inode_with_ty<T: Wasip1LFSBase + 'static>(inode: <T as Wasip1LFSBase>::Inode) -> Self {
        let boxed: SmallBox<dyn InodeIdCommon, [usize; 4]> = smallbox::smallbox!(inode);
        Self(boxed)
    }

    fn as_inode(&self) -> &dyn InodeIdCommon {
        &*self.0
    }
}

unsafe impl Send for BoxedInodeNormal {}
unsafe impl Sync for BoxedInodeNormal {}

impl core::ops::Deref for BoxedInodeNormal {
    type Target = dyn InodeIdCommon;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl AsRef<dyn InodeIdCommon> for BoxedInodeNormal {
    fn as_ref(&self) -> &(dyn InodeIdCommon + 'static) {
        &*self.0
    }
}

/// Represents an active, open file descriptor referencing an Inode
#[derive(Debug)]
pub struct DetailedDynamicOpenFd {
    /// The inode ID this file descriptor refers to
    pub inode_id: BoxedInodeNormal,
    /// The current byte offset within the file (for reads/writes)
    pub cursor: usize,
    /// The base rights granted to this file descriptor
    pub base_rights: wasip1::Rights,
    /// The inheriting rights for new files created under this fd
    pub inheriting_rights: wasip1::Rights,
    /// The flags used to open this file
    pub fd_flags: wasip1::Fdflags,
}

unsafe impl Send for DetailedDynamicOpenFd {}
unsafe impl Sync for DetailedDynamicOpenFd {}

impl OpenFdInfo for DetailedDynamicOpenFd {
    fn cursor(&self) -> usize {
        self.cursor
    }

    fn set_cursor(&mut self, cursor: usize) {
        self.cursor = cursor;
    }

    fn base_rights(&self) -> wasip1::Rights {
        self.base_rights
    }

    fn set_base_rights(&mut self, base_rights: wasip1::Rights) {
        self.base_rights = base_rights;
    }

    fn inheriting_rights(&self) -> wasip1::Rights {
        self.inheriting_rights
    }

    fn set_inheriting_rights(&mut self, inheriting_rights: wasip1::Rights) {
        self.inheriting_rights = inheriting_rights;
    }

    fn fd_flags(&self) -> wasip1::Fdflags {
        self.fd_flags
    }

    fn set_fd_flags(&mut self, fd_flags: wasip1::Fdflags) {
        self.fd_flags = fd_flags;
    }
}

impl OpenFdInfoWithInode for DetailedDynamicOpenFd {
    type InodeId = BoxedInodeNormal;

    fn from_inode_id(inode_id: Self::InodeId) -> Self {
        Self {
            inode_id,
            cursor: 0,
            base_rights: 0,
            inheriting_rights: 0,
            fd_flags: 0,
        }
    }

    fn inode_id(&self) -> &Self::InodeId {
        &self.inode_id
    }

    fn set_inode_id(&mut self, inode_id: Self::InodeId) {
        self.inode_id = inode_id;
    }
}
