use crate::{
    generator::{
        abi_connect::Wasip1ABI, shared_global::SharedGlobalFns, start_section::StartAlternative,
    },
    util::WasmName,
};

pub enum UniqueName<'a> {
    EachReset(&'a WasmName),
    StartAlternative(&'a StartAlternative),
    SharedGlobalFns(&'a SharedGlobalFns),
    Wasip1ABI(&'a Wasip1ABI<'a>),
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

impl UniqueName<'_> {
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
    /// todo!(); to unique names
    pub const EACH_RESET: &'static str = "";
    /// todo!(); to unique names
    pub const WASIP1_ABI: &'static str = "";

    fn to_str(&self) -> String {
        match self {
            UniqueName::EachReset(name) => fmt!(EachReset; "{name}_reset"),
            UniqueName::StartAlternative(alt) => {
                let alt_name = alt.as_ref();
                match alt {
                    StartAlternative::WasmName(name) | StartAlternative::VFS(name) => {
                        fmt!(StartAlternative; "{alt_name}_{name}")
                    }
                    _ => fmt!(StartAlternative; "{alt_name}"),
                }
            }
            UniqueName::SharedGlobalFns(func) => {
                let func_name = func.as_ref();
                match func {
                    SharedGlobalFns::Locker(n) => {
                        fmt!(SharedGlobalFns; "{func_name}_{n}")
                    }
                    _ => fmt!(SharedGlobalFns; "{func_name}"),
                }
            }
            UniqueName::Wasip1ABI(t) => {
                let name = t.as_ref();
                match t {
                    Wasip1ABI::SelfDefault { import } => fmt!(Wasip1ABI; "{name}_{import}"),
                    Wasip1ABI::TargetTemporal { wasm, import } => {
                        fmt!(Wasip1ABI; "{wasm}_{import}")
                    }
                    Wasip1ABI::WasiThreadStart(wasm) | Wasip1ABI::WasiThreadStartAnchor(wasm) => {
                        fmt!(Wasip1ABI; "{wasm}_{name}")
                    }
                    Wasip1ABI::WasiThreadStartDestination(wasm)
                    | Wasip1ABI::WasiThreadSpawn(wasm) => {
                        fmt!(Wasip1ABI; "{name}_{wasm}")
                    }
                }
            }
        }
    }
}

pub trait UniqueNameMarker: ToString + Copy {}

impl UniqueNameMarker for &'_ UniqueName<'_> {}

impl ToString for &'_ UniqueName<'_> {
    fn to_string(&self) -> String {
        self.to_str()
    }
}

impl ToString for UniqueName<'_> {
    fn to_string(&self) -> String {
        self.to_str()
    }
}
