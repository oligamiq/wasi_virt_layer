use std::{fs, str::FromStr};

use camino::Utf8PathBuf;
use eyre::Context as _;

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
    pub target_memory_type: TargetMemoryType,
    pub unstable_print_debug: bool,
    pub dwarf: bool,
    pub threads: bool,
}

pub trait Generator: std::fmt::Debug {
    /// Operations performed on the built VFS module.
    #[allow(unused_variables)]
    fn pre_vfs(&mut self, module: &mut walrus::Module, ctx: &GeneratorCtx) -> eyre::Result<()> {
        Ok(())
    }

    /// Operations performed on the target module.
    #[allow(unused_variables)]
    fn pre_target(&mut self, module: &mut walrus::Module, ctx: &GeneratorCtx) -> eyre::Result<()> {
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
pub struct GeneratorRunner {
    pub generators: Vec<Box<dyn Generator + 'static>>,
    pub ctx: GeneratorCtx,
    pub path: WasmPath,
    pub targets: Vec<WasmPath>,
    pub toml_restorers: Option<TomlRestorers>,
}

impl GeneratorRunner {
    pub fn run_pre_vfs(&mut self, module: &mut walrus::Module) -> eyre::Result<()> {
        for generator in &mut self.generators {
            generator.pre_vfs(module, &self.ctx)?;
        }
        Ok(())
    }

    pub fn run_pre_target(&mut self, module: &mut walrus::Module) -> eyre::Result<()> {
        for generator in &mut self.generators {
            generator.pre_target(module, &self.ctx)?;
        }
        Ok(())
    }

    pub fn run_post_combine(&mut self, module: &mut walrus::Module) -> eyre::Result<()> {
        for generator in &mut self.generators {
            generator.post_combine(module, &self.ctx)?;
        }
        Ok(())
    }

    pub fn run_post_lower_memory(&mut self, module: &mut walrus::Module) -> eyre::Result<()> {
        for generator in &mut self.generators {
            generator.post_lower_memory(module, &self.ctx)?;
        }
        Ok(())
    }

    pub fn run_post_components(&mut self, module: &mut walrus::Module) -> eyre::Result<()> {
        for generator in &mut self.generators {
            generator.post_components(module, &self.ctx)?;
        }
        Ok(())
    }
}

pub(crate) trait WrapRunner {
    fn wrap_run(self, runner: &mut GeneratorRunner) -> eyre::Result<()>;
}

impl<F: FnOnce(&mut GeneratorRunner, &mut walrus::Module) -> eyre::Result<()>> WrapRunner for F {
    fn wrap_run(self, runner: &mut GeneratorRunner) -> eyre::Result<()> {
        let path = runner.path.path()?.clone();
        let module = &mut walrus::Module::load(&path, runner.ctx().dwarf)
            .wrap_err("Failed to load Wasm module")?;

        (self)(runner, module)?;

        let new_path = path.with_extension("adjusted.wasm");

        if fs::metadata(&new_path).is_ok() {
            fs::remove_file(&new_path)
                .wrap_err_with(|| format!("Failed to remove existing file {new_path}"))?;
        }

        module
            .emit_wasm_file(&new_path)
            .to_eyre()
            .wrap_err_with(|| format!("Failed to write adjusted Wasm to {new_path}"))?;

        runner.path.set_path(new_path)?;

        Ok(())
    }
}

pub(crate) trait EndWithOpt<T> {
    fn with_opt(self, t: T, runner: &mut GeneratorRunner) -> eyre::Result<()>;
}

impl<T, F: FnOnce(T, &mut GeneratorRunner) -> eyre::Result<()>> EndWithOpt<T> for F {
    fn with_opt(self, t: T, runner: &mut GeneratorRunner) -> eyre::Result<()> {
        (self)(t, runner).wrap_err("Failed to run with with_opt")?;

        println!("Optimizing VFS Wasm...");
        let new_path = building::optimize_wasm(runner.path.path()?, &[], false, runner.ctx.dwarf)
            .wrap_err("Failed to optimize Wasm")?;

        runner.path.set_path(new_path)?;

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
    ) -> eyre::Result<Self> {
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
            },
            path,
            targets,
            toml_restorers: Some(toml_restorers),
        })
    }

    pub fn add_generator<G: Generator + 'static>(&mut self, generator: G) {
        self.generators.push(Box::new(generator));
    }

    #[deprecated(
        note = "Ensure this function is self-contained. This is a temporary measure for debugging purposes."
    )]
    pub const fn path(&self) -> &WasmPath {
        &self.path
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

        WrapRunner::wrap_run.with_opt(Self::run_pre_vfs, self)?;
        WrapRunner::wrap_run.with_opt(Self::run_pre_target, self)?;
        WrapRunner::wrap_run.with_opt(Self::run_post_combine, self)?;
        WrapRunner::wrap_run.with_opt(Self::run_post_lower_memory, self)?;
        WrapRunner::wrap_run.with_opt(Self::run_post_components, self)?;
        for i in 0..self.generators.len() {
            (|runner: &mut GeneratorRunner, module: &mut walrus::Module| {
                runner.generators[i]
                    .post_all_optimize(module, &runner.ctx)
                    .wrap_err("Failed in post_all_optimize")
            })
            .wrap_run(self)?;
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
