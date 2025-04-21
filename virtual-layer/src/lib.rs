pub mod memory;
pub mod wasi;
pub mod wit;
pub use wasip1;
pub use paste;

pub trait VirtualFileSystem {
    fn new() -> Self;
    fn get_file(&self, path: &str) -> Option<Vec<u8>>;
    fn set_file(&mut self, path: &str, data: Vec<u8>);
    fn remove_file(&mut self, path: &str);
    fn list_files(&self) -> Vec<String>;
    fn clear(&mut self);
}
