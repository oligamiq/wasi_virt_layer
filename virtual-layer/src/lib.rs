pub mod memory;
pub mod wasi;
pub mod wit;
pub use paste;
pub use wasip1;

pub mod prelude {
    pub use crate::wasi::env::{VirtualEnv, VirtualEnvConstState};
    pub use crate::{export_env, import_wasm};
}
