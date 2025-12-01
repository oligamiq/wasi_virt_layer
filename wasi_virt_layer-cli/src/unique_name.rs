use crate::{
    generator::{
        abi_connect::Wasip1ABIName, shared_global::SharedGlobalFnsName,
        special_func::SpecialFuncUniqueName, start_section::StartAlternativeName,
        threads::ThreadsSpawnName,
    },
    util::WasmName,
};

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum UniqueName<'a, 'b> {
    EachReset(&'a WasmName),
    StartAlternative(&'a StartAlternativeName),
    SharedGlobalFns(&'a SharedGlobalFnsName),
    Wasip1ABI(&'a Wasip1ABIName<'b>),
    ThreadsSpawn(&'a ThreadsSpawnName<'b>),
    SpecialFunc(&'a SpecialFuncUniqueName<'b>),
}

macro_rules! fmt {
    (
        Wasip1ABI; $($arg:tt)*
    ) => {
        paste::paste! {
            {
                format!(
                    "__wasip1_vfs_{}{}",
                    { $crate::unique_name::UniqueName::WASIP1_ABI },
                    format!($($arg)*)
                )
            }
        }
    };

    (
        $_ty:ty; $($arg:tt)*
    ) => {
        paste::paste! {
            {
                format!(
                    "__wasip1_vfs_{}{}",
                    { $crate::unique_name::UniqueName:: [<$_ty:snake:upper>] },
                    format!($($arg)*)
                )
            }
        }
    };

    (
        $($arg:tt)*
    ) => {
        { format!("__wasip1_vfs_{}", format!($($arg)*)) }
    };
}
pub(crate) use fmt;
use strum::EnumCount;

#[cfg(test)]
mod tests {
    #[test]
    fn test_unique_name_to_str() {
        let r = "abc";
        let fmt = super::fmt!("{r}def");
        assert_eq!(fmt, format!("{}abcdef", super::UniqueName::PREFIX));
    }

    #[test]
    fn test_unique_name_start_alternative() {
        let fmt = super::fmt!(StartAlternative; "suffix");
        assert_eq!(
            fmt,
            format!("{}start_alt_suffix", super::UniqueName::PREFIX)
        );
    }
}

impl UniqueName<'_, '_> {
    pub const CORE_MODULE_ROOT: &'static str = "wasip1-vfs:host/virtual-file-system-wasip1-core";
    pub const THREADS_MODULE_ROOT: &'static str =
        "wasip1-vfs:host/virtual-file-system-wasip1-threads-import";
    pub const THREADS_EXPORT_MODULE_ROOT: &'static str =
        "wasip1-vfs:host/virtual-file-system-wasip1-threads-export#wasi-thread-start";
    pub const CORE_NON_RECURSIVE_MODULE_ROOT: &'static str = "non_recursive_wasi_snapshot_preview1";
    pub const NAMESPACE: &'static str = "wasip1-vfs";
    pub const CRATE_NAME: &'static str = "wasi_virt_layer";
    pub const WASIP1_ABI_MODULE: &'static str = "wasi_snapshot_preview1";
    pub const WASIP1_ABI_MODULE_ALT: &'static str = "wasip1";
    pub const WASIP1_THREADS_ABI_MODULE: &'static str = "wasi";
    pub const WASIP1_THREADS_ABI_MODULE_ALT: &'static str = "wasip1-threads";

    pub const PREFIX: &'static str = "__wasip1_vfs_";

    pub const START_ALTERNATIVE: &'static str = "start_alt_";
    pub const SHARED_GLOBAL_FNS: &'static str = "memory_grow_";
    pub const SPECIAL_FUNC: &'static str = "";
    /// todo!(); to unique names
    pub const EACH_RESET: &'static str = "";
    /// todo!(); to unique names
    pub const WASIP1_ABI: &'static str = "";
    /// todo!(); to unique names
    pub const THREADS_SPAWN: &'static str = "";

    fn to_str(&self) -> String {
        match self {
            UniqueName::EachReset(name) => fmt!(EachReset; "{name}_reset"),
            UniqueName::StartAlternative(alt) => {
                let alt_name = alt.as_ref();
                match alt {
                    StartAlternativeName::WasmName(name) | StartAlternativeName::VFS(name) => {
                        fmt!(StartAlternative; "{alt_name}_{name}")
                    }
                    _ => fmt!(StartAlternative; "{alt_name}"),
                }
            }
            UniqueName::SharedGlobalFns(func) => {
                let func_name = func.as_ref();
                match func {
                    SharedGlobalFnsName::Locker(n) => {
                        fmt!(SharedGlobalFns; "{func_name}_{n}")
                    }
                    _ => fmt!(SharedGlobalFns; "{func_name}"),
                }
            }
            UniqueName::Wasip1ABI(t) => {
                let name = t.as_ref();
                match t {
                    Wasip1ABIName::SelfDefault { import } => fmt!(Wasip1ABI; "{name}_{import}"),
                    Wasip1ABIName::TargetTemporal { wasm, import } => {
                        fmt!(Wasip1ABI; "{wasm}_{import}")
                    }
                }
            }
            UniqueName::ThreadsSpawn(t) => {
                let name = t.as_ref();
                match t {
                    ThreadsSpawnName::ImportAnchor(wasm) => {
                        // todo!(); use unique name
                        format!("{wasm}_{name}")
                    }
                    ThreadsSpawnName::WasiThreadStart(wasm)
                    | ThreadsSpawnName::WasiThreadStartAnchor(wasm) => {
                        fmt!(ThreadsSpawn; "{wasm}_{name}")
                    }
                    ThreadsSpawnName::WasiThreadStartDestination(wasm)
                    | ThreadsSpawnName::WasiThreadSpawn(wasm) => {
                        fmt!(ThreadsSpawn; "{name}_{wasm}")
                    }
                    _ => {
                        fmt!(ThreadsSpawn; "{name}")
                    }
                }
            }
            UniqueName::SpecialFunc(t) => {
                let name = t.as_ref();
                match t {
                    SpecialFuncUniqueName::Resetter(wasm) => {
                        fmt!(SpecialFunc; "{wasm}_{name}")
                    }
                    SpecialFuncUniqueName::Start(wasm) => {
                        fmt!(SpecialFunc; "{wasm}__{name}")
                    }
                    SpecialFuncUniqueName::MainVoid(wasm) => {
                        fmt!(SpecialFunc; "{wasm}___{name}")
                    }
                    _ => {
                        fmt!(SpecialFunc; "{name}")
                    }
                }
            }
        }
    }
}

pub trait UniqueNameMarker: ToString + Copy {}

impl UniqueNameMarker for &'_ UniqueName<'_, '_> {}

impl ToString for &'_ UniqueName<'_, '_> {
    fn to_string(&self) -> String {
        self.to_str()
    }
}

impl ToString for UniqueName<'_, '_> {
    fn to_string(&self) -> String {
        self.to_str()
    }
}

impl<'a> From<&'a StartAlternativeName> for UniqueName<'_, 'a> {
    fn from(value: &'a StartAlternativeName) -> Self {
        UniqueName::StartAlternative(value)
    }
}

impl<'a> From<&'a SharedGlobalFnsName> for UniqueName<'_, 'a> {
    fn from(value: &'a SharedGlobalFnsName) -> Self {
        UniqueName::SharedGlobalFns(value)
    }
}

impl<'a> From<&'a Wasip1ABIName<'_>> for UniqueName<'_, 'a> {
    fn from(value: &'a Wasip1ABIName<'_>) -> Self {
        UniqueName::Wasip1ABI(value)
    }
}

impl<'a> From<&'a ThreadsSpawnName<'_>> for UniqueName<'_, 'a> {
    fn from(value: &'a ThreadsSpawnName<'_>) -> Self {
        UniqueName::ThreadsSpawn(value)
    }
}

impl<'a> From<&'a SpecialFuncUniqueName<'_>> for UniqueName<'_, 'a> {
    fn from(value: &'a SpecialFuncUniqueName<'_>) -> Self {
        UniqueName::SpecialFunc(value)
    }
}

/// To verify whether an identical entry exists
/// in the destination for generating UniqueName, prepare an iterator.
trait UniqueNameIterator<'a>
where
    Self: Sized,
{
    type REQUIRED;

    fn iter_unique_names(require: &'a Self::REQUIRED) -> Vec<Self>;
}

impl<'a> UniqueNameIterator<'a> for WasmName {
    type REQUIRED = WasmName;

    fn iter_unique_names(require: &'a Self::REQUIRED) -> Vec<Self> {
        let v = vec![require.clone()];
        assert_eq!(v.len(), 1);
        v
    }
}

impl<'a> UniqueNameIterator<'a> for StartAlternativeName {
    type REQUIRED = WasmName;

    fn iter_unique_names(require: &'a Self::REQUIRED) -> Vec<Self> {
        let v = vec![
            StartAlternativeName::WasmName(require.clone()),
            StartAlternativeName::VFS(require.clone()),
            StartAlternativeName::AfterMemoryReset,
        ];
        assert_eq!(v.len(), StartAlternativeName::COUNT);
        v
    }
}

impl<'a> UniqueNameIterator<'a> for SharedGlobalFnsName {
    type REQUIRED = usize;

    fn iter_unique_names(require: &'a Self::REQUIRED) -> Vec<Self> {
        let v = vec![
            SharedGlobalFnsName::GlobalAltSet,
            SharedGlobalFnsName::GlobalAltGet,
            SharedGlobalFnsName::GlobalAltGetNoWait,
            SharedGlobalFnsName::GlobalAltInitOnce,
            SharedGlobalFnsName::GlobalAltPos,
            SharedGlobalFnsName::Locker(*require),
        ];
        assert_eq!(v.len(), SharedGlobalFnsName::COUNT);
        v
    }
}

impl<'a> UniqueNameIterator<'a> for Wasip1ABIName<'a> {
    type REQUIRED = (WasmName, &'a str);

    fn iter_unique_names(require: &'a Self::REQUIRED) -> Vec<Self> {
        let (wasm, import) = require;
        let v = vec![
            Wasip1ABIName::SelfDefault { import },
            Wasip1ABIName::TargetTemporal { import, wasm },
        ];
        assert_eq!(v.len(), Wasip1ABIName::COUNT);
        v
    }
}

impl<'a> UniqueNameIterator<'a> for ThreadsSpawnName<'a> {
    type REQUIRED = (WasmName, &'a str);

    fn iter_unique_names(require: &'a Self::REQUIRED) -> Vec<Self> {
        let (wasm, import) = require;
        let v = vec![
            ThreadsSpawnName::ImportAnchor(import),
            ThreadsSpawnName::IsRootSpawn,
            ThreadsSpawnName::WasiThreadSpawnSelf,
            ThreadsSpawnName::SelfWasiThreadStart,
            ThreadsSpawnName::SelfWasiThreadStartAnchor,
            ThreadsSpawnName::RealThreadSpawnFn,
            ThreadsSpawnName::WasiThreadStartEntry,
            ThreadsSpawnName::WasiThreadSpawn(wasm),
            ThreadsSpawnName::WasiThreadStart(wasm),
            ThreadsSpawnName::WasiThreadStartAnchor(wasm),
            ThreadsSpawnName::WasiThreadStartDestination(wasm),
        ];
        assert_eq!(v.len(), ThreadsSpawnName::COUNT);
        v
    }
}

impl<'a> UniqueNameIterator<'a> for SpecialFuncUniqueName<'a> {
    type REQUIRED = WasmName;

    fn iter_unique_names(require: &'a Self::REQUIRED) -> Vec<Self> {
        let v = vec![
            SpecialFuncUniqueName::Resetter(require),
            SpecialFuncUniqueName::ResetOnThread,
            SpecialFuncUniqueName::ResetOnThreadOnce,
            SpecialFuncUniqueName::StartInitOld,
            SpecialFuncUniqueName::Start(require),
            SpecialFuncUniqueName::MainVoid(require),
        ];
        assert_eq!(v.len(), SpecialFuncUniqueName::COUNT);
        v
    }
}

#[cfg(test)]
mod unique_name_iterator_tests {
    use std::collections::HashSet;

    use crate::util::WasmNameHolder;

    use super::*;

    #[test]
    fn test_unique_name_iterator_start_alternative() {
        let holder = WasmNameHolder::new(vec!["#original_import".into()].into_boxed_slice());
        let require_import = holder.iter().next().unwrap();
        let require_name = "#original_name";
        let require_num = 5;
        let requires = (require_import.clone(), require_name);
        let t1 = WasmName::iter_unique_names(&require_import);
        let t2 = StartAlternativeName::iter_unique_names(&require_import);
        let t3 = SharedGlobalFnsName::iter_unique_names(&require_num);
        let t4 = Wasip1ABIName::iter_unique_names(&requires);
        let t5 = ThreadsSpawnName::iter_unique_names(&requires);
        let t6 = SpecialFuncUniqueName::iter_unique_names(&require_import);
        let t1 = t1.iter().map(UniqueName::EachReset);
        let t2 = t2.iter().map(Into::into);
        let t3 = t3.iter().map(Into::into);
        let t4 = t4.iter().map(Into::into);
        let t5 = t5.iter().map(Into::into);
        let t6 = t6.iter().map(Into::into);
        let t = t1
            .chain(t2)
            .chain(t3)
            .chain(t4)
            .chain(t5)
            .chain(t6)
            .collect::<Vec<UniqueName>>();

        // Check whether there is the same output destination
        let mut seen = std::collections::HashMap::new();
        let mut duplicates = HashSet::new();
        for t in &t {
            let name = t.to_string();
            if seen.insert(name.clone(), t).is_some() {
                duplicates.insert(t);
                duplicates.insert(seen.get(&name).unwrap());
            }
        }
        assert!(
            duplicates.is_empty(),
            "Found duplicate unique names: {duplicates:?}",
        );
    }
}
