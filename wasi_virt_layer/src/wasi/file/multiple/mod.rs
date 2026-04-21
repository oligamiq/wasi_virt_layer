#![cfg(feature = "multiple-fs")]
#![allow(missing_docs)]

pub mod dynamic_wasm;
pub mod inode;
pub mod lfs;
pub mod wasm;

pub use inode::DetailedDynamicOpenFd;
pub use lfs::StandardMultipleFileSystem;
pub use wasm::WasmAccessDynCompatibleWrapper;
