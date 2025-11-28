use crate::{
    generator::{shared_global::SharedGlobalFns, start_section::StartAlternative},
    util::WasmName,
};

pub enum UniqueName<'a> {
    EachReset(&'a WasmName),
    StartAlternative(&'a StartAlternative),
    SharedGlobalFns(&'a SharedGlobalFns),
}

macro_rules! fmt {
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
    pub const PREFIX: &'static str = "__wasip1_vfs_";
    pub const START_ALTERNATIVE: &'static str = "start_alt_";
    pub const SHARED_GLOBAL_FNS: &'static str = "memory_grow_";
    /// todo!(); to unique names
    pub const EACH_RESET: &'static str = "";

    fn to_str(&self) -> String {
        let s = match self {
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
        };
        println!("UniqueName to_str: {:?}", s);
        s
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
