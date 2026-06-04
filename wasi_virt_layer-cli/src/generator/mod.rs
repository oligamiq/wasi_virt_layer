use std::{any::Any, collections::HashMap, fs, io::Read as _, str::FromStr};

use camino::Utf8PathBuf;
use compact_str::{CompactString, ToCompactString as _};
use eyre::{Context as _, ContextCompat};
use itertools::Itertools;

use crate::{
    args::{self, TargetMemoryType},
    compile,
    config_checker::TomlRestorers,
    fallback_command,
    unique_name::{MemoryUniqueName, UniqueName},
    util::{CaminoUtilModule as _, ResultUtil, WasmName, WasmNameHolder},
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
    pub wrap_unreachable_targets: std::collections::HashSet<String>,
    pub main_void_synthesized_targets: Option<std::collections::HashSet<String>>,
}

/// Sub-context for extracting and storing component variables during execution.
#[derive(Debug, Default)]
pub struct ComponentCtx {
    /// Optional name of the VFS module.
    pub vfs_name: Option<compact_str::CompactString>,
    /// Optional names of the target modules.
    pub target_names: Option<Box<[compact_str::CompactString]>>,
    /// Optional memory type of the target modules.
    pub target_memory_type: Option<args::TargetMemoryType>,
    /// Optional flag for unstable debug printing.
    pub unstable_print_debug: Option<bool>,
    /// Flag for including DWARF debug information.
    pub dwarf: bool,
    /// Optional flag for multi-threading support.
    pub threads: Option<bool>,
    /// Flag for ABI adjustment.
    pub adjust_abi: bool,
}

impl ComponentCtx {
    /// Creates a new `ComponentCtx` with the specified context parameters.
    pub fn new(
        vfs_name: compact_str::CompactString,
        target_names: Box<[compact_str::CompactString]>,
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
    pub fn vfs_name(&self) -> &compact_str::CompactString {
        self.vfs_name.as_ref().unwrap()
    }

    /// Returns the names of the target modules.
    pub fn target_names(&self) -> &Box<[compact_str::CompactString]> {
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

/// Coordinates iterating over registered generators and merging external module logic into the VFS.
#[derive(Debug)]
pub struct GeneratorRunner {
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

impl<T, F: FnOnce() -> eyre::Result<T>> WrapRunner<T> for F {
    fn wrap_run(
        self,
        path: &mut WasmPath,
        _dwarf: bool,
        keep_build_artifacts: bool,
        mut stream_pipeline: Option<crate::wasm_stream::pipeline::Pipeline>,
    ) -> eyre::Result<T> {
        let old_path = path.path()?.clone();

        let result = (self)()?;

        if let Some(pipeline) = &mut stream_pipeline {
            let input_wasm = fs::read(&old_path).wrap_err("Failed to read Wasm file")?;
            let output_wasm = pipeline
                .run(&input_wasm)
                .wrap_err("Failed to run StreamPipeline pre-walrus")?;

            let new_path = old_path.with_extension("adjusted.wasm");

            if fs::metadata(&new_path).is_ok() {
                fs::remove_file(&new_path)
                    .wrap_err_with(|| format!("Failed to remove existing file {new_path}"))?;
            }

            std::fs::write(&new_path, &output_wasm)
                .wrap_err_with(|| format!("Failed to write adjusted Wasm to {new_path}"))?;

            if !keep_build_artifacts && !path.is_original(&old_path) {
                std::fs::remove_file(&old_path).unwrap_or_else(|e| {
                    log::warn!("Failed to remove intermediate file {old_path}: {e}")
                });
            }

            path.set_path(new_path)?;
        }

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

                vfs_is_library: false,

                wrap_unreachable_targets: std::collections::HashSet::new(),
                main_void_synthesized_targets: None,
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

        let dwarf = self.ctx.dwarf;

        println!("Remove existing output directory...");
        if std::fs::metadata(&out_dir).is_ok() {
            std::fs::remove_dir_all(&out_dir).expect("Failed to remove existing directory");
        }
        std::fs::create_dir_all(&out_dir).expect("Failed to create output directory");

        println!("Adjusting VFS Wasm...");
        let skip_vfs_opt = self.vfs_build_opts.no_opt > 0 || self.vfs_build_opts.no_opt_all > 0;
        (|path: &mut WasmPath| {
            let wasm_bytes = std::fs::read(&path.path()?).unwrap();
            let mut vfs_is_library = true;
            let mut wrap_unreachable_targets = std::collections::HashSet::new();
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
                            for target in self.ctx.target_names.iter() {
                                let marker = format!(
                                    "__wasip1_virt_layer_{}_wrap_unreachable",
                                    target.as_ref()
                                );
                                if e.name == marker {
                                    wrap_unreachable_targets.insert(target.as_ref().to_string());
                                }
                            }
                        }
                    }
                }
            }
            self.ctx.vfs_is_library = vfs_is_library;
            self.ctx.wrap_unreachable_targets = wrap_unreachable_targets;
            let pipeline_is_library = vfs_is_library;
            let cloned_ctx = self.ctx.clone();

            (|| {
                // Detect VFS library mode: if the VFS module has no start section,
                // it is a library and has no initialization entry point.
                if pipeline_is_library {
                    log::info!(
                        "VFS module `{}` has no start section — treating as a library.",
                        self.ctx.vfs_name
                    );
                }

                Ok(())
            })
            .wrap_run(path, dwarf, keep_build_artifacts, {
                let mut pipeline = crate::wasm_stream::pipeline::Pipeline::new();
                use crate::wasm_stream::passes::{
                    abi_connect::{
                        ConnectWasip1ABIPreVfsStreamPass, NonRecursiveWasiABIPreVfsStreamPass,
                    },
                    anonymous::AnonymousStreamPass,
                    check::CheckUseWasiVirtLayerChecker,
                    dummy_injector::DummyInjectorStreamPass,
                    patch_component::PatchComponentStreamPass,
                    pre_vfs_memory_refuge::TemporaryRefugeMemoryStreamPass,
                    starts_pre::StartsPreStreamPass,
                };

                let check_pass =
                    crate::wasm_stream::pipeline::ParallelCheckStreamPass::new(vec![Box::new(
                        CheckUseWasiVirtLayerChecker::new(),
                    )]);
                pipeline.add_pass(Box::new(check_pass));
                pipeline.add_pass(Box::new(AnonymousStreamPass::new(cloned_ctx.clone())));
                pipeline.add_pass(Box::new(ConnectWasip1ABIPreVfsStreamPass::new()));
                pipeline.add_pass(Box::new(
                    crate::wasm_stream::passes::threads_spawn::ThreadsSpawnPreVfsStreamPass::new(
                        cloned_ctx.threads,
                    ),
                ));
                pipeline.add_pass(Box::new(NonRecursiveWasiABIPreVfsStreamPass::new()));
                pipeline.add_pass(Box::new(PatchComponentStreamPass::new()));

                let fn_in_starts =
                    crate::wasm_stream::passes::fn_in_starts::FnInStarts::new::<String>(&[]);
                pipeline.add_pass(Box::new(StartsPreStreamPass::new(
                    true,
                    pipeline_is_library,
                    fn_in_starts.flesh_vfs_start.clone(),
                )));
                pipeline.add_pass(Box::new(DummyInjectorStreamPass::new(vec![
                    fn_in_starts.thread_patch.clone(),
                    fn_in_starts.init_offset_global.clone(),
                    fn_in_starts.save_target_memory.clone(),
                    fn_in_starts.simple_debug_pre_init.clone(),
                ])));
                pipeline.add_pass(Box::new(TemporaryRefugeMemoryStreamPass::new(None)));
                Some(pipeline)
            })
        })
        .with_opt(&mut self.path, dwarf, keep_build_artifacts, skip_vfs_opt)?;

        println!("Adjusting target Wasm...");

        for (i, (target, target_name)) in self
            .targets
            .iter_mut()
            .zip(self.ctx.target_names.clone())
            .enumerate()
        {
            let skip_target_opt = self.vfs_build_opts.no_opt_all > 0
                || self.target_vfs_build_opts[i].no_opt > 0
                || self.target_vfs_build_opts[i].no_opt_all > 0;
            let _cloned_ctx = self.ctx.clone();
            let is_synthesized = std::sync::Arc::new(std::sync::Mutex::new(false));
            (|path: &mut WasmPath| {
                (|| {
                    Ok(())
                })
                .wrap_run(path, dwarf, keep_build_artifacts, {
                    let mut pipeline = crate::wasm_stream::pipeline::Pipeline::new();
                    use crate::wasm_stream::passes::{
                        abi_connect::{
                            ConnectWasip1ABIPreTargetStreamPass,
                            ConnectWasip1ThreadsABIPreTargetStreamPass,
                        },
                        check::IsRustWasmChecker,
                        dummy_injector::DummyInjectorStreamPass,
                        pre_vfs_memory_refuge::TemporaryRefugeMemoryStreamPass,
                        producer::ProducerStreamPass,
                        starts_pre::StartsPreStreamPass,
                        special_func::SpecialFuncPreTargetStreamPass,
                        wrap_unreachable::WrapUnreachablePreTargetStreamPass,
                        atomic_patch::AtomicPatchStreamPass,
                    };

                    pipeline.add_pass(Box::new(ProducerStreamPass::new()));
                    pipeline.add_pass(Box::new(SpecialFuncPreTargetStreamPass::new(
                        target_name.to_string(),
                        is_synthesized.clone(),
                    )));

                    let is_opted_in = _cloned_ctx.wrap_unreachable_targets.contains(&target_name.to_string());
                    pipeline.add_pass(Box::new(WrapUnreachablePreTargetStreamPass::new(
                        target_name.to_string(),
                        is_opted_in,
                    )));

                    pipeline.add_pass(Box::new(ConnectWasip1ABIPreTargetStreamPass::new(
                        target_name.to_string(),
                    )));
                    pipeline.add_pass(Box::new(ConnectWasip1ThreadsABIPreTargetStreamPass::new(
                        target_name.to_string(),
                    )));
                    pipeline.add_pass(Box::new(crate::wasm_stream::passes::threads_spawn::ThreadsSpawnPreTargetStreamPass::new(
                        _cloned_ctx.threads,
                        target_name.to_string(),
                    )));
                    pipeline.add_pass(Box::new(AtomicPatchStreamPass::new(
                        _cloned_ctx.threads,
                        i as u32,
                    )));

                    let check_pass = crate::wasm_stream::pipeline::ParallelCheckStreamPass::new(
                        vec![Box::new(IsRustWasmChecker::new())],
                    );
                    pipeline.add_pass(Box::new(check_pass));

                    let export_name = format!("__flesh_{}_start", target_name);
                    pipeline.add_pass(Box::new(StartsPreStreamPass::new(
                        false,
                        false,
                        export_name.clone(),
                    )));
                    pipeline.add_pass(Box::new(DummyInjectorStreamPass::new(vec![export_name])));
                    let new_memory_name = crate::generator::UniqueName::Memory(
                        &crate::unique_name::MemoryUniqueName::Memory(&target_name),
                    )
                    .to_string();
                    pipeline.add_pass(Box::new(TemporaryRefugeMemoryStreamPass::new(Some(
                        new_memory_name,
                    ))));

                    Some(pipeline)
                })
            })
            .with_opt(target, dwarf, keep_build_artifacts, skip_target_opt)?;

            if *is_synthesized.lock().unwrap() {
                self.ctx
                    .main_void_synthesized_targets
                    .get_or_insert_default()
                    .insert(target_name.to_string());
            }
        }

        let skip_all_opt = self.vfs_build_opts.no_opt_all > 0
            || self
                .target_vfs_build_opts
                .iter()
                .any(|opts| opts.no_opt_all > 0);

        println!("Combining Wasm modules...");

        let output = format!("{out_dir}/merged.wasm");
        let mut defined_funcs_counts = Vec::new();
        (|path: &mut WasmPath| {
            let old_path = path.path()?.clone();
            defined_funcs_counts.push(count_defined_funcs(&old_path)?);
            for target in self.targets.iter() {
                defined_funcs_counts.push(count_defined_funcs(target.path()?)?);
            }
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

        println!("Adjusting Merged Wasm (streaming pipeline)...");

        (|path: &mut WasmPath| {
            let old_path = path.path()?.clone();
            let input_wasm = std::fs::read(&old_path).wrap_err("Failed to read Wasm file")?;

            let mut pipeline = crate::wasm_stream::pipeline::Pipeline::new();
            let target_names: Vec<String> = self.ctx.target_names.iter().map(|n| n.as_ref().to_string()).collect();

            let vfs_name = self.ctx.vfs_name.as_ref().to_string();
            pipeline.add_pass(Box::new(
                crate::wasm_stream::passes::post_combine::PostCombineStreamPass::new(vfs_name, target_names.clone(), defined_funcs_counts.clone())
            ));

            pipeline.add_pass(Box::new(
                crate::wasm_stream::passes::poll::PollWaitStreamPass::new(self.ctx.threads, 0)
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
        let component = compile::wasm_to_component(&old_path, &self.ctx.target_names)
            .wrap_err("Failed to translate Wasm to Component")?;
        let new_component = format!("{out_dir}/{}.component.wasm", self.ctx.vfs_name);
        let component_bytes = std::fs::read(&component)?;
        let component_bytes_with_ctx =
            crate::wasm_stream::passes::component_ctx::append_component_ctx(
                &component_bytes,
                &self.ctx,
            )?;
        std::fs::write(&new_component, &component_bytes_with_ctx)?;

        if !keep_build_artifacts {
            std::fs::remove_file(&component)
                .wrap_err_with(|| format!("Failed to remove existing file {component}"))?;
            std::fs::remove_file(&old_path)
                .wrap_err_with(|| format!("Failed to remove existing file {old_path}"))?;
        }

        self.path.set_path(Utf8PathBuf::from(new_component))?;

        Ok(ComponentRunner {
            ctx: None,
            path: self.path,
            wasm_name_holder: None,
        })
    }
}

impl ComponentRunner {
    /// Instantiates a fresh `ComponentRunner` targeting a specific underlying WebAssembly structure.
    pub fn new(path: WasmPath) -> Self {
        Self {
            ctx: None,
            path,
            wasm_name_holder: None,
        }
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
        let component_bytes = std::fs::read(&self.path.path()?)?;
        self.ctx =
            Some(crate::wasm_stream::passes::component_ctx::read_component_ctx(&component_bytes)?);

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
        .with_opt(
            &mut self.path,
            dwarf,
            parsed_args.keep_build_artifacts(),
            parsed_args.dev(),
        )?;

        let mem_sizes = {
            let wasm_bytes =
                std::fs::read(&self.path.path()?).wrap_err("Failed to read core wasm")?;
            crate::wasm_stream::passes::extract_mem_sizes::extract_memory_sizes(&wasm_bytes)?
        };

        println!("Adjusting component Merged Wasm...");
        (|path: &mut WasmPath| {
            let old_path = path.path()?.clone();
            let input_wasm = std::fs::read(&old_path).wrap_err("Failed to read Wasm file")?;

            let mut pipeline = crate::wasm_stream::pipeline::Pipeline::new();

            pipeline.add_pass(Box::new(
                crate::wasm_stream::passes::memory_post_components::PostComponentsMemoryFixStreamPass::new(self.ctx.as_ref().unwrap().threads.unwrap_or(false))
            ));

            let output_wasm = pipeline.run(&input_wasm).wrap_err("Failed to run StreamPipeline")?;
            let new_path = old_path.with_extension("post-comp-stream.wasm");
            std::fs::write(&new_path, output_wasm).wrap_err("Failed to write Wasm file")?;

            if !parsed_args.keep_build_artifacts() {
                std::fs::remove_file(&old_path).unwrap_or_default();
            }
            path.set_path(new_path)?;

            Ok(())
        })
        .with_opt(
            &mut self.path,
            dwarf,
            parsed_args.keep_build_artifacts(),
            parsed_args.dev(),
        )?;

        let _dwarf = {
            let new_dwarf = self.ctx.as_ref().unwrap().dwarf;
            if dwarf && !new_dwarf {
                log::warn!(
                    "Dwarf was disabled in component processing, you should re-run with --dwarf"
                );
            }
            new_dwarf
        };

        std::fs::rename(self.path.path()?, &core_wasm_path).wrap_err_with(|| {
            eyre::eyre!(
                "Failed to rename final wasm from {} to {}",
                self.path.path().unwrap(),
                core_wasm_path
            )
        })?;

        Ok((
            self.ctx.as_ref().unwrap().threads.unwrap_or(false),
            core_wasm_path
                .get_file_main_name()
                .ok_or_else(|| eyre::eyre!("Failed to get file name"))?,
            mem_sizes,
        ))
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
fn count_defined_funcs(wasm_path: &camino::Utf8PathBuf) -> eyre::Result<u32> {
    let wasm = std::fs::read(wasm_path)?;
    for payload in wasmparser::Parser::new(0).parse_all(&wasm) {
        let payload = payload.map_err(|e| eyre::eyre!("Parse error: {}", e))?;
        if let wasmparser::Payload::FunctionSection(s) = payload {
            return Ok(s.count());
        }
    }
    Ok(0)
}

pub fn merge(
    vfs: &Utf8PathBuf,
    wasm: &[impl AsRef<std::path::Path>],
    output: impl AsRef<std::path::Path>,
    _threads: bool,
    dwarf: bool,
) -> eyre::Result<()> {
    let custom_sections = {
        let wasm = std::fs::read(vfs)?;
        let mut sections = Vec::new();
        for payload in wasmparser::Parser::new(0).parse_all(&wasm) {
            let payload = payload?;
            if let wasmparser::Payload::CustomSection(c) = payload {
                if c.name().starts_with("component-type:") {
                    sections.push((c.name().to_string(), c.data().to_vec()));
                }
            }
        }
        sections
    };

    let mut merge_cmd = fallback_command::get_fallback_command("wasm-merge");

    if _threads {
        merge_cmd.arg("--enable-threads");
        merge_cmd.arg("--enable-multimemory");
    } else {
        merge_cmd.arg("--enable-multimemory");
    }

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
        .args([
            "--enable-threads",
            "--enable-bulk-memory",
            "--enable-reference-types",
            "--enable-simd",
            "--enable-exception-handling",
            "--enable-shared-everything",
            "--enable-multivalue",
            "--enable-multimemory",
            "--enable-gc",
        ]);

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

    let mut module_bytes = std::fs::read(output.as_ref())?;
    for (name, data) in custom_sections {
        let section = wasm_encoder::CustomSection {
            name: name.into(),
            data: data.into(),
        };
        wasm_encoder::Section::append_to(&section, &mut module_bytes);
    }
    std::fs::write(output.as_ref(), &module_bytes)?;

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
