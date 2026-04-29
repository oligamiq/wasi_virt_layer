use crate::{
    args::{PostBuildArgs, PostBuildContext},
    generator::{self, WasmPath},
    test_run,
};

macro_rules! add_generator {
    ($runner:expr) => {{
        use crate::generator::{
            abi_connect, anonymous, check, debug, memory, patch_component, producer, shared_global,
            special_func, threads, wrap_unreachable,
        };

        generator::add_generators_by_type!(
            $runner,
            check::IsRustWasm,
            producer::Producer,
            anonymous::Anonymous,
            check::CheckUseLibrary,
            check::CheckVFSMemoryType,
            check::CheckUnusedThreads,
            wrap_unreachable::WrapUnreachableGenerator,
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

/// Shared internal logic for the postbuild phase.
///
/// Takes a `ComponentRunner` and transpiles the Component WASM into JavaScript files,
/// then generates test runner scripts.
pub(crate) fn run_postbuild(
    component_runner: &mut generator::ComponentRunner,
    parsed_args: &(impl PostBuildContext + ?Sized),
    dwarf: Option<bool>,
) -> eyre::Result<()> {
    let (threads, name, memory) = component_runner
        .component_to_files(
            parsed_args,
            dwarf.unwrap_or(false),
            parsed_args.adjust_abi(),
        )
        .wrap_err("Failed to run component to files")?;

    if !parsed_args.adjust_abi() {
        if threads {
            test_run::thread::gen_threads_run(name, memory, parsed_args.out_dir());

            let out_dir = parsed_args.out_dir();
            println!("\nBuilding for Deno...");
            let status = std::process::Command::new("deno")
                .arg("install")
                .current_dir(&out_dir)
                .status();

            if let Ok(s) = status {
                if s.success() {
                    println!("Deno dependencies installed successfully.");
                } else {
                    eprintln!("Warning: Failed to install Deno dependencies automatically.");
                }
            }
        } else {
            test_run::gen_test_run(name, parsed_args.out_dir());
        }

        let out_dir = parsed_args.out_dir();
        println!("\nBuild completed successfully!");
        println!("To run the generated program:");
        if threads {
            println!("  cd {out_dir}");
            println!("  deno run -A test_run.ts");
            println!("\n  # If using Bun:");
            println!("  bun install");
            println!("  bun run run");
            println!("\nOr run in browser:");
            println!("  cd {out_dir}");
            println!("  bun install");
            println!("  bun run dev");
        } else {
            println!("  cd {out_dir}");
            println!("  deno run -A test_run.ts");
            println!("\nOr run in browser:");
            println!("  cd {out_dir}");
            println!("  bunx serve  # or python3 -m http.server");
            println!("  open test_run.html in browser");
        }
    }

    Ok(())
}

use eyre::Context as _;

/// Executes the postbuild command, transpiling a Component WASM into JavaScript.
pub fn postbuild(parsed_args: PostBuildArgs) -> eyre::Result<()> {
    let package = parsed_args.package.clone();

    if !matches!(package, WasmPath::Component(_)) {
        eyre::bail!(
            "postbuild expects a Component WASM file. Got a non-component file. Use prebuild first to generate a Component WASM."
        );
    }

    let mut component_runner = generator::ComponentRunner::new(package);
    add_generator!(component_runner);

    run_postbuild(&mut component_runner, &parsed_args, parsed_args.dwarf)?;

    Ok(())
}
