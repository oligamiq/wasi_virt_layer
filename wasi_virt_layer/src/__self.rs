use crate::__private::wasip1;
use crate::memory::{WasmAccessDynCompatibleRaw, WasmAccessName, WasmAccessRaw};

/// Internal struct representing the host context itself for WasmAccess.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct __self;

impl WasmAccessName for __self {
    const NAME: &'static str = "__self";
}

impl WasmAccessDynCompatibleRaw for __self {
    fn memcpy_raw(&self, offset: *mut u8, src: *const u8, len: usize) {
        unsafe {
            core::ptr::copy_nonoverlapping(src, offset, len);
        }
    }

    fn memcpy_to_raw(&self, offset: *mut u8, src: *const u8, len: usize) {
        unsafe { core::ptr::copy_nonoverlapping(src, offset, len) };
    }

    #[cfg(not(feature = "multi_memory"))]
    fn memory_director_raw(&self, ptr: isize) -> isize {
        ptr
    }

    fn _main_raw(&self) -> wasip1::Errno {
        unreachable!();
    }

    fn _reset_raw(&self) {
        unreachable!();
    }

    fn _start_raw(&self) {
        unreachable!();
    }
}

impl WasmAccessRaw for __self {
    fn memcpy_raw(offset: *mut u8, src: *const u8, len: usize) {
        unsafe {
            core::ptr::copy_nonoverlapping(src, offset, len);
        }
    }

    fn memcpy_to_raw(offset: *mut u8, src: *const u8, len: usize) {
        unsafe { core::ptr::copy_nonoverlapping(src, offset, len) };
    }

    fn _main_raw() -> wasip1::Errno {
        unreachable!();
    }

    fn _reset_raw() {
        unreachable!();
    }

    fn _start_raw() {
        unreachable!();
    }

    #[cfg(not(feature = "multi_memory"))]
    fn memory_director_raw(ptr: isize) -> isize {
        ptr
    }
}
