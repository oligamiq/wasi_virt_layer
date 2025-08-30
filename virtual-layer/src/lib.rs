#![cfg_attr(not(feature = "std"), no_std)]

pub mod binary_map;
pub mod memory;
pub mod transporter;
pub mod wasi;
pub mod wit;

#[cfg(not(target_os = "wasi"))]
pub mod wasip1;

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod prelude {
    pub use crate::wasi::env::{VirtualEnv, VirtualEnvConstState};
    pub use crate::{export_env, import_wasm};
}

pub mod __private {
    #[cfg(not(target_os = "wasi"))]
    pub use super::wasip1;
    pub use const_for::const_for;
    pub use paste;
    #[cfg(target_os = "wasi")]
    pub use wasip1;
}
