/// Module for ABI connection generation and overrides.
pub mod abi_connect;
/// Utilities for handling anonymous and generic WASM rewrites.
pub mod anonymous;
/// Type-checking routines for imported/exported functions.

/// Internal debug formatting and trace generators.
pub mod debug;
/// Generates and bridges memory copy, trap, and related VFS interactions.
pub mod memory;
pub mod memory_post_components;
/// Routines for stripping or patching unused WASM components.
pub mod patch_component;
/// Embeds module metadata.
pub mod producer;
/// Low-level multi-threading global locks and shared allocations.
pub mod shared_global;
/// Bridges custom initializers, `_start`, `main_void`, and reset routines.
pub mod special_func;
pub mod starts;
/// Internal logic for rewriting WASI threads spawn imports to the VFS.
pub mod threads;
/// Handles logic for rewriting unreachable instructions to prevent Wasm execution traps.
pub mod wrap_unreachable;
pub mod vfs_host;
/// Generates the atomic wait implementation for `WaitPoll` timeout handling.
pub mod poll;
/// Reimplementation of binaryen's `--multi-memory-lowering` pass using walrus.
pub mod multi_memory_lowering;

use std::{any::Any, collections::HashMap, fs, io::Read as _, str::FromStr};

use camino::Utf8PathBuf;
use compact_str::{CompactString, ToCompactString as _};
use eyre::{Context as _, ContextCompat};
use itertools::Itertools;
use walrus::MemoryId;

use crate::{
    args::{self, TargetMemoryType},
    compile,
    config_checker::TomlRestorers,
    fallback_command,
    unique_name::UniqueName,
    util::{
        CaminoUtilModule as _, ResultUtil, WalrusFID as _, WalrusUtilExport as _, WalrusUtilModule,
        WasmName, WasmNameHolder,
    },
};

/// Represents the generation context, holding configuration arguments and targeted module info.
#[derive(Debug, Clone)]
pub struct GeneratorCtx {
    /// The name of the VFS module.
    pub vfs_name: WasmName,
    /// The names of the target modules.
    pub target_names: Box<[WasmName]>,
    /// Including one's own WASI ABI
    pub target_names_with_self: Box<[WasmName]>,
    /// only pre_vfs, post_combine, post_lower_memory
    pub vfs_used_memory_id: Option<MemoryId>,
    /// only post_combine
    pub vfs_used_global_id: Option<Box<[walrus::GlobalId]>>,
    /// only pre_target, post_combine, post_lower_memory
    pub target_used_memory_id: Option<HashMap<WasmName, MemoryId>>,
    /// only post_combine
    pub target_used_global_id: Option<HashMap<WasmName, Box<[walrus::GlobalId]>>>,
    /// not start section.
    /// only post_combine.
    pub start_func_id: Option<HashMap<WasmName, walrus::FunctionId>>,
    /// The memory type of the target modules (Single or Multi).
    pub target_memory_type: TargetMemoryType,
    /// Whether to print unstable debug information.
    pub unstable_print_debug: bool,
    /// Whether to include DWARF debug information.
    pub dwarf: bool,
    /// Whether to enable multi-threading support.
    pub threads: bool,
    /// Whether to adjust the ABI for standard environments.
    pub adjust_abi: bool,
    /// Whether to keep intermediate build artifacts.
    pub keep_build_artifacts: bool,
    /// Whether the VFS module is a library (has no start section / `_start` export).
    /// Detected during `pre_vfs`; when `true`, the VFS start slot in the
    /// combined start chain is skipped entirely.
    pub vfs_is_library: bool,
    pub starts: starts::FnInStarts,
}

/// Sub-context for extracting and storing component variables during execution.
#[derive(Debug, Default)]
pub struct ComponentCtx {
    /// Optional name of the VFS module.
    vfs_name: Option<WasmName>,
    /// Optional names of the target modules.
    target_names: Option<Box<[WasmName]>>,
    /// Optional memory type of the target modules.
    target_memory_type: Option<TargetMemoryType>,
    /// Optional flag for unstable debug printing.
    unstable_print_debug: Option<bool>,
    /// Flag for including DWARF debug information.
    dwarf: bool,
    /// Optional flag for multi-threading support.
    threads: Option<bool>,
    /// Flag for ABI adjustment.
    adjust_abi: bool,
}

struct CompressNames {
    names: Box<[String]>,
}

impl ToString for CompressNames {
    fn to_string(&self) -> String {
        self.names
            .iter()
            .map(|s| {
                let len = s.len();
                let len_len = len.to_string().len();
                format!("{:09}{}{}", len_len, len, s)
            })
            .join("")
    }
}

impl FromStr for CompressNames {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut names = Vec::new();
        let mut rest = s;
        while !rest.is_empty() {
            let len_len = &rest[0..9];
            let len_len: usize = len_len[len_len
                .chars()
                .position(|c| c != '0')
                .unwrap_or(len_len.chars().count() - 1)..]
                .parse()
                .wrap_err_with(|| format!("Failed to parse length of length: {len_len}"))?;
            rest = rest.get(9..).unwrap();
            let len: usize = rest
                .get(0..len_len)
                .ok_or_else(|| eyre::eyre!("Failed to get length"))?
                .parse()
                .wrap_err_with(|| format!("Failed to parse length: {rest}"))?;
            rest = rest.get(len_len..).unwrap();
            let name = rest
                .get(0..len)
                .ok_or_else(|| eyre::eyre!("Failed to get name"))?;
            names.push(name.to_string());
            rest = rest.get(len..).unwrap();
        }
        Ok(Self {
            names: names.into_boxed_slice(),
        })
    }
}

/// A generator acting as a visitor that propagates component contextual data.
#[derive(Debug, Default)]
pub struct ComponentCtxVisitor {
    vfs_name: Option<CompactString>,
    target_names: Option<Box<[CompactString]>>,
    target_memory_type: Option<TargetMemoryType>,
    unstable_print_debug: Option<bool>,
    dwarf: Option<bool>,
    threads: Option<bool>,
    adjust_abi: bool,
}

impl ComponentCtxVisitor {
    /// Creates a new `ComponentCtxVisitor` with the specified context parameters.
    pub fn new(
        vfs_name: WasmName,
        target_names: Box<[WasmName]>,
        target_memory_type: TargetMemoryType,
        unstable_print_debug: bool,
        dwarf: bool,
        threads: bool,
        adjust_abi: bool,
    ) -> Self {
        Self {
            vfs_name: Some(vfs_name.to_compact_string()),
            target_names: Some(
                target_names
                    .into_iter()
                    .map(|s| s.to_compact_string())
                    .collect(),
            ),
            target_memory_type: Some(target_memory_type),
            unstable_print_debug: Some(unstable_print_debug),
            dwarf: Some(dwarf),
            threads: Some(threads),
            adjust_abi,
        }
    }
}

impl Generator for ComponentCtxVisitor {
    fn pre_vfs(&mut self, module: &mut walrus::Module, ctx: &GeneratorCtx) -> eyre::Result<()> {
        let vfs_name = ctx.vfs_name.to_compact_string();
        let target_names = CompressNames {
            names: ctx
                .target_names
                .iter()
                .map(|s| s.to_string())
                .collect::<Box<_>>(),
        };
        let GeneratorCtx {
            vfs_name: _,
            target_names: _,
            target_memory_type,
            unstable_print_debug,
            dwarf,
            threads,
            adjust_abi,
            target_names_with_self: _,
            vfs_used_memory_id: _,
            vfs_used_global_id: _,
            target_used_memory_id: _,
            target_used_global_id: _,
            start_func_id: _,
            keep_build_artifacts: _,
            vfs_is_library: _,
            starts: _,
        } = ctx;
        module.save_info("vfs_name", vfs_name.to_string())?;
        module.save_info("target_names", target_names)?;
        module.save_info("target_memory_type", *target_memory_type)?;
        module.save_info("unstable_print_debug", *unstable_print_debug)?;
        module.save_info("dwarf", *dwarf)?;
        module.save_info("threads", *threads)?;
        module.save_info("adjust_abi", *adjust_abi)?;
        Ok(())
    }

    fn post_components(
        &mut self,
        module: &mut walrus::Module,
        _: &ComponentCtx,
    ) -> eyre::Result<()> {
        let vfs_name = module.load_info::<String>("vfs_name")?;
        let target_names = module.load_info::<CompressNames>("target_names")?;
        let target_memory_type = module.load_info::<TargetMemoryType>("target_memory_type")?;
        let unstable_print_debug = module.load_info::<bool>("unstable_print_debug")?;
        let dwarf = module.load_info::<bool>("dwarf")?;
        let threads = module.load_info::<bool>("threads")?;
        let adjust_abi = module.load_info::<bool>("adjust_abi")?;
        self.vfs_name = Some(vfs_name.to_compact_string());
        self.target_names = Some(
            target_names
                .names
                .into_iter()
                .map(|s| s.to_compact_string())
                .collect(),
        );
        self.target_memory_type = Some(target_memory_type);
        self.unstable_print_debug = Some(unstable_print_debug);
        self.dwarf = Some(dwarf);
        self.threads = Some(threads);
        self.adjust_abi = adjust_abi;

        Ok(())
    }
}

impl ComponentCtx {
    /// Creates a new `ComponentCtx` with the specified context parameters.
    pub fn new(
        vfs_name: WasmName,
        target_names: Box<[WasmName]>,
        target_memory_type: TargetMemoryType,
        unstable_print_debug: bool,
        dwarf: bool,
        threads: bool,
        adjust_abi: bool,
    ) -> Self {
        Self {
            vfs_name: Some(vfs_name),
            target_names: Some(target_names),
            target_memory_type: Some(target_memory_type),
            unstable_print_debug: Some(unstable_print_debug),
            dwarf,
            threads: Some(threads),
            adjust_abi,
        }
    }

    /// Returns the name of the VFS module.
    pub fn vfs_name(&self) -> &WasmName {
        self.vfs_name.as_ref().unwrap()
    }

    /// Returns the names of the target modules.
    pub fn target_names(&self) -> &Box<[WasmName]> {
        self.target_names.as_ref().unwrap()
    }

    /// Returns the memory type of the target modules.
    pub fn target_memory_type(&self) -> TargetMemoryType {
        self.target_memory_type.unwrap()
    }

    /// Returns whether to print unstable debug information.
    pub fn unstable_print_debug(&self) -> bool {
        self.unstable_print_debug.unwrap()
    }

    /// Returns whether to include DWARF debug information.
    pub fn dwarf(&self) -> bool {
        self.dwarf
    }

    /// Returns whether to enable multi-threading support.
    pub fn threads(&self) -> bool {
        self.threads.unwrap()
    }
}

/// Defines the core trait for WASM transformations, hooks, and component optimizations over the build lifecycle.
pub trait Generator: std::fmt::Debug + Any {
    /// Operations performed on the built VFS module.
    #[allow(unused_variables)]
    fn pre_vfs(&mut self, module: &mut walrus::Module, ctx: &GeneratorCtx) -> eyre::Result<()> {
        Ok(())
    }

    /// Operations performed on the target module.
    #[allow(unused_variables)]
    fn pre_target(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
        external: &ModuleExternal,
    ) -> eyre::Result<()> {
        Ok(())
    }

    /// Operations performed on the combined module.
    #[allow(unused_variables)]
    fn post_combine(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        Ok(())
    }

    /// Operations performed after lowerings memory operations.
    /// Only called if the target memory type is `Single`.
    #[allow(unused_variables)]
    fn post_lower_memory(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        Ok(())
    }

    /// Operations performed after components.
    #[allow(unused_variables)]
    fn post_components(
        &mut self,
        module: &mut walrus::Module,
        ctx: &ComponentCtx,
    ) -> eyre::Result<()> {
        Ok(())
    }

    /// Operations performed after last optimizations.
    /// Generating debug functions is a delicate process,
    /// so in this case, output once per structure.
    /// Return true if there are changes.
    #[allow(unused_variables)]
    fn post_all_optimize(
        &mut self,
        module: &mut walrus::Module,
        ctx: &ComponentCtx,
    ) -> eyre::Result<bool> {
        Ok(false)
    }
}
impl<T: std::fmt::Debug + Any + Generator> Generator for [T] {
    fn pre_vfs(&mut self, module: &mut walrus::Module, ctx: &GeneratorCtx) -> eyre::Result<()> {
        for generator in self {
            generator.pre_vfs(module, ctx).wrap_err_with(|| {
                eyre::eyre!(format!("Failed to run pre_vfs for {generator:?}"))
            })?;
        }
        Ok(())
    }

    fn pre_target(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
        external: &ModuleExternal,
    ) -> eyre::Result<()> {
        for generator in self {
            generator
                .pre_target(module, ctx, external)
                .wrap_err_with(|| {
                    eyre::eyre!(format!("Failed to run pre_target for {generator:?}"))
                })?;
        }
        Ok(())
    }

    fn post_combine(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        for generator in self {
            generator.post_combine(module, ctx).wrap_err_with(|| {
                eyre::eyre!(format!("Failed to run post_combine for {generator:?}"))
            })?;
        }
        Ok(())
    }

    fn post_lower_memory(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        for generator in self {
            generator.post_lower_memory(module, ctx).wrap_err_with(|| {
                eyre::eyre!(format!("Failed to run post_lower_memory for {generator:?}"))
            })?;
        }
        Ok(())
    }

    fn post_components(
        &mut self,
        module: &mut walrus::Module,
        ctx: &ComponentCtx,
    ) -> eyre::Result<()> {
        for generator in self {
            generator.post_components(module, ctx).wrap_err_with(|| {
                eyre::eyre!(format!("Failed to run post_components for {generator:?}"))
            })?;
        }
        Ok(())
    }

    fn post_all_optimize(
        &mut self,
        module: &mut walrus::Module,
        ctx: &ComponentCtx,
    ) -> eyre::Result<bool> {
        let mut changed = false;
        for generator in self {
            changed |= generator.post_all_optimize(module, ctx).wrap_err_with(|| {
                eyre::eyre!(format!("Failed to run post_all_optimize for {generator:?}"))
            })?;
        }
        Ok(changed)
    }
}
impl Generator for Box<dyn Generator + 'static> {
    fn pre_vfs(&mut self, module: &mut walrus::Module, ctx: &GeneratorCtx) -> eyre::Result<()> {
        (**self).pre_vfs(module, ctx)
    }

    fn pre_target(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
        external: &ModuleExternal,
    ) -> eyre::Result<()> {
        (**self).pre_target(module, ctx, external)
    }

    fn post_combine(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        (**self).post_combine(module, ctx)
    }

    fn post_lower_memory(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        (**self).post_lower_memory(module, ctx)
    }

    fn post_components(
        &mut self,
        module: &mut walrus::Module,
        ctx: &ComponentCtx,
    ) -> eyre::Result<()> {
        (**self).post_components(module, ctx)
    }

    fn post_all_optimize(
        &mut self,
        module: &mut walrus::Module,
        ctx: &ComponentCtx,
    ) -> eyre::Result<bool> {
        (**self).post_all_optimize(module, ctx)
    }
}
impl<'a> Generator for &'a mut (dyn Generator + 'a) {
    fn pre_vfs(&mut self, module: &mut walrus::Module, ctx: &GeneratorCtx) -> eyre::Result<()> {
        (**self).pre_vfs(module, ctx)
    }

    fn pre_target(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
        external: &ModuleExternal,
    ) -> eyre::Result<()> {
        (**self).pre_target(module, ctx, external)
    }

    fn post_combine(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        (**self).post_combine(module, ctx)
    }

    fn post_lower_memory(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        (**self).post_lower_memory(module, ctx)
    }

    fn post_components(
        &mut self,
        module: &mut walrus::Module,
        ctx: &ComponentCtx,
    ) -> eyre::Result<()> {
        (**self).post_components(module, ctx)
    }

    fn post_all_optimize(
        &mut self,
        module: &mut walrus::Module,
        ctx: &ComponentCtx,
    ) -> eyre::Result<bool> {
        (**self).post_all_optimize(module, ctx)
    }
}

/// Stores the identity of an external loaded Wasm target.
#[derive(Debug)]
pub struct ModuleExternal {
    /// The name of the external module.
    pub name: WasmName,
}

impl ModuleExternal {
    /// Creates a new `ModuleExternal` with the specified name.
    pub fn new(name: &WasmName) -> Self {
        Self { name: name.clone() }
    }
}

/// Coordinates iterating over registered generators and merging external module logic into the VFS.
#[derive(Debug)]
pub struct GeneratorRunner {
    pub generators: Vec<Box<dyn Generator + 'static>>,
    /// The context used during generation.
    pub ctx: GeneratorCtx,
    /// The path to the WASM module.
    pub path: WasmPath,
    /// The paths to the target WASM modules.
    pub targets: Box<[WasmPath]>,
    /// Options for building the VFS module.
    pub vfs_build_opts: args::VfsBuildOptions,
    /// Options for building the target WASM modules.
    pub target_vfs_build_opts: Box<[args::VfsBuildOptions]>,
    /// The TOML restorers used to reset configuration files.
    pub toml_restorers: Option<TomlRestorers>,
    /// Memory hints for the target modules.
    pub memory_hint: HashMap<WasmName, usize>,
    /// Holder for the names of the WASM modules.
    pub wasm_name_holder: WasmNameHolder,
}

/// A runner that coordinates the generation of a WebAssembly component.
#[derive(Debug)]
pub struct ComponentRunner {
    /// The generators used to transform the component.
    pub generators: Vec<Box<dyn Generator + 'static>>,
    /// Encapsulates contextual settings across all generator components during transpilation.
    pub ctx: Option<ComponentCtx>,
    /// Specifies the target active WASM module destination.
    pub path: WasmPath,
    /// Manages dynamically generated and globally referenced named functions.
    pub wasm_name_holder: Option<WasmNameHolder>,
}

pub(crate) trait WrapRunner<T> {
    #[allow(unused_variables)]
    fn wrap_run(
        self,
        path: &mut WasmPath,
        dwarf: bool,
        keep_build_artifacts: bool,
        stream_pipeline: Option<crate::wasm_stream::pipeline::Pipeline>,
    ) -> eyre::Result<T>
    where
        Self: Sized;
}

impl<T, F: FnOnce(&mut walrus::Module) -> eyre::Result<T>> WrapRunner<T> for F {
    fn wrap_run(
        self,
        path: &mut WasmPath,
        dwarf: bool,
        keep_build_artifacts: bool,
        mut stream_pipeline: Option<crate::wasm_stream::pipeline::Pipeline>,
    ) -> eyre::Result<T> {
        let old_path = path.path()?.clone();
        
        let mut input_wasm = fs::read(&old_path).wrap_err("Failed to read Wasm file")?;
        
        if let Some(pipeline) = &mut stream_pipeline {
            input_wasm = pipeline.run(&input_wasm).wrap_err("Failed to run StreamPipeline pre-walrus")?;
        }
        
        let module =
            &mut walrus::Module::from_buffer(&input_wasm).map_err(|e| eyre::eyre!(e)).wrap_err("Failed to load Wasm module from buffer")?;

        let result = (self)(module)?;

        let new_path = old_path.with_extension("adjusted.wasm");

        if fs::metadata(&new_path).is_ok() {
            fs::remove_file(&new_path)
                .wrap_err_with(|| format!("Failed to remove existing file {new_path}"))?;
        }

        module
            .emit_wasm_file(&new_path)
            .to_eyre()
            .wrap_err_with(|| format!("Failed to write adjusted Wasm to {new_path}"))?;

        if !keep_build_artifacts && !path.is_original(&old_path) {
            std::fs::remove_file(&old_path)
                .unwrap_or_else(|e| log::warn!("Failed to remove intermediate file {old_path}: {e}"));
        }

        path.set_path(new_path)?;

        Ok(result)
    }
}

pub(crate) trait EndWithOpt<T> {
    #[allow(unused_variables)]
    fn with_opt(
        self,
        path: &mut WasmPath,
        dwarf: bool,
        keep_build_artifacts: bool,
        skip_opt: bool,
    ) -> eyre::Result<T>
    where
        Self: Sized;

    #[allow(dead_code)]
    fn with_opt_args(
        self,
        path: &mut WasmPath,
        args: &[&str],
        require_update: bool,
        dwarf: bool,
        keep_build_artifacts: bool,
        skip_opt: bool,
    ) -> eyre::Result<T>
    where
        Self: Sized;
}

impl<T, F: FnOnce(&mut WasmPath) -> eyre::Result<T>> EndWithOpt<T> for F {
    fn with_opt(
        self,
        path: &mut WasmPath,
        dwarf: bool,
        keep_build_artifacts: bool,
        skip_opt: bool,
    ) -> eyre::Result<T>
    where
        Self: Sized,
    {
        let result = (self)(path).wrap_err("Failed to run with with_opt")?;

        if skip_opt {
            println!("Skipping Wasm optimization...");
            return Ok(result);
        }

        println!("Optimizing Wasm...");
        let old_path = path.path()?.clone();
        let new_path = compile::optimize_wasm(&old_path, &[], false, dwarf)
            .wrap_err("Failed to optimize Wasm")?;

        if !keep_build_artifacts && old_path != new_path && !path.is_original(&old_path) {
            std::fs::remove_file(&old_path)
                .wrap_err_with(|| format!("Failed to remove existing file {old_path}"))?;
        }

        path.set_path(new_path)?;

        Ok(result)
    }

    fn with_opt_args(
        self,
        path: &mut WasmPath,
        args: &[&str],
        require_update: bool,
        dwarf: bool,
        keep_build_artifacts: bool,
        skip_opt: bool,
    ) -> eyre::Result<T>
    where
        Self: Sized,
    {
        let result = (self)(path).wrap_err("Failed to run with with_opt_args")?;

        if skip_opt {
            println!("Skipping Wasm optimization...");
            return Ok(result);
        }

        println!("Optimizing Wasm... with args: {}", args.iter().join(" "));
        let old_path = path.path()?.clone();
        let new_path = compile::optimize_wasm(&old_path, args, require_update, dwarf)
            .wrap_err("Failed to optimize Wasm")?;

        if !keep_build_artifacts && old_path != new_path && !path.is_original(&old_path) {
            std::fs::remove_file(&old_path)
                .wrap_err_with(|| format!("Failed to remove existing file {old_path}"))?;
        }

        path.set_path(new_path)?;

        Ok(result)
    }
}

impl GeneratorRunner {
    /// Initializes a `GeneratorRunner` capturing all parameters needed for complete application-level transpilation mapping.
    pub fn new(
        path: WasmPath,
        targets: Box<[WasmPath]>,
        threads: bool,
        dwarf: bool,
        unstable_print_debug: bool,
        adjust_abi: bool,
        keep_build_artifacts: bool,
        memory_type: TargetMemoryType,
        vfs_build_opts: args::VfsBuildOptions,
        target_vfs_build_opts: Box<[args::VfsBuildOptions]>,
        toml_restorers: TomlRestorers,
        memory_hint: Box<[Option<usize>]>,
    ) -> eyre::Result<Self> {
        let target_names_with_self = core::iter::once(Ok(path.name()?.to_compact_string()))
            .chain(targets.iter().map(|t| Ok(t.name()?.to_compact_string())))
            .collect::<eyre::Result<Box<_>>>()?;

        let wasm_name_holder = WasmNameHolder::new(target_names_with_self);
        let mut wasm_name_holder_iter = wasm_name_holder.iter();
        let vfs_name = wasm_name_holder_iter
            .next()
            .ok_or_else(|| eyre::eyre!("Failed to get VFS name"))?;

        let target_names = wasm_name_holder_iter.collect::<Box<_>>();

        let starts = starts::FnInStarts::new(&target_names);

        // log targeting info
        for name in target_names.iter() {
            log::info!("Targeting module: {name}");
        }
        log::info!("VFS module: {vfs_name}");

        let target_names_with_self = target_names
            .iter()
            .cloned()
            .chain(core::iter::once(vfs_name.clone()))
            .collect::<Box<_>>();

        let memory_hint = memory_hint
            .into_iter()
            .zip(target_names.iter().cloned())
            .filter_map(|(hint, name)| hint.map(|h| (name, h)))
            .collect::<HashMap<_, _>>();

        Ok(Self {
            generators: Vec::new(),
            ctx: GeneratorCtx {
                vfs_name,
                target_names,
                target_names_with_self,
                target_memory_type: memory_type,
                unstable_print_debug,
                dwarf,
                threads,
                adjust_abi,
                keep_build_artifacts,
                vfs_used_memory_id: None,
                vfs_used_global_id: None,
                target_used_memory_id: None,
                target_used_global_id: None,
                start_func_id: None,
                vfs_is_library: false,
                starts,
            },
            path,
            targets,
            vfs_build_opts,
            target_vfs_build_opts,
            toml_restorers: Some(toml_restorers),
            memory_hint,
            wasm_name_holder,
        })
    }

    /// Registers a custom generator to be applied sequentially during transformation phases.
    pub fn add_generator<G: Generator + 'static>(&mut self, generator: G) {
        self.generators.push(Box::new(generator));
    }



    /// Resolves and retrieves a shared reference to a specific structured generator dynamically at runtime.
    pub fn get_generator_ref<T: Generator + 'static>(&self) -> eyre::Result<&T> {
        fn downcast_ref<T: 'static>(b: &dyn Any) -> Option<&'_ T> {
            if b.is::<T>() {
                Some(b.downcast_ref::<T>().unwrap())
            } else {
                None
            }
        }

        self.generators
            .iter()
            .map(|g| g.as_ref())
            .find_map(|g| downcast_ref::<T>(g))
            .wrap_err_with(|| {
                eyre::eyre!("Failed to get generator: {}", core::any::type_name::<T>())
            })
            .wrap_err_with(|| eyre::eyre!("Available generators: {:?}", self.generators))
    }

    #[deprecated(
        note = "Ensure this function is self-contained. This is a temporary measure for debugging purposes."
    )]
    /// Provides direct immutable access tracking the primary target structural path configuration.
    pub const fn path(&self) -> &WasmPath {
        &self.path
    }

    #[deprecated(
        note = "Ensure this function is self-contained. This is a temporary measure for debugging purposes."
    )]
    /// Directly evaluates currently linked ancillary wasm dependencies.
    pub const fn targets(&self) -> &Box<[WasmPath]> {
        &self.targets
    }

    /// Securely uncovers operational variables tracking internal generational properties dynamically.
    pub const fn ctx(&self) -> &GeneratorCtx {
        &self.ctx
    }

    /// Confirms mapping configurations and statically evaluates lazy initializations strictly allocating paths.
    pub fn definitely(&mut self) -> eyre::Result<()> {
        self.path
            .definitely(self.ctx.threads, &self.vfs_build_opts)?;
        for (i, target) in self.targets.iter_mut().enumerate() {
            target.definitely(self.ctx.threads, &self.target_vfs_build_opts[i])?;
        }
        Ok(())
    }

    /// Primary operation chaining logic seamlessly running structural modifiers across all layered stages sequentially terminating out into WebAssembly components.
    pub fn run_layers_to_component(
        mut self,
        out_dir: &Utf8PathBuf,
        keep_build_artifacts: bool,
    ) -> eyre::Result<ComponentRunner> {
        self.definitely()?;

        let toml_restorers = self
            .toml_restorers
            .take()
            .ok_or_else(|| eyre::eyre!("TomlRestorers already taken"))?;

        toml_restorers
            .restore()
            .wrap_err("Failed to restore toml files")?;

        let mut mem_id_visitor = MemoryIDVisitor {
            memory_hint: self.memory_hint.clone(),
            used_vfs_memory_id: None,
            used_target_memory_id: None,
        };
        let mut global_id_visitor = GlobalIdVisitor {
            vfs_global_id: None,
            global_id: None,
        };
        let dwarf = self.ctx.dwarf;

        println!("Remove existing output directory...");
        if std::fs::metadata(&out_dir).is_ok() {
            std::fs::remove_dir_all(&out_dir).expect("Failed to remove existing directory");
        }
        std::fs::create_dir_all(&out_dir).expect("Failed to create output directory");

        let mut component_ctx_visitor = ComponentCtxVisitor::new(
            self.ctx.vfs_name.clone(),
            self.ctx.target_names.clone(),
            self.ctx.target_memory_type,
            self.ctx.unstable_print_debug,
            self.ctx.dwarf,
            self.ctx.threads,
            self.ctx.adjust_abi,
        );

        println!("Adjusting VFS Wasm...");
        let skip_vfs_opt = self.vfs_build_opts.no_opt > 0 || self.vfs_build_opts.no_opt_all > 0;
        (|path: &mut WasmPath| {
            let wasm_bytes = std::fs::read(&path.path()?).unwrap();
            let mut vfs_is_library = true;
            for payload in wasmparser::Parser::new(0).parse_all(&wasm_bytes) {
                if let Ok(wasmparser::Payload::StartSection { .. }) = payload {
                    vfs_is_library = false;
                }
                if let Ok(wasmparser::Payload::ExportSection(s)) = payload {
                    for e in s {
                        if let Ok(e) = e {
                            if e.name == "_start" {
                                vfs_is_library = false;
                            }
                        }
                    }
                }
            }
            self.ctx.vfs_is_library = vfs_is_library;
            let pipeline_is_library = vfs_is_library;
            let cloned_ctx = self.ctx.clone();

            (|module: &mut walrus::Module| {
                mem_id_visitor
                    .pre_vfs(module, &self.ctx)
                    .wrap_err("Failed in pre_vfs")?;
                global_id_visitor
                    .pre_vfs(module, &self.ctx)
                    .wrap_err("Failed in pre_vfs")?;

                self.ctx.vfs_used_memory_id = mem_id_visitor.used_vfs_memory_id;

                // Detect VFS library mode: if the VFS module has no start section,
                // it is a library and has no initialization entry point.
                if pipeline_is_library {
                    log::info!(
                        "VFS module `{}` has no start section — treating as a library.",
                        self.ctx.vfs_name
                    );
                }

                component_ctx_visitor
                    .pre_vfs(module, &self.ctx)
                    .wrap_err("Failed in pre_vfs")?;

                self.generators
                    .pre_vfs(module, &self.ctx)
                    .wrap_err("Failed in run_pre_vfs")
            })
            .wrap_run(path, dwarf, keep_build_artifacts, {
                let mut pipeline = crate::wasm_stream::pipeline::Pipeline::new();
                use crate::wasm_stream::passes::{
                    starts_pre::StartsPreStreamPass, dummy_injector::DummyInjectorStreamPass, pre_vfs_memory_refuge::TemporaryRefugeMemoryStreamPass,
                    check::CheckUseWasiVirtLayerChecker,
                    patch_component::PatchComponentStreamPass,
                    anonymous::AnonymousStreamPass,
                    abi_connect::{ConnectWasip1ABIPreVfsStreamPass, NonRecursiveWasiABIPreVfsStreamPass},
                };

                let check_pass = crate::wasm_stream::pipeline::ParallelCheckStreamPass::new(vec![
                    Box::new(CheckUseWasiVirtLayerChecker::new()),
                ]);
                pipeline.add_pass(Box::new(check_pass));
                pipeline.add_pass(Box::new(AnonymousStreamPass::new(cloned_ctx)));
                pipeline.add_pass(Box::new(ConnectWasip1ABIPreVfsStreamPass::new()));
                pipeline.add_pass(Box::new(NonRecursiveWasiABIPreVfsStreamPass::new()));
                pipeline.add_pass(Box::new(PatchComponentStreamPass::new()));

                pipeline.add_pass(Box::new(StartsPreStreamPass::new(true, pipeline_is_library, "__flesh_vfs_start".to_string())));
                pipeline.add_pass(Box::new(DummyInjectorStreamPass::new(vec![
                    "__thread_patch".to_string(),
                    "__init_offset_global".to_string(),
                    "__save_target_memory".to_string(),
                    "__simple_debug_wasip1_vfs_pre_init".to_string(),
                ])));
                pipeline.add_pass(Box::new(TemporaryRefugeMemoryStreamPass::new(None)));
                Some(pipeline)
            })
        })
        .with_opt(&mut self.path, dwarf, keep_build_artifacts, skip_vfs_opt)?;

        println!("Adjusting target Wasm...");
        self.ctx.vfs_used_memory_id = None;
        for (i, (target, target_name)) in self.targets.iter_mut().zip(self.ctx.target_names.clone()).enumerate() {
            let skip_target_opt = self.vfs_build_opts.no_opt_all > 0 || self.target_vfs_build_opts[i].no_opt > 0 || self.target_vfs_build_opts[i].no_opt_all > 0;
            let cloned_ctx = self.ctx.clone();
            (|path: &mut WasmPath| {
                (|module: &mut walrus::Module| {
                    let external = ModuleExternal::new(&target_name);
                    mem_id_visitor
                        .pre_target(module, &self.ctx, &external)
                        .wrap_err("Failed in pre_target")?;
                    global_id_visitor
                        .pre_target(module, &self.ctx, &external)
                        .wrap_err("Failed in pre_target")?;

                    self.ctx.target_used_memory_id = mem_id_visitor.used_target_memory_id.clone();

                    self.generators
                        .pre_target(module, &self.ctx, &external)
                        .wrap_err("Failed in run_pre_target")
                })
                .wrap_run(path, dwarf, keep_build_artifacts, {
                    let mut pipeline = crate::wasm_stream::pipeline::Pipeline::new();
                    use crate::wasm_stream::passes::{
                        starts_pre::StartsPreStreamPass, dummy_injector::DummyInjectorStreamPass, pre_vfs_memory_refuge::TemporaryRefugeMemoryStreamPass,
                        producer::ProducerStreamPass, anonymous::AnonymousStreamPass, check_unused_threads::CheckUnusedThreadsStreamPass,
                        check::IsRustWasmChecker,
                        abi_connect::{ConnectWasip1ABIPreTargetStreamPass, ConnectWasip1ThreadsABIPreTargetStreamPass},
                    };
                    
                    pipeline.add_pass(Box::new(ProducerStreamPass::new()));
                    pipeline.add_pass(Box::new(CheckUnusedThreadsStreamPass::new(cloned_ctx.clone())));
                    pipeline.add_pass(Box::new(ConnectWasip1ABIPreTargetStreamPass::new(target_name.to_string())));
                    pipeline.add_pass(Box::new(ConnectWasip1ThreadsABIPreTargetStreamPass::new(target_name.to_string())));
                    
                    let check_pass = crate::wasm_stream::pipeline::ParallelCheckStreamPass::new(vec![
                        Box::new(IsRustWasmChecker::new()),
                    ]);
                    pipeline.add_pass(Box::new(check_pass));

                    let export_name = format!("__flesh_{}_start", target_name);
                    pipeline.add_pass(Box::new(StartsPreStreamPass::new(false, false, export_name.clone())));
                    pipeline.add_pass(Box::new(crate::wasm_stream::passes::dummy_injector::DummyInjectorStreamPass::new(vec![export_name])));
                    let new_memory_name = format!("__wasip1_vfs_{}_memory", target_name);
                    pipeline.add_pass(Box::new(TemporaryRefugeMemoryStreamPass::new(Some(new_memory_name))));
                    Some(pipeline)
                })
            })
            .with_opt(target, dwarf, keep_build_artifacts, skip_target_opt)?;
        }

        let skip_all_opt = self.vfs_build_opts.no_opt_all > 0
            || self.target_vfs_build_opts.iter().any(|opts| opts.no_opt_all > 0);

        println!("Combining Wasm modules...");
        self.ctx.vfs_used_memory_id = None;
        self.ctx.target_used_memory_id = None;
        let output = format!("{out_dir}/merged.wasm");
        (|path: &mut WasmPath| {
            let old_path = path.path()?.clone();
            merge(
                &old_path,
                &self
                    .targets
                    .iter()
                    .map(|t| t.path())
                    .collect::<eyre::Result<Vec<_>>>()?,
                &output,
                self.ctx.threads,
                dwarf,
            )
            .wrap_err("Failed to combine Wasm modules")?;

            if !keep_build_artifacts {
                std::fs::remove_file(&old_path)
                    .wrap_err_with(|| format!("Failed to remove existing file {old_path}"))?;
                for target in self.targets.iter() {
                    if let Ok(target_path) = target.path() {
                        if !target.is_original(target_path) {
                            std::fs::remove_file(target_path).unwrap_or_else(|e| {
                                log::warn!(
                                    "Failed to remove intermediate target file {target_path}: {e}"
                                );
                            });
                        }
                    }
                }
            }

            path.set_path(output.into())
        })
        .with_opt(&mut self.path, dwarf, keep_build_artifacts, skip_all_opt)?;
        println!("Adjusting Merged Wasm (walrus post_combine)...");
        (|path: &mut WasmPath| {
            (|module: &mut walrus::Module| {
                let mut mem_id_visitor = MemoryIDVisitor {
                    memory_hint: self.memory_hint.clone(),
                    used_vfs_memory_id: None,
                    used_target_memory_id: None,
                };
                let mut global_id_visitor = GlobalIdVisitor {
                    vfs_global_id: None,
                    global_id: None,
                };
                mem_id_visitor
                    .post_combine(module, &self.ctx)
                    .wrap_err("Failed in post_combine")?;
                global_id_visitor
                    .post_combine(module, &self.ctx)
                    .wrap_err("Failed in post_combine")?;
                let mut start_func_id_visitor = StartFuncIdVisitor::default();
                start_func_id_visitor
                    .post_combine(module, &self.ctx)
                    .wrap_err("Failed in post_combine")?;

                self.ctx.vfs_used_memory_id = mem_id_visitor.used_vfs_memory_id.take();
                self.ctx.target_used_memory_id = mem_id_visitor.used_target_memory_id.take();

                self.ctx.vfs_used_global_id = global_id_visitor.vfs_global_id.take();
                self.ctx.target_used_global_id = global_id_visitor.global_id.take();

                self.ctx.start_func_id = start_func_id_visitor.start_func_id.take();

                self.generators.post_combine(module, &self.ctx)?;

                Ok(())
            })
            .wrap_run(path, dwarf, keep_build_artifacts, None)
        })
        .with_opt(&mut self.path, dwarf, keep_build_artifacts, skip_all_opt)?;

        println!("Adjusting Merged Wasm (streaming pipeline)...");

        (|path: &mut WasmPath| {
            let old_path = path.path()?.clone();
            let input_wasm = std::fs::read(&old_path).wrap_err("Failed to read Wasm file")?;
            
            let mut pipeline = crate::wasm_stream::pipeline::Pipeline::new();
            let target_names: Vec<String> = self.ctx.target_names.iter().map(|n| n.as_ref().to_string()).collect();
            
            pipeline.add_pass(Box::new(
                crate::wasm_stream::passes::post_combine::PostCombineStreamPass::new(target_names.clone())
            ));

            if self.ctx.target_memory_type == TargetMemoryType::Single {
                println!("Generating single memory Merged Wasm (streaming lowering)...");
                pipeline.add_pass(Box::new(
                    crate::wasm_stream::passes::multi_memory_lowering::MultiMemoryLoweringStreamPass::new(self.ctx.threads)
                ));
                pipeline.add_pass(Box::new(
                    crate::wasm_stream::passes::shared_global::SharedGlobalStreamPass::new(self.ctx.threads, target_names)
                ));
            }
            
            let output_wasm = pipeline.run(&input_wasm).wrap_err("Failed to run StreamPipeline")?;
            
            let new_path = old_path.with_extension("lowered.wasm");
            std::fs::write(&new_path, output_wasm).wrap_err("Failed to write lowered Wasm file")?;
            
            if !keep_build_artifacts {
                std::fs::remove_file(&old_path).wrap_err_with(|| format!("Failed to remove existing file {old_path}"))?;
            }
            
            path.set_path(new_path)?;
            Ok(())
        }).with_opt(&mut self.path, dwarf, keep_build_artifacts, skip_all_opt)?;

        println!("Translating Wasm to Component...");
        let old_path = self.path.path()?.clone();
        std::fs::copy(&old_path, "debug_before_component.wasm").unwrap();
        let component = compile::wasm_to_component(&old_path, &self.ctx.target_names)
            .wrap_err("Failed to translate Wasm to Component")?;
        if !keep_build_artifacts {
            std::fs::remove_file(&old_path)
                .wrap_err_with(|| format!("Failed to remove existing file {old_path}"))?;
        }

        let new_component = format!("{out_dir}/{}.component.wasm", self.ctx.vfs_name);
        std::fs::rename(&component, &new_component)
            .wrap_err_with(|| format!("Failed to rename file {component} to {new_component}"))?;

        self.path.set_path(Utf8PathBuf::from(new_component))?;

        Ok(ComponentRunner::with_generators(self.path, self.generators))
    }
}

impl ComponentRunner {
    /// Instantiates a fresh `ComponentRunner` targeting a specific underlying WebAssembly structure.
    pub fn new(path: WasmPath) -> Self {
        Self {
            generators: Vec::new(),
            ctx: None,
            path,
            wasm_name_holder: None,
        }
    }

    /// Installs a specific invariant-checking generator executing without altering primary execution pathways.
    pub fn checker(&mut self, checker: impl Generator + 'static) {
        self.generators.push(Box::new(checker));
    }

    /// Quickly constructs a runner wrapping a pre-defined array containing generator strategies.
    pub fn with_generators(path: WasmPath, generators: Vec<Box<dyn Generator + 'static>>) -> Self {
        Self {
            generators,
            ctx: None,
            path,
            wasm_name_holder: None,
        }
    }

    /// Queues an advanced generative instruction step executed successively upon evaluation.
    pub fn add_generator<G: Generator + 'static>(&mut self, generator: G) {
        self.generators.push(Box::new(generator));
    }

    /// Allows querying the internal generator stack dynamically retrieving explicitly modeled type structures.
    pub fn get_generator_ref<T: Generator + 'static>(&self) -> eyre::Result<&T> {
        fn downcast_ref<T: 'static>(b: &dyn Any) -> Option<&'_ T> {
            if b.is::<T>() {
                Some(b.downcast_ref::<T>().unwrap())
            } else {
                None
            }
        }

        self.generators
            .iter()
            .map(|g| g.as_ref())
            .find_map(|g| downcast_ref::<T>(g))
            .wrap_err_with(|| {
                eyre::eyre!("Failed to get generator: {}", core::any::type_name::<T>())
            })
            .wrap_err_with(|| eyre::eyre!("Available generators: {:?}", self.generators))
    }

    /// return is_threads, core_name, mem_size
    pub fn component_to_files(
        &mut self,
        parsed_args: &(impl args::PostBuildContext + ?Sized),
        dwarf: bool,
        only_core: bool,
    ) -> eyre::Result<(bool, CompactString, HashMap<CompactString, (u64, u64)>)> {
        let out_dir = parsed_args.out_dir();

        let name = self.path.name()?;

        println!("Translating Component to JS...");
        let core_wasm_path = (|path: &mut WasmPath| {
            let old_path = path.path()?.clone();
            let binary = std::fs::read(&old_path).wrap_err("Failed to read component")?;
            let transpiled = parsed_args
                .transpile_to_js(&binary, &name)
                .wrap_err("Failed to transpile to JS")?;

            let mut core_wasm = None;
            for (name, data) in transpiled.files.iter() {
                let name = camino::Utf8PathBuf::from(name);
                let file_name = out_dir.join(&name);
                if std::fs::metadata(&file_name).is_ok() {
                    std::fs::remove_file(&file_name).wrap_err_with(|| {
                        eyre::eyre!("Failed to remove existing file: {file_name}")
                    })?;
                }
                if name.as_str().ends_with(".core.wasm") {
                    let file_name = camino::Utf8PathBuf::from(file_name);
                    std::fs::write(&file_name, &data).wrap_err_with(|| {
                        eyre::eyre!("Failed to write core wasm file: {file_name}")
                    })?;
                    core_wasm = Some(file_name);
                } else {
                    if let Some(parent) = name.parent() {
                        if !parent.as_str().is_empty() {
                            let dir = name.ancestors().nth(1).wrap_err_with(|| {
                                eyre::eyre!("Failed to get parent directory: {}", name)
                            })?;
                            let joined_dir = out_dir.join(dir);
                            if !std::fs::metadata(&joined_dir).is_ok() {
                                if dir.as_str() != "interfaces" {
                                    log::warn!("Creating directory: {joined_dir}");
                                }
                                if only_core {
                                    continue;
                                }
                                std::fs::create_dir_all(&joined_dir).wrap_err_with(|| {
                                    eyre::eyre!("Failed to create directory: {joined_dir}")
                                })?;
                            }
                        }
                    }
                    if only_core {
                        continue;
                    }
                    std::fs::write(&file_name, &data).wrap_err_with(|| {
                        eyre::eyre!("Failed to write transpiled file: {file_name}")
                    })?;
                }
            }

            let core_wasm = core_wasm
                .as_ref()
                .ok_or_else(|| eyre::eyre!("Failed to find core wasm"))?;

            if !parsed_args.keep_build_artifacts() {
                std::fs::remove_file(&old_path)
                    .wrap_err_with(|| format!("Failed to remove existing file {old_path}"))?;
            }
            path.set_path(core_wasm.clone())?;

            Ok(core_wasm.clone())
        })
        .with_opt(&mut self.path, dwarf, parsed_args.keep_build_artifacts(), parsed_args.dev())?;

        let mem_size_visitor = MemorySizeVisitor::default();
        self.generators.push(Box::new(mem_size_visitor));

        println!("Adjusting component Merged Wasm...");
        (|path: &mut WasmPath| {
            (|module: &mut walrus::Module| {
                let mut visitor = ComponentCtxVisitor::default();
                visitor
                    .post_components(
                        module,
                        &ComponentCtx {
                            dwarf,
                            ..Default::default()
                        },
                    )
                    .wrap_err("Failed in post_components")?;
                let wasm_name_holder = WasmNameHolder::new(
                    visitor
                        .target_names
                        .unwrap()
                        .into_iter()
                        .chain(core::iter::once(visitor.vfs_name.unwrap()))
                        .collect::<Box<_>>(),
                );
                self.wasm_name_holder = Some(wasm_name_holder);
                let mut wasm_name_holder_iter = self.wasm_name_holder.as_ref().unwrap().iter();
                self.ctx = Some(ComponentCtx {
                    vfs_name: Some(wasm_name_holder_iter.next().unwrap()),
                    target_names: Some(wasm_name_holder_iter.collect::<Box<_>>()),
                    target_memory_type: Some(visitor.target_memory_type.unwrap()),
                    unstable_print_debug: Some(visitor.unstable_print_debug.unwrap()),
                    dwarf: visitor.dwarf.unwrap(),
                    threads: Some(visitor.threads.unwrap()),
                    adjust_abi: visitor.adjust_abi,
                });

                self.generators
                    .post_components(module, self.ctx.as_ref().unwrap())
                    .wrap_err("Failed in run_post_components")
            })
            .wrap_run(path, dwarf, parsed_args.keep_build_artifacts(), None)
        })
        .with_opt(&mut self.path, dwarf, parsed_args.keep_build_artifacts(), parsed_args.dev())?;

        let dwarf = {
            let new_dwarf = self.ctx.as_ref().unwrap().dwarf;
            if dwarf && !new_dwarf {
                log::warn!(
                    "Dwarf was disabled in component processing, you should re-run with --dwarf"
                );
            }
            new_dwarf
        };

        println!("Final optimizing Merged Wasm...");
        let mut i = 0;
        while i < self.generators.len() {
            (|module: &mut walrus::Module| {
                loop {
                    if self.generators[i]
                        .post_all_optimize(module, self.ctx.as_ref().unwrap())
                        .wrap_err("Failed in post_all_optimize")?
                    {
                        i += 1;
                        return Ok(());
                    }
                    i += 1;
                    if i >= self.generators.len() {
                        return Ok(());
                    }
                }
            })
            .wrap_run(&mut self.path, dwarf, parsed_args.keep_build_artifacts(), None)?;
        }

        std::fs::rename(self.path.path()?, &core_wasm_path).wrap_err_with(|| {
            eyre::eyre!(
                "Failed to rename final wasm from {} to {}",
                self.path.path().unwrap(),
                core_wasm_path
            )
        })?;

        Ok((
            self.ctx.as_ref().unwrap().threads(),
            core_wasm_path
                .get_file_main_name()
                .ok_or_else(|| eyre::eyre!("Failed to get file name"))?,
            self.get_generator_ref::<MemorySizeVisitor>()?
                .mem_size
                .clone()
                .unwrap(),
        ))
    }
}

#[derive(Debug, Default, Clone)]
struct MemoryIDVisitor {
    pub memory_hint: HashMap<WasmName, usize>,
    pub used_vfs_memory_id: Option<MemoryId>,
    pub used_target_memory_id: Option<HashMap<WasmName, MemoryId>>,
}

impl Generator for MemoryIDVisitor {
    fn pre_vfs(
        &mut self,
        module: &mut walrus::Module,
        _: &crate::generator::GeneratorCtx,
    ) -> eyre::Result<()> {
        println!("Finding VFS memory id...");

        let id = module
            .get_memory_anchor("vfs", false)
            .wrap_err("Failed to get target memory id")?;
        self.used_vfs_memory_id = Some(id);
        Ok(())
    }

    fn pre_target(
        &mut self,
        module: &mut walrus::Module,
        _: &crate::generator::GeneratorCtx,
        external: &ModuleExternal,
    ) -> eyre::Result<()> {
        let id = module
            .find_used_memory_id(self.memory_hint.get(&external.name).copied())
            .wrap_err("Failed to find used memory id")?;
        module
            .create_memory_anchor(&external.name, id)
            .wrap_err("Failed to create memory anchor")?;
        self.used_target_memory_id
            .get_or_insert_default()
            .insert(external.name.clone(), id);
        Ok(())
    }

    fn post_combine(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        let id = module
            .get_memory_anchor("vfs", true)
            .wrap_err("Failed to find used memory id after combine")?;
        self.used_vfs_memory_id = Some(id);

        self.used_target_memory_id.get_or_insert_default().clear();
        for wasm in &ctx.target_names {
            let id = module
                .get_memory_anchor(wasm, true)
                .wrap_err("Failed to find used memory id after combine")?;
            self.used_target_memory_id
                .as_mut()
                .unwrap()
                .insert(wasm.clone(), id);
        }

        Ok(())
    }

    fn post_lower_memory(
        &mut self,
        module: &mut walrus::Module,
        _: &GeneratorCtx,
    ) -> eyre::Result<()> {
        let id = module
            .get_memory_id()
            .to_eyre()
            .wrap_err("Failed to get single memory id after lowering")?;
        self.used_vfs_memory_id = Some(id);
        self.used_target_memory_id = None;

        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
struct MemorySizeVisitor {
    mem_size: Option<HashMap<CompactString, (u64, u64)>>,
}

impl Generator for MemorySizeVisitor {
    fn post_components(
        &mut self,
        module: &mut walrus::Module,
        _: &ComponentCtx,
    ) -> eyre::Result<()> {
        let mem_size = module
            .memories
            .iter()
            .filter(|mem| mem.import.is_some())
            .map(|mem| {
                (
                    module
                        .imports
                        .get(mem.import.unwrap())
                        .name
                        .to_compact_string(),
                    (
                        mem.initial as u64,
                        mem.maximum.unwrap_or(mem.initial) as u64,
                    ),
                )
            })
            .collect::<HashMap<_, _>>();
        self.mem_size = Some(mem_size);

        Ok(())
    }
}

#[derive(Debug, Default)]
struct GlobalIdVisitor {
    vfs_global_id: Option<Box<[walrus::GlobalId]>>,
    global_id: Option<HashMap<WasmName, Box<[walrus::GlobalId]>>>,
}
impl Generator for GlobalIdVisitor {
    fn pre_vfs(&mut self, module: &mut walrus::Module, _: &GeneratorCtx) -> eyre::Result<()> {
        module
            .create_global_anchor("vfs")
            .wrap_err("Failed to create global anchor")?;

        Ok(())
    }

    fn pre_target(
        &mut self,
        module: &mut walrus::Module,
        _: &GeneratorCtx,
        external: &ModuleExternal,
    ) -> eyre::Result<()> {
        module
            .create_global_anchor(&external.name)
            .wrap_err("Failed to create global anchor")?;

        Ok(())
    }

    fn post_combine(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        self.global_id = Some(HashMap::new());

        let vfs_globals = module
            .get_global_anchor("vfs")
            .wrap_err("Failed to get global anchor for vfs")?;
        self.vfs_global_id = Some(vfs_globals);

        for wasm in &ctx.target_names {
            let globals = module
                .get_global_anchor(wasm)
                .wrap_err_with(|| format!("Failed to get global anchor for {wasm}"))?;
            self.global_id
                .as_mut()
                .unwrap()
                .insert(wasm.clone(), globals);
        }

        Ok(())
    }
}

/// To be used from both `special_func`'s `main_void` and `start`,
/// it must be prepared in `ctx`.
#[derive(Debug, Default)]
struct StartFuncIdVisitor {
    start_func_id: Option<HashMap<WasmName, walrus::FunctionId>>,
}

impl Generator for StartFuncIdVisitor {
    fn post_combine(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        for wasm in &ctx.target_names {
            // NOTE: wasm-merge preserves the package name with dashes for some exports,
            // but the __start_anchor from the Rust macros has underscores.
            let normalized_wasm = wasm.as_ref().replace('-', "_");
            // let export_name = format!("__wasip1_vfs_{wasm}__start");

            let export = format!("__wasip1_vfs_{wasm}__start").get_fid(&module.exports)?;
            self.start_func_id
                .get_or_insert_default()
                .insert(wasm.clone(), export);

            // NOTE: Don't erase the __start export - it's needed for StartFunc
            // to find it as an import or to redirect calls to it.

            // The anchor export was created by import_wasm! macro and uses underscores
            let anchor_name = format!("__wasip1_vfs_{normalized_wasm}__start_anchor");
            if let Some(_) = anchor_name.find_fid(&module.exports) {
                module.exports.erase_with(
                    &anchor_name,
                    ctx.unstable_print_debug,
                )?;
            }
        }

        Ok(())
    }
}

/// Represents the resolution state and file format targeting for manipulating WebAssembly modules intelligently.
#[derive(Debug, Clone, Hash)]
pub enum WasmPath {
    /// Indicates the target still needs compilation through standard cargo dependencies.
    Maybe {
        /// Fully resolved physical path to the corresponding Cargo.toml manifest definition.
        manifest_path: Utf8PathBuf,
        /// Specified internal crate package identifier.
        package: String,
    },
    /// A strictly resolved raw WebAssembly binary file ready for patching operations.
    Definitely(Utf8PathBuf),
    /// A completed explicitly transpiled WebAssembly Component standard architecture module.
    Component(Utf8PathBuf),
    /// A user-provided raw WebAssembly binary file that should never be deleted.
    /// The inner path tracks the original file location.
    Original {
        /// The current working path (may change as transformations produce new files).
        current: Utf8PathBuf,
        /// The original user-provided path that must be preserved.
        original: Utf8PathBuf,
    },
}

impl FromStr for WasmPath {
    type Err = eyre::Error;

    /// manifest_path :: package
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_path_and_package(s: &str) -> eyre::Result<Option<WasmPath>> {
            let split = s.split("::").collect::<Vec<_>>();
            if split.len() == 2 {
                let manifest_path = Utf8PathBuf::from_str(split[0])
                    .wrap_err_with(|| format!("Failed to parse manifest path: {}", split[0]))?;
                let package = split[1].to_string();
                return Ok(Some(WasmPath::Maybe {
                    manifest_path,
                    package,
                }));
            }
            Ok(None)
        }

        if let Some(wasm_path) =
            parse_path_and_package(s).wrap_err("Failed to parse path and package")?
        {
            return Ok(wasm_path);
        }

        if s.ends_with(".wasm") {
            let path = Utf8PathBuf::from_str(s)
                .wrap_err_with(|| format!("Failed to parse wasm path: {s}"))?;
            return WasmPath::with_wasm(path);
        }

        if s.ends_with("Cargo.toml") {
            let manifest_path = Utf8PathBuf::from_str(s)
                .wrap_err_with(|| format!("Failed to parse manifest path: {s}"))?;
            return WasmPath::with_maybe_only_manifest(manifest_path);
        }

        WasmPath::with_maybe_only_package(s.to_string())
            .wrap_err_with(|| format!("Failed to parse package name: {s}"))
    }
}

impl WasmPath {
    /// Computes the unique short name used during module linking and identification.
    pub fn name(&self) -> eyre::Result<CompactString> {
        match self {
            WasmPath::Maybe { package, .. } => {
                // Normalize package name by replacing hyphens with underscores
                // This is necessary because Rust package names can have hyphens (e.g., "test-package")
                // but they are converted to underscores in module identifiers (e.g., "test_package")
                let normalized = package.replace('-', "_");
                Ok(normalized.to_compact_string())
            }
            WasmPath::Definitely(path)
            | WasmPath::Component(path)
            | WasmPath::Original { current: path, .. } => path
                .get_file_main_name()
                .ok_or_else(|| eyre::eyre!("Failed to get file name from {path}")),
        }
    }

    /// Exposes the manifest path conditionally if the dependency is unresolved logically.
    pub fn manifest_path(&self) -> Option<&Utf8PathBuf> {
        match self {
            WasmPath::Maybe { manifest_path, .. } => Some(manifest_path),
            WasmPath::Definitely(_) | WasmPath::Component(_) | WasmPath::Original { .. } => None,
        }
    }

    /// Calculates the overarching workspace root via dependency graphing context implicitly.
    pub fn root_manifest_path(&self) -> Option<Utf8PathBuf> {
        match self {
            WasmPath::Maybe { manifest_path, .. } => {
                let cargo_metadata = {
                    let mut metadata_command = cargo_metadata::MetadataCommand::new();
                    metadata_command.manifest_path(&manifest_path);
                    metadata_command.exec().unwrap()
                };
                Some(cargo_metadata.workspace_root.join("Cargo.toml"))
            }
            WasmPath::Definitely(_) | WasmPath::Component(_) | WasmPath::Original { .. } => None,
        }
    }

    /// Statically assigns an unresolved target using defined package specifications.
    pub const fn with_maybe(manifest_path: Utf8PathBuf, package: String) -> Self {
        Self::Maybe {
            manifest_path,
            package,
        }
    }

    /// Constructs an unresolved dependency resolving package tracking inherently from a file manifest dynamically.
    pub fn with_maybe_only_manifest(manifest_path: Utf8PathBuf) -> eyre::Result<Self> {
        let cargo_metadata = {
            let mut metadata_command = cargo_metadata::MetadataCommand::new();
            metadata_command.manifest_path(&manifest_path);
            metadata_command.exec().unwrap()
        };
        let building_crate = compile::get_building_crate(&cargo_metadata, &None)?;

        Ok(Self::Maybe {
            manifest_path,
            package: building_crate.name.to_string(),
        })
    }

    /// Assesses global cargo metadata configuring definitions matching solely the provided string identifier.
    pub fn with_maybe_only_package(package: String) -> eyre::Result<Self> {
        let cargo_metadata = {
            let metadata_command = cargo_metadata::MetadataCommand::new();
            metadata_command.exec().unwrap()
        };
        let building_crate = compile::get_building_crate(&cargo_metadata, &Some(package.clone()))?;

        Ok(Self::Maybe {
            manifest_path: building_crate.manifest_path,
            package: building_crate.name.to_string(),
        })
    }

    /// Loads an unresolved dependency using current environment cargo bounds natively.
    pub fn with_maybe_none() -> eyre::Result<Self> {
        let cargo_metadata = {
            let metadata_command = cargo_metadata::MetadataCommand::new();
            metadata_command.exec().unwrap()
        };
        let building_crate = compile::get_building_crate(&cargo_metadata, &None)?;

        Ok(Self::Maybe {
            manifest_path: building_crate.manifest_path,
            package: building_crate.name.to_string(),
        })
    }

    /// Scans an exact provided `.wasm` artifact verifying magic byte signatures correctly to assign a resolution state.
    /// User-provided `.wasm` files are wrapped in `Original` to prevent deletion of the original file.
    pub fn with_wasm(path: Utf8PathBuf) -> eyre::Result<Self> {
        if path.extension() != Some("wasm") {
            eyre::bail!("Wasm file does not have .wasm extension: {path}");
        }
        if !fs::metadata(&path).is_ok() {
            eyre::bail!("Wasm file does not exist: {path}");
        }

        let mut file =
            fs::File::open(&path).wrap_err_with(|| format!("Failed to open wasm file: {path}"))?;
        let mut magic = [0u8; 8];
        file.read_exact(&mut magic)
            .wrap_err_with(|| format!("Failed to read magic number from wasm file: {path}"))?;
        if !magic.starts_with(b"\0asm") {
            eyre::bail!("Wasm file does not have valid magic number: {path}");
        }
        let version = &magic[4..8];
        // https://github.com/WebAssembly/component-model/blob/main/design/mvp/Binary.md#component-definitions
        if version == [0x0D, 0x00, 0x01, 0x00] {
            return Ok(Self::Component(path));
        }
        // https://webassembly.github.io/spec/core/binary/modules.html#binary-module
        if version != [0x01, 0x00, 0x00, 0x00] {
            eyre::bail!("Wasm file does not have valid version: {path}");
        }

        Ok(Self::Original {
            current: path.clone(),
            original: path,
        })
    }

    /// Forcefully invokes internal compiling mechanisms translating a declarative specification down into an executable binary structurally.
    pub fn definitely(
        &mut self,
        threads: bool,
        vfs_build_opts: &args::VfsBuildOptions,
    ) -> eyre::Result<()> {
        if let WasmPath::Maybe {
            manifest_path,
            package,
        } = self
        {
            let cargo_metadata = {
                let mut metadata_command = cargo_metadata::MetadataCommand::new();
                metadata_command.manifest_path(&manifest_path);
                metadata_command.exec().unwrap()
            };
            let building_crate =
                compile::get_building_crate(&cargo_metadata, &Some(package.clone()))?;
            let vfs_name = building_crate.name.to_string();

            let path = compile::build_vfs(
                Some(&manifest_path.to_string()),
                &building_crate,
                threads,
                vfs_build_opts,
            )
            .wrap_err_with(|| eyre::eyre!("Failed to build VFS: {vfs_name}"))?;
            *self = WasmPath::Definitely(path);
        }

        // Original variant is already resolved, no action needed.

        Ok(())
    }

    /// Returns the path to the WASM module.
    pub fn path(&self) -> eyre::Result<&Utf8PathBuf> {
        match self {
            WasmPath::Maybe { .. } => {
                eyre::bail!("WasmPath is not definitely set: {self:?}")
            }
            WasmPath::Definitely(p)
            | WasmPath::Component(p)
            | WasmPath::Original { current: p, .. } => Ok(p),
        }
    }

    /// Returns `true` if the given path is the original user-provided file that should be preserved.
    pub fn is_original(&self, path: &Utf8PathBuf) -> bool {
        matches!(self, WasmPath::Original { original, .. } if original == path)
    }

    /// Forcefully overrides the explicitly resolved file path for definite path targets.
    /// For `Original` variants, preserves the original path tracking.
    pub fn set_path(&mut self, path: Utf8PathBuf) -> eyre::Result<()> {
        match self {
            WasmPath::Maybe { .. } => {
                eyre::bail!("WasmPath is not definitely set: {path}")
            }
            WasmPath::Original { current, .. } => {
                // Keep the original path, only update current
                *current = path;
            }
            _ => {
                *self = WasmPath::Definitely(path);
            }
        }
        Ok(())
    }
}

/// Coordinates the underlying binary `wasm-merge` utility invocations to bundle outputs physically.
pub fn merge(
    vfs: &Utf8PathBuf,
    wasm: &[impl AsRef<std::path::Path>],
    output: impl AsRef<std::path::Path>,
    _threads: bool,
    dwarf: bool,
) -> eyre::Result<()> {
    let custom_section = {
        let mut vfs_module = walrus::Module::load(vfs, dwarf)?;
        let custom_section_names = vfs_module
            .customs
            .iter()
            .map(|(_, section)| section.name().to_string())
            .filter(|name| name.starts_with("component-type:"))
            .collect::<Vec<_>>();
        // let custom_section = vfs_module
        //     .customs.delete(custom_section_names)
        let custom_section = custom_section_names
            .iter()
            .filter_map(|id| vfs_module.customs.remove_raw(id))
            .collect::<Vec<_>>();

        custom_section
    };

    let mut merge_cmd = fallback_command::get_fallback_command("wasm-merge");

    // if threads {
    //     merge_cmd.arg("--enable-threads");
    // }

    if dwarf {
        merge_cmd.arg("--debuginfo");
    }

    merge_cmd.arg(vfs).arg(UniqueName::WASIP1_ABI_MODULE);

    for wasm in wasm {
        merge_cmd
            .arg(wasm.as_ref().as_os_str().to_str().unwrap())
            .arg(format!(
                "wasip1_vfs_{}",
                wasm.as_ref().get_file_main_name().unwrap()
            ));
    }

    merge_cmd
        .arg("--output")
        .arg(output.as_ref().as_os_str().to_str().unwrap())
        // .arg("--rename-export-conflicts")
        .args(["--enable-threads", "--enable-bulk-memory", "--enable-reference-types", "--enable-simd", "--enable-exception-handling", "--enable-shared-everything", "--enable-multivalue", "--enable-multimemory", "--enable-gc"]);

    let result = merge_cmd
        .spawn()
        .map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => eyre::eyre!(
                "wasm-merge command not found. Please install wasm-merge from https://github.com/WebAssembly/binaryen/releases/latest"
            ),
            _ => e.into(),
        })?
        .wait_with_output()
        .wrap_err("Failed to wait for wasm-merge process")?;

    if !result.success {
        let stdout = String::from_utf8_lossy(&result.stdout);
        let stderr = String::from_utf8_lossy(&result.stderr);
        let error_message = format!("stdout: {stdout}\nstderr: {stderr}");

        return Err(eyre::eyre!("wasm-merge command failed: {error_message}"));
    }

    let mut module = walrus::Module::load(output.as_ref(), dwarf)?;
    for section in custom_section {
        module.customs.add(section);
    }

    // to output
    fs::remove_file(output.as_ref()).expect("Failed to remove existing file");

    module
        .emit_wasm_file(output.as_ref())
        .expect("Failed to emit wasm file");

    Ok(())
}

macro_rules! _add_generators_by_type {
    ($runner:expr, $($ty:ty),* $(,)?) => {
        $(
            if let Some(_) = $runner.get_generator_ref::<$ty>().ok() {
                panic!("Generator of type {} already exists", std::any::type_name::<$ty>());
            }
            $runner.add_generator(<$ty>::default());
        )*
    };
}
pub(crate) use _add_generators_by_type as add_generators_by_type;


