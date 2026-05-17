#![cfg(feature = "dynamic-fs")]

use crate::__private::wasip1;
use crate::memory::{WasmAccessDynCompatible, WasmAccessDynCompatibleRaw, WasmAccessName};
use crate::wasi::file::{BoxedInode, DerefToStrCustom, InodeIdCommon, Wasip1LFSBaseWrapper};
use crate::wasi::file::{DynamicLFS, Wasip1DynCompatibleLFS, Wasip1LFSBase};
use crate::{
    memory::{WasmAccess, WasmPathAccess},
    wasi::file::{
        DefaultAddInfo, FilestatWithoutDevice, WasiAddInfo,
        dynamic::inode::{DirMap, Inode, InodeData, InodeId, InodeMetadata},
        stdio::StdIO,
    },
};
use alloc::{string::String, vec::Vec};
use smallstr::SmallString;

#[cfg(feature = "threads")]
use dashmap::DashMap;

#[cfg(not(feature = "threads"))]
use alloc::collections::BTreeMap;

#[cfg(not(feature = "threads"))]
use core::cell::UnsafeCell;

/// A local file system that allows runtime modifications
/// This structure implements many file operations.
/// However, it provides direct access, and there are no functions such as `open_file` to open files.
/// Therefore, operations on open files, such as moving the file cursor, are not possible.
/// Conversely, by using inodes, you can directly access files at deep levels.
pub struct StandardDynamicLFS<
    StdIo: StdIO + 'static,
    AddInfo: WasiAddInfo + 'static = DefaultAddInfo,
> {
    #[cfg(feature = "threads")]
    pub inodes: DashMap<InodeId, Inode<AddInfo>>,
    #[cfg(not(feature = "threads"))]
    pub inodes: UnsafeCell<BTreeMap<InodeId, Inode<AddInfo>>>,
    #[cfg(feature = "threads")]
    pub preopens: DashMap<InodeId, String>,
    #[cfg(not(feature = "threads"))]
    pub preopens: UnsafeCell<BTreeMap<InodeId, String>>,
    #[cfg(feature = "threads")]
    pub next_inode: std::sync::atomic::AtomicU32,
    #[cfg(not(feature = "threads"))]
    pub next_inode: UnsafeCell<InodeId>,
    __marker: core::marker::PhantomData<StdIo>,
}

impl<StdIo: StdIO + 'static, AddInfo: WasiAddInfo + 'static> core::fmt::Debug
    for StandardDynamicLFS<StdIo, AddInfo>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut debug_struct = f.debug_struct("StandardDynamicLFS");
        #[cfg(feature = "threads")]
        {
            debug_struct.field("inodes", &self.inodes);
            debug_struct.field("preopens", &self.preopens);
            debug_struct.field("next_inode", &self.next_inode);
        }
        #[cfg(not(feature = "threads"))]
        {
            debug_struct.field("inodes", unsafe { &*self.inodes.get() });
            debug_struct.field("preopens", unsafe { &*self.preopens.get() });
            debug_struct.field("next_inode", unsafe { &*self.next_inode.get() });
        }
        debug_struct.finish()
    }
}

impl<StdIo: StdIO + 'static, AddInfo: WasiAddInfo + Default + 'static>
    StandardDynamicLFS<StdIo, AddInfo>
{
    pub fn add_preopen(&self, name: impl Into<String>) -> InodeId {
        let name_str = name.into();

        let new_node = Inode {
            meta: InodeMetadata::new(
                wasip1::FILETYPE_DIRECTORY,
                wasip1::RIGHTS_FD_READ
                    | wasip1::RIGHTS_FD_READDIR
                    | wasip1::RIGHTS_FD_WRITE
                    | wasip1::RIGHTS_PATH_CREATE_DIRECTORY
                    | wasip1::RIGHTS_PATH_CREATE_FILE
                    | wasip1::RIGHTS_PATH_FILESTAT_GET
                    | wasip1::RIGHTS_PATH_FILESTAT_SET_SIZE
                    | wasip1::RIGHTS_PATH_FILESTAT_SET_TIMES
                    | wasip1::RIGHTS_PATH_LINK_SOURCE
                    | wasip1::RIGHTS_PATH_LINK_TARGET
                    | wasip1::RIGHTS_PATH_OPEN
                    | wasip1::RIGHTS_PATH_READLINK
                    | wasip1::RIGHTS_PATH_REMOVE_DIRECTORY
                    | wasip1::RIGHTS_PATH_RENAME_SOURCE
                    | wasip1::RIGHTS_PATH_RENAME_TARGET
                    | wasip1::RIGHTS_PATH_SYMLINK
                    | wasip1::RIGHTS_PATH_UNLINK_FILE,
                AddInfo::default(),
            ),
            data: InodeData::Dir(DirMap::new()),
        };

        let inode = self.allocate_inode(new_node);

        self.modify_inode(&inode, |node| {
            if let InodeData::Dir(map) = &mut node.data {
                map.insert(SmallString::from_str("."), inode);
                map.insert(SmallString::from_str(".."), inode);
            }
        });

        #[cfg(feature = "threads")]
        {
            self.preopens.insert(inode, name_str);
        }
        #[cfg(not(feature = "threads"))]
        {
            unsafe { &mut *self.preopens.get() }.insert(inode, name_str);
        }

        inode
    }

    pub fn add_dir(&self, parent: InodeId, name: &str) -> Result<InodeId, wasip1::Errno> {
        let mut map = DirMap::new();
        map.insert(SmallString::from_str("."), 0); // updated below
        map.insert(SmallString::from_str(".."), parent);
        let new_inode = Inode {
            meta: InodeMetadata::new(
                wasip1::FILETYPE_DIRECTORY,
                wasip1::RIGHTS_FD_READ
                    | wasip1::RIGHTS_FD_READDIR
                    | wasip1::RIGHTS_FD_WRITE
                    | wasip1::RIGHTS_PATH_CREATE_DIRECTORY
                    | wasip1::RIGHTS_PATH_CREATE_FILE
                    | wasip1::RIGHTS_PATH_FILESTAT_GET
                    | wasip1::RIGHTS_PATH_FILESTAT_SET_SIZE
                    | wasip1::RIGHTS_PATH_FILESTAT_SET_TIMES
                    | wasip1::RIGHTS_PATH_LINK_SOURCE
                    | wasip1::RIGHTS_PATH_LINK_TARGET
                    | wasip1::RIGHTS_PATH_OPEN
                    | wasip1::RIGHTS_PATH_READLINK
                    | wasip1::RIGHTS_PATH_REMOVE_DIRECTORY
                    | wasip1::RIGHTS_PATH_RENAME_SOURCE
                    | wasip1::RIGHTS_PATH_RENAME_TARGET
                    | wasip1::RIGHTS_PATH_SYMLINK
                    | wasip1::RIGHTS_PATH_UNLINK_FILE,
                AddInfo::default(),
            ),
            data: InodeData::Dir(map),
        };
        let new_id = self.allocate_inode(new_inode);

        self.modify_inode(&new_id, |node| {
            if let InodeData::Dir(map) = &mut node.data {
                map.insert(SmallString::from_str("."), new_id);
            }
        });

        let success = self
            .modify_inode(&parent, |node| {
                if let InodeData::Dir(parent_map) = &mut node.data {
                    parent_map.insert(SmallString::from_str(name), new_id);
                    true
                } else {
                    false
                }
            })
            .unwrap_or(false);

        if !success {
            return Err(wasip1::ERRNO_NOTDIR);
        }
        Ok(new_id)
    }

    pub fn add_file(
        &self,
        parent: InodeId,
        name: &str,
        content: Vec<u8>,
    ) -> Result<InodeId, wasip1::Errno> {
        let new_inode = Inode {
            meta: InodeMetadata::new(
                wasip1::FILETYPE_REGULAR_FILE,
                wasip1::RIGHTS_FD_READ | wasip1::RIGHTS_FD_WRITE | wasip1::RIGHTS_FD_FILESTAT_GET,
                AddInfo::default(),
            ),
            data: InodeData::File(content),
        };
        let new_id = self.allocate_inode(new_inode);

        let success = self
            .modify_inode(&parent, |node| {
                if let InodeData::Dir(parent_map) = &mut node.data {
                    parent_map.insert(SmallString::from_str(name), new_id);
                    true
                } else {
                    false
                }
            })
            .unwrap_or(false);

        if !success {
            return Err(wasip1::ERRNO_NOTDIR);
        }
        Ok(new_id)
    }
}

impl<StdIo: StdIO + 'static, AddInfo: WasiAddInfo + 'static> StandardDynamicLFS<StdIo, AddInfo> {
    /// Creates a new StandardDynamicLFS without any pre-existing inodes.
    /// Root directories and preopens should be added via `add_preopen`.
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "threads")]
            inodes: DashMap::new(),
            #[cfg(not(feature = "threads"))]
            inodes: UnsafeCell::new(BTreeMap::new()),
            #[cfg(feature = "threads")]
            preopens: DashMap::new(),
            #[cfg(not(feature = "threads"))]
            preopens: UnsafeCell::new(BTreeMap::new()),
            #[cfg(feature = "threads")]
            next_inode: std::sync::atomic::AtomicU32::new(0),
            #[cfg(not(feature = "threads"))]
            next_inode: UnsafeCell::new(0),
            __marker: core::marker::PhantomData,
        }
    }

    fn read_inode<F, R>(&self, id: &InodeId, f: F) -> Option<R>
    where
        F: FnOnce(&Inode<AddInfo>) -> R,
    {
        #[cfg(feature = "threads")]
        {
            self.inodes.get(id).map(|i| f(&i))
        }
        #[cfg(not(feature = "threads"))]
        {
            unsafe { &*self.inodes.get() }.get(id).map(f)
        }
    }

    fn modify_inode<F, R>(&self, id: &InodeId, f: F) -> Option<R>
    where
        F: FnOnce(&mut Inode<AddInfo>) -> R,
    {
        #[cfg(feature = "threads")]
        {
            self.inodes.get_mut(id).map(|mut i| f(&mut i))
        }
        #[cfg(not(feature = "threads"))]
        {
            unsafe { &mut *self.inodes.get() }.get_mut(id).map(|i| f(i))
        }
    }

    fn insert_inode(&self, id: InodeId, inode: Inode<AddInfo>) {
        #[cfg(feature = "threads")]
        {
            self.inodes.insert(id, inode);
        }
        #[cfg(not(feature = "threads"))]
        {
            unsafe { &mut *self.inodes.get() }.insert(id, inode);
        }
    }

    fn remove_inode(&self, id: &InodeId) -> Option<Inode<AddInfo>> {
        #[cfg(feature = "threads")]
        {
            self.inodes.remove(id).map(|(_, i)| i)
        }
        #[cfg(not(feature = "threads"))]
        {
            unsafe { &mut *self.inodes.get() }.remove(id)
        }
    }

    pub fn read_file(&self, inode: InodeId) -> Result<alloc::vec::Vec<u8>, wasip1::Errno> {
        self.read_inode(&inode, |node| {
            if let InodeData::File(ref vec) = node.data {
                Ok(vec.clone())
            } else {
                Err(wasip1::ERRNO_ISDIR)
            }
        })
        .unwrap_or(Err(wasip1::ERRNO_BADF))
    }

    pub fn write_file(
        &self,
        inode: InodeId,
        content: alloc::vec::Vec<u8>,
    ) -> Result<(), wasip1::Errno> {
        self.modify_inode(&inode, |node| {
            if let InodeData::File(ref mut vec) = node.data {
                *vec = content;
                Ok(())
            } else {
                Err(wasip1::ERRNO_ISDIR)
            }
        })
        .unwrap_or(Err(wasip1::ERRNO_BADF))
    }

    pub fn metadata(&self, inode: InodeId) -> Result<InodeMetadata<AddInfo>, wasip1::Errno>
    where
        AddInfo: Clone,
    {
        self.read_inode(&inode, |node| Ok(node.meta.clone()))
            .unwrap_or(Err(wasip1::ERRNO_BADF))
    }

    pub fn set_metadata(
        &self,
        inode: InodeId,
        metadata: InodeMetadata<AddInfo>,
    ) -> Result<(), wasip1::Errno> {
        self.modify_inode(&inode, |node| {
            node.meta = metadata;
            Ok(())
        })
        .unwrap_or(Err(wasip1::ERRNO_BADF))
    }

    pub fn read_dir(
        &self,
        inode: InodeId,
    ) -> Result<alloc::vec::Vec<(alloc::string::String, InodeId)>, wasip1::Errno> {
        self.read_inode(&inode, |node| {
            if let InodeData::Dir(ref map) = node.data {
                Ok(map
                    .iter()
                    .map(|(name, id)| (alloc::string::String::from(name.as_str()), *id))
                    .collect())
            } else {
                Err(wasip1::ERRNO_NOTDIR)
            }
        })
        .unwrap_or(Err(wasip1::ERRNO_BADF))
    }

    pub fn remove_file(&self, parent: InodeId, name: &str) -> Result<(), wasip1::Errno> {
        let name_small = SmallString::<[u8; 32]>::from_str(name);
        let target_id = self
            .read_inode(&parent, |node| {
                if let InodeData::Dir(ref map) = node.data {
                    map.get(&name_small).copied().ok_or(wasip1::ERRNO_NOENT)
                } else {
                    Err(wasip1::ERRNO_NOTDIR)
                }
            })
            .unwrap_or(Err(wasip1::ERRNO_BADF))?;

        let is_file = self
            .read_inode(&target_id, |node| {
                node.meta.filetype != wasip1::FILETYPE_DIRECTORY
            })
            .unwrap_or(false);

        if !is_file {
            return Err(wasip1::ERRNO_ISDIR);
        }

        self.modify_inode(&parent, |node| {
            if let InodeData::Dir(ref mut map) = node.data {
                map.remove(&name_small);
            }
        });

        self.remove_inode(&target_id);
        Ok(())
    }

    pub fn remove_dir(&self, parent: InodeId, name: &str) -> Result<(), wasip1::Errno> {
        let name_small = SmallString::<[u8; 32]>::from_str(name);
        let target_id = self
            .read_inode(&parent, |node| {
                if let InodeData::Dir(ref map) = node.data {
                    map.get(&name_small).copied().ok_or(wasip1::ERRNO_NOENT)
                } else {
                    Err(wasip1::ERRNO_NOTDIR)
                }
            })
            .unwrap_or(Err(wasip1::ERRNO_BADF))?;

        let (is_dir, is_empty) = self
            .read_inode(&target_id, |node| {
                if let InodeData::Dir(ref map) = node.data {
                    let mut empty = true;
                    for key in map.keys() {
                        if key.as_str() != "." && key.as_str() != ".." {
                            empty = false;
                            break;
                        }
                    }
                    (true, empty)
                } else {
                    (false, false)
                }
            })
            .unwrap_or((false, false));

        if !is_dir {
            return Err(wasip1::ERRNO_NOTDIR);
        }
        if !is_empty {
            return Err(wasip1::ERRNO_NOTEMPTY);
        }

        self.modify_inode(&parent, |node| {
            if let InodeData::Dir(ref mut map) = node.data {
                map.remove(&name_small);
            }
        });

        self.remove_inode(&target_id);
        Ok(())
    }

    pub fn add_symlink(
        &self,
        parent: InodeId,
        name: &str,
        target: &str,
    ) -> Result<InodeId, wasip1::Errno> {
        let new_inode = Inode {
            meta: InodeMetadata::new(
                wasip1::FILETYPE_SYMBOLIC_LINK,
                wasip1::RIGHTS_FD_READ | wasip1::RIGHTS_PATH_READLINK,
                AddInfo::DEFAULT,
            ),
            data: InodeData::Symlink(alloc::string::String::from(target)),
        };
        let new_id = self.allocate_inode(new_inode);

        let success = self
            .modify_inode(&parent, |node| {
                if let InodeData::Dir(parent_map) = &mut node.data {
                    parent_map.insert(SmallString::from_str(name), new_id);
                    true
                } else {
                    false
                }
            })
            .unwrap_or(false);

        if !success {
            self.remove_inode(&new_id);
            return Err(wasip1::ERRNO_NOTDIR);
        }
        Ok(new_id)
    }

    pub fn rename(
        &self,
        old_parent: InodeId,
        old_name: &str,
        new_parent: InodeId,
        new_name: &str,
    ) -> Result<(), wasip1::Errno> {
        let old_name_small = SmallString::<[u8; 32]>::from_str(old_name);
        let new_name_small = SmallString::<[u8; 32]>::from_str(new_name);

        let target_id = self
            .read_inode(&old_parent, |node| {
                if let InodeData::Dir(ref map) = node.data {
                    map.get(&old_name_small).copied().ok_or(wasip1::ERRNO_NOENT)
                } else {
                    Err(wasip1::ERRNO_NOTDIR)
                }
            })
            .unwrap_or(Err(wasip1::ERRNO_BADF))?;

        // 1. Try to insert into new parent
        let success_insert = self
            .modify_inode(&new_parent, |node| {
                if let InodeData::Dir(ref mut map) = node.data {
                    map.insert(new_name_small.clone(), target_id);
                    true
                } else {
                    false
                }
            })
            .unwrap_or(false);

        if !success_insert {
            return Err(wasip1::ERRNO_NOTDIR);
        }

        // 2. Remove from old parent
        let success_remove = self
            .modify_inode(&old_parent, |node| {
                if let InodeData::Dir(ref mut map) = node.data {
                    map.remove(&old_name_small).is_some()
                } else {
                    false
                }
            })
            .unwrap_or(false);

        if !success_remove {
            // Revert insert if remove failed
            self.modify_inode(&new_parent, |node| {
                if let InodeData::Dir(ref mut map) = node.data {
                    map.remove(&new_name_small);
                }
            });
            return Err(wasip1::ERRNO_BADF);
        }

        // 3. Update ".." if it's a directory
        self.modify_inode(&target_id, |node| {
            if let InodeData::Dir(ref mut map) = node.data {
                map.insert(SmallString::from_str(".."), new_parent);
            }
        });

        Ok(())
    }

    pub fn get_inode_by_path_str(&self, start_inode: InodeId, path: &str) -> Option<InodeId> {
        let mut current_inode = start_inode;
        for component in path.split('/') {
            if component.is_empty() || component == "." {
                continue;
            } else if component == ".." {
                let parent = self
                    .read_inode(&current_inode, |node| match &node.data {
                        InodeData::Dir(map) => map.get(&SmallString::from_str("..")).copied(),
                        _ => None,
                    })
                    .flatten();

                if let Some(parent_id) = parent {
                    current_inode = parent_id;
                } else {
                    return None;
                }
            } else {
                let name = SmallString::<[u8; 32]>::from_str(component);
                let child = self
                    .read_inode(&current_inode, |node| match &node.data {
                        InodeData::Dir(map) => map.get(&name).copied(),
                        _ => None,
                    })
                    .flatten();

                if let Some(child_id) = child {
                    current_inode = child_id;
                } else {
                    return None;
                }
            }
        }
        Some(current_inode)
    }

    fn read_preopen<F, R>(&self, inode: &InodeId, f: F) -> Option<R>
    where
        F: FnOnce(&String) -> R,
    {
        #[cfg(feature = "threads")]
        {
            self.preopens.get(inode).map(|s| f(&s))
        }
        #[cfg(not(feature = "threads"))]
        {
            unsafe { &*self.preopens.get() }.get(inode).map(f)
        }
    }

    pub fn allocate_inode(&self, inode: Inode<AddInfo>) -> InodeId {
        let id;
        #[cfg(feature = "threads")]
        {
            id = self
                .next_inode
                .fetch_add(1, core::sync::atomic::Ordering::SeqCst) as InodeId;
        }
        #[cfg(not(feature = "threads"))]
        {
            id = unsafe { *self.next_inode.get() };
            unsafe { *self.next_inode.get() += 1 };
        }
        self.insert_inode(id, inode);
        id
    }

    pub fn update_next_inode(&self, min_val: InodeId) {
        #[cfg(feature = "threads")]
        {
            let mut current = self.next_inode.load(core::sync::atomic::Ordering::SeqCst);
            while current < min_val as u32 {
                match self.next_inode.compare_exchange_weak(
                    current,
                    min_val as u32,
                    core::sync::atomic::Ordering::SeqCst,
                    core::sync::atomic::Ordering::Relaxed,
                ) {
                    Ok(_) => break,
                    Err(x) => current = x,
                }
            }
        }
        #[cfg(not(feature = "threads"))]
        {
            let current = unsafe { &mut *self.next_inode.get() };
            if *current < min_val {
                *current = min_val;
            }
        }
    }

    fn get_inode_for_path_inner(
        &self,
        dir_ino: &InodeId,
        path: impl crate::memory::WasmPathAccessCommon,
    ) -> Option<InodeId> {
        use crate::memory::WasmPathComponentCommon;
        let mut current_inode = *dir_ino;

        for component in path.components_common() {
            if component.as_cur_dir() {
                continue;
            } else if component.as_parent_dir() {
                let parent = self
                    .read_inode(&current_inode, |node| match &node.data {
                        InodeData::Dir(map) => map.get(&SmallString::from_str("..")).copied(),
                        _ => None,
                    })
                    .flatten();

                if let Some(parent_id) = parent {
                    current_inode = parent_id;
                } else {
                    return None;
                }
            } else if component.as_root_dir() {
                current_inode = 0;
            } else if let Some(name_iter) = component.as_normal() {
                let mut s = SmallString::<[u8; 32]>::new();
                for b in name_iter {
                    s.push(b as char);
                }
                let child = self
                    .read_inode(&current_inode, |node| match &node.data {
                        InodeData::Dir(map) => map.get(&s).copied(),
                        _ => None,
                    })
                    .flatten();

                if let Some(child_id) = child {
                    current_inode = child_id;
                } else {
                    return None;
                }
            }
        }
        Some(current_inode)
    }

    fn resolve_parent_for_path_inner(
        &self,
        dir_ino: &InodeId,
        path: impl crate::memory::WasmPathAccessCommon,
    ) -> Option<(InodeId, smallstr::SmallString<[u8; 32]>)> {
        use crate::memory::WasmPathComponentCommon;
        let mut current_inode = *dir_ino;
        let mut components = path.components_common().peekable();

        while let Some(component) = components.next() {
            if components.peek().is_none() {
                if let Some(name_iter) = component.as_normal() {
                    let mut s = smallstr::SmallString::<[u8; 32]>::new();
                    for b in name_iter {
                        s.push(b as char);
                    }
                    return Some((current_inode, s));
                } else {
                    return None;
                }
            }

            if component.as_cur_dir() {
                continue;
            } else if component.as_parent_dir() {
                let parent = self
                    .read_inode(&current_inode, |node| match &node.data {
                        InodeData::Dir(map) => map.get(&smallstr::SmallString::from_str("..")).copied(),
                        _ => None,
                    })
                    .flatten();

                if let Some(parent_id) = parent {
                    current_inode = parent_id;
                } else {
                    return None;
                }
            } else if component.as_root_dir() {
                current_inode = 0;
            } else if let Some(name_iter) = component.as_normal() {
                let mut s = smallstr::SmallString::<[u8; 32]>::new();
                for b in name_iter {
                    s.push(b as char);
                }
                let child = self
                    .read_inode(&current_inode, |node| match &node.data {
                        InodeData::Dir(map) => map.get(&s).copied(),
                        _ => None,
                    })
                    .flatten();

                if let Some(child_id) = child {
                    current_inode = child_id;
                } else {
                    return None;
                }
            }
        }
        None
    }

    pub fn get_inode_for_path<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        dir_ino: &InodeId,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Option<InodeId> {
        let path = WasmPathAccess::<Wasm>::new(path_ptr, path_len);
        self.get_inode_for_path_inner(dir_ino, path)
    }

    pub fn get_inode_for_path_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        dir_ino: &InodeId,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Option<InodeId> {
        let path = crate::memory::WasmPathAccessDynCompatible::new(access, path_ptr, path_len);
        self.get_inode_for_path_inner(dir_ino, path)
    }

    pub fn resolve_inode(&self, inode: InodeId, _depth: usize) -> Option<InodeId> {
        Some(inode)
    }
}

impl<StdIo: StdIO + 'static, AddInfo: WasiAddInfo + Default + 'static> Wasip1LFSBase
    for StandardDynamicLFS<StdIo, AddInfo>
{
    type Inode = InodeId;

    fn fd_write_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
        data: *const u8,
        data_len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        self.modify_inode(&inode, |inode_entry| {
            if let InodeData::File(ref mut vec) = inode_entry.data {
                let mut buf = alloc::vec![0u8; data_len];
                Wasm::memcpy_to(&mut buf, data);
                vec.extend_from_slice(&buf);
                let mtim = inode_entry.meta.add_info.modification_time();
                inode_entry.meta.add_info.set_modification_time(mtim + 1); // Simplistic time update
                Ok(data_len)
            } else {
                Err(wasip1::ERRNO_ISDIR)
            }
        })
        .unwrap_or(Err(wasip1::ERRNO_BADF))
    }

    fn fd_write_stdout_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        data: *const u8,
        data_len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        #[cfg(not(feature = "multi_memory"))]
        {
            StdIo::write_direct::<Wasm>(data, data_len)
        }
        #[cfg(feature = "multi_memory")]
        {
            let mut buf = alloc::vec![0u8; data_len];
            Wasm::memcpy_to(&mut buf, data);
            StdIo::write(&buf)
        }
    }

    fn fd_write_stderr_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        data: *const u8,
        data_len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        #[cfg(not(feature = "multi_memory"))]
        {
            StdIo::ewrite_direct::<Wasm>(data, data_len)
        }
        #[cfg(feature = "multi_memory")]
        {
            let mut buf = alloc::vec![0u8; data_len];
            Wasm::memcpy_to(&mut buf, data);
            StdIo::ewrite(&buf)
        }
    }

    fn is_dir(&self, inode: &Self::Inode) -> bool {
        self.read_inode(&inode, |i| matches!(i.data, InodeData::Dir(_)))
            .unwrap_or(false)
    }

    fn fd_readdir_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
        buf: *mut u8,
        buf_len: usize,
        cookie: wasip1::Dircookie,
    ) -> Result<(wasip1::Size, wasip1::Dircookie), wasip1::Errno> {
        let dir_map = self
            .read_inode(&inode, |i| match &i.data {
                InodeData::Dir(map) => Some(map.clone()),
                _ => None,
            })
            .flatten()
            .ok_or(wasip1::ERRNO_NOTDIR)?;

        let mut current_cookie = cookie as usize;
        let mut total_written = 0;
        let mut out_ptr = buf;
        let mut rem_len = buf_len;

        for (i, (name, &child_inode)) in dir_map.iter().enumerate() {
            if i < current_cookie {
                continue;
            }
            let filetype = self
                .read_inode(&child_inode, |c| c.meta.filetype)
                .ok_or(wasip1::ERRNO_NOENT)?;
            let name_bytes = name.as_bytes();
            let entry = wasip1::Dirent {
                d_next: (i + 1) as u64,
                d_ino: child_inode as u64,
                d_namlen: name_bytes.len() as u32,
                d_type: filetype,
            };

            let entry_size = core::mem::size_of::<wasip1::Dirent>();

            if rem_len < entry_size {
                break;
            }

            // Write dirent
            let entry_slice =
                unsafe { core::slice::from_raw_parts(&entry as *const _ as *const u8, entry_size) };
            Wasm::memcpy(out_ptr, entry_slice);

            out_ptr = unsafe { out_ptr.add(entry_size) };
            rem_len -= entry_size;
            total_written += entry_size;

            // Write name
            let write_name_len = core::cmp::min(name_bytes.len(), rem_len);
            if write_name_len > 0 {
                Wasm::memcpy(out_ptr, &name_bytes[..write_name_len]);
                out_ptr = unsafe { out_ptr.add(write_name_len) };
                rem_len -= write_name_len;
                total_written += write_name_len;
            }

            current_cookie = i + 1;
            if rem_len == 0 {
                break;
            }
        }

        Ok((total_written, current_cookie as u64))
    }

    fn path_filestat_get_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno> {
        let mut target_inode = self
            .get_inode_for_path::<Wasm>(inode, path_ptr, path_len)
            .ok_or(wasip1::ERRNO_NOENT)?;

        if flags & wasip1::LOOKUPFLAGS_SYMLINK_FOLLOW == wasip1::LOOKUPFLAGS_SYMLINK_FOLLOW {
            target_inode = self
                .resolve_inode(target_inode, 0)
                .ok_or(wasip1::ERRNO_LOOP)?;
        }

        self.fd_filestat_get_raw::<Wasm>(&target_inode)
    }

    fn fd_prestat_get_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
    ) -> Result<wasip1::Prestat, wasip1::Errno> {
        self.read_preopen(&inode, |name| {
            Ok(wasip1::Prestat {
                tag: 0,
                u: wasip1::PrestatU {
                    dir: wasip1::PrestatDir {
                        pr_name_len: name.len() as usize,
                    },
                },
            })
        })
        .unwrap_or(Err(wasip1::ERRNO_BADF))
    }

    fn fd_prestat_dir_name_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        self.read_preopen(&inode, |name| {
            let copy_len = core::cmp::min(name.len(), dir_path_len);
            if copy_len > 0 {
                Wasm::memcpy(dir_path_ptr, &name.as_bytes()[..copy_len]);
            }
            Ok(())
        })
        .unwrap_or(Err(wasip1::ERRNO_BADF))
    }

    fn fd_filestat_get_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno> {
        self.read_inode(&inode, |node| {
            Ok(FilestatWithoutDevice {
                ino: *inode as u64,
                filetype: node.meta.filetype,
                nlink: node.meta.nlink,
                size: node.meta.size(&node.data),
                atim: node.meta.add_info.access_time(),
                mtim: node.meta.add_info.modification_time(),
                ctim: node.meta.add_info.creation_time(),
            })
        })
        .unwrap_or(Err(wasip1::ERRNO_BADF))
    }

    fn fd_pread_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
        buf: *mut u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        self.modify_inode(&inode, |node| {
            if let InodeData::File(ref file_data) = node.data {
                if offset >= file_data.len() {
                    return Ok(0);
                }
                let available = file_data.len() - offset;
                let read_len = core::cmp::min(buf_len, available);
                Wasm::memcpy(buf, &file_data[offset..offset + read_len]);
                let atim = node.meta.add_info.access_time();
                node.meta.add_info.set_access_time(atim + 1);
                Ok(read_len)
            } else {
                Err(wasip1::ERRNO_ISDIR)
            }
        })
        .unwrap_or(Err(wasip1::ERRNO_BADF))
    }

    fn fd_pwrite_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
        buf: *const u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        self.modify_inode(&inode, |node| {
            if let InodeData::File(ref mut vec) = node.data {
                let end = offset + buf_len;
                if end > vec.len() {
                    vec.resize(end, 0);
                }
                let mut tmp = alloc::vec![0u8; buf_len];
                Wasm::memcpy_to(&mut tmp, buf);
                vec[offset..end].copy_from_slice(&tmp);
                let mtim = node.meta.add_info.modification_time();
                node.meta.add_info.set_modification_time(mtim + 1);
                Ok(buf_len)
            } else {
                Err(wasip1::ERRNO_ISDIR)
            }
        })
        .unwrap_or(Err(wasip1::ERRNO_BADF))
    }

    fn fd_advise_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        _inode: &Self::Inode,
        _offset: u64,
        _len: u64,
        _advice: wasip1::Advice,
    ) -> Result<(), wasip1::Errno> {
        // In-memory filesystem: advice is a no-op
        Ok(())
    }

    fn fd_allocate_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
        offset: u64,
        len: u64,
    ) -> Result<(), wasip1::Errno> {
        let required = (offset + len) as usize;
        self.modify_inode(&inode, |node| {
            if let InodeData::File(ref mut vec) = node.data {
                if required > vec.len() {
                    vec.resize(required, 0);
                }
                Ok(())
            } else {
                Err(wasip1::ERRNO_ISDIR)
            }
        })
        .unwrap_or(Err(wasip1::ERRNO_BADF))
    }

    fn fd_datasync_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        _inode: &Self::Inode,
    ) -> Result<(), wasip1::Errno> {
        // In-memory filesystem: datasync is a no-op
        Ok(())
    }

    fn fd_sync_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        _inode: &Self::Inode,
    ) -> Result<(), wasip1::Errno> {
        // In-memory filesystem: sync is a no-op
        Ok(())
    }

    fn fd_filestat_set_size_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
        size: u64,
    ) -> Result<(), wasip1::Errno> {
        self.modify_inode(&inode, |node| {
            if let InodeData::File(ref mut vec) = node.data {
                vec.resize(size as usize, 0);
                let mtim = node.meta.add_info.modification_time();
                node.meta.add_info.set_modification_time(mtim + 1);
                Ok(())
            } else {
                Err(wasip1::ERRNO_ISDIR)
            }
        })
        .unwrap_or(Err(wasip1::ERRNO_BADF))
    }

    fn fd_filestat_set_times_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
        atim: wasip1::Timestamp,
        mtim: wasip1::Timestamp,
        fst_flags: wasip1::Fstflags,
    ) -> Result<(), wasip1::Errno> {
        self.modify_inode(&inode, |node| {
            if fst_flags & wasip1::FSTFLAGS_ATIM != 0 {
                node.meta.add_info.set_access_time(atim);
            }
            if fst_flags & wasip1::FSTFLAGS_MTIM != 0 {
                node.meta.add_info.set_modification_time(mtim);
            }
            // *_NOW flags: we use a simplistic +1 increment since there's no real clock
            if fst_flags & wasip1::FSTFLAGS_ATIM_NOW != 0 {
                let t = node.meta.add_info.access_time();
                node.meta.add_info.set_access_time(t + 1);
            }
            if fst_flags & wasip1::FSTFLAGS_MTIM_NOW != 0 {
                let t = node.meta.add_info.modification_time();
                node.meta.add_info.set_modification_time(t + 1);
            }
            Ok(())
        })
        .unwrap_or(Err(wasip1::ERRNO_BADF))
    }

    fn path_filestat_set_times_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
        atim: wasip1::Timestamp,
        mtim: wasip1::Timestamp,
        fst_flags: wasip1::Fstflags,
    ) -> Result<(), wasip1::Errno> {
        let mut target_inode = self
            .get_inode_for_path::<Wasm>(inode, path_ptr, path_len)
            .ok_or(wasip1::ERRNO_NOENT)?;

        if flags & wasip1::LOOKUPFLAGS_SYMLINK_FOLLOW == wasip1::LOOKUPFLAGS_SYMLINK_FOLLOW {
            target_inode = self
                .resolve_inode(target_inode, 0)
                .ok_or(wasip1::ERRNO_LOOP)?;
        }

        self.fd_filestat_set_times_raw::<Wasm>(&target_inode, atim, mtim, fst_flags)
    }

    fn path_symlink_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
        old_path_ptr: *const u8,
        old_path_len: usize,
        new_path_ptr: *const u8,
        new_path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        // Read the old (target) path
        let old_path_str = {
            let path = WasmPathAccess::<Wasm>::new(old_path_ptr, old_path_len);
            let mut s = alloc::string::String::new();
            for component in path.components() {
                if !s.is_empty() {
                    s.push('/');
                }
                match component {
                    crate::memory::WasmPathComponent::Normal(name) => {
                        for i in 0..name.len() {
                            s.push(name.get(i) as char);
                        }
                    }
                    crate::memory::WasmPathComponent::CurDir => s.push('.'),
                    crate::memory::WasmPathComponent::ParentDir => s.push_str(".."),
                    crate::memory::WasmPathComponent::RootDir => s.push('/'),
                }
            }
            s
        };

        let new_path = WasmPathAccess::<Wasm>::new(new_path_ptr, new_path_len);
        if let Some((parent_ino, new_name)) = self.resolve_parent_for_path_inner(inode, new_path) {
            // Check if the new path already exists
            if self.read_inode(&parent_ino, |node| {
                if let InodeData::Dir(map) = &node.data {
                    map.contains_key(&new_name)
                } else {
                    false
                }
            }).unwrap_or(false) {
                return Err(wasip1::ERRNO_EXIST);
            }

            let symlink_inode = Inode {
                meta: InodeMetadata::new(wasip1::FILETYPE_SYMBOLIC_LINK, 0, AddInfo::default()),
                data: InodeData::Symlink(old_path_str),
            };
            let new_id = self.allocate_inode(symlink_inode);
            self.modify_inode(&parent_ino, |node| {
                if let InodeData::Dir(dir_map) = &mut node.data {
                    dir_map.insert(new_name, new_id);
                }
            });
            Ok(())
        } else {
            Err(wasip1::ERRNO_NOENT)
        }
    }

    fn fd_read_stdin_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        buf: *mut u8,
        buf_len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        #[cfg(not(feature = "multi_memory"))]
        {
            StdIo::read_direct::<Wasm>(buf, buf_len)
        }

        #[cfg(feature = "multi_memory")]
        {
            let mut internal_buf = alloc::vec![0u8; buf_len];
            let read = StdIo::read(&mut internal_buf).map_err(|_| wasip1::ERRNO_IO)?;
            Wasm::memcpy(buf, &internal_buf[..read]);
            Ok(read)
        }
    }

    fn path_open_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        dir_ino: &Self::Inode,
        _: wasip1::Fdflags,
        path_ptr: *const u8,
        path_len: usize,
        o_flags: wasip1::Oflags,
        fs_rights_base: wasip1::Rights,
        _: wasip1::Rights,
        _: wasip1::Fdflags,
    ) -> Result<Self::Inode, wasip1::Errno> {
        if let Some(inode) = self.get_inode_for_path::<Wasm>(dir_ino, path_ptr, path_len) {
            if o_flags & wasip1::OFLAGS_EXCL == wasip1::OFLAGS_EXCL {
                return Err(wasip1::ERRNO_EXIST);
            }
            if o_flags & wasip1::OFLAGS_DIRECTORY == wasip1::OFLAGS_DIRECTORY
                && !self.is_dir(&inode)
            {
                return Err(wasip1::ERRNO_NOTDIR);
            }
            if o_flags & wasip1::OFLAGS_TRUNC == wasip1::OFLAGS_TRUNC {
                self.modify_inode(&inode, |node| {
                    if let InodeData::File(ref mut vec) = node.data {
                        vec.clear();
                    }
                });
            }
            Ok(inode)
        } else {
            if o_flags & wasip1::OFLAGS_CREAT == wasip1::OFLAGS_CREAT {
                let path = WasmPathAccess::<Wasm>::new(path_ptr, path_len);
                
                if let Some((parent_ino, s)) = self.resolve_parent_for_path_inner(dir_ino, path) {
                    let is_dir = o_flags & wasip1::OFLAGS_DIRECTORY == wasip1::OFLAGS_DIRECTORY;
                    let filetype = if is_dir {
                        wasip1::FILETYPE_DIRECTORY
                    } else {
                        wasip1::FILETYPE_REGULAR_FILE
                    };

                    let data = if is_dir {
                        let mut map = DirMap::new();
                        map.insert(SmallString::from_str("."), 0); // Self (will update)
                        map.insert(SmallString::from_str(".."), parent_ino); // Parent
                        InodeData::Dir(map)
                    } else {
                        InodeData::File(Vec::new())
                    };

                    let new_inode = Inode {
                        meta: InodeMetadata::new(filetype, fs_rights_base, AddInfo::default()),
                        data,
                    };

                    let new_id = self.allocate_inode(new_inode);

                    // Update self reference if dir
                    if is_dir {
                        self.modify_inode(&new_id, |node| {
                            if let InodeData::Dir(map) = &mut node.data {
                                map.insert(SmallString::from_str("."), new_id);
                            }
                        });
                    }

                    // Link in parent
                    self.modify_inode(&parent_ino, |node| {
                        if let InodeData::Dir(dir_map) = &mut node.data {
                            dir_map.insert(s, new_id);
                        }
                    });

                    return Ok(new_id);
                }
                Err(wasip1::ERRNO_NOENT)
            } else {
                Err(wasip1::ERRNO_NOENT)
            }
        }
    }

    fn path_readlink_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        inode: &Self::Inode,
        path_ptr: *const u8,
        path_len: usize,
        buf: *mut u8,
        buf_len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        let target_inode = self
            .get_inode_for_path::<Wasm>(inode, path_ptr, path_len)
            .ok_or(wasip1::ERRNO_NOENT)?;

        self.read_inode(&target_inode, |node| {
            if let InodeData::Symlink(ref target) = node.data {
                let target_bytes = target.as_bytes();
                let n = core::cmp::min(target_bytes.len(), buf_len);
                Wasm::memcpy(buf, &target_bytes[..n]);
                Ok(n)
            } else {
                Err(wasip1::ERRNO_INVAL)
            }
        })
        .unwrap_or(Err(wasip1::ERRNO_BADF))
    }

    fn path_create_directory_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        dir_ino: &Self::Inode,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        if self
            .get_inode_for_path::<Wasm>(dir_ino, path_ptr, path_len)
            .is_some()
        {
            return Err(wasip1::ERRNO_EXIST);
        }

        let path = WasmPathAccess::<Wasm>::new(path_ptr, path_len);
        if let Some((parent_ino, s)) = self.resolve_parent_for_path_inner(dir_ino, path) {
            let mut map = DirMap::new();
            map.insert(SmallString::from_str("."), 0); // Will update
            map.insert(SmallString::from_str(".."), parent_ino);

            let new_inode = Inode {
                meta: InodeMetadata::new(wasip1::FILETYPE_DIRECTORY, 0, AddInfo::default()),
                data: InodeData::Dir(map),
            };

            let new_id = self.allocate_inode(new_inode);
            self.modify_inode(&new_id, |node| {
                if let InodeData::Dir(map) = &mut node.data {
                    map.insert(SmallString::from_str("."), new_id);
                }
            });

            self.modify_inode(&parent_ino, |node| {
                if let InodeData::Dir(dir_map) = &mut node.data {
                    dir_map.insert(s, new_id);
                }
            });

            return Ok(());
        }
        Err(wasip1::ERRNO_NOENT)
    }

    fn path_link_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        old_dir_ino: &Self::Inode,
        _: wasip1::Lookupflags,
        old_path_ptr: *const u8,
        old_path_len: usize,
        new_dir_ino: &Self::Inode,
        new_path_ptr: *const u8,
        new_path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        let inode = self
            .get_inode_for_path::<Wasm>(old_dir_ino, old_path_ptr, old_path_len)
            .ok_or(wasip1::ERRNO_NOENT)?;

        if self.is_dir(&inode) {
            return Err(wasip1::ERRNO_PERM);
        }

        let new_path = WasmPathAccess::<Wasm>::new(new_path_ptr, new_path_len);
        if let Some((parent_ino, new_s)) = self.resolve_parent_for_path_inner(new_dir_ino, new_path) {
            self.modify_inode(&parent_ino, |node| {
                if let InodeData::Dir(dir_map) = &mut node.data {
                    dir_map.insert(new_s, inode);
                }
            });

            return Ok(());
        }
        Err(wasip1::ERRNO_NOENT)
    }

    fn path_remove_directory_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        dir_ino: &Self::Inode,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        let inode = self
            .get_inode_for_path::<Wasm>(dir_ino, path_ptr, path_len)
            .ok_or(wasip1::ERRNO_NOENT)?;

        if !self.is_dir(&inode) {
            return Err(wasip1::ERRNO_NOTDIR);
        }

        // Check if directory is empty (only "." and "..")
        let is_empty = self
            .read_inode(&inode, |node| {
                if let InodeData::Dir(dir_map) = &node.data {
                    dir_map.len() <= 2
                } else {
                    false
                }
            })
            .unwrap_or(false);

        if !is_empty {
            return Err(wasip1::ERRNO_NOTEMPTY);
        }

        let path = WasmPathAccess::<Wasm>::new(path_ptr, path_len);
        if let Some((parent_ino, s)) = self.resolve_parent_for_path_inner(dir_ino, path) {
            self.modify_inode(&parent_ino, |node| {
                if let InodeData::Dir(dir_map) = &mut node.data {
                    dir_map.remove(&s);
                }
            });
            return Ok(());
        }
        Err(wasip1::ERRNO_NOENT)
    }

    fn path_rename_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        old_dir_ino: &Self::Inode,
        old_path_ptr: *const u8,
        old_path_len: usize,
        new_dir_ino: &Self::Inode,
        new_path_ptr: *const u8,
        new_path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        let inode = self
            .get_inode_for_path::<Wasm>(old_dir_ino, old_path_ptr, old_path_len)
            .ok_or(wasip1::ERRNO_NOENT)?;

        let old_path = WasmPathAccess::<Wasm>::new(old_path_ptr, old_path_len);
        let new_path = WasmPathAccess::<Wasm>::new(new_path_ptr, new_path_len);

        if let (Some((old_parent_ino, old_s)), Some((new_parent_ino, new_s))) = (
            self.resolve_parent_for_path_inner(old_dir_ino, old_path),
            self.resolve_parent_for_path_inner(new_dir_ino, new_path),
        ) {
            // Remove from old
            self.modify_inode(&old_parent_ino, |node| {
                if let InodeData::Dir(dir_map) = &mut node.data {
                    dir_map.remove(&old_s);
                }
            });

            // Add to new
            self.modify_inode(&new_parent_ino, |node| {
                if let InodeData::Dir(dir_map) = &mut node.data {
                    dir_map.insert(new_s, inode);
                }
            });

            // If it's a directory, update ".."
            if self.is_dir(&inode) {
                self.modify_inode(&inode, |node| {
                    if let InodeData::Dir(dir_map) = &mut node.data {
                        dir_map.insert(SmallString::from_str(".."), new_parent_ino);
                    }
                });
            }

            return Ok(());
        }
        Err(wasip1::ERRNO_NOENT)
    }

    fn path_unlink_file_raw<Wasm: WasmAccess + WasmAccessName + 'static>(
        &self,
        dir_ino: &Self::Inode,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        let inode = self
            .get_inode_for_path::<Wasm>(dir_ino, path_ptr, path_len)
            .ok_or(wasip1::ERRNO_NOENT)?;

        if self.is_dir(&inode) {
            return Err(wasip1::ERRNO_ISDIR);
        }

        let path = WasmPathAccess::<Wasm>::new(path_ptr, path_len);
        if let Some((parent_ino, s)) = self.resolve_parent_for_path_inner(dir_ino, path) {
            self.modify_inode(&parent_ino, |node| {
                if let InodeData::Dir(dir_map) = &mut node.data {
                    dir_map.remove(&s);
                }
            });
            return Ok(());
        }
        Err(wasip1::ERRNO_NOENT)
    }
}

impl<B: BoxedInode, StdIo: StdIO + 'static, AddInfo: WasiAddInfo + Default + 'static>
    Wasip1DynCompatibleLFS<B> for StandardDynamicLFS<StdIo, AddInfo>
{
    fn fd_write_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        data: *const u8,
        data_len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        let inode_id = Self::downcast_inode(inode);
        self.modify_inode(&inode_id, |inode_entry| {
            if let InodeData::File(ref mut vec) = inode_entry.data {
                let mut buf = alloc::vec![0u8; data_len];
                access.memcpy_to_with(&mut buf, data);
                vec.extend_from_slice(&buf);
                let mtim = inode_entry.meta.add_info.modification_time();
                inode_entry.meta.add_info.set_modification_time(mtim + 1); // Simplistic time update
                Ok(data_len)
            } else {
                Err(wasip1::ERRNO_ISDIR)
            }
        })
        .unwrap_or(Err(wasip1::ERRNO_BADF))
    }

    fn fd_write_stdout_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        data: *const u8,
        data_len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        #[cfg(not(feature = "multi_memory"))]
        {
            StdIo::write_direct_dyn_compatible(access, data, data_len)
        }
        #[cfg(feature = "multi_memory")]
        {
            let mut buf = alloc::vec![0u8; data_len];
            access.memcpy_to_with(&mut buf, data);
            StdIo::write(&buf)
        }
    }

    fn fd_write_stderr_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        data: *const u8,
        data_len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        #[cfg(not(feature = "multi_memory"))]
        {
            StdIo::ewrite_direct_dyn_compatible(access, data, data_len)
        }
        #[cfg(feature = "multi_memory")]
        {
            let mut buf = alloc::vec![0u8; data_len];
            access.memcpy_to_with(&mut buf, data);
            StdIo::ewrite(&buf)
        }
    }

    fn fd_readdir_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        buf: *mut u8,
        buf_len: usize,
        cookie: wasip1::Dircookie,
    ) -> Result<(wasip1::Size, wasip1::Dircookie), wasip1::Errno> {
        let inode = Self::downcast_inode(inode);

        let dir_map = self
            .read_inode(&inode, |i| match &i.data {
                InodeData::Dir(map) => Some(map.clone()),
                _ => None,
            })
            .flatten()
            .ok_or(wasip1::ERRNO_NOTDIR)?;

        let mut current_cookie = cookie as usize;
        let mut total_written = 0;
        let mut out_ptr = buf;
        let mut rem_len = buf_len;

        for (i, (name, &child_inode)) in dir_map.iter().enumerate() {
            if i < current_cookie {
                continue;
            }
            let filetype = self
                .read_inode(&child_inode, |c| c.meta.filetype)
                .ok_or(wasip1::ERRNO_NOENT)?;
            let name_bytes = name.as_bytes();
            let entry = wasip1::Dirent {
                d_next: (i + 1) as u64,
                d_ino: child_inode as u64,
                d_namlen: name_bytes.len() as u32,
                d_type: filetype,
            };

            let entry_size = core::mem::size_of::<wasip1::Dirent>();

            if rem_len < entry_size {
                break;
            }

            // Write dirent
            let entry_slice =
                unsafe { core::slice::from_raw_parts(&entry as *const _ as *const u8, entry_size) };
            access.memcpy_with(out_ptr, entry_slice);

            out_ptr = unsafe { out_ptr.add(entry_size) };
            rem_len -= entry_size;
            total_written += entry_size;

            // Write name
            let write_name_len = core::cmp::min(name_bytes.len(), rem_len);
            if write_name_len > 0 {
                access.memcpy_with(out_ptr, &name_bytes[..write_name_len]);
                out_ptr = unsafe { out_ptr.add(write_name_len) };
                rem_len -= write_name_len;
                total_written += write_name_len;
            }

            current_cookie = i + 1;
            if rem_len == 0 {
                break;
            }
        }

        Ok((total_written, current_cookie as u64))
    }

    fn path_filestat_get_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        flags: wasip1::Lookupflags,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno> {
        let inode = Self::downcast_inode(inode);

        let mut target_inode = self
            .get_inode_for_path_dyn_compatible(access, inode, path_ptr, path_len)
            .ok_or(wasip1::ERRNO_NOENT)?;

        if flags & wasip1::LOOKUPFLAGS_SYMLINK_FOLLOW == wasip1::LOOKUPFLAGS_SYMLINK_FOLLOW {
            target_inode = self
                .resolve_inode(target_inode, 0)
                .ok_or(wasip1::ERRNO_LOOP)?;
        }

        Wasip1DynCompatibleLFS::<B>::fd_filestat_get_raw_dyn_compatible(self, access, &target_inode)
    }

    fn fd_prestat_get_raw_dyn_compatible(
        &self,
        _: &dyn crate::memory::WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
    ) -> Result<wasip1::Prestat, wasip1::Errno> {
        let inode = Self::downcast_inode(inode);

        self.read_preopen(&inode, |name| {
            Ok(wasip1::Prestat {
                tag: 0,
                u: wasip1::PrestatU {
                    dir: wasip1::PrestatDir {
                        pr_name_len: name.len() as usize,
                    },
                },
            })
        })
        .unwrap_or(Err(wasip1::ERRNO_BADF))
    }
    fn fd_prestat_dir_name_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        dir_path_ptr: *mut u8,
        dir_path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        let inode = Self::downcast_inode(inode);

        self.read_preopen(&inode, |name| {
            let copy_len = core::cmp::min(name.len(), dir_path_len);
            if copy_len > 0 {
                access.memcpy_with(dir_path_ptr, &name.as_bytes()[..copy_len]);
            }
            Ok(())
        })
        .unwrap_or(Err(wasip1::ERRNO_BADF))
    }

    fn fd_filestat_get_raw_dyn_compatible(
        &self,
        _: &dyn crate::memory::WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
    ) -> Result<FilestatWithoutDevice, wasip1::Errno> {
        let inode = Self::downcast_inode(inode);

        self.read_inode(&inode, |node| {
            Ok(FilestatWithoutDevice {
                ino: *inode as u64,
                filetype: node.meta.filetype,
                nlink: node.meta.nlink,
                size: node.meta.size(&node.data),
                atim: node.meta.add_info.access_time(),
                mtim: node.meta.add_info.modification_time(),
                ctim: node.meta.add_info.creation_time(),
            })
        })
        .unwrap_or(Err(wasip1::ERRNO_BADF))
    }

    fn fd_pread_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        buf: *mut u8,
        buf_len: usize,
        offset: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        let inode = Self::downcast_inode(inode);

        self.modify_inode(&inode, |node| {
            if let InodeData::File(ref file_data) = node.data {
                if offset >= file_data.len() {
                    return Ok(0);
                }
                let available = file_data.len() - offset;
                let read_len = core::cmp::min(buf_len, available);
                access.memcpy_with(buf, &file_data[offset..offset + read_len]);
                let atim = node.meta.add_info.access_time();
                node.meta.add_info.set_access_time(atim + 1);
                Ok(read_len)
            } else {
                Err(wasip1::ERRNO_ISDIR)
            }
        })
        .unwrap_or(Err(wasip1::ERRNO_BADF))
    }

    fn fd_read_stdin_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        buf: *mut u8,
        buf_len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        #[cfg(not(feature = "multi_memory"))]
        {
            StdIo::read_direct_dyn_compatible(access, buf, buf_len)
        }

        #[cfg(feature = "multi_memory")]
        {
            let mut internal_buf = alloc::vec![0u8; buf_len];
            let read = StdIo::read(&mut internal_buf).map_err(|_| wasip1::ERRNO_IO)?;
            access.memcpy_with(buf, &internal_buf[..read]);
            Ok(read)
        }
    }

    fn path_open_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        dir_ino: &dyn InodeIdCommon,
        _: wasip1::Fdflags,
        path_ptr: *const u8,
        path_len: usize,
        o_flags: wasip1::Oflags,
        fs_rights_base: wasip1::Rights,
        _: wasip1::Rights,
        _: wasip1::Fdflags,
    ) -> Result<B, wasip1::Errno> {
        let dir_ino = Self::downcast_inode(dir_ino);

        if let Some(inode) =
            self.get_inode_for_path_dyn_compatible(access, &dir_ino, path_ptr, path_len)
        {
            if o_flags & wasip1::OFLAGS_EXCL == wasip1::OFLAGS_EXCL {
                return Err(wasip1::ERRNO_EXIST);
            }
            if o_flags & wasip1::OFLAGS_DIRECTORY == wasip1::OFLAGS_DIRECTORY
                && !self.is_dir(&inode)
            {
                return Err(wasip1::ERRNO_NOTDIR);
            }
            if o_flags & wasip1::OFLAGS_TRUNC == wasip1::OFLAGS_TRUNC {
                self.modify_inode(&inode, |node| {
                    if let InodeData::File(ref mut vec) = node.data {
                        vec.clear();
                    }
                });
            }
            Ok(B::from_inode_with_ty::<Self>(inode))
        } else {
            if o_flags & wasip1::OFLAGS_CREAT == wasip1::OFLAGS_CREAT {
                let path =
                    crate::memory::WasmPathAccessDynCompatible::new(access, path_ptr, path_len);
                let components: Vec<_> = path.components().collect();

                if components.is_empty() {
                    return Err(wasip1::ERRNO_NOENT);
                }

                if components.len() == 1 {
                    if let crate::memory::WasmPathComponentDynCompatible::Normal(name) =
                        &components[0]
                    {
                        let mut s = SmallString::<[u8; 32]>::new();
                        for j in 0..name.len() {
                            s.push(name.get(j) as char);
                        }

                        let is_dir = o_flags & wasip1::OFLAGS_DIRECTORY == wasip1::OFLAGS_DIRECTORY;
                        let filetype = if is_dir {
                            wasip1::FILETYPE_DIRECTORY
                        } else {
                            wasip1::FILETYPE_REGULAR_FILE
                        };

                        let data = if is_dir {
                            let mut map = DirMap::new();
                            map.insert(SmallString::from_str("."), 0); // Self (will update)
                            map.insert(SmallString::from_str(".."), *dir_ino);
                            InodeData::Dir(map)
                        } else {
                            InodeData::File(Vec::new())
                        };

                        let new_inode = Inode {
                            meta: InodeMetadata::new(filetype, fs_rights_base, AddInfo::default()),
                            data,
                        };

                        let new_id = self.allocate_inode(new_inode);

                        // Update self reference if dir
                        if is_dir {
                            self.modify_inode(&new_id, |node| {
                                if let InodeData::Dir(map) = &mut node.data {
                                    map.insert(SmallString::from_str("."), new_id);
                                }
                            });
                        }

                        // Link in parent
                        self.modify_inode(&dir_ino, |node| {
                            if let InodeData::Dir(dir_map) = &mut node.data {
                                dir_map.insert(s, new_id);
                            }
                        });

                        return Ok(B::from_inode_with_ty::<Self>(new_id));
                    }
                }
                Err(wasip1::ERRNO_NOENT)
            } else {
                Err(wasip1::ERRNO_NOENT)
            }
        }
    }

    fn path_readlink_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        path_ptr: *const u8,
        path_len: usize,
        buf: *mut u8,
        buf_len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        let inode_id = Self::downcast_inode(inode);
        let target_inode = self
            .get_inode_for_path_dyn_compatible(access, &inode_id, path_ptr, path_len)
            .ok_or(wasip1::ERRNO_NOENT)?;

        self.read_inode(&target_inode, |node| {
            if let InodeData::Symlink(ref target) = node.data {
                let target_bytes = target.as_bytes();
                let n = core::cmp::min(target_bytes.len(), buf_len);
                access.memcpy_with(buf, &target_bytes[..n]);
                Ok(n)
            } else {
                Err(wasip1::ERRNO_INVAL)
            }
        })
        .unwrap_or(Err(wasip1::ERRNO_BADF))
    }

    fn path_create_directory_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        dir_inode: &dyn InodeIdCommon,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        let dir_ino = Self::downcast_inode(dir_inode);
        
        if self
            .get_inode_for_path_dyn_compatible(access, dir_ino, path_ptr, path_len)
            .is_some()
        {
            return Err(wasip1::ERRNO_EXIST);
        }

        let path = crate::memory::WasmPathAccessDynCompatible::new(access, path_ptr, path_len);
        if let Some((parent_ino, s)) = self.resolve_parent_for_path_inner(dir_ino, path) {
            let mut map = DirMap::new();
            map.insert(SmallString::from_str("."), 0); // Will update
            map.insert(SmallString::from_str(".."), parent_ino);

            let new_inode = Inode {
                meta: InodeMetadata::new(wasip1::FILETYPE_DIRECTORY, 0, AddInfo::default()),
                data: InodeData::Dir(map),
            };

            let new_id = self.allocate_inode(new_inode);
            self.modify_inode(&new_id, |node| {
                if let InodeData::Dir(map) = &mut node.data {
                    map.insert(SmallString::from_str("."), new_id);
                }
            });

            self.modify_inode(&parent_ino, |node| {
                if let InodeData::Dir(dir_map) = &mut node.data {
                    dir_map.insert(s, new_id);
                }
            });

            return Ok(());
        }
        Err(wasip1::ERRNO_NOENT)
    }

    fn path_link_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        old_dir_inode: &dyn InodeIdCommon,
        _: wasip1::Lookupflags,
        old_path_ptr: *const u8,
        old_path_len: usize,
        new_dir_inode: &dyn InodeIdCommon,
        new_path_ptr: *const u8,
        new_path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        let old_dir_ino = Self::downcast_inode(old_dir_inode);
        let new_dir_ino = Self::downcast_inode(new_dir_inode);
        
        let inode = self
            .get_inode_for_path_dyn_compatible(
                access,
                old_dir_ino,
                old_path_ptr,
                old_path_len,
            )
            .ok_or(wasip1::ERRNO_NOENT)?;

        if self.is_dir(&inode) {
            return Err(wasip1::ERRNO_PERM);
        }

        let new_path =
            crate::memory::WasmPathAccessDynCompatible::new(access, new_path_ptr, new_path_len);
        if let Some((parent_ino, new_s)) = self.resolve_parent_for_path_inner(new_dir_ino, new_path) {
            self.modify_inode(&parent_ino, |node| {
                if let InodeData::Dir(dir_map) = &mut node.data {
                    dir_map.insert(new_s, inode);
                }
            });

            return Ok(());
        }
        Err(wasip1::ERRNO_NOENT)
    }

    fn path_remove_directory_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        dir_inode: &dyn InodeIdCommon,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        let dir_ino = Self::downcast_inode(dir_inode);
        
        let inode = self
            .get_inode_for_path_dyn_compatible(access, dir_ino, path_ptr, path_len)
            .ok_or(wasip1::ERRNO_NOENT)?;

        if !self.is_dir(&inode) {
            return Err(wasip1::ERRNO_NOTDIR);
        }

        // Check if directory is empty (only "." and "..")
        let is_empty = self
            .read_inode(&inode, |node| {
                if let InodeData::Dir(dir_map) = &node.data {
                    dir_map.len() <= 2
                } else {
                    false
                }
            })
            .unwrap_or(false);

        if !is_empty {
            return Err(wasip1::ERRNO_NOTEMPTY);
        }

        let path = crate::memory::WasmPathAccessDynCompatible::new(access, path_ptr, path_len);
        if let Some((parent_ino, s)) = self.resolve_parent_for_path_inner(dir_ino, path) {
            self.modify_inode(&parent_ino, |node| {
                if let InodeData::Dir(dir_map) = &mut node.data {
                    dir_map.remove(&s);
                }
            });
            return Ok(());
        }
        Err(wasip1::ERRNO_NOENT)
    }

    fn path_rename_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        old_dir_inode: &dyn InodeIdCommon,
        old_path_ptr: *const u8,
        old_path_len: usize,
        new_dir_inode: &dyn InodeIdCommon,
        new_path_ptr: *const u8,
        new_path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        let old_dir_ino = Self::downcast_inode(old_dir_inode);
        let new_dir_ino = Self::downcast_inode(new_dir_inode);
        
        let inode = self
            .get_inode_for_path_dyn_compatible(
                access,
                old_dir_ino,
                old_path_ptr,
                old_path_len,
            )
            .ok_or(wasip1::ERRNO_NOENT)?;

        let old_path =
            crate::memory::WasmPathAccessDynCompatible::new(access, old_path_ptr, old_path_len);
        let new_path =
            crate::memory::WasmPathAccessDynCompatible::new(access, new_path_ptr, new_path_len);

        if let (Some((old_parent_ino, old_s)), Some((new_parent_ino, new_s))) = (
            self.resolve_parent_for_path_inner(old_dir_ino, old_path),
            self.resolve_parent_for_path_inner(new_dir_ino, new_path),
        ) {
            // Remove from old
            self.modify_inode(&old_parent_ino, |node| {
                if let InodeData::Dir(dir_map) = &mut node.data {
                    dir_map.remove(&old_s);
                }
            });

            // Add to new
            self.modify_inode(&new_parent_ino, |node| {
                if let InodeData::Dir(dir_map) = &mut node.data {
                    dir_map.insert(new_s, inode);
                }
            });

            // If it's a directory, update ".."
            if self.is_dir(&inode) {
                self.modify_inode(&inode, |node| {
                    if let InodeData::Dir(dir_map) = &mut node.data {
                        dir_map.insert(SmallString::from_str(".."), new_parent_ino);
                    }
                });
            }

            return Ok(());
        }
        Err(wasip1::ERRNO_NOENT)
    }

    fn path_unlink_file_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        dir_inode: &dyn InodeIdCommon,
        path_ptr: *const u8,
        path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        let dir_ino = Self::downcast_inode(dir_inode);
        
        let inode = self
            .get_inode_for_path_dyn_compatible(access, dir_ino, path_ptr, path_len)
            .ok_or(wasip1::ERRNO_NOENT)?;

        if self.is_dir(&inode) {
            return Err(wasip1::ERRNO_ISDIR);
        }

        let path = crate::memory::WasmPathAccessDynCompatible::new(access, path_ptr, path_len);
        if let Some((parent_ino, s)) = self.resolve_parent_for_path_inner(dir_ino, path) {
            self.modify_inode(&parent_ino, |node| {
                if let InodeData::Dir(dir_map) = &mut node.data {
                    dir_map.remove(&s);
                }
            });
            return Ok(());
        }
        Err(wasip1::ERRNO_NOENT)
    }

    fn pre_open_inodes(
        &self,
        f: &mut dyn for<'a> FnMut(&'a dyn crate::wasi::file::Wasip1DynCompatibleLFSSlice),
    ) {
        #[derive(Debug)]
        struct PreOpenInodesIndex<'a, StdIo: StdIO + 'static, AddInfo: WasiAddInfo + 'static>(
            &'a StandardDynamicLFS<StdIo, AddInfo>,
        );

        impl<'a, StdIo: StdIO + 'static, AddInfo: WasiAddInfo + 'static>
            crate::wasi::file::Wasip1DynCompatibleLFSSlice
            for PreOpenInodesIndex<'a, StdIo, AddInfo>
        {
            fn index(
                &self,
                idx: usize,
                f: &mut dyn for<'b, 'c> FnMut(
                    Option<(
                        &'b dyn InodeIdCommon,
                        &'c dyn crate::wasi::file::DerefToStrCustom,
                    )>,
                ),
            ) {
                #[cfg(feature = "threads")]
                {
                    if let Some(entry) = self.0.preopens.iter().nth(idx) {
                        let inode = entry.key();
                        let name = entry.value();
                        f(Some((
                            inode as &dyn InodeIdCommon,
                            name as &dyn crate::wasi::file::DerefToStrCustom,
                        )));
                    } else {
                        f(None);
                    }
                }
                #[cfg(not(feature = "threads"))]
                {
                    let map = unsafe { &*self.0.preopens.get() };
                    if let Some((inode, name)) = map.iter().nth(idx) {
                        f(Some((
                            inode as &dyn InodeIdCommon,
                            name as &dyn crate::wasi::file::DerefToStrCustom,
                        )));
                    } else {
                        f(None);
                    }
                }
            }
        }

        f(&PreOpenInodesIndex(self));
    }

    fn fd_pwrite_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        data: *const u8,
        data_len: usize,
        offset: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        let inode_id = Self::downcast_inode(inode);
        self.modify_inode(&inode_id, |inode_entry| {
            if let InodeData::File(ref mut vec) = inode_entry.data {
                let end = offset + data_len;
                if end > vec.len() {
                    vec.resize(end, 0);
                }
                let mut buf = alloc::vec![0u8; data_len];
                access.memcpy_to_with(&mut buf, data);
                vec[offset..end].copy_from_slice(&buf);
                let mtim = inode_entry.meta.add_info.modification_time();
                inode_entry.meta.add_info.set_modification_time(mtim + 1);
                Ok(data_len)
            } else {
                Err(wasip1::ERRNO_ISDIR)
            }
        })
        .unwrap_or(Err(wasip1::ERRNO_BADF))
    }

    fn fd_advise_raw_dyn_compatible(
        &self,
        _access: &dyn WasmAccessDynCompatibleRaw,
        _inode: &dyn InodeIdCommon,
        _offset: u64,
        _len: u64,
        _advice: wasip1::Advice,
    ) -> Result<(), wasip1::Errno> {
        Ok(())
    }

    fn fd_allocate_raw_dyn_compatible(
        &self,
        _access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        offset: u64,
        len: u64,
    ) -> Result<(), wasip1::Errno> {
        let required = (offset + len) as usize;
        let inode_id = Self::downcast_inode(inode);
        self.modify_inode(&inode_id, |node| {
            if let InodeData::File(ref mut vec) = node.data {
                if required > vec.len() {
                    vec.resize(required, 0);
                }
                Ok(())
            } else {
                Err(wasip1::ERRNO_ISDIR)
            }
        })
        .unwrap_or(Err(wasip1::ERRNO_BADF))
    }

    fn fd_datasync_raw_dyn_compatible(
        &self,
        _access: &dyn WasmAccessDynCompatibleRaw,
        _inode: &dyn InodeIdCommon,
    ) -> Result<(), wasip1::Errno> {
        Ok(())
    }

    fn fd_sync_raw_dyn_compatible(
        &self,
        _access: &dyn WasmAccessDynCompatibleRaw,
        _inode: &dyn InodeIdCommon,
    ) -> Result<(), wasip1::Errno> {
        Ok(())
    }

    fn fd_filestat_set_size_raw_dyn_compatible(
        &self,
        _access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        size: u64,
    ) -> Result<(), wasip1::Errno> {
        let inode_id = Self::downcast_inode(inode);
        self.modify_inode(&inode_id, |node| {
            if let InodeData::File(ref mut vec) = node.data {
                vec.resize(size as usize, 0);
                let mtim = node.meta.add_info.modification_time();
                node.meta.add_info.set_modification_time(mtim + 1);
                Ok(())
            } else {
                Err(wasip1::ERRNO_ISDIR)
            }
        })
        .unwrap_or(Err(wasip1::ERRNO_BADF))
    }

    fn fd_filestat_set_times_raw_dyn_compatible(
        &self,
        _access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        atim: wasip1::Timestamp,
        mtim: wasip1::Timestamp,
        fst_flags: wasip1::Fstflags,
    ) -> Result<(), wasip1::Errno> {
        let inode_id = Self::downcast_inode(inode);
        self.modify_inode(&inode_id, |node| {
            if fst_flags & wasip1::FSTFLAGS_ATIM != 0 {
                node.meta.add_info.set_access_time(atim);
            }
            if fst_flags & wasip1::FSTFLAGS_MTIM != 0 {
                node.meta.add_info.set_modification_time(mtim);
            }
            if fst_flags & wasip1::FSTFLAGS_ATIM_NOW != 0 {
                let t = node.meta.add_info.access_time();
                node.meta.add_info.set_access_time(t + 1);
            }
            if fst_flags & wasip1::FSTFLAGS_MTIM_NOW != 0 {
                let t = node.meta.add_info.modification_time();
                node.meta.add_info.set_modification_time(t + 1);
            }
            Ok(())
        })
        .unwrap_or(Err(wasip1::ERRNO_BADF))
    }

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
    ) -> Result<(), wasip1::Errno> {
        let inode_id = Self::downcast_inode(inode);
        let mut target_inode = self
            .get_inode_for_path_dyn_compatible(access, inode_id, path_ptr, path_len)
            .ok_or(wasip1::ERRNO_NOENT)?;

        if flags & wasip1::LOOKUPFLAGS_SYMLINK_FOLLOW == wasip1::LOOKUPFLAGS_SYMLINK_FOLLOW {
            target_inode = self
                .resolve_inode(target_inode, 0)
                .ok_or(wasip1::ERRNO_LOOP)?;
        }

        Wasip1DynCompatibleLFS::<B>::fd_filestat_set_times_raw_dyn_compatible(self, access, &target_inode, atim, mtim, fst_flags)
    }

    fn path_symlink_raw_dyn_compatible(
        &self,
        access: &dyn WasmAccessDynCompatibleRaw,
        inode: &dyn InodeIdCommon,
        old_path_ptr: *const u8,
        old_path_len: usize,
        new_path_ptr: *const u8,
        new_path_len: usize,
    ) -> Result<(), wasip1::Errno> {
        let inode_id = Self::downcast_inode(inode);
        
        // Read the old (target) path
        let old_path_str = {
            let path = crate::memory::WasmPathAccessDynCompatible::new(access, old_path_ptr, old_path_len);
            let mut s = alloc::string::String::new();
            for component in path.components() {
                if !s.is_empty() {
                    s.push('/');
                }
                match component {
                    crate::memory::WasmPathComponentDynCompatible::Normal(name) => {
                        for i in 0..name.len() {
                            s.push(name.get(i) as char);
                        }
                    }
                    crate::memory::WasmPathComponentDynCompatible::CurDir => s.push('.'),
                    crate::memory::WasmPathComponentDynCompatible::ParentDir => s.push_str(".."),
                    crate::memory::WasmPathComponentDynCompatible::RootDir => s.push('/'),
                }
            }
            s
        };

        let new_path = crate::memory::WasmPathAccessDynCompatible::new(access, new_path_ptr, new_path_len);
        if let Some((parent_ino, new_name)) = self.resolve_parent_for_path_inner(inode_id, new_path) {
            // Check if the new path already exists
            if self.read_inode(&parent_ino, |node| {
                if let InodeData::Dir(map) = &node.data {
                    map.contains_key(&new_name)
                } else {
                    false
                }
            }).unwrap_or(false) {
                return Err(wasip1::ERRNO_EXIST);
            }

            let symlink_inode = Inode {
                meta: InodeMetadata::new(wasip1::FILETYPE_SYMBOLIC_LINK, 0, AddInfo::default()),
                data: InodeData::Symlink(old_path_str),
            };
            let new_id = self.allocate_inode(symlink_inode);
            self.modify_inode(&parent_ino, |node| {
                if let InodeData::Dir(dir_map) = &mut node.data {
                    dir_map.insert(new_name, new_id);
                }
            });
            Ok(())
        } else {
            Err(wasip1::ERRNO_NOENT)
        }
    }
}

impl<StdIo: StdIO + 'static, AddInfo: WasiAddInfo + Default + 'static> DynamicLFS
    for StandardDynamicLFS<StdIo, AddInfo>
{
    fn pre_open_inodes(&self) -> impl IntoIterator<Item = (Self::Inode, impl DerefToStrCustom)> {
        #[cfg(feature = "threads")]
        {
            self.preopens.iter().map(|v| {
                let (ino, name) = v.pair();

                (*ino, name.clone())
            })
        }
        #[cfg(not(feature = "threads"))]
        {
            unsafe { &*self.preopens.get() }
                .iter()
                .map(|(ino, name)| (*ino, name.clone()))
        }
    }
}
