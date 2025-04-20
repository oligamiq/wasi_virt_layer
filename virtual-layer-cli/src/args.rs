use std::collections::HashMap;

use clap::{Parser, command};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Path to the Cargo.toml file
    #[arg(long)]
    pub manifest_path: Option<String>,

    /// Output directory for the generated files
    #[arg(long, default_value = "./dist")]
    pub out_dir: String,

    /// Disable transpile to JS, you can use jco to transpile wasm to js.
    #[arg(long, default_value = "false")]
    pub no_transpile: bool,

    /// Disable transpile to Component, you can use wasm-tools component to transpile wasm to component.
    #[arg(long, default_value = "false")]
    pub no_component: bool,

    // transpile options
    #[command(flatten)]
    pub transpile_opts: TranspileOpts,
}

impl Args {
    pub fn new() -> Self {
        Args::parse()
    }

    pub fn get_manifest_path(&self) -> &'_ Option<String> {
        &self.manifest_path
    }

    pub fn transpile_to_js(
        &self,
        component: &[u8],
        name: &str,
    ) -> Result<js_component_bindgen::Transpiled, anyhow::Error> {
        js_component_bindgen::transpile(
            component,
            js_component_bindgen::TranspileOpts {
                name: name.to_string(),
                no_typescript: self.transpile_opts.no_typescript,
                instantiation: self.transpile_opts.instantiation.clone(),
                import_bindings: self.transpile_opts.import_bindings.clone(),
                map: {
                    if let Some(opts_map) = &self.transpile_opts.map {
                        let mut map = HashMap::new();
                        for (k, v) in opts_map.iter() {
                            map.insert(k.clone(), v.clone());
                        }
                        Some(map)
                    } else {
                        None
                    }
                },
                no_nodejs_compat: self.transpile_opts.no_nodejs_compat,
                base64_cutoff: self.transpile_opts.base64_cutoff,
                tla_compat: self.transpile_opts.tla_compat,
                valid_lifting_optimization: self.transpile_opts.valid_lifting_optimization,
                tracing: self.transpile_opts.tracing,
                no_namespaced_exports: self.transpile_opts.no_namespaced_exports,
                multi_memory: self.transpile_opts.multi_memory,
                guest: self.transpile_opts.guest,
                async_mode: None,
            },
        )
    }
}

#[derive(Parser, Debug)]
pub struct TranspileOpts {
    /// Disables generation of *.d.ts files and instead only generates *.js source files.
    #[arg(long, default_value = "false")]
    no_typescript: bool,

    /// Provide a custom JS instantiation API for the component instead of the direct importable native ESM output.
    #[arg(long, value_parser = analysis::analysis_instantiation)]
    instantiation: Option<js_component_bindgen::InstantiationMode>,

    /// Configure how import bindings are provided, as high-level JS bindings, or as hybrid optimized bindings.
    #[arg(long, value_parser = analysis::analysis_import_bindings)]
    import_bindings: Option<js_component_bindgen::BindingsMode>,

    /// Comma-separated list of “from-specifier=./to-specifier.js” mappings of component import specifiers to JS import specifiers.
    #[arg(long, value_delimiter = ',', value_parser = analysis::parse_mapping, num_args = 0.., action = clap::ArgAction::Append)]
    map: Option<HashMap<String, String>>,

    /// Disables compatibility in Node.js without a fetch global.
    #[arg(long, default_value = "true")]
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
    #[arg(long, default_value = "true")]
    tracing: bool,

    /// Whether to generate namespaced exports like foo as "local:package/foo". These exports can break typescript builds.
    #[arg(long, default_value = "false")]
    no_namespaced_exports: bool,

    /// Whether to output core Wasm utilizing multi-memory or to polyfill this handling.
    #[arg(long, default_value = "false")]
    multi_memory: bool,

    /// Whether to generate types for a guest module using module declarations.
    #[arg(long, default_value = "false")]
    guest: bool,
}

pub(super) mod analysis {
    use js_component_bindgen::{BindingsMode, InstantiationMode};

    pub fn analysis_instantiation(s: &str) -> Result<Option<InstantiationMode>, clap::Error> {
        match s {
            "Sync" => Ok(Some(InstantiationMode::Sync)),
            "Async" => Ok(Some(InstantiationMode::Async)),
            _ => Ok(None),
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
