use crate::memory::WasmAccess;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct __self;

impl WasmAccess for __self {
    fn memcpy<T>(offset: *mut T, data: &[T]) {
        todo!()
    }

    fn memcpy_to<T>(offset: &mut [T], src: *const T) {
        todo!()
    }

    fn store_le<T>(offset: *mut T, value: T) {
        todo!()
    }

    fn load_le<T: core::fmt::Debug>(offset: *const T) -> T {
        todo!()
    }

    #[cfg(not(feature = "multi_memory"))]
    fn memory_director<T>(ptr: *const T) -> *const T {
        todo!()
    }

    #[cfg(not(feature = "multi_memory"))]
    fn memory_director_mut<T>(ptr: *mut T) -> *mut T {
        todo!()
    }

    fn _main() {
        todo!()
    }

    fn reset() {
        todo!()
    }

    fn _start() {
        todo!()
    }
}
