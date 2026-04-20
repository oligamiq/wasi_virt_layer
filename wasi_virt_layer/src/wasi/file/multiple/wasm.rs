#![cfg(feature = "multiple-fs")]

use crate::{
    memory::{WasmAccessDynCompatibleRaw, WasmAccessNameDynCompatible},
};

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

impl WasmAccessDynCompatibleWrapper {
    /// Creates a new `WasmAccessDynCompatibleWrapper` from a given access tuple.
    pub fn new<T: WasmAccessDynCompatibleTuple + 'static>(access: T) -> Self {
        Self(alloc::boxed::Box::new(access))
    }
}
