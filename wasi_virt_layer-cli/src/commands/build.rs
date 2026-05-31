use crate::{
    args::BuildArgs,
    commands::{postbuild, prebuild},
    generator::{self, WasmPath},
};



/// Executes the build command, coordinating the compilation and transformation of WASM modules.
///
/// This is a convenience command that runs `prebuild` followed by `postbuild`.
pub fn build(parsed_args: BuildArgs) -> eyre::Result<()> {
    let package = parsed_args.get_package()?;

    if parsed_args.dwarf.unwrap_or(false) {
        log::error!("Warning: dwarf support is experimental and may not work as expected.");
    }

    if matches!(package, WasmPath::Component(_)) {
        let mut component_runner = generator::ComponentRunner::new(package.clone());


        postbuild::run_postbuild(&mut component_runner, &parsed_args, parsed_args.dwarf)?;

        return Ok(());
    }

    let mut vfs_build_opts = parsed_args.vfs_build_opts.clone();
    if parsed_args.dev {
        vfs_build_opts.no_opt_all = vfs_build_opts.no_opt_all.saturating_add(1);
    }

    let (mut component_runner, dwarf) = prebuild::run_prebuild_internal(
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

    postbuild::run_postbuild(&mut component_runner, &parsed_args, Some(dwarf))?;

    Ok(())
}
