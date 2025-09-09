#![cfg_attr(not(feature = "std"), no_std)]

// #[cfg(target_os = "wasi")]
// #[cfg(feature = "std")]
// #[cfg(feature = "debug")]
// use core::sync::atomic;

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
            pub use crate::wasi::thread::ThreadRunnerBase;
        }

        pub use crate::wit::virtual_file_system;
    }

    pub mod utils {
        pub use crate::binary_map::StaticArrayBuilder;
    }
}

#[cfg(feature = "debug")]
#[cfg(feature = "std")]
#[cfg(target_os = "wasi")]
#[unsafe(no_mangle)]
unsafe extern "C" fn debug_call_indirect(tid: i32, idx: i32) {
    // static NEST: atomic::AtomicU32 = atomic::AtomicU32::new(0);

    #[cfg(target_os = "wasi")]
    {
        // let nest = NEST.fetch_add(1, atomic::Ordering::SeqCst);
        // if nest == 0 {
        transporter::Wasip1Transporter::write_to_stderr(b"debug_call_indirect: tid=").unwrap();
        // transporter::Wasip1Transporter::write_to_stderr(tid.to_string().as_bytes()).unwrap();
        num_to_str(tid, transporter::Wasip1Transporter::write_to_stderr).unwrap();
        transporter::Wasip1Transporter::write_to_stderr(b", idx=").unwrap();
        // transporter::Wasip1Transporter::write_to_stderr(idx.to_string().as_bytes()).unwrap();
        num_to_str(idx, transporter::Wasip1Transporter::write_to_stderr).unwrap();
        transporter::Wasip1Transporter::write_to_stderr(b"\n").unwrap();
        // }
        // NEST.fetch_sub(1, atomic::Ordering::SeqCst);
    }

    #[cfg(not(target_os = "wasi"))]
    {
        panic!("This function is only available on WASI");
    }
}
fn num_to_str(
    n: i32,
    writer: impl Fn(&[u8]) -> Result<wasip1::Size, wasip1::Errno>,
) -> Result<(), wasip1::Errno> {
    if n == 0 {
        writer(&[b'0'])?;
        return Ok(());
    }

    let mut buf = [0u8; 11];
    let mut i = buf.len();

    let negative = n < 0;
    let mut num = if negative { -(n as i64) } else { n as i64 };

    while num > 0 {
        i -= 1;
        buf[i] = b'0' + (num % 10) as u8;
        num /= 10;
    }

    if negative {
        i -= 1;
        buf[i] = b'-';
    }

    writer(&buf[i..])?;
    Ok(())
}

#[cfg(feature = "debug")]
#[cfg(feature = "std")]
#[cfg(target_os = "wasi")]
#[unsafe(no_mangle)]
unsafe extern "C" fn debug_call_function(idx: i32) {
    // static NEST: atomic::AtomicU32 = atomic::AtomicU32::new(0);

    #[cfg(target_os = "wasi")]
    {
        // let nest = NEST.fetch_add(1, atomic::Ordering::SeqCst);
        // if nest == 0 {
        transporter::Wasip1Transporter::write_to_stderr(b"debug_call_function: idx=").unwrap();
        num_to_str(idx, transporter::Wasip1Transporter::write_to_stderr).unwrap();
        transporter::Wasip1Transporter::write_to_stderr(b"\n").unwrap();
        // }
        // NEST.fetch_sub(1, atomic::Ordering::SeqCst);
    }

    #[cfg(not(target_os = "wasi"))]
    {
        panic!("This function is only available on WASI");
    }
}

#[cfg(feature = "debug")]
#[cfg(feature = "std")]
#[cfg(target_os = "wasi")]
#[unsafe(no_mangle)]
unsafe extern "C" fn debug_atomic_wait(ptr: *const i32, expression: *const i32, timeout_ns: i64) {
    // static NEST: atomic::AtomicU32 = atomic::AtomicU32::new(0);

    #[cfg(target_os = "wasi")]
    {
        // let nest = NEST.fetch_add(1, atomic::Ordering::SeqCst);
        // if nest == 0 {
        transporter::Wasip1Transporter::write_to_stderr(b"debug_atomic_wait: ptr=").unwrap();
        transporter::Wasip1Transporter::write_to_stderr(format!("{ptr:?}").as_bytes()).unwrap();
        transporter::Wasip1Transporter::write_to_stderr(b", expression=").unwrap();
        transporter::Wasip1Transporter::write_to_stderr(format!("{expression:?}").as_bytes())
            .unwrap();
        transporter::Wasip1Transporter::write_to_stderr(b", timeout_ns=").unwrap();
        transporter::Wasip1Transporter::write_to_stderr(timeout_ns.to_string().as_bytes()).unwrap();
        transporter::Wasip1Transporter::write_to_stderr(b"\n").unwrap();
        // }
        // NEST.fetch_sub(1, atomic::Ordering::SeqCst);
    }

    #[cfg(not(target_os = "wasi"))]
    {
        panic!("This function is only available on WASI");
    }
}
