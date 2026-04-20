#![cfg(feature = "multiple-fs")]
#![allow(missing_docs)]

pub mod inode;
pub mod lfs;
pub mod wasm;

pub use lfs::Wasip1MultipleVFS;
pub use wasm::WasmAccessDynCompatibleWrapper;
pub use inode::DetailedDynamicOpenFd;
