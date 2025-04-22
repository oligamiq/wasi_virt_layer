pub mod memory;
pub mod wasi;
pub mod wit;
pub use paste;
pub use wasip1;

pub mod prelude {
    pub use crate::wasi::env::VirtualEnvConstState;
    pub use crate::{export_env, import_wasm};
}

pub trait VirtualFileSystem {
    fn new() -> Self;
    fn get_file(&self, path: &str) -> Option<Vec<u8>>;
    fn set_file(&mut self, path: &str, data: Vec<u8>);
    fn remove_file(&mut self, path: &str);
    fn list_files(&self) -> Vec<String>;
    fn clear(&mut self);
}
