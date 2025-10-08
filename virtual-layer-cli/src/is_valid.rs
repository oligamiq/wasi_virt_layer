use std::collections::HashMap;

use eyre::{Context as _, ContextCompat};
use strum::EnumMessage;

use crate::{common::Wasip1ABIFunc, util::ResultUtil as _};

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
                "Extra imports remain. You must use the `{name}!` macro exporter to export these functions: {}{}",
                variants.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "),
                format!("\nExtra message: {}", name.get_message().unwrap_or(""))
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
    strum::EnumString,
    strum::EnumIter,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    strum::Display,
    Hash,
    strum::EnumMessage,
)]
#[strum(serialize_all = "snake_case")]
pub enum Wasip1SnapshotPreview1Exporter {
    ExportArgs,
    ExportEnv,
    #[strum(
        message = "Export Fs is complex and difficult so you should see the documentation for more details."
    )]
    ExportFs,
    #[strum(message = "Export Socks but this is not implemented")]
    ExportSocks,
    #[strum(message = "Export Clock but this is not implemented")]
    ExportClock,
    #[strum(message = "Export Random but this is not implemented")]
    ExportRandom,
    #[strum(message = "Export Process is default so this message should not be shown")]
    ExportProcess,
    #[strum(message = "Export Sched but this is not implemented")]
    ExportSched,
    #[strum(message = "Export Poll but this is not implemented")]
    ExportPoll,
}

use Wasip1ABIFunc::*;
impl Wasip1SnapshotPreview1Exporter {
    const EXPORT_ENV: &'static [Wasip1ABIFunc] = &[EnvironSizesGet, EnvironGet];
    const EXPORT_FS: &'static [Wasip1ABIFunc] = &[
        FdAdvise,
        FdAllocate,
        FdDatasync,
        FdFdstatSetFlags,
        FdFdstatSetRights,
        FdWrite,
        FdPwrite,
        FdReaddir,
        FdClose,
        FdPrestatGet,
        FdPrestatDirName,
        FdFilestatGet,
        FdRead,
        FdFdstatGet,
        FdPread,
        FdFilestatSetSize,
        FdFilestatSetTimes,
        FdRenumber,
        FdSeek,
        FdSync,
        FdTell,
        PathCreateDirectory,
        PathFilestatGet,
        PathFilestatSetTimes,
        PathLink,
        PathReadlink,
        PathRemoveDirectory,
        PathRename,
        PathOpen,
        PathSymlink,
        PathUnlinkFile,
    ];
    const EXPORT_ARGS: &'static [Wasip1ABIFunc] = &[ArgsGet, ArgsSizesGet];
    const EXPORT_SOCKS: &'static [Wasip1ABIFunc] =
        &[SockAccept, SockRecv, SockSend, SockShutdown];
    const EXPORT_CLOCK: &'static [Wasip1ABIFunc] = &[ClockTimeGet, ClockResGet];
    const EXPORT_RANDOM: &'static [Wasip1ABIFunc] = &[RandomGet];
    const EXPORT_PROCESS: &'static [Wasip1ABIFunc] = &[ProcExit];
    const EXPORT_SCHED: &'static [Wasip1ABIFunc] = &[SchedYield];
    const EXPORT_POLL: &'static [Wasip1ABIFunc] = &[PollOneoff];

    pub const fn variants(self) -> &'static [Wasip1ABIFunc] {
        match self {
            Wasip1SnapshotPreview1Exporter::ExportEnv => Self::EXPORT_ENV,
            Wasip1SnapshotPreview1Exporter::ExportFs => Self::EXPORT_FS,
            Wasip1SnapshotPreview1Exporter::ExportArgs => Self::EXPORT_ARGS,
            Wasip1SnapshotPreview1Exporter::ExportSocks => Self::EXPORT_SOCKS,
            Wasip1SnapshotPreview1Exporter::ExportClock => Self::EXPORT_CLOCK,
            Wasip1SnapshotPreview1Exporter::ExportRandom => Self::EXPORT_RANDOM,
            Wasip1SnapshotPreview1Exporter::ExportProcess => Self::EXPORT_PROCESS,
            Wasip1SnapshotPreview1Exporter::ExportSched => Self::EXPORT_SCHED,
            Wasip1SnapshotPreview1Exporter::ExportPoll => Self::EXPORT_POLL,
        }
    }

    pub fn from_variant(
        func: &Wasip1ABIFunc,
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
