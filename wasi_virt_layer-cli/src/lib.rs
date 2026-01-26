use eyre::Context;

use crate::{
    args::TargetMemoryType,
    config_checker::{FeatureChecker, HasFeature, TomlRestorers},
    fallback_command::CommandLock,
    generator::WasmPath,
    unique_name::UniqueName,
};

pub mod abi;
pub mod args;
pub mod compile;
pub mod config_checker;
pub mod ctrlc_handler;
pub mod down_color;
pub mod fallback_command; // fallback logic guarded by DISABLE_FALLBACK
pub mod generator;
pub mod instrs;
pub mod test_run;
pub mod unique_name;
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
            abi_connect::AdjustABI,
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
    ctrlc_handler::init();

    let command_lock = std::sync::Arc::new(std::sync::Mutex::new(Some(CommandLock::acquire()?)));
    let cl_clone = command_lock.clone();
    ctrlc_handler::register(move || {
        if let Ok(mut lock) = cl_clone.lock() {
            if let Some(l) = lock.take() {
                drop(l);
            }
        }
    });

    struct MainCommandLockGuard(std::sync::Arc<std::sync::Mutex<Option<CommandLock>>>);
    impl Drop for MainCommandLockGuard {
        fn drop(&mut self) {
            if let Ok(mut lock) = self.0.lock() {
                let _ = lock.take();
            }
        }
    }
    let _command_lock_guard = MainCommandLockGuard(command_lock);

    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();
    color_eyre::install()?;

    let parsed_args = args::Args::new(args);
    let package = parsed_args.get_package()?;

    let mut toml_restores = TomlRestorers::new();
    let tr_clone = toml_restores.clone();
    ctrlc_handler::register(move || {
        tr_clone.restore_if_needed();
    });

    if matches!(package, WasmPath::Component(_)) {
        let mut component_runner = generator::ComponentRunner::new(package.clone());
        add_generator!(component_runner);

        last(&mut component_runner, &parsed_args, parsed_args.dwarf)?;

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
            UniqueName::CRATE_NAME,
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
            UniqueName::CRATE_NAME,
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

    let mut generator = generator::GeneratorRunner::new(
        package.clone(),
        parsed_args.wasm.clone().into_boxed_slice(),
        threads,
        dwarf,
        unstable_print_debug,
        parsed_args.no_transpile,
        parsed_args.adjust_abi,
        parsed_args.keep_build_artifacts,
        memory_type,
        toml_restores,
        parsed_args.get_wasm_memory_hints(),
    )?;

    add_generator!(generator);

    let mut component_runner = generator
        .run_layers_to_component(&parsed_args.out_dir, parsed_args.keep_build_artifacts)
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

    last(&mut component_runner, &parsed_args, Some(dwarf))?;

    Ok(())
}

// deno run dist/example_vfs.js

fn last(
    component_runner: &mut generator::ComponentRunner,
    parsed_args: &args::Args,
    dwarf: Option<bool>,
) -> eyre::Result<()> {
    let (threads, name, memory) = component_runner
        .component_to_files(&parsed_args, dwarf.unwrap_or(false), parsed_args.adjust_abi)
        .wrap_err("Failed to run component to files")?;

    if !parsed_args.adjust_abi {
        if threads {
            test_run::thread::gen_threads_run(name, memory, &parsed_args.out_dir);
        } else {
            test_run::gen_test_run(name, &parsed_args.out_dir);
        }
    }

    Ok(())
}
