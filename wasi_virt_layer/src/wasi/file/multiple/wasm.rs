use core::ops::Deref as _;

use alloc::collections::BTreeMap;

use smallvec::SmallVec;

use crate::{
    __private::wasip1::{self, *},
    file::FilestatWithoutDevice,
    memory::{
        WasmAccess, WasmAccessDynCompatible, WasmAccessDynCompatibleRaw,
        WasmAccessNameDynCompatible,
    },
};

pub trait WasmAccessDynCompatibleTuple:
    WasmAccessNameDynCompatible + WasmAccessDynCompatibleRaw
{
}

impl<T: WasmAccessNameDynCompatible + WasmAccessDynCompatibleRaw> WasmAccessDynCompatibleTuple
    for T
{
}

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
    pub fn new<T: WasmAccessDynCompatibleTuple + 'static>(access: T) -> Self {
        Self(alloc::boxed::Box::new(access))
    }
}
