/// Module for validating WASM modules against the expected WASI ABI.
pub mod is_valid {


    /// Validates if unresolved WASI imports remain, ensuring no un-plugged custom WASIP1 imports are dropped silently.
    pub fn validate_unresolved_imports(
        unresolved_imports: &[&str],
        wasm_names: &[impl AsRef<str>],
    ) -> eyre::Result<()> {
        let mut err_wasm_names = HashMap::<_, Vec<_>>::new();

        for i_name in unresolved_imports {
            let name_str = i_name.strip_prefix("__wasip1_vfs_");
            if let Some(name) = name_str {
                if let Some(func_name) = name.strip_prefix("__self_") {
                    if let Ok(func) = func_name.parse::<super::Wasip1ABIFunc>() {
                        let plugger = Wasip1ABIPlugger::from_variant(&func).unwrap();
                        err_wasm_names
                            .entry(("__self".to_string(), plugger))
                            .or_default()
                            .push(func.to_string());
                        continue;
                    }
                }
                if let Some((wasm_name, plugger, func_name)) =
                    wasm_names.iter().find_map(|n| {
                        let func_name =
                            name.strip_prefix(n.as_ref())?.strip_prefix("_")?;
                        if func_name == "thread_spawn" {
                            return Some((
                                n.as_ref().to_string(),
                                Wasip1ABIPlugger::PlugThread,
                                "thread_spawn".to_string(),
                            ));
                        }
                        let func: super::Wasip1ABIFunc = func_name.parse().ok()?;
                        Some((
                            n.as_ref().to_string(),
                            Wasip1ABIPlugger::from_variant(&func).unwrap(),
                            func.to_string(),
                        ))
                    })
                {
                    err_wasm_names
                        .entry((wasm_name, plugger))
                        .or_default()
                        .push(func_name);
                } else {
                    return Err(eyre::eyre!(
                        "Invalid import: Failed to parse wasm target and WASI function name. \
                         This import is not a valid custom import or the function name is malformed: {}",
                        i_name
                    ));
                }
            } else {
                if let Ok(func) = i_name.parse::<super::Wasip1ABIFunc>() {
                    let plugger = Wasip1ABIPlugger::from_variant(&func).unwrap();
                    err_wasm_names
                        .entry(("__self".to_string(), plugger))
                        .or_default()
                        .push(func.to_string());
                } else {
                    return Err(eyre::eyre!(
                        "This import is not a valid this library custom import: {}",
                        i_name
                    ));
                }
            }
        }

        let err_wasm_names = err_wasm_names
        .into_iter()
        .map(|((wasm_name, plugger), variants)| {
            log::error!(
                "Extra imports remain for `{wasm_name}`. You must use the `{plugger}!` macro plugger to export these functions: {}{}",
                variants.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "),
                format!("\n    Extra message: {}", plugger.get_message().unwrap_or(""))
            );
            wasm_name
        })
        .collect::<std::collections::HashSet<_>>();

        if !err_wasm_names.is_empty() {
            let mut names = err_wasm_names.into_iter().collect::<Vec<_>>();
            names.sort();
            Err(eyre::eyre!(
                "Extra imports remain for `{names}`. This is not allowed in a component",
                names = names.join(", ")
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
    /// Identifies specific WASI feature sets that need to be plugged during virtualization.
    pub enum Wasip1ABIPlugger {
        /// Plugins for command-line arguments.
        PlugArgs,
        /// Plugins for environment variables.
        PlugEnv,
        #[strum(
            message = "Plug Fs is complex and difficult so you should see the documentation for more details."
        )]
        /// Plugins for file system operations.
        PlugFs,
        #[strum(message = "Plug Socks but this is not implemented")]
        /// Plugins for socket operations (currently unimplemented).
        PlugSocks,
        /// Plugins for clock and timing operations.
        PlugClock,
        /// Plugins for random number generation.
        PlugRandom,
        #[strum(message = "Plug Process is default so this message should not be shown")]
        /// Plugins for process-related operations.
        PlugProcess,
        /// Plugins for scheduler operations.
        PlugSched,
        /// Plugins for polling operations.
        PlugPoll,
        /// Plugins for thread operations.
        PlugThread,
    }

    use std::collections::HashMap;

    use strum::EnumMessage as _;

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

        /// Returns the list of `Wasip1ABIFunc` variants associated with this plugger.
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
                Wasip1ABIPlugger::PlugThread => &[],
            }
        }

        /// Identifies the appropriate `Wasip1ABIPlugger` for a specific WASI function.
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
/// Enumeration of all supported WASI preview1 functions.
pub enum Wasip1ABIFunc {
    /// environ_sizes_get
    EnvironSizesGet,
    /// environ_get
    EnvironGet,
    /// proc_exit
    ProcExit,
    /// random_get
    RandomGet,
    /// sched_yield
    SchedYield,
    /// clock_time_get
    ClockTimeGet,
    /// clock_res_get
    ClockResGet,
    /// fd_advise
    FdAdvise,
    /// fd_allocate
    FdAllocate,
    /// fd_datasync
    FdDatasync,
    /// fd_fdstat_set_flags
    FdFdstatSetFlags,
    /// fd_fdstat_set_rights
    FdFdstatSetRights,
    /// fd_fdstat_get
    FdFdstatGet,
    /// fd_write
    FdWrite,
    /// fd_pwrite
    FdPwrite,
    /// fd_readdir
    FdReaddir,
    /// fd_close
    FdClose,
    /// fd_prestat_get
    FdPrestatGet,
    /// fd_prestat_dir_name
    FdPrestatDirName,
    /// fd_filestat_get
    FdFilestatGet,
    /// fd_read
    FdRead,
    /// fd_pread
    FdPread,
    /// fd_filestat_set_size
    FdFilestatSetSize,
    /// fd_filestat_set_times
    FdFilestatSetTimes,
    /// fd_renumber
    FdRenumber,
    /// fd_seek
    FdSeek,
    /// fd_sync
    FdSync,
    /// fd_tell
    FdTell,
    /// path_create_directory
    PathCreateDirectory,
    /// path_filestat_get
    PathFilestatGet,
    /// path_filestat_set_times
    PathFilestatSetTimes,
    /// path_link
    PathLink,
    /// path_readlink
    PathReadlink,
    /// path_remove_directory
    PathRemoveDirectory,
    /// path_rename
    PathRename,
    /// path_open
    PathOpen,
    /// path_symlink
    PathSymlink,
    /// path_unlink_file
    PathUnlinkFile,
    /// poll_oneoff
    PollOneoff,
    /// args_get
    ArgsGet,
    /// args_sizes_get
    ArgsSizesGet,
    /// sock_accept
    SockAccept,
    /// sock_recv
    SockRecv,
    /// sock_send
    SockSend,
    /// sock_shutdown
    SockShutdown,
}

#[derive(
    strum::EnumString, strum::VariantArray, strum::VariantNames, PartialEq, strum::Display,
)]
#[strum(serialize_all = "kebab_case")]
/// Enumeration of WASI threads functions.
pub enum Wasip1ThreadsABIFunc {
    /// thread_spawn
    ThreadSpawn,
}

#[derive(
    strum::EnumString, strum::VariantArray, strum::VariantNames, PartialEq, strum::Display,
)]
#[strum(serialize_all = "snake_case")]
/// Enumeration of internal exports required by WASI threads.
pub enum Wasip1ThreadsABIExportFunc {
    /// wasi_thread_start
    WasiThreadStart,
    /// wasi_thread_start_entry
    WasiThreadStartEntry,
}
