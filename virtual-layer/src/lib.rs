#![cfg_attr(not(feature = "std"), no_std)]

pub mod memory;
pub mod wasi;
pub mod wit;
pub use wasip1;
pub mod binary_map;
pub mod transporter;

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod prelude {
    pub use crate::wasi::env::{VirtualEnv, VirtualEnvConstState};
    pub use crate::{export_env, import_wasm};
}

pub mod __private {
    pub use const_for::const_for;
    pub use paste;
}
