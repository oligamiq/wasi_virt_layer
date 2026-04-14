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
pub struct InodeMetadata {
    /// File type (e.g., File, Directory, Symlink)
    pub filetype: wasip1::Filetype,
    /// Last access timestamp
    pub atim: wasip1::Timestamp,
    /// Last modification timestamp
    pub mtim: wasip1::Timestamp,
    /// Creation timestamp (or last status change)
    pub ctim: wasip1::Timestamp,
    /// Hard link count
    pub nlink: wasip1::Linkcount,
    /// User-defined WASI permissions (base rights that can be acquired)
    pub rights: wasip1::Rights,
}

impl InodeMetadata {
    /// Create new default metadata for a given file type
    pub fn new(filetype: wasip1::Filetype, rights: wasip1::Rights) -> Self {
        Self {
            filetype,
            atim: 0,
            mtim: 0,
            ctim: 0,
            nlink: 1,
            rights,
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
pub struct Inode {
    /// Metadata like timestamps and type
    pub meta: InodeMetadata,
    /// Actual contents of the inode
    pub data: InodeData,
}

/// Represents an active, open file descriptor referencing an Inode
#[derive(Debug, Clone)]
pub struct OpenFd {
    /// The target inode being accessed
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
