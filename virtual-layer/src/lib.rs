pub mod wasi;
pub mod wit;

pub trait VirtualFileSystem {
    fn new() -> Self;
    fn get_file(&self, path: &str) -> Option<Vec<u8>>;
    fn set_file(&mut self, path: &str, data: Vec<u8>);
    fn remove_file(&mut self, path: &str);
    fn list_files(&self) -> Vec<String>;
    fn clear(&mut self);
}

pub trait MemoryAccess {
    fn get_memory(&self) -> &[u8];
    fn set_memory(&mut self, offset: usize, data: &[u8]);
    fn get_memory_mut(&mut self) -> &mut [u8];
}

pub struct Memory {
    data: Vec<u8>,
}

impl MemoryAccess for Memory {
    fn get_memory(&self) -> &[u8] {
        &self.data
    }

    fn set_memory(&mut self, offset: usize, data: &[u8]) {
        self.data[offset..offset + data.len()].copy_from_slice(data);
    }

    fn get_memory_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}
