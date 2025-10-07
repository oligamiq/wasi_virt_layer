use std::{collections::HashMap, fs, str::FromStr};

use camino::Utf8PathBuf;
use eyre::{Context as _, ContextCompat};
use itertools::Itertools;
use walrus::MemoryId;

use crate::{
    args::{self, TargetMemoryType},
    building,
    config_checker::TomlRestorers,
    merge,
    util::{CaminoUtilModule as _, LString, ResultUtil, WalrusUtilModule},
};

#[derive(Debug)]
pub struct GeneratorCtx {
    pub vfs_name: LString,
    pub target_names: Vec<LString>,
    pub vfs_used_memory_id: Option<MemoryId>,
    pub target_used_memory_id: Option<HashMap<LString, MemoryId>>,
    pub target_used_global_id: Option<HashMap<LString, Vec<walrus::GlobalId>>>,
    pub target_memory_type: TargetMemoryType,
    pub unstable_print_debug: bool,
    pub dwarf: bool,
    pub threads: bool,
    pub no_transpile: bool,
}

pub trait Generator: std::fmt::Debug + std::any::Any {
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
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        Ok(())
    }

    /// Operations performed after last optimizations.
    /// Generating debug functions is a delicate process,
    /// so in this case, output once per structure.
    #[allow(unused_variables)]
    fn post_all_optimize(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct ModuleExternal {
    pub name: LString,
}
impl ModuleExternal {
    pub fn new(name: &LString) -> Self {
        Self { name: name.clone() }
    }
}

#[derive(Debug)]
pub struct GeneratorRunner {
    pub generators: Vec<Box<dyn Generator + 'static>>,
    pub ctx: GeneratorCtx,
    pub path: WasmPath,
    pub targets: Vec<WasmPath>,
    pub toml_restorers: Option<TomlRestorers>,
    pub memory_hint: HashMap<LString, usize>,
}

impl GeneratorRunner {
    pub fn run_pre_vfs(
        generators: &mut Vec<Box<dyn Generator + 'static>>,
        ctx: &GeneratorCtx,
        module: &mut walrus::Module,
    ) -> eyre::Result<()> {
        for generator in generators {
            generator.pre_vfs(module, ctx)?;
        }
        Ok(())
    }

    pub fn run_pre_target(
        generators: &mut Vec<Box<dyn Generator + 'static>>,
        ctx: &GeneratorCtx,
        module: &mut walrus::Module,
        external: &ModuleExternal,
    ) -> eyre::Result<()> {
        for generator in generators {
            generator.pre_target(module, ctx, external)?;
        }
        Ok(())
    }

    pub fn run_post_combine(
        generators: &mut Vec<Box<dyn Generator + 'static>>,
        ctx: &GeneratorCtx,
        module: &mut walrus::Module,
    ) -> eyre::Result<()> {
        for generator in generators {
            generator.post_combine(module, ctx)?;
        }
        Ok(())
    }

    /// Only called if the target memory type is `Single`.
    pub fn run_post_lower_memory(
        generators: &mut Vec<Box<dyn Generator + 'static>>,
        ctx: &GeneratorCtx,
        module: &mut walrus::Module,
    ) -> eyre::Result<()> {
        for generator in generators {
            generator.post_lower_memory(module, ctx)?;
        }
        Ok(())
    }

    pub fn run_post_components(
        generators: &mut Vec<Box<dyn Generator + 'static>>,
        ctx: &GeneratorCtx,
        module: &mut walrus::Module,
    ) -> eyre::Result<()> {
        for generator in generators {
            generator.post_components(module, ctx)?;
        }
        Ok(())
    }
}

pub(crate) trait WrapRunner<T> {
    #[allow(unused_variables)]
    fn wrap_run(self, path: &mut WasmPath, dwarf: bool) -> eyre::Result<T>
    where
        Self: Sized;
}

impl<T, F: FnOnce(&mut walrus::Module) -> eyre::Result<T>> WrapRunner<T> for F {
    fn wrap_run(self, path: &mut WasmPath, dwarf: bool) -> eyre::Result<T> {
        let old_path = path.path()?;
        let module =
            &mut walrus::Module::load(old_path, dwarf).wrap_err("Failed to load Wasm module")?;

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

        path.set_path(new_path)?;

        Ok(result)
    }
}

pub(crate) trait EndWithOpt<T> {
    #[allow(unused_variables)]
    fn with_opt(self, path: &mut WasmPath, dwarf: bool) -> eyre::Result<T>
    where
        Self: Sized;

    #[allow(dead_code)]
    fn with_opt_args(
        self,
        path: &mut WasmPath,
        args: &[&str],
        require_update: bool,
        dwarf: bool,
    ) -> eyre::Result<T>
    where
        Self: Sized;
}

impl<T, F: FnOnce(&mut WasmPath) -> eyre::Result<T>> EndWithOpt<T> for F {
    fn with_opt(self, path: &mut WasmPath, dwarf: bool) -> eyre::Result<T>
    where
        Self: Sized,
    {
        let result = (self)(path).wrap_err("Failed to run with with_opt")?;

        println!("Optimizing Wasm...");
        let new_path = building::optimize_wasm(path.path()?, &[], false, dwarf)
            .wrap_err("Failed to optimize Wasm")?;

        path.set_path(new_path)?;

        Ok(result)
    }

    fn with_opt_args(
        self,
        path: &mut WasmPath,
        args: &[&str],
        require_update: bool,
        dwarf: bool,
    ) -> eyre::Result<T>
    where
        Self: Sized,
    {
        let result = (self)(path).wrap_err("Failed to run with with_opt_args")?;

        println!("Optimizing Wasm... with args: {}", args.iter().join(" "));
        let new_path = building::optimize_wasm(path.path()?, args, require_update, dwarf)
            .wrap_err("Failed to optimize Wasm")?;

        path.set_path(new_path)?;

        Ok(result)
    }
}

impl GeneratorRunner {
    pub fn new(
        path: WasmPath,
        targets: Vec<WasmPath>,
        threads: bool,
        dwarf: bool,
        unstable_print_debug: bool,
        no_transpile: bool,
        memory_type: TargetMemoryType,
        toml_restorers: TomlRestorers,
        memory_hint: Vec<Option<usize>>,
    ) -> eyre::Result<Self> {
        let target_names = targets
            .iter()
            .map(|t| Ok(t.name()?.into()))
            .collect::<eyre::Result<Vec<LString>>>()?;

        let memory_hint = memory_hint
            .into_iter()
            .zip(target_names.iter().cloned())
            .filter_map(|(hint, name)| hint.map(|h| (name, h)))
            .collect::<HashMap<_, _>>();

        Ok(Self {
            generators: Vec::new(),
            ctx: GeneratorCtx {
                vfs_name: path.name()?.into(),
                target_names,
                target_memory_type: memory_type,
                unstable_print_debug,
                dwarf,
                threads,
                no_transpile,
                vfs_used_memory_id: None,
                target_used_memory_id: None,
                target_used_global_id: None,
            },
            path,
            targets,
            toml_restorers: Some(toml_restorers),
            memory_hint,
        })
    }

    pub fn add_generator<G: Generator + 'static>(&mut self, generator: G) {
        self.generators.push(Box::new(generator));
    }

    pub fn insert_generator<G: Generator + 'static>(&mut self, index: usize, generator: G) {
        self.generators.insert(index, Box::new(generator));
    }

    pub fn get_generator_ref<T: Generator + 'static>(&self) -> eyre::Result<&T> {
        fn downcast_ref<T: 'static>(b: &dyn std::any::Any) -> Option<&'_ T> {
            if b.is::<T>() {
                Some(b.downcast_ref::<T>().unwrap())
            } else {
                None
            }
        }

        self.generators
            .iter()
            .find_map(|g| downcast_ref::<T>(g))
            .wrap_err("Failed to get generator")
    }

    #[deprecated(
        note = "Ensure this function is self-contained. This is a temporary measure for debugging purposes."
    )]
    pub const fn path(&self) -> &WasmPath {
        &self.path
    }

    #[deprecated(
        note = "Ensure this function is self-contained. This is a temporary measure for debugging purposes."
    )]
    pub const fn targets(&self) -> &Vec<WasmPath> {
        &self.targets
    }

    pub const fn ctx(&self) -> &GeneratorCtx {
        &self.ctx
    }

    pub fn definitely(&mut self) -> eyre::Result<()> {
        self.path.definitely(self.ctx.threads)?;
        for target in &mut self.targets {
            target.definitely(self.ctx.threads)?;
        }
        Ok(())
    }

    pub fn run_layers_to_component(&mut self, out_dir: &Utf8PathBuf) -> eyre::Result<()> {
        self.definitely()?;

        let toml_restorers = self
            .toml_restorers
            .take()
            .ok_or_else(|| eyre::eyre!("TomlRestorers already taken"))?;

        toml_restorers
            .restore()
            .wrap_err("Failed to restore toml files")?;

        let mut mem_holder = MemoryIDVisitor {
            memory_hint: self.memory_hint.clone(),
            used_vfs_memory_id: None,
            used_target_memory_id: None,
        };

        let dwarf = self.ctx.dwarf;

        println!("Adjusting VFS Wasm...");
        (|path: &mut WasmPath| {
            (|module: &mut walrus::Module| {
                mem_holder
                    .pre_vfs(module, &self.ctx)
                    .wrap_err("Failed in pre_vfs")?;

                self.ctx.vfs_used_memory_id = mem_holder.used_vfs_memory_id;

                Self::run_pre_vfs(&mut self.generators, &self.ctx, module)
                    .wrap_err("Failed in run_pre_vfs")
            })
            .wrap_run(path, dwarf)
        })
        .with_opt(&mut self.path, dwarf)?;

        println!("Adjusting target Wasm...");

        for (target, target_name) in self.targets.iter_mut().zip(self.ctx.target_names.clone()) {
            (|path: &mut WasmPath| {
                (|module: &mut walrus::Module| {
                    let external = ModuleExternal::new(&target_name);
                    mem_holder
                        .pre_target(module, &self.ctx, &external)
                        .wrap_err("Failed in pre_target")?;

                    self.ctx.target_used_memory_id = mem_holder.used_target_memory_id.clone();

                    Self::run_pre_target(&mut self.generators, &self.ctx, module, &external)
                        .wrap_err("Failed in run_pre_target")
                })
                .wrap_run(path, dwarf)
            })
            .with_opt(target, dwarf)?;
        }

        println!("Combining Wasm modules...");
        let output = format!("{out_dir}/merged.wasm");
        (|path: &mut WasmPath| {
            merge::merge(
                path.path()?,
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

            path.set_path(output.into())
        })
        .with_opt(&mut self.path, dwarf)?;

        println!("Adjusting Merged Wasm...");
        (|path: &mut WasmPath| {
            (|module: &mut walrus::Module| {
                mem_holder
                    .post_combine(module, &self.ctx)
                    .wrap_err("Failed in post_combine")?;

                self.ctx.vfs_used_memory_id = mem_holder.used_vfs_memory_id.take();
                self.ctx.target_used_memory_id = mem_holder.used_target_memory_id.take();

                Self::run_post_combine(&mut self.generators, &self.ctx, module)
                    .wrap_err("Failed in run_post_combine")
            })
            .wrap_run(path, dwarf)
        })
        .with_opt(&mut self.path, dwarf)?;

        if self.ctx.target_memory_type == TargetMemoryType::Single {
            println!("Generating single memory Merged Wasm...");
            let optimized_path = building::optimize_wasm(
                self.path.path()?,
                &["--multi-memory-lowering"],
                true,
                dwarf,
            )?;
            self.path.set_path(optimized_path)?;

            (|path: &mut WasmPath| {
                (|module: &mut walrus::Module| {
                    Self::run_post_lower_memory(&mut self.generators, &self.ctx, module)
                        .wrap_err("Failed in run_post_lower_memory")
                })
                .wrap_run(path, dwarf)
            })
            .with_opt(&mut self.path, dwarf)?;
        }

        println!("Translating Wasm to Component...");
        let component = building::wasm_to_component(self.path.path()?, &self.ctx.target_names)
            .wrap_err("Failed to translate Wasm to Component")?;
        self.path.set_path(component)?;

        // todo!();
        let mem_size_visitor = MemorySizeVisitor::default();
        self.add_generator(mem_size_visitor);

        println!("Adjusting component Merged Wasm...");
        (|path: &mut WasmPath| {
            (|module: &mut walrus::Module| {
                Self::run_post_components(&mut self.generators, &self.ctx, module)
                    .wrap_err("Failed in run_post_components")
            })
            .wrap_run(path, dwarf)
        })
        .with_opt(&mut self.path, dwarf)?;

        if self.ctx.no_transpile {
            println!("Skipping transpiling Component to JS as per --no-transpile flag...");
            return Ok(());
        }

        Ok(())
    }

    pub fn component_to_files(
        &mut self,
        parsed_args: &args::Args,
    ) -> eyre::Result<(String, Vec<(u64, u64)>)> {
        let dwarf = self.ctx.dwarf;
        let out_dir = &parsed_args.out_dir;

        println!("Translating Component to JS...");
        let core_wasm_path = (|path: &mut WasmPath| {
            let binary = std::fs::read(path.path()?).wrap_err("Failed to read component")?;
            let transpiled = parsed_args
                .transpile_to_js(&binary, &self.ctx.vfs_name)
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
                                std::fs::create_dir_all(&joined_dir).wrap_err_with(|| {
                                    eyre::eyre!("Failed to create directory: {joined_dir}")
                                })?;
                            }
                        }
                    }
                    std::fs::write(&file_name, &data).wrap_err_with(|| {
                        eyre::eyre!("Failed to write transpiled file: {file_name}")
                    })?;
                }
            }

            let core_wasm = core_wasm
                .as_ref()
                .ok_or_else(|| eyre::eyre!("Failed to find core wasm"))?;

            path.set_path(core_wasm.clone())?;

            Ok(core_wasm.clone())
        })
        .with_opt(&mut self.path, dwarf)?;

        // If it cannot be done in the component state, do it here.
        // println!("Adjusting component Merged Wasm...");
        // (|path: &mut WasmPath| {
        //     (|module: &mut walrus::Module| {
        //         Self::run_post_components(&mut self.generators, &self.ctx, module)
        //             .wrap_err("Failed in run_post_components")
        //     })
        //     .wrap_run(path, dwarf)
        // })
        // .with_opt(&mut self.path, dwarf)?;

        println!("Final optimizing Merged Wasm...");
        for generator in &mut self.generators {
            (|module: &mut walrus::Module| {
                generator
                    .post_all_optimize(module, &self.ctx)
                    .wrap_err("Failed in post_all_optimize")
            })
            .wrap_run(&mut self.path, dwarf)?;
        }

        std::fs::rename(self.path.path()?, &core_wasm_path).wrap_err_with(|| {
            eyre::eyre!(
                "Failed to rename final wasm from {} to {}",
                self.path.path().unwrap(),
                core_wasm_path
            )
        })?;

        Ok((
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
    pub memory_hint: HashMap<LString, usize>,
    pub used_vfs_memory_id: Option<MemoryId>,
    pub used_target_memory_id: Option<HashMap<LString, MemoryId>>,
}

impl Generator for MemoryIDVisitor {
    fn pre_vfs(
        &mut self,
        module: &mut walrus::Module,
        _: &crate::generator::GeneratorCtx,
    ) -> eyre::Result<()> {
        println!("Finding VFS memory id...");

        let id = module
            .get_target_memory_id("vfs", false)
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
            .get_target_memory_id("vfs", true)
            .wrap_err("Failed to find used memory id after combine")?;
        self.used_vfs_memory_id = Some(id);

        self.used_target_memory_id.get_or_insert_default().clear();
        for wasm in &ctx.target_names {
            let id = module
                .get_target_memory_id(wasm, true)
                .wrap_err("Failed to find used memory id after combine")?;
            self.used_target_memory_id
                .as_mut()
                .unwrap()
                .insert(wasm.clone(), id);
        }

        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
struct MemorySizeVisitor {
    mem_size: Option<Vec<(u64, u64)>>,
}

impl Generator for MemorySizeVisitor {
    fn post_components(
        &mut self,
        module: &mut walrus::Module,
        _: &GeneratorCtx,
    ) -> eyre::Result<()> {
        let mem_size = module
            .memories
            .iter()
            .map(|mem| {
                (
                    mem.initial as u64,
                    mem.maximum.unwrap_or(mem.initial) as u64,
                )
            })
            .collect::<Vec<_>>();
        self.mem_size = Some(mem_size);

        Ok(())
    }
}

pub struct GlobalIdVisitor {
    pub global_id: Option<walrus::GlobalId>,
}

#[derive(Debug, Clone)]
pub enum WasmPath {
    Maybe {
        manifest_path: Utf8PathBuf,
        package: String,
    },
    Definitely(Utf8PathBuf),
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
    pub fn name(&self) -> eyre::Result<String> {
        match self {
            WasmPath::Maybe { package, .. } => Ok(package.clone()),
            WasmPath::Definitely(path) => path
                .get_file_main_name()
                .ok_or_else(|| eyre::eyre!("Failed to get file name from {path}")),
        }
    }

    pub fn manifest_path(&self) -> Option<&Utf8PathBuf> {
        match self {
            WasmPath::Maybe { manifest_path, .. } => Some(manifest_path),
            WasmPath::Definitely(_) => None,
        }
    }

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
            WasmPath::Definitely(_) => None,
        }
    }

    pub const fn with_maybe(manifest_path: Utf8PathBuf, package: String) -> Self {
        Self::Maybe {
            manifest_path,
            package,
        }
    }

    pub fn with_maybe_only_manifest(manifest_path: Utf8PathBuf) -> eyre::Result<Self> {
        let cargo_metadata = {
            let mut metadata_command = cargo_metadata::MetadataCommand::new();
            metadata_command.manifest_path(&manifest_path);
            metadata_command.exec().unwrap()
        };
        let building_crate = building::get_building_crate(&cargo_metadata, &None)?;

        Ok(Self::Maybe {
            manifest_path,
            package: building_crate.name.to_string(),
        })
    }

    pub fn with_maybe_only_package(package: String) -> eyre::Result<Self> {
        let cargo_metadata = {
            let metadata_command = cargo_metadata::MetadataCommand::new();
            metadata_command.exec().unwrap()
        };
        let building_crate = building::get_building_crate(&cargo_metadata, &Some(package.clone()))?;

        Ok(Self::Maybe {
            manifest_path: building_crate.manifest_path,
            package: building_crate.name.to_string(),
        })
    }

    pub fn with_maybe_none() -> eyre::Result<Self> {
        let cargo_metadata = {
            let metadata_command = cargo_metadata::MetadataCommand::new();
            metadata_command.exec().unwrap()
        };
        let building_crate = building::get_building_crate(&cargo_metadata, &None)?;

        Ok(Self::Maybe {
            manifest_path: building_crate.manifest_path,
            package: building_crate.name.to_string(),
        })
    }

    pub fn with_wasm(path: Utf8PathBuf) -> eyre::Result<Self> {
        if path.extension() != Some("wasm") {
            eyre::bail!("Wasm file does not have .wasm extension: {path}");
        }
        if !fs::metadata(&path).is_ok() {
            eyre::bail!("Wasm file does not exist: {path}");
        }
        Ok(Self::Definitely(path))
    }

    pub fn definitely(&mut self, threads: bool) -> eyre::Result<()> {
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
                building::get_building_crate(&cargo_metadata, &Some(package.clone()))?;
            let vfs_name = building_crate.name.to_string();

            let path =
                building::build_vfs(Some(&manifest_path.to_string()), &building_crate, threads)
                    .wrap_err_with(|| eyre::eyre!("Failed to build VFS: {vfs_name}"))?;
            *self = WasmPath::Definitely(path);
        }

        Ok(())
    }

    pub fn path(&self) -> eyre::Result<&Utf8PathBuf> {
        match self {
            WasmPath::Maybe { .. } => {
                eyre::bail!("WasmPath is not definitely set: {self:?}")
            }
            WasmPath::Definitely(p) => Ok(p),
        }
    }

    pub fn set_path(&mut self, path: Utf8PathBuf) -> eyre::Result<()> {
        if matches!(self, WasmPath::Maybe { .. }) {
            eyre::bail!("WasmPath is not definitely set: {path}")
        }
        *self = WasmPath::Definitely(path);
        Ok(())
    }
}
