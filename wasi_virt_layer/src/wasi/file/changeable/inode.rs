use crate::wasi::file::WasiAddInfo;
use crate::wasi::file::constant::vfs::OpenFdInfoWithInode;
use crate::{__private::wasip1, wasi::file::constant::vfs::OpenFdInfo};
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

impl<AddInfo: WasiAddInfo> InodeMetadata<AddInfo> {
    /// Create new default metadata for a given file type
    pub fn new(filetype: wasip1::Filetype, rights: wasip1::Rights) -> Self {
        Self {
            filetype,
            nlink: 1,
            rights,
            add_info: AddInfo::DEFAULT,
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
pub struct DetailedOpenFd {
    /// The current byte offset within the file (for reads/writes)
    pub cursor: usize,
    /// The base rights granted to this file descriptor
    pub base_rights: wasip1::Rights,
    /// The inheriting rights for new files created under this fd
    pub inheriting_rights: wasip1::Rights,
    /// The flags used to open this file
    pub fd_flags: wasip1::Fdflags,
}

impl OpenFdInfo for DetailedOpenFd {
    const DEFAULT: Self = Self {
        cursor: 0,
        base_rights: 0,
        inheriting_rights: 0,
        fd_flags: 0,
    };

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

impl OpenFdInfoWithInode for DetailedOpenFd {
    type InodeId = InodeId;

    fn inode_id(&self) -> Self::InodeId {
        // This is a placeholder. The actual implementation would need to store the inode ID.
        0
    }

    fn set_inode_id(&mut self, _inode_id: Self::InodeId) {
        // This is a placeholder. The actual implementation would need to store the inode ID.
    }
}
