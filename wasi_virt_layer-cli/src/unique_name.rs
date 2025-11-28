use crate::{generator::start_section::StartAlternative, util::WasmName};

pub enum UniqueName<'a> {
    EachReset(&'a WasmName),
    StartAlternative(&'a StartAlternative),
}

impl UniqueName<'_> {
    fn to_str(&self) -> String {
        match self {
            UniqueName::EachReset(name) => format!("__wasip1_vfs_{name}_reset"),
            UniqueName::StartAlternative(alt) => {
                let alt_name = alt.as_ref();
                match alt {
                    StartAlternative::WasmName(name) | StartAlternative::VFS(name) => {
                        format!("__wasip1_vfs_start_alt_{alt_name}_{name}")
                    }
                    _ => format!("__wasip1_vfs_start_alt_{alt_name}"),
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
