use eyre::Context as _;

use crate::{
    args::{PreBuildArgs, TargetMemoryType},
    config_checker::{FeatureChecker, HasFeature, TomlRestorers},
    ctrlc_handler,
    generator::{self, WasmPath},
    unique_name::UniqueName,
};

/// Shared internal logic for the prebuild phase.
///
/// Compiles VFS and target WASM modules, runs all generator stages,
/// and produces a Component WASM file.
pub(crate) fn run_prebuild_internal(
    package: WasmPath,
    wasm: &[WasmPath],
    target_memory_type: Option<TargetMemoryType>,
    threads: Option<bool>,
    dwarf: Option<bool>,
    adjust_abi: bool,
    keep_build_artifacts: bool,
    out_dir: &camino::Utf8PathBuf,
    wasm_memory_hints: Box<[Option<usize>]>,
    vfs_build_opts: &crate::args::VfsBuildOptions,
    target_vfs_build_opts: Box<[crate::args::VfsBuildOptions]>,
) -> eyre::Result<(generator::ComponentRunner, bool)> {
    let vfs_package = package
        .clone()
        .name()
        .wrap_err("Failed to get package name")?;
    log::info!("Using package: {}", vfs_package);

    let vfs_manifest_path = package.manifest_path().unwrap();
    let vfs_root_manifest_path = package.root_manifest_path().unwrap();

    let mut toml_restores = TomlRestorers::new();
    let tr_clone = toml_restores.clone();
    ctrlc_handler::register(move || {
        tr_clone.restore_if_needed();
    });

    let memory_type = {
        let memory_type_checker = FeatureChecker::new(
            "multi_memory",
            &vfs_manifest_path,
            &vfs_root_manifest_path,
            UniqueName::CRATE_NAME,
        );

        if let Some(target_memory_type) = target_memory_type {
            if let Some(restorer) = memory_type_checker.set(target_memory_type.is_multi())? {
                toml_restores.push(restorer);
            }

            target_memory_type
        } else {
            match memory_type_checker.has()? {
                HasFeature::EnabledOnNormal | HasFeature::EnabledOnWorkspace => {
                    TargetMemoryType::Multi
                }
                HasFeature::Disabled => TargetMemoryType::Single,
            }
        }
    };

    let threads = {
        let threads_feature_checker = FeatureChecker::new(
            "threads",
            &vfs_manifest_path,
            &vfs_root_manifest_path,
            UniqueName::CRATE_NAME,
        );
        if let Some(threads) = threads {
            if let Some(restorer) = threads_feature_checker.set(threads)? {
                toml_restores.push(restorer);
            }
            threads
        } else {
            matches!(
                threads_feature_checker.has()?,
                HasFeature::EnabledOnNormal | HasFeature::EnabledOnWorkspace
            )
        }
    };

    let dwarf = if let Some(dwarf) = dwarf {
        let checker = FeatureChecker::new_no_feature(
            &vfs_manifest_path,
            &vfs_root_manifest_path,
            UniqueName::CRATE_NAME,
        );

        toml_restores.push(checker.set_dwarf(dwarf)?);

        dwarf
    } else {
        false
    };

    let unstable_print_debug = {
        let checker = FeatureChecker::new(
            "unstable_print_debug",
            &vfs_manifest_path,
            &vfs_root_manifest_path,
            UniqueName::CRATE_NAME,
        );

        matches!(
            checker.has()?,
            HasFeature::EnabledOnNormal | HasFeature::EnabledOnWorkspace
        )
    };

    let generator = generator::GeneratorRunner::new(
        package,
        wasm.to_vec().into_boxed_slice(),
        threads,
        dwarf,
        unstable_print_debug,
        adjust_abi,
        keep_build_artifacts,
        memory_type,
        vfs_build_opts.clone(),
        target_vfs_build_opts,
        toml_restores,
        wasm_memory_hints,
    )?;

    let component_runner = generator
        .run_layers_to_component(out_dir, keep_build_artifacts)
        .wrap_err("Failed to run layers to component")?;

    Ok((component_runner, dwarf))
}

/// Executes the prebuild command, generating a Component WASM from VFS and target modules.
pub fn prebuild(parsed_args: PreBuildArgs) -> eyre::Result<()> {
    let package = parsed_args.get_package()?;

    if matches!(package, WasmPath::Component(_)) {
        eyre::bail!("prebuild does not accept Component WASM files. Use postbuild instead.");
    }

    let mut vfs_build_opts = parsed_args.vfs_build_opts.clone();
    if parsed_args.dev {
        vfs_build_opts.no_opt_all = vfs_build_opts.no_opt_all.saturating_add(1);
    }

    let (component_runner, dwarf) = run_prebuild_internal(
        package,
        &parsed_args.get_wasm_paths(),
        parsed_args.target_memory_type,
        parsed_args.threads,
        parsed_args.dwarf,
        parsed_args.adjust_abi,
        parsed_args.keep_build_artifacts,
        &parsed_args.out_dir,
        parsed_args.get_wasm_memory_hints(),
        &vfs_build_opts,
        parsed_args
            .target_vfs_build_opts
            .clone()
            .unwrap_or_else(|| Box::new([])),
    )?;

    let path = component_runner.path.path()?;
    println!("\nPrebuild completed successfully!");
    println!("Component WASM generated at: {path}");
    let mut cmd = format!("cargo r -- postbuild -p {path}");
    if dwarf {
        cmd.push_str(" --dwarf true");
    }
    println!("To transpile to JS, run: `{cmd}`");

    Ok(())
}
