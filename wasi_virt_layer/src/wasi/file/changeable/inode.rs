#![cfg(feature = "changeable-fs")]

use crate::wasi::file::{ConstDefault, InodeIdCommon, OpenFdInfo, OpenFdInfoWithInode, WasiAddInfo};
use crate::__private::wasip1;
use alloc::{collections::BTreeMap, string::String, vec::Vec};
use smallstr::SmallString;

/// Unique identifier for an inode
pub type InodeId = usize;

/// Directory entries map a file name to its underlying inode ID
pub type DirMap = BTreeMap<SmallString<[u8; 32]>, InodeId>;

/// Represents the data contained within an Inode
#[derive(Debug, Clone)]
pub enum InodeData {
    /// A standard file containing a byte vector
    File(Vec<u8>),
    /// A directory containing a map of filenames to child inodes
    Dir(DirMap),
    /// A symbolic link containing its target path
    Symlink(String),
}

/// Metadata for an inode, mirroring typical POSIX filesystem stats
#[derive(Debug, Clone)]
pub struct InodeMetadata<AddInfo: WasiAddInfo> {
    /// File type (e.g., File, Directory, Symlink)
    pub filetype: wasip1::Filetype,
    /// Hard link count
    pub nlink: wasip1::Linkcount,
    /// User-defined WASI permissions (base rights that can be acquired)
    pub rights: wasip1::Rights,
    /// Additional file info (e.g. timestamps)
    pub add_info: AddInfo,
}

impl<AddInfo: WasiAddInfo + ConstDefault> InodeMetadata<AddInfo> {
    /// Create new default metadata for a given file type
    pub const fn const_new(filetype: wasip1::Filetype, rights: wasip1::Rights) -> Self {
        Self {
            filetype,
            nlink: 1,
            rights,
            add_info: AddInfo::DEFAULT,
        }
    }
}

impl<AddInfo: WasiAddInfo> InodeMetadata<AddInfo> {
    pub fn new(filetype: wasip1::Filetype, rights: wasip1::Rights, add_info: AddInfo) -> Self {
        Self {
            filetype,
            nlink: 1,
            rights,
            add_info,
        }
    }

    /// Calculate the size based on the inode data
    pub fn size(&self, data: &InodeData) -> wasip1::Filesize {
        match data {
            InodeData::File(vec) => vec.len() as wasip1::Filesize,
            InodeData::Dir(_) => 0,
            InodeData::Symlink(target) => target.len() as wasip1::Filesize,
        }
    }
}

/// An inode couples its metadata with its actual data payload
#[derive(Debug, Clone)]
pub struct Inode<AddInfo: WasiAddInfo> {
    /// Metadata like timestamps and type
    pub meta: InodeMetadata<AddInfo>,
    /// Actual contents of the inode
    pub data: InodeData,
}

/// Represents an active, open file descriptor referencing an Inode
#[derive(Debug, Clone)]
pub struct DetailedOpenFd<InodeId: InodeIdCommon> {
    /// The inode ID this file descriptor refers to
    pub inode_id: InodeId,
    /// The current byte offset within the file (for reads/writes)
    pub cursor: usize,
    /// The base rights granted to this file descriptor
    pub base_rights: wasip1::Rights,
    /// The inheriting rights for new files created under this fd
    pub inheriting_rights: wasip1::Rights,
    /// The flags used to open this file
    pub fd_flags: wasip1::Fdflags,
}

impl<InodeId: InodeIdCommon> OpenFdInfo for DetailedOpenFd<InodeId> {
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

impl<InodeId: InodeIdCommon> OpenFdInfoWithInode for DetailedOpenFd<InodeId> {
    type InodeId = InodeId;

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

/// A trait defining constraints for an Inode ID type, requiring it to be debuggable and convertible to/from `InodeId`.
#[allow(dead_code)]
pub trait InodeConstraint: core::fmt::Debug + Into<InodeId> + From<InodeId> {}
