use crate::__private::wasip1;
use crate::memory::WasmAccess;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct __self;

impl WasmAccess for __self {
    const NAME: &'static str = "__self";

    fn memcpy<T>(offset: *mut T, data: &[T]) {
        unsafe {
            core::ptr::copy_nonoverlapping(data.as_ptr(), offset, data.len());
        }
    }

    fn memcpy_to<T>(offset: &mut [T], src: *const T) {
        unsafe { core::ptr::copy_nonoverlapping(src, offset.as_mut_ptr(), offset.len()) };
    }

    fn store_le<T>(offset: *mut T, value: T) {
        unsafe { *offset = value };
    }

    fn load_le<T: core::fmt::Debug + Copy>(offset: *const T) -> T {
        unsafe { *offset }
    }

    #[cfg(not(feature = "multi_memory"))]
    fn memory_director<T>(ptr: *const T) -> *const T {
        ptr
    }

    #[cfg(not(feature = "multi_memory"))]
    fn memory_director_mut<T>(ptr: *mut T) -> *mut T {
        ptr
    }

    fn _main() -> wasip1::Errno {
        unreachable!();
    }

    fn _reset() {
        unreachable!();
    }

    fn _start() {
        unreachable!();
    }
}
