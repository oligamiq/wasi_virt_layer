use std::{collections::HashMap, fs, str::FromStr};

use camino::Utf8PathBuf;
use eyre::{Context as _, ContextCompat};
use itertools::Itertools;
use walrus::MemoryId;

use crate::{
    args::TargetMemoryType,
    building,
    config_checker::TomlRestorers,
    util::{CaminoUtilModule as _, ResultUtil, WalrusUtilModule},
};

#[derive(Debug)]
pub struct GeneratorCtx {
    pub vfs_name: String,
    pub target_names: Vec<String>,
    pub vfs_used_memory_id: Option<MemoryId>,
    pub target_used_memory_id: Option<Vec<MemoryId>>,
    pub target_memory_type: TargetMemoryType,
    pub unstable_print_debug: bool,
    pub dwarf: bool,
    pub threads: bool,
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
    pub name: String,
}

#[derive(Debug)]
pub struct GeneratorRunner {
    pub generators: Vec<Box<dyn Generator + 'static>>,
    pub ctx: GeneratorCtx,
    pub path: WasmPath,
    pub targets: Vec<WasmPath>,
    pub toml_restorers: Option<TomlRestorers>,
    pub memory_hint: HashMap<String, usize>,
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

pub(crate) trait WrapRunner<const N: usize> {
    #[allow(unused_variables)]
    fn wrap_run_with_external(
        self,
        generators: &mut Vec<Box<dyn Generator + 'static>>,
        ctx: &GeneratorCtx,
        path: &mut WasmPath,
        external: &ModuleExternal,
    ) -> eyre::Result<()>
    where
        Self: Sized,
    {
        unreachable!()
    }

    #[allow(unused_variables)]
    fn wrap_run(
        self,
        generators: &mut Vec<Box<dyn Generator + 'static>>,
        ctx: &GeneratorCtx,
        path: &mut WasmPath,
    ) -> eyre::Result<()>
    where
        Self: Sized,
    {
        unreachable!()
    }
}

impl<
    F: FnOnce(
        &mut Vec<Box<dyn Generator + 'static>>,
        &GeneratorCtx,
        &mut walrus::Module,
        &ModuleExternal,
    ) -> eyre::Result<()>,
> WrapRunner<0> for F
{
    fn wrap_run_with_external(
        self,
        generators: &mut Vec<Box<dyn Generator + 'static>>,
        ctx: &GeneratorCtx,
        path: &mut WasmPath,
        external: &ModuleExternal,
    ) -> eyre::Result<()> {
        let old_path = path.path()?;
        let module = &mut walrus::Module::load(old_path, ctx.dwarf)
            .wrap_err("Failed to load Wasm module")?;

        (self)(generators, ctx, module, external)?;

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

        Ok(())
    }
}

impl<
    F: FnOnce(
        &mut Vec<Box<dyn Generator + 'static>>,
        &GeneratorCtx,
        &mut walrus::Module,
    ) -> eyre::Result<()>,
> WrapRunner<1> for F
{
    fn wrap_run(
        self,
        generators: &mut Vec<Box<dyn Generator + 'static>>,
        ctx: &GeneratorCtx,
        path: &mut WasmPath,
    ) -> eyre::Result<()> {
        let old_path = path.path()?;
        let module = &mut walrus::Module::load(old_path, ctx.dwarf)
            .wrap_err("Failed to load Wasm module")?;

        (self)(generators, ctx, module)?;

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

        Ok(())
    }
}

pub(crate) trait EndWithOpt<T, const N: usize> {
    #[allow(unused_variables)]
    fn with_opt_with_external(
        self,
        t: T,
        generators: &mut Vec<Box<dyn Generator + 'static>>,
        ctx: &GeneratorCtx,
        path: &mut WasmPath,
        external: &ModuleExternal,
    ) -> eyre::Result<()>
    where
        Self: Sized,
    {
        unreachable!()
    }

    #[allow(unused_variables)]
    fn with_opt(
        self,
        t: T,
        generators: &mut Vec<Box<dyn Generator + 'static>>,
        ctx: &GeneratorCtx,
        path: &mut WasmPath,
    ) -> eyre::Result<()>
    where
        Self: Sized,
    {
        unreachable!()
    }
}

impl<
    T,
    F: FnOnce(
        T,
        &mut Vec<Box<dyn Generator + 'static>>,
        &GeneratorCtx,
        &mut WasmPath,
        &ModuleExternal,
    ) -> eyre::Result<()>,
> EndWithOpt<T, 0> for F
{
    fn with_opt_with_external(
        self,
        t: T,
        generators: &mut Vec<Box<dyn Generator + 'static>>,
        ctx: &GeneratorCtx,
        path: &mut WasmPath,
        external: &ModuleExternal,
    ) -> eyre::Result<()> {
        (self)(t, generators, ctx, path, external).wrap_err("Failed to run with with_opt")?;

        println!("Optimizing Wasm...");
        let new_path = building::optimize_wasm(path.path()?, &[], false, ctx.dwarf)
            .wrap_err("Failed to optimize Wasm")?;

        path.set_path(new_path)?;

        Ok(())
    }
}

impl<
    T,
    F: FnOnce(
        T,
        &mut Vec<Box<dyn Generator + 'static>>,
        &GeneratorCtx,
        &mut WasmPath,
    ) -> eyre::Result<()>,
> EndWithOpt<T, 1> for F
{
    fn with_opt(
        self,
        t: T,
        generators: &mut Vec<Box<dyn Generator + 'static>>,
        ctx: &GeneratorCtx,
        path: &mut WasmPath,
    ) -> eyre::Result<()>
    where
        Self: Sized,
    {
        (self)(t, generators, ctx, path).wrap_err("Failed to run with with_opt")?;

        println!("Optimizing Wasm...");
        let new_path = building::optimize_wasm(path.path()?, &[], false, ctx.dwarf)
            .wrap_err("Failed to optimize Wasm")?;

        path.set_path(new_path)?;

        Ok(())
    }
}

impl GeneratorRunner {
    pub fn new(
        path: WasmPath,
        targets: Vec<WasmPath>,
        threads: bool,
        dwarf: bool,
        unstable_print_debug: bool,
        memory_type: TargetMemoryType,
        toml_restorers: TomlRestorers,
        memory_hint: Vec<Option<usize>>,
    ) -> eyre::Result<Self> {
        let memory_hint = memory_hint
            .into_iter()
            .zip(targets.iter().map(|t| t.name()))
            .map(|(hint, name)| Ok((name?, hint)))
            .filter_map_ok(|(name, hint)| Some((name, hint?)))
            .collect::<eyre::Result<HashMap<_, _>>>()?;

        Ok(Self {
            generators: Vec::new(),
            ctx: GeneratorCtx {
                vfs_name: path.name()?,
                target_names: targets
                    .iter()
                    .map(|t| t.name())
                    .collect::<eyre::Result<_>>()?,
                target_memory_type: memory_type,
                unstable_print_debug,
                dwarf,
                threads,
                vfs_used_memory_id: None,
                target_used_memory_id: None,
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

    pub fn get_generator_ref<T: Generator + 'static>(&mut self) -> eyre::Result<&T> {
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

    pub fn run_layers(&mut self) -> eyre::Result<()> {
        self.definitely()?;

        let toml_restorers = self
            .toml_restorers
            .take()
            .ok_or_else(|| eyre::eyre!("TomlRestorers already taken"))?;

        toml_restorers
            .restore()
            .wrap_err("Failed to restore toml files")?;

        let mem_holder = MemoryIDHolder {
            memory_hint: self.memory_hint.clone(),
            used_vfs_memory_id: None,
            used_target_memory_id: None,
        };

        self.add_generator(mem_holder);

        WrapRunner::wrap_run.with_opt(
            Self::run_pre_vfs,
            &mut self.generators,
            &self.ctx,
            &mut self.path,
        )?;

        for target in self.targets.iter_mut() {
            WrapRunner::wrap_run_with_external.with_opt_with_external(
                Self::run_pre_target,
                &mut self.generators,
                &self.ctx,
                target,
                &ModuleExternal {
                    name: target.name()?,
                },
            )?;
        }

        let holder = self.get_generator_ref::<MemoryIDHolder>()?.clone();
        self.ctx.vfs_used_memory_id = holder.used_vfs_memory_id;
        self.ctx.target_used_memory_id = holder.used_target_memory_id;

        // WrapRunner::wrap_run.with_opt(Self::run_post_combine, self)?;
        // WrapRunner::wrap_run.with_opt(Self::run_post_lower_memory, self)?;
        // WrapRunner::wrap_run.with_opt(Self::run_post_components, self)?;
        for i in 0..self.generators.len() {
            (|generators: &mut Vec<Box<dyn Generator + 'static>>,
              ctx: &GeneratorCtx,
              module: &mut walrus::Module| {
                generators[i]
                    .post_all_optimize(module, ctx)
                    .wrap_err("Failed in post_all_optimize")
            })
            .wrap_run(&mut self.generators, &self.ctx, &mut self.path)?;
        }

        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
struct MemoryIDHolder {
    pub memory_hint: HashMap<String, usize>,
    pub used_vfs_memory_id: Option<MemoryId>,
    pub used_target_memory_id: Option<Vec<MemoryId>>,
}

impl Generator for MemoryIDHolder {
    fn pre_vfs(
        &mut self,
        module: &mut walrus::Module,
        _: &crate::generator::GeneratorCtx,
    ) -> eyre::Result<()> {
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
        self.used_target_memory_id.get_or_insert_default().push(id);
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
            self.used_target_memory_id.as_mut().unwrap().push(id);
        }

        Ok(())
    }
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
