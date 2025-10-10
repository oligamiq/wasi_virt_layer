pub mod is_valid {
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
                .wrap_err("This import is not a valid this library custom import.")?;
            wasm_names.iter().find_map(|n| {
                name.strip_prefix(n.as_ref())?.strip_prefix("_")?.parse().ok()
            })
            .wrap_err("Failed to parse wasm target and WASI function name")
        })
        .filter_map(|v| v.inspect_err(|e| {
            log::error!("Invalid import: {e}");
        }).ok())
        .map(|v| (Wasip1ABIPlugger::from_variant(&v).unwrap(), v))
        .fold(HashMap::<_, Vec<_>>::new(), |mut acc, (plugger, v)| {
            acc
                .entry(plugger)
                .or_default()
                .push(v);
            acc
        })
        .into_iter()
        .map(|(name, variants)| {
            // println!("Extra imports remain. You must use the `{name}!` macro plugger to export these functions.");
            log::error!(
                "Extra imports remain. You must use the `{name}!` macro plugger to export these functions: {}{}",
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
    pub enum Wasip1ABIPlugger {
        PlugArgs,
        PlugEnv,
        #[strum(
            message = "Plug Fs is complex and difficult so you should see the documentation for more details."
        )]
        PlugFs,
        #[strum(message = "Plug Socks but this is not implemented")]
        PlugSocks,
        #[strum(message = "Plug Clock but this is not implemented")]
        PlugClock,
        #[strum(message = "Plug Random but this is not implemented")]
        PlugRandom,
        #[strum(message = "Plug Process is default so this message should not be shown")]
        PlugProcess,
        #[strum(message = "Plug Sched but this is not implemented")]
        PlugSched,
        #[strum(message = "Plug Poll but this is not implemented")]
        PlugPoll,
    }

    use std::collections::HashMap;

    use eyre::{Context as _, ContextCompat};
    use strum::EnumMessage as _;

    use crate::util::ResultUtil as _;

    use super::{Wasip1ABIFunc, Wasip1ABIFunc::*};
    impl Wasip1ABIPlugger {
        const PLUG_ENV: &'static [Wasip1ABIFunc] = &[EnvironSizesGet, EnvironGet];
        const PLUG_FS: &'static [Wasip1ABIFunc] = &[
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
        const PLUG_ARGS: &'static [Wasip1ABIFunc] = &[ArgsGet, ArgsSizesGet];
        const PLUG_SOCKS: &'static [Wasip1ABIFunc] =
            &[SockAccept, SockRecv, SockSend, SockShutdown];
        const PLUG_CLOCK: &'static [Wasip1ABIFunc] = &[ClockTimeGet, ClockResGet];
        const PLUG_RANDOM: &'static [Wasip1ABIFunc] = &[RandomGet];
        const PLUG_PROCESS: &'static [Wasip1ABIFunc] = &[ProcExit];
        const PLUG_SCHED: &'static [Wasip1ABIFunc] = &[SchedYield];
        const PLUG_POLL: &'static [Wasip1ABIFunc] = &[PollOneoff];

        pub const fn variants(self) -> &'static [Wasip1ABIFunc] {
            match self {
                Wasip1ABIPlugger::PlugEnv => Self::PLUG_ENV,
                Wasip1ABIPlugger::PlugFs => Self::PLUG_FS,
                Wasip1ABIPlugger::PlugArgs => Self::PLUG_ARGS,
                Wasip1ABIPlugger::PlugSocks => Self::PLUG_SOCKS,
                Wasip1ABIPlugger::PlugClock => Self::PLUG_CLOCK,
                Wasip1ABIPlugger::PlugRandom => Self::PLUG_RANDOM,
                Wasip1ABIPlugger::PlugProcess => Self::PLUG_PROCESS,
                Wasip1ABIPlugger::PlugSched => Self::PLUG_SCHED,
                Wasip1ABIPlugger::PlugPoll => Self::PLUG_POLL,
            }
        }

        pub fn from_variant(func: &Wasip1ABIFunc) -> Option<Wasip1ABIPlugger> {
            use strum::IntoEnumIterator;

            for plugger in Self::iter() {
                if plugger.variants().contains(func) {
                    return Some(plugger);
                }
            }
            None
        }
    }
}

#[derive(
    strum::EnumString, strum::VariantArray, strum::VariantNames, PartialEq, strum::Display,
)]
#[strum(serialize_all = "snake_case")]
pub enum Wasip1ABIFunc {
    EnvironSizesGet,
    EnvironGet,
    ProcExit,
    RandomGet,
    SchedYield,
    ClockTimeGet,
    ClockResGet,
    FdAdvise,
    FdAllocate,
    FdDatasync,
    FdFdstatSetFlags,
    FdFdstatSetRights,
    FdFdstatGet,
    FdWrite,
    FdPwrite,
    FdReaddir,
    FdClose,
    FdPrestatGet,
    FdPrestatDirName,
    FdFilestatGet,
    FdRead,
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
    PollOneoff,
    ArgsGet,
    ArgsSizesGet,
    SockAccept,
    SockRecv,
    SockSend,
    SockShutdown,
}

#[derive(
    strum::EnumString, strum::VariantArray, strum::VariantNames, PartialEq, strum::Display,
)]
#[strum(serialize_all = "snake_case")]
pub enum Wasip1ThreadsABIFunc {
    ThreadSpawn,
}
