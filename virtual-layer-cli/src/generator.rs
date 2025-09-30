use std::fs;

use camino::Utf8PathBuf;
use eyre::Context as _;

use crate::{
    args::TargetMemoryType,
    building,
    util::{ResultUtil, WalrusUtilModule},
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
    pub const fn new(ctx: GeneratorCtx, path: WasmPath) -> Self {
        Self {
            generators: Vec::new(),
            ctx,
            path,
        }
    }

    pub fn add_generator<G: Generator + 'static>(&mut self, generator: G) {
        self.generators.push(Box::new(generator));
    }

    pub const fn ctx(&self) -> &GeneratorCtx {
        &self.ctx
    }

    pub fn run_layers(&mut self) -> eyre::Result<()> {
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

#[derive(Debug)]
pub enum WasmPath {
    Maybe(Utf8PathBuf),
    Definitely(Utf8PathBuf),
}

impl WasmPath {
    pub const fn new(path: Utf8PathBuf) -> Self {
        Self::Maybe(path)
    }

    pub fn path(&self) -> eyre::Result<&Utf8PathBuf> {
        match self {
            WasmPath::Maybe(p) => {
                eyre::bail!("WasmPath is not definitely set: {p}")
            }
            WasmPath::Definitely(p) => Ok(p),
        }
    }

    pub fn set_path(&mut self, path: Utf8PathBuf) -> eyre::Result<()> {
        if matches!(self, WasmPath::Maybe(_)) {
            eyre::bail!("WasmPath is not definitely set: {path}")
        }
        *self = WasmPath::Definitely(path);
        Ok(())
    }
}
