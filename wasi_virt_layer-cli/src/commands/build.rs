use crate::{
    args::BuildArgs,
    commands::{postbuild, prebuild},
    generator::{self, WasmPath},
};

macro_rules! add_generator {
    ($runner:expr) => {{
        use crate::generator::{
            abi_connect, anonymous, check, debug, memory, patch_component, producer, shared_global,
            special_func, threads,
        };

        generator::add_generators_by_type!(
            $runner,
            check::IsRustWasm,
            producer::Producer,
            anonymous::Anonymous,
            check::CheckUseLibrary,
            check::CheckVFSMemoryType,
            check::CheckUnusedThreads,
            threads::ThreadsSpawn,
            threads::ThreadsSpawnPatch,
            special_func::StartFunc,
            special_func::MainVoidFunc,
            special_func::ResetFunc,
            shared_global::SharedGlobal,
            memory::TemporaryRefugeMemory,
            memory::MemoryBridge,
            memory::MemoryTrap,
            abi_connect::ConnectWasip1ABI,
            abi_connect::ConnectWasip1ThreadsABI,
            abi_connect::NonRecursiveWasiABI,
            debug::SimpleDebug,
            debug::DebugCallMemoryGrow,
            debug::DebugExportVFSFunctions,
            debug::DebugCallFunctionSmallScale,
            debug::DebugCallFunctionMain,
            patch_component::PatchComponent,
        );

        $runner.checker(check::CheckUseWasiVirtLayer);
    }};
}

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
        add_generator!(component_runner);

        postbuild::run_postbuild(&mut component_runner, &parsed_args, parsed_args.dwarf)?;

        return Ok(());
    }

    let (mut component_runner, dwarf) = prebuild::run_prebuild_internal(
        package,
        &parsed_args.wasm,
        parsed_args.target_memory_type,
        parsed_args.threads,
        parsed_args.dwarf,
        parsed_args.adjust_abi,
        parsed_args.keep_build_artifacts,
        &parsed_args.out_dir,
        parsed_args.get_wasm_memory_hints(),
    )?;

    postbuild::run_postbuild(&mut component_runner, &parsed_args, Some(dwarf))?;

    Ok(())
}
