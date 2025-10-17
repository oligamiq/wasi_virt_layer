use eyre::Context;

use crate::{
    args::TargetMemoryType,
    config_checker::{FeatureChecker, HasFeature, TomlRestorers},
    generator::WasmPath,
};

pub mod abi;
pub mod args;
pub mod compile;
pub mod config_checker;
pub mod down_color;
pub mod generator;
pub mod instrs;
pub mod test_run;
pub mod util;

macro_rules! add_generator {
    ($runner:expr) => {{
        use crate::generator::{
            abi_connect, check, debug, memory, patch_component, shared_global, special_func,
            threads,
        };

        generator::add_generators_by_type!(
            $runner,
            check::IsRustWasm,
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
            debug::DebugBase,
            debug::DebugCallMemoryGrow,
            debug::DebugExportVFSFunctions,
            debug::DebugCallFunctionSmallScale,
            debug::DebugCallFunctionMain,
            patch_component::PatchComponent,
        );

        $runner.checker(check::CheckUseWasiVirtLayer);
    }};
}

pub fn main(args: impl IntoIterator<Item = impl Into<String>>) -> eyre::Result<()> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();
    color_eyre::install()?;

    let parsed_args = args::Args::new(args);
    let package = parsed_args.get_package()?;

    let mut toml_restores = TomlRestorers::new();

    if matches!(package, WasmPath::Component(_)) {
        let mut component_runner = generator::ComponentRunner::new(package.clone());
        add_generator!(component_runner);

        let (threads, name, memory) = component_runner
            .component_to_files(&parsed_args, parsed_args.dwarf.unwrap_or(false))
            .wrap_err("Failed to run component to files")?;

        if threads {
            test_run::thread::gen_threads_run(name, memory, &parsed_args.out_dir);
        } else {
            test_run::gen_test_run(name, &parsed_args.out_dir);
        }

        return Ok(());
    }

    let vfs_package = parsed_args
        .get_package()
        .wrap_err("Failed to get package")?;
    let vfs_manifest_path = vfs_package.manifest_path().unwrap();
    let vfs_root_manifest_path = vfs_package.root_manifest_path().unwrap();

    let memory_type = {
        let memory_type_checker = FeatureChecker::new(
            "multi_memory",
            &vfs_manifest_path,
            &vfs_root_manifest_path,
            util::CRATE_NAME,
        );

        if let Some(target_memory_type) = parsed_args.target_memory_type {
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
            util::CRATE_NAME,
        );
        if let Some(threads) = parsed_args.threads {
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

    let dwarf = if let Some(dwarf) = parsed_args.dwarf {
        let checker = FeatureChecker::new_no_feature(
            &vfs_manifest_path,
            &vfs_root_manifest_path,
            util::CRATE_NAME,
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
            util::CRATE_NAME,
        );

        matches!(
            checker.has()?,
            HasFeature::EnabledOnNormal | HasFeature::EnabledOnWorkspace
        )
    };

    let mut generator = generator::GeneratorRunner::new(
        package.clone(),
        parsed_args.wasm.clone().into_boxed_slice(),
        threads,
        dwarf,
        unstable_print_debug,
        parsed_args.no_transpile,
        memory_type,
        toml_restores.clone(),
        parsed_args.get_wasm_memory_hints(),
    )?;

    add_generator!(generator);

    let mut component_runner = generator
        .run_layers_to_component(&parsed_args.out_dir)
        .wrap_err("Failed to run layers to component")?;

    if parsed_args.no_transpile {
        println!("Skipping transpile Component to JS as per --no-transpile flag...");
        let path = component_runner.path.path()?;
        let mut cmd = format!("cargo r -- -p {path}");
        if dwarf {
            cmd.push_str(" --dwarf true");
        }
        println!("You should custom component and run `{cmd}`");
        return Ok(());
    }

    let (_, name, memory) = component_runner
        .component_to_files(&parsed_args, dwarf)
        .wrap_err("Failed to run component to files")?;

    if threads {
        test_run::thread::gen_threads_run(name, memory, &parsed_args.out_dir);
    } else {
        test_run::gen_test_run(name, &parsed_args.out_dir);
    }

    Ok(())
}

// deno run dist/example_vfs.js
