use std::collections::HashMap;

use eyre::{Context as _, ContextCompat};

use crate::{common::Wasip1SnapshotPreview1Func, util::ResultUtil as _};

pub fn is_valid_wasm_for_component(
    wasm_bytes: &[u8],
    wasm_names: &[impl AsRef<str>],
) -> eyre::Result<()> {
    let module = walrus::Module::from_buffer(wasm_bytes)
        .to_eyre()
        .wrap_err_with(|| eyre::eyre!("Failed to load module from buffer"))?;

    let import = module.imports;

    if import
        .iter()
        .filter(|import| {
            import.module == "wasi_snapshot_preview1"
                && matches!(import.kind, walrus::ImportKind::Function(_))
        })
        .map(|import| {
            let name = import.name.strip_prefix("__wasip1_vfs_")
                .context("This import is not a valid this library custom import.")?;
            wasm_names.iter().find_map(|n| {
                name.strip_prefix(n.as_ref())?.strip_prefix("_")?.parse().ok()
            })
            .context("Failed to parse wasm target and WASI function name")
        })
        .filter_map(|v| v.inspect_err(|e| {
            log::error!("Invalid import: {e}");
        }).ok())
        .map(|v| (Wasip1SnapshotPreview1Exporter::from_variant(&v).unwrap(), v))
        .fold(HashMap::<_, Vec<_>>::new(), |mut acc, (exporter, v)| {
            acc
                .entry(exporter)
                .or_default()
                .push(v);
            acc
        })
        .into_iter()
        .map(|(name, variants)| {
            // println!("Extra imports remain. You must use the `{name}!` macro exporter to export these functions.");
            log::error!(
                "Extra imports remain. You must use the `{name}!` macro exporter to export these functions: {}",
                variants.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ")
            );
        })
        .count()
        > 0
    {
        Err(eyre::eyre!(
            "Extra imports remain. This is not allowed in a component"
        ))
    } else {
        Ok(())
    }
}

#[derive(
    strum::EnumString, strum::EnumIter, Clone, Copy, Debug, PartialEq, Eq, strum::Display, Hash,
)]
#[strum(serialize_all = "snake_case")]
pub enum Wasip1SnapshotPreview1Exporter {
    ExportEnv,
    ExportFs,
}

use Wasip1SnapshotPreview1Func::*;
impl Wasip1SnapshotPreview1Exporter {
    const EXPORT_ENV: &'static [Wasip1SnapshotPreview1Func] = &[EnvironSizesGet, EnvironGet];
    const EXPORT_FS: &'static [Wasip1SnapshotPreview1Func] = &[
        FdWrite,
        FdReaddir,
        PathFilestatGet,
        PathOpen,
        FdClose,
        FdPrestatGet,
        FdPrestatDirName,
    ];

    pub const fn variants(self) -> &'static [Wasip1SnapshotPreview1Func] {
        match self {
            Wasip1SnapshotPreview1Exporter::ExportEnv => Self::EXPORT_ENV,
            Wasip1SnapshotPreview1Exporter::ExportFs => Self::EXPORT_FS,
        }
    }

    pub fn from_variant(
        func: &Wasip1SnapshotPreview1Func,
    ) -> Option<Wasip1SnapshotPreview1Exporter> {
        use strum::IntoEnumIterator;

        for exporter in Self::iter() {
            if exporter.variants().contains(func) {
                return Some(exporter);
            }
        }
        None
    }
}
