#![cfg_attr(not(feature = "std"), no_std)]

mod __self;
mod binary_map;
pub mod memory;
mod transporter;
mod wasi;
mod wit;

#[cfg(not(target_os = "wasi"))]
pub mod wasip1;

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod prelude {
    #[cfg(feature = "threads")]
    pub use crate::export_thread;
    pub use crate::memory::WasmAccess;
    pub use crate::wasi::env::{VirtualEnv, VirtualEnvConstState};
    pub use crate::wasi::file::constant::vfs::Wasip1ConstVFS;
    pub use crate::{ConstFiles, export_env, export_fs, import_wasm};
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

        pub mod thread {
            pub use crate::wasi::thread::ThreadRunnerBase;
        }

        pub use crate::wit::virtual_file_system;
    }

    pub mod utils {
        pub use crate::binary_map::StaticArrayBuilder;
    }
}
