use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use eyre::Context as _;

use crate::{generator::WasmPath, util::ResultUtil as _};

/// Common interface for post-build (transpilation) arguments.
pub trait PostBuildContext {
    /// Returns the output directory path.
    fn out_dir(&self) -> &Utf8PathBuf;
    /// Returns whether to keep intermediate build artifacts.
    fn keep_build_artifacts(&self) -> bool;
    /// Returns whether to adjust the ABI.
    fn adjust_abi(&self) -> bool;
    /// Transpiles the given WebAssembly component into JavaScript source files.
    fn transpile_to_js(
        &self,
        component: &[u8],
        name: impl AsRef<str>,
    ) -> Result<js_component_bindgen::Transpiled, eyre::Error>;
    /// Returns whether to skip Wasm optimization (dev mode).
    fn dev(&self) -> bool;
}

/// The main command-line interface for `wasi_virt_layer-cli`.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, help_template = "
{name} {version}
{author}
{about}

{usage-heading} {usage}

{all-args}")]
#[clap(propagate_version = true)]
pub struct Cli {
    /// The command to execute.
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
#[clap(author, version, about, long_about = None, help_template = "
{name} {version}
{author}
{about}

{usage-heading} {usage}

{all-args}")]
/// Supported subcommands for the CLI.
pub enum Command {
    /// Builds a virtualized WASM module (prebuild + postbuild combined).
    Build(BuildArgs),
    /// Generates a Component WASM from VFS and target WASM modules.
    Prebuild(PreBuildArgs),
    /// Transpiles a Component WASM into JavaScript files.
    Postbuild(PostBuildArgs),
    /// Initializes a new WASI Virt Layer project.
    New(NewArgs),
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, help_template = "
{name} {version}
{author}
{about}

{usage-heading} {usage}

{all-args}")]
/// Arguments for the `new` command.
pub struct NewArgs {
    /// The path where the new project will be created.
    #[arg(value_name = "PATH")]
    pub path: Utf8PathBuf,

    /// Whether to enable multi-threading support in the new project.
    #[arg(long, default_value = "false")]
    pub threads: bool,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, help_template = "
{name} {version}
{author}
{about}

{usage-heading} {usage}

{all-args}")]
/// Arguments for the `build` command.
pub struct BuildArgs {
    /// Path to the wasip1 wasm file
    /// This allow 4 patterns:
    /// 1. only manifest path, like `./Cargo.toml` or `./some/dir/Cargo.toml`
    /// 2. only package name, like `my_package`
    /// 3. manifest path and package name, like `./Cargo.toml::my_package` or `./some/dir/Cargo.toml::my_package`
    /// 4. direct path to wasm file, like `./target/wasm32-wasi/release/my_crate.wasm`
    pub wasm: Vec<WasmPath>,

    /// Path to the primary package; used for single-package mode or component translation.
    #[arg(short, long)]
    package: Option<WasmPath>,

    /// Path to Cargo.toml
    #[arg(long)]
    pub manifest_path: Option<Utf8PathBuf>,

    /// Memory hints for the WASM files, used if automatic detection fails.
    #[arg(long)]
    wasm_memory_hint: Vec<isize>,

    /// Enable unwind for the target WASM modules.
    #[arg(long = "wasm-unwind")]
    pub wasm_unwind: Vec<bool>,

    /// Output directory for the generated files
    #[arg(long, default_value = "./dist")]
    pub out_dir: Utf8PathBuf,

    /// Target memory type
    /// Change crate feature flags based on the target memory type
    #[arg(short, long)]
    pub target_memory_type: Option<TargetMemoryType>,

    // transpile options
    /// Options for transpiling to JavaScript.
    #[command(flatten)]
    pub transpile_opts: TranspileOpts,

    /// If wasm run on multiple threads, enable thread support
    /// This will change the crate feature flags to enable multi-threading.
    #[arg(long)]
    pub threads: Option<bool>,

    /// Enable dwarf
    /// This is broken currently.
    /// See https://github.com/wasm-bindgen/walrus/issues/258
    #[arg(long)]
    pub dwarf: Option<bool>,

    /// Finally, align the ABI with wasip1-threads.
    /// Only WASM will be generated.
    #[arg(long, default_value = "false")]
    pub adjust_abi: bool,

    /// Keep all intermediate build artifacts instead of deleting them.
    #[arg(long, default_value = "false")]
    pub keep_build_artifacts: bool,

    /// Enable development mode (skips Wasm optimization).
    #[arg(long, default_value = "false")]
    pub dev: bool,

    /// Options for building the VFS module.
    #[command(flatten)]
    pub vfs_build_opts: VfsBuildOptions,

    /// Options for building target Wasm modules.
    #[arg(skip)]
    pub target_vfs_build_opts: Option<Box<[VfsBuildOptions]>>,

    /// Enable own-memory mode.
    #[arg(long, default_value = "false")]
    pub own_memory: bool,
}

impl BuildArgs {
    /// Returns the memory hints for each target module.
    pub fn get_wasm_memory_hints(&self) -> Box<[Option<usize>]> {
        self.wasm_memory_hint
            .iter()
            .map(|&hint| if hint < 0 { None } else { Some(hint as usize) })
            .chain(std::iter::repeat(None))
            .take(self.wasm.len())
            .collect::<Box<_>>()
    }

    /// Returns the unwind configuration for each target module.
    pub fn get_wasm_unwinds(&self) -> Box<[bool]> {
        self.wasm_unwind
            .iter()
            .copied()
            .chain(std::iter::repeat(false))
            .take(self.wasm.len())
            .collect::<Box<_>>()
    }

    /// Resolves and returns the path to the WASM package.
    pub fn get_package(&self) -> eyre::Result<WasmPath> {
        let mut p = Ok(self.package.clone()).transpose().unwrap_or_else(|| {
            if let Some(ref manifest) = self.manifest_path {
                WasmPath::with_maybe_only_manifest(manifest.clone())
            } else {
                WasmPath::with_maybe_none()
            }
        })?;
        if let Some(ref manifest) = self.manifest_path {
            if let WasmPath::Maybe { manifest_path, .. } = &mut p {
                *manifest_path = manifest.clone();
            }
        }
        Ok(p)
    }

    /// Gets the list of wasm targets, applying the manifest path if specified.
    pub fn get_wasm_paths(&self) -> Box<[WasmPath]> {
        let mut paths = self.wasm.clone();
        if let Some(ref manifest) = self.manifest_path {
            for p in &mut paths {
                if let WasmPath::Maybe { manifest_path, .. } = p {
                    *manifest_path = manifest.clone();
                }
            }
        }
        paths.into_boxed_slice()
    }
}

impl PostBuildContext for BuildArgs {
    fn out_dir(&self) -> &Utf8PathBuf {
        &self.out_dir
    }

    fn keep_build_artifacts(&self) -> bool {
        self.keep_build_artifacts
    }

    fn adjust_abi(&self) -> bool {
        self.adjust_abi
    }

    fn transpile_to_js(
        &self,
        component: &[u8],
        name: impl AsRef<str>,
    ) -> Result<js_component_bindgen::Transpiled, eyre::Error> {
        self.transpile_opts.transpile_to_js(component, name)
    }

    fn dev(&self) -> bool {
        self.dev
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, help_template = "
{name} {version}
{author}
{about}

{usage-heading} {usage}

{all-args}")]
/// Arguments for the `prebuild` command.
pub struct PreBuildArgs {
    /// Path to the wasip1 wasm file
    /// This allow 4 patterns:
    /// 1. only manifest path, like `./Cargo.toml` or `./some/dir/Cargo.toml`
    /// 2. only package name, like `my_package`
    /// 3. manifest path and package name, like `./Cargo.toml::my_package` or `./some/dir/Cargo.toml::my_package`
    /// 4. direct path to wasm file, like `./target/wasm32-wasi/release/my_crate.wasm`
    pub wasm: Vec<WasmPath>,

    /// Path to the primary package.
    #[arg(short, long)]
    package: Option<WasmPath>,

    /// Path to Cargo.toml
    #[arg(long)]
    pub manifest_path: Option<Utf8PathBuf>,

    /// Memory hints for the WASM files, used if automatic detection fails.
    #[arg(long)]
    wasm_memory_hint: Vec<isize>,

    /// Enable unwind for the target WASM modules.
    #[arg(long = "wasm-unwind")]
    pub wasm_unwind: Vec<bool>,

    /// Output directory for the generated Component WASM
    #[arg(long, default_value = "./dist")]
    pub out_dir: Utf8PathBuf,

    /// Target memory type
    /// Change crate feature flags based on the target memory type
    #[arg(short, long)]
    pub target_memory_type: Option<TargetMemoryType>,

    /// If wasm run on multiple threads, enable thread support
    /// This will change the crate feature flags to enable multi-threading.
    #[arg(long)]
    pub threads: Option<bool>,

    /// Enable dwarf
    /// This is broken currently.
    /// See https://github.com/wasm-bindgen/walrus/issues/258
    #[arg(long)]
    pub dwarf: Option<bool>,

    /// Finally, align the ABI with wasip1-threads.
    /// Only WASM will be generated.
    #[arg(long, default_value = "false")]
    pub adjust_abi: bool,

    /// Keep all intermediate build artifacts instead of deleting them.
    #[arg(long, default_value = "false")]
    pub keep_build_artifacts: bool,

    /// Enable development mode (skips Wasm optimization).
    #[arg(long, default_value = "false")]
    pub dev: bool,

    /// Options for building the VFS module.
    #[command(flatten)]
    pub vfs_build_opts: VfsBuildOptions,

    /// Options for building target Wasm modules.
    #[arg(skip)]
    pub target_vfs_build_opts: Option<Box<[VfsBuildOptions]>>,

    /// Enable own-memory mode.
    #[arg(long, default_value = "false")]
    pub own_memory: bool,
}

impl PreBuildArgs {
    /// Returns the memory hints for each target module.
    pub fn get_wasm_memory_hints(&self) -> Box<[Option<usize>]> {
        self.wasm_memory_hint
            .iter()
            .map(|&hint| if hint < 0 { None } else { Some(hint as usize) })
            .chain(std::iter::repeat(None))
            .take(self.wasm.len())
            .collect::<Box<_>>()
    }

    /// Returns the unwind configuration for each target module.
    pub fn get_wasm_unwinds(&self) -> Box<[bool]> {
        self.wasm_unwind
            .iter()
            .copied()
            .chain(std::iter::repeat(false))
            .take(self.wasm.len())
            .collect::<Box<_>>()
    }

    /// Resolves and returns the path to the WASM package.
    pub fn get_package(&self) -> eyre::Result<WasmPath> {
        let mut p = Ok(self.package.clone()).transpose().unwrap_or_else(|| {
            if let Some(ref manifest) = self.manifest_path {
                WasmPath::with_maybe_only_manifest(manifest.clone())
            } else {
                WasmPath::with_maybe_none()
            }
        })?;
        if let Some(ref manifest) = self.manifest_path {
            if let WasmPath::Maybe { manifest_path, .. } = &mut p {
                *manifest_path = manifest.clone();
            }
        }
        Ok(p)
    }

    /// Gets the list of wasm targets, applying the manifest path if specified.
    pub fn get_wasm_paths(&self) -> Box<[WasmPath]> {
        let mut paths = self.wasm.clone();
        if let Some(ref manifest) = self.manifest_path {
            for p in &mut paths {
                if let WasmPath::Maybe { manifest_path, .. } = p {
                    *manifest_path = manifest.clone();
                }
            }
        }
        paths.into_boxed_slice()
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, help_template = "
{name} {version}
{author}
{about}

{usage-heading} {usage}

{all-args}")]
/// Arguments for the `postbuild` command.
pub struct PostBuildArgs {
    /// Path to the Component WASM file to transpile.
    #[arg(short, long)]
    pub package: WasmPath,

    /// Output directory for the generated files
    #[arg(long, default_value = "./dist")]
    pub out_dir: Utf8PathBuf,

    /// Options for transpiling to JavaScript.
    #[command(flatten)]
    pub transpile_opts: TranspileOpts,

    /// Enable dwarf
    #[arg(long)]
    pub dwarf: Option<bool>,

    /// Finally, align the ABI with wasip1-threads.
    /// Only WASM will be generated.
    #[arg(long, default_value = "false")]
    pub adjust_abi: bool,

    /// Keep all intermediate build artifacts instead of deleting them.
    #[arg(long, default_value = "false")]
    pub keep_build_artifacts: bool,

    /// Enable development mode (skips Wasm optimization).
    #[arg(long, default_value = "false")]
    pub dev: bool,
}

impl PostBuildContext for PostBuildArgs {
    fn out_dir(&self) -> &Utf8PathBuf {
        &self.out_dir
    }

    fn keep_build_artifacts(&self) -> bool {
        self.keep_build_artifacts
    }

    fn adjust_abi(&self) -> bool {
        self.adjust_abi
    }

    fn transpile_to_js(
        &self,
        component: &[u8],
        name: impl AsRef<str>,
    ) -> Result<js_component_bindgen::Transpiled, eyre::Error> {
        self.transpile_opts.transpile_to_js(component, name)
    }

    fn dev(&self) -> bool {
        self.dev
    }
}

/// Options for transpiling WebAssembly components into JavaScript.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, help_template = "
{name} {version}
{author}
{about}

{usage-heading} {usage}

{all-args}")]
pub struct TranspileOpts {
    /// Disables generation of *.d.ts files and instead only generates *.js source files.
    #[arg(long, default_value = "false")]
    no_typescript: bool,

    /// Provide a custom JS instantiation API for the component instead of the direct importable native ESM output.
    /// Sync, Async, Normal, Default is Async.
    #[arg(long, value_parser = analysis::analysis_instantiation, default_value = "CustomInstantiationMode(None)")]
    instantiation: CustomInstantiationMode,

    /// Configure how import bindings are provided, as high-level JS bindings, or as hybrid optimized bindings.
    #[arg(long, value_parser = analysis::analysis_import_bindings)]
    import_bindings: Option<js_component_bindgen::BindingsMode>,

    /// Comma-separated list of "from-specifier=./to-specifier.js" mappings of component import specifiers to JS import specifiers.
    #[arg(long, value_delimiter = ',', value_parser = analysis::parse_mapping, action = clap::ArgAction::Append)]
    map: Vec<(String, String)>,

    /// Disables compatibility in Node.js without a fetch global.
    #[arg(long, default_value = "false")]
    no_nodejs_compat: bool,

    /// Set the cutoff byte size for base64 inlining core Wasm in instantiation mode (set to 0 to disable all base64 inlining)
    #[arg(long, default_value = "0")]
    base64_cutoff: usize,

    /// Enables compatibility for JS environments without top-level await support via an async $init promise export to wait for instead.
    #[arg(long, default_value = "false")]
    tla_compat: bool,

    /// Disable verification of component Wasm data structures when lifting as a production optimization
    #[arg(long, default_value = "false")]
    valid_lifting_optimization: bool,

    /// Whether or not to emit tracing calls on function entry/exit.
    #[arg(long, default_value = "false")]
    tracing: bool,

    /// Whether to generate namespaced exports like foo as "local:package/foo ". These exports can break typescript builds.
    #[arg(long, default_value = "false")]
    no_namespaced_exports: bool,

    /// Whether to generate types for a guest module using module declarations.
    #[arg(long, default_value = "false")]
    pub guest: bool,
}

impl TranspileOpts {
    /// Transpiles the given WebAssembly component into JavaScript source files.
    pub fn transpile_to_js(
        &self,
        component: &[u8],
        name: impl AsRef<str>,
    ) -> Result<js_component_bindgen::Transpiled, eyre::Error> {
        js_component_bindgen::transpile(
            component,
            js_component_bindgen::TranspileOpts {
                name: name.as_ref().to_string(),
                no_typescript: self.no_typescript,
                instantiation_mode: self.instantiation.clone().0,
                import_bindings: self.import_bindings.clone(),
                map: if !self.map.is_empty() {
                    Some(self.map.iter().cloned().collect())
                } else {
                    None
                },
                nodejs_compat_disabled: self.no_nodejs_compat,
                base64_cutoff: self.base64_cutoff,
                tla_compat: self.tla_compat,
                valid_lifting_optimization: self.valid_lifting_optimization,
                tracing: self.tracing,
                no_namespaced_exports: self.no_namespaced_exports,
                multi_memory: true,
                guest: self.guest,
                async_mode: None,
                strict: false,
                asmjs: false,
            },
        )
        .to_eyre()
        .wrap_err("Failed to transpile to JS.")
    }
}

/// Represents a custom instantiation mode for the generated JavaScript.
#[derive(Clone, Debug)]
pub struct CustomInstantiationMode(Option<js_component_bindgen::InstantiationMode>);

pub(super) mod analysis {
    use js_component_bindgen::{BindingsMode, InstantiationMode};

    use super::CustomInstantiationMode;

    pub fn analysis_instantiation(s: &str) -> Result<CustomInstantiationMode, clap::Error> {
        match s {
            "Sync" => Ok(CustomInstantiationMode(Some(InstantiationMode::Sync))),
            "Async" => Ok(CustomInstantiationMode(Some(InstantiationMode::Async))),
            "Normal" => Ok(CustomInstantiationMode(None)),
            _ => Ok(CustomInstantiationMode(Some(InstantiationMode::Async))),
        }
    }

    pub fn analysis_import_bindings(s: &str) -> Result<Option<BindingsMode>, clap::Error> {
        match s {
            "Hybrid" => Ok(Some(BindingsMode::Hybrid)),
            "Js" => Ok(Some(BindingsMode::Js)),
            "Optimized" => Ok(Some(BindingsMode::Optimized)),
            "DirectOptimized" => Ok(Some(BindingsMode::DirectOptimized)),
            _ => Ok(None),
        }
    }

    pub fn parse_mapping(s: &str) -> Result<(String, String), String> {
        let parts: Vec<&str> = s.splitn(2, '=').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid mapping format: '{}'", s));
        }
        Ok((parts[0].to_string(), parts[1].to_string()))
    }
}

/// Specifies the memory architecture of the target WebAssembly module.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    strum::EnumString,
    strum::Display,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum TargetMemoryType {
    /// Traditional single memory environment.
    #[strum(ascii_case_insensitive)]
    Single,
    /// Multi-memory environment where modules have their own address spaces.
    #[strum(ascii_case_insensitive)]
    Multi,
}

impl TargetMemoryType {
    /// Returns `true` if this memory type is `Multi`.
    pub fn is_multi(&self) -> bool {
        matches!(self, TargetMemoryType::Multi)
    }

    /// Returns `true` if this memory type is `Single`.
    pub fn is_single(&self) -> bool {
        matches!(self, TargetMemoryType::Single)
    }
}

/// Options for building the VFS module.
#[derive(Parser, Debug, Clone, Default)]
pub struct VfsBuildOptions {
    /// Space or comma separated list of features to activate for the VFS module.
    #[arg(long, action = clap::ArgAction::Append)]
    pub features: Vec<String>,

    /// Do not activate the `default` feature of the VFS module.
    #[arg(long, action = clap::ArgAction::Count)]
    pub no_default_features: u8,

    /// Enable unwind for the module.
    #[arg(long = "vfs-unwind", default_value = "false")]
    pub unwind: bool,

    /// Disable optimization for this specific module.
    #[arg(long, action = clap::ArgAction::Count)]
    pub no_opt: u8,

    /// Disable all optimizations for this module and subsequent merged modules.
    #[arg(long, action = clap::ArgAction::Count)]
    pub no_opt_all: u8,
}
