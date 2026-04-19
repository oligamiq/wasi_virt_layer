#![warn(missing_docs)]
//! `wasi_virt_layer` provides a virtualization layer for WebAssembly System Interface (WASI).
//!
//! This crate facilitates merging a Virtual File System (VFS) and threading mechanisms into
//! standard WASI modules without modifying their source code in complex ways. It allows a host
//! environment (like browsers via JavaScript bindings) to seamlessly interact with in-memory Wasm modules.
//!
//! # Core Concepts
//! - **Virtual File System (VFS)**: Overrides filesystem-related WASI calls to a custom virtualized implementation.
//! - **Threading**: Provides components that patch how Wasm spawns and manages threads using shared memory.
//! - **Memory Bridge**: Manages memory boundaries and host-guest interaction.

#![cfg_attr(not(feature = "std"), no_std)]
/// Procedural macros for generating WASIP1 boilerplate.
pub mod wasip1_derive;

#[cfg(feature = "simple-debug")]
/// Simple, fast debugging utilities for WebAssembly execution.
pub mod simple_debug;

// #[cfg(target_os = "wasi")]
// #[cfg(feature = "std")]
// #[cfg(feature = "unstable_print_debug")]
// use core::sync::atomic;

mod __self;
#[cfg(all(feature = "unstable_print_debug", target_os = "wasi"))]
mod debug;
mod initializer;
/// Memory operations to bridge host and WebAssembly memory models.
pub mod memory;
#[cfg(all(
    target_arch = "wasm32",
    feature = "threads",
    not(feature = "multi_memory")
))]
mod shared_global;
mod transporter;
mod utils;
mod wasi;
mod wit;

#[cfg(not(target_os = "wasi"))]
/// Definitions and bindings for the original WASIP1 API.
pub mod wasip1;

#[cfg(feature = "alloc")]
extern crate alloc;

/// Common traits, structs, and macros representing the core functionality.
pub mod prelude {
    pub use crate::memory::WasmAccess;
    #[cfg(feature = "threads")]
    pub use crate::plug_thread;
    pub use crate::wasi::env::{VirtualEnv, VirtualEnvConstState};
    pub use crate::wasi::file::constant::vfs::Wasip1ConstVFS;
    pub use crate::{ConstFiles, import_wasm, plug_env, plug_fs, plug_poll, plug_process};
}

#[cfg(feature = "threads")]
/// Threading support for WASI.
pub mod thread {
    pub use crate::wasi::thread::{
        DirectThreadPool, ThreadAccess, ThreadRunner, VirtualThread, VirtualThreadPool, root_spawn,
        root_spawn_unchecked,
    };
}

/// Virtual File System operations and definitions.
pub mod file {
    pub use crate::wasi::file::{
        DefaultAddInfo, FilestatWithoutDevice, NoAddInfo, WasiAddInfo, Wasip1FileSystem,
        Wasip1FileTrait,
        constant::{
            lfs::VFSConstNormalLFS,
            lfs_raw::{VFSConstNormalFiles, WasiConstFile},
            vfs::Wasip1ConstVFS,
        },
        stdio::DefaultStdIO,
    };

    #[cfg(feature = "alloc")]
    pub use crate::wasi::file::changeable::{
        inode::{DirMap, Inode, InodeData, InodeId, InodeMetadata},
        lfs::ChangeableLFS,
        vfs::ChangeableVFS,
    };

    #[cfg(feature = "multiple_lfs")]
    pub use crate::wasi::file::multiple;
}

/// Process execution and lifecycle virtualization.
pub mod process {
    pub use crate::wasi::process::{DefaultProcess, ProcessExit};
}

/// I/O polling and event waiting mechanisms.
pub mod poll {
    pub use crate::wasi::poll::{DefaultPoll, PollOneoff};
}

#[doc(hidden)]
#[allow(missing_docs)]
pub mod __private {
    #[cfg(not(target_os = "wasi"))]
    pub use super::wasip1;
    pub use crate::__self::__self;
    pub use const_for::const_for;
    pub use paste;
    #[cfg(target_os = "wasi")]
    pub use wasip1;

    pub mod inner {
        pub mod env {
            #[cfg(target_os = "wasi")]
            pub use crate::wasi::env::{
                environ_get_const_inner, environ_get_inner, environ_sizes_get_const_inner,
                environ_sizes_get_inner,
            };
        }

        pub mod fs {
            pub use crate::wasi::file::constant::lfs_raw::{
                VFSConstNormalFiles, VFSConstNormalInode, WasiConstPrimitiveFile,
            };
        }

        #[cfg(feature = "threads")]
        pub mod thread {
            pub use crate::wasi::thread::ThreadRunner;
        }

        pub use crate::wit::virtual_file_system;
    }

    pub mod utils {
        pub use crate::utils::StaticArrayBuilder;
        #[cfg(feature = "alloc")]
        pub use crate::utils::alloc_buff;
    }
}
