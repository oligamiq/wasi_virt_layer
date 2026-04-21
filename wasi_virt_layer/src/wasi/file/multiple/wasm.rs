#![cfg(feature = "multiple-fs")]

use crate::memory::{WasmAccessDynCompatibleRaw, WasmAccessNameDynCompatible};
use crate::__private::wasip1;

/// A trait tuple combining `WasmAccessNameDynCompatible` and `WasmAccessDynCompatibleRaw`.
pub trait WasmAccessDynCompatibleTuple:
    WasmAccessNameDynCompatible + WasmAccessDynCompatibleRaw
{
}

impl<T: WasmAccessNameDynCompatible + WasmAccessDynCompatibleRaw> WasmAccessDynCompatibleTuple
    for T
{
}

/// A wrapper around a dynamic `WasmAccessDynCompatibleTuple`.
#[derive(Debug)]
pub struct WasmAccessDynCompatibleWrapper(pub alloc::boxed::Box<dyn WasmAccessDynCompatibleTuple>);

unsafe impl Send for WasmAccessDynCompatibleWrapper {}
unsafe impl Sync for WasmAccessDynCompatibleWrapper {}

impl AsRef<dyn WasmAccessNameDynCompatible> for WasmAccessDynCompatibleWrapper {
    fn as_ref(&self) -> &(dyn WasmAccessNameDynCompatible + 'static) {
        self.0.as_ref()
    }
}

impl AsRef<dyn WasmAccessDynCompatibleRaw> for WasmAccessDynCompatibleWrapper {
    fn as_ref(&self) -> &(dyn WasmAccessDynCompatibleRaw + 'static) {
        self.0.as_ref()
    }
}

impl WasmAccessDynCompatibleRaw for WasmAccessDynCompatibleWrapper {
    #[inline(always)]
    fn memcpy_raw(&self, offset: *mut u8, src: *const u8, len: usize) {
        self.0.memcpy_raw(offset, src, len)
    }

    #[inline(always)]
    fn memcpy_to_raw(&self, offset: *mut u8, src: *const u8, len: usize) {
        self.0.memcpy_to_raw(offset, src, len)
    }

    #[cfg(not(feature = "multi_memory"))]
    #[inline(always)]
    fn memory_director_raw(&self, ptr: isize) -> Option<isize> {
        self.0.memory_director_raw(ptr)
    }

    #[inline(always)]
    fn _main_raw(&self) -> wasip1::Errno {
        self.0._main_raw()
    }

    #[inline(always)]
    fn _reset_raw(&self) {
        self.0._reset_raw()
    }

    #[inline(always)]
    fn _start_raw(&self) {
        self.0._start_raw()
    }
}

impl WasmAccessNameDynCompatible for WasmAccessDynCompatibleWrapper {
    fn with_name(&self, f: &mut dyn FnMut(&str)) {
        self.0.with_name(f)
    }
}

impl WasmAccessDynCompatibleWrapper {
    /// Creates a new `WasmAccessDynCompatibleWrapper` from a given access tuple.
    pub fn new<T: WasmAccessDynCompatibleTuple + 'static>(access: T) -> Self {
        Self(alloc::boxed::Box::new(access))
    }
}
