#![cfg(feature = "multiple-fs")]

use crate::__private::wasip1;
use crate::memory::{WasmAccessDynCompatibleRaw, WasmAccessNameDynCompatible};
use smallbox::SmallBox;

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
pub struct WasmAccessDynCompatibleWrapper(
    pub SmallBox<dyn WasmAccessDynCompatibleTuple, [usize; 24]>,
);

unsafe impl Send for WasmAccessDynCompatibleWrapper {}
unsafe impl Sync for WasmAccessDynCompatibleWrapper {}

impl AsRef<dyn WasmAccessNameDynCompatible> for WasmAccessDynCompatibleWrapper {
    fn as_ref(&self) -> &(dyn WasmAccessNameDynCompatible + 'static) {
        &*self.0
    }
}

impl AsRef<dyn WasmAccessDynCompatibleRaw> for WasmAccessDynCompatibleWrapper {
    fn as_ref(&self) -> &(dyn WasmAccessDynCompatibleRaw + 'static) {
        &*self.0
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
        Self(smallbox::smallbox!(access))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__self::__self;
    use crate::wasi::file::multiple::dynamic_wasm::StandardPseudoWasmHolder;

    #[test]
    fn test_wasm_access_wrapper_allocation() {
        // Test ZST (__self)
        let wrapper_self = WasmAccessDynCompatibleWrapper::new(__self);
        assert!(!wrapper_self.0.is_heap(), "__self should fit in stack");

        // Test StandardPseudoWasmHolder (The largest common implementation)
        let holder = StandardPseudoWasmHolder::new_const();
        let wrapper_holder = WasmAccessDynCompatibleWrapper::new(holder);

        // This should fit in [usize; 24]
        assert!(
            !wrapper_holder.0.is_heap(),
            "StandardPseudoWasmHolder should fit in stack"
        );
    }

    #[test]
    fn test_size_expectations() {
        // Ensure our stack size is reasonable
        // [usize; 24] is 96 bytes on 32-bit, 192 bytes on 64-bit
        let stack_size = core::mem::size_of::<[usize; 24]>();
        let holder_size = core::mem::size_of::<StandardPseudoWasmHolder>();

        assert!(
            holder_size <= stack_size,
            "Holder size {} should be <= stack size {}",
            holder_size,
            stack_size
        );
    }
}
