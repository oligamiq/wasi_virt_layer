#![cfg_attr(not(feature = "std"), no_std)]

// #[cfg(target_os = "wasi")]
// #[cfg(feature = "std")]
// #[cfg(feature = "unstable_print_debug")]
// use core::sync::atomic;

mod __self;
#[cfg(all(feature = "unstable_print_debug", target_os = "wasi"))]
mod debug;
mod initializer;
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
pub mod wasip1;

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod prelude {
    pub use crate::memory::WasmAccess;
    #[cfg(feature = "threads")]
    pub use crate::plug_thread;
    pub use crate::wasi::env::{VirtualEnv, VirtualEnvConstState};
    pub use crate::wasi::file::constant::vfs::Wasip1ConstVFS;
    pub use crate::{ConstFiles, import_wasm, plug_env, plug_fs};
}

#[cfg(feature = "threads")]
pub mod thread {
    pub use crate::wasi::thread::{
        DirectThreadPool, ThreadAccess, ThreadRunner, VirtualThread, root_spawn,
    };
}

pub mod file {
    pub use crate::wasi::file::{
        FilestatWithoutDevice, Wasip1FileSystem, Wasip1FileTrait, Wasip1LFS,
        constant::{
            lfs::VFSConstNormalLFS,
            lfs_raw::{VFSConstNormalFiles, WasiConstFile},
            vfs::Wasip1ConstVFS,
        },
        stdio::DefaultStdIO,
    };
}

pub mod process {
    pub use crate::wasi::process::{DefaultProcess, ProcessExit};
}

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
    }
}
