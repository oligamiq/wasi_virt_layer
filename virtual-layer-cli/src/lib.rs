use eyre::Context;

use crate::{
    args::TargetMemoryType,
    config_checker::{FeatureChecker, HasFeature, TomlRestorers},
};

pub mod args;
pub mod compile;
pub mod config_checker;
pub mod down_color;
pub mod generator;
pub mod instrs;
pub mod abi;
pub mod rewrite;
pub mod test_run;
pub mod util;

pub fn main(args: impl IntoIterator<Item = impl Into<String>>) -> eyre::Result<()> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();
    color_eyre::install()?;

    // let mut tmp_files = Vec::new();
    let mut toml_restores = TomlRestorers::new();

    let parsed_args = args::Args::new(args);

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
            "wasip1-virtual-layer",
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
            "wasip1-virtual-layer",
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
            "wasip1-virtual-layer",
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
            "wasip1-virtual-layer",
        );

        matches!(
            checker.has()?,
            HasFeature::EnabledOnNormal | HasFeature::EnabledOnWorkspace
        )
    };

    let package = parsed_args.get_package()?;
    let mut generator = generator::GeneratorRunner::new(
        package,
        parsed_args.wasm.clone().into_boxed_slice(),
        threads,
        dwarf,
        unstable_print_debug,
        parsed_args.no_transpile,
        memory_type,
        toml_restores.clone(),
        parsed_args.get_wasm_memory_hints(),
    )?;
    generator
        .run_layers_to_component(&parsed_args.out_dir)
        .wrap_err("Failed to run layers to component")?;

    generator
        .component_to_files(&parsed_args)
        .wrap_err("Failed to run component to files")?;

    // let vfs_name = generator.ctx().vfs_name.clone();

    // toml_restores.restore()?;

    // let ret = generator.path().path()?.clone();

    // println!("Optimizing VFS Wasm...");
    // let ret =
    //     building::optimize_wasm(&ret, &[], false, dwarf).wrap_err("Failed to optimize Wasm")?;

    // println!("Adjusting VFS Wasm...");
    // let ret = adjust_wasm(
    //     &ret,
    //     &generator
    //         .targets()
    //         .iter()
    //         .map(|p| p.name())
    //         .collect::<eyre::Result<Vec<_>>>()?,
    //     threads,
    //     unstable_print_debug,
    //     dwarf,
    // )
    // .wrap_err("Failed to adjust Wasm")?;

    // println!("Optimizing VFS Wasm...");
    // let ret =
    //     building::optimize_wasm(&ret, &[], false, dwarf).wrap_err("Failed to optimize Wasm")?;

    // println!("Generated VFS: {ret}");

    // println!("Remove existing output directory...");
    // if std::fs::metadata(&out_dir).is_ok() {
    //     std::fs::remove_dir_all(&out_dir).expect("Failed to remove existing directory");
    // }
    // std::fs::create_dir_all(&out_dir).expect("Failed to create output directory");

    // println!("Preparing target Wasm...");
    // let (wasm_paths, wasm_names) = generator
    //     .targets()
    //     .iter()
    //     .zip(parsed_args.get_wasm_memory_hints())
    //     .map(|(old_wasm, memory_hint)| {
    //         let file_name = old_wasm.name().unwrap();
    //         let old_wasm = old_wasm.path()?;
    //         let wasm = format!("{out_dir}/{file_name}");
    //         std::fs::copy(old_wasm, &wasm)
    //             .wrap_err_with(|| eyre::eyre!("Failed to find Wasm file {old_wasm}"))?;
    //         let name = old_wasm.get_file_main_name().unwrap();
    //         println!("Optimizing target Wasm [{name}]...");
    //         tmp_files.push(wasm.to_string());
    //         let wasm = building::optimize_wasm(&wasm.into(), &[], false, dwarf)
    //             .wrap_err("Failed to optimize Wasm")?;
    //         tmp_files.push(wasm.to_string());
    //         println!("Adjusting target Wasm [{name}]...");
    //         let wasm = target::adjust_target_wasm(&wasm, threads, unstable_print_debug)
    //             .wrap_err("Failed to adjust Wasm")?;
    //         tmp_files.push(wasm.to_string());
    //         Ok((wasm, name))
    //     })
    //     .collect::<eyre::Result<(Vec<_>, Vec<_>)>>()?;

    // println!("Merging Wasm...");

    // let output = format!("{out_dir}/merged.wasm");
    // if std::fs::metadata(&output).is_ok() {
    //     std::fs::remove_file(&output).expect("Failed to remove existing file");
    // }
    // merge::merge(&ret, &wasm_paths, &output, dwarf).wrap_err("Failed to merge Wasm")?;
    // tmp_files.push(output.clone());

    // println!("Optimizing Merged Wasm...");
    // let ret = building::optimize_wasm(&output.clone().into(), &[], false, dwarf)
    //     .wrap_err("Failed to optimize merged Wasm")?;
    // tmp_files.push(ret.to_string());

    // println!("Adjusting Merged Wasm...");
    // let ret = adjust::adjust_merged_wasm(
    //     &ret,
    //     &wasm_paths,
    //     threads,
    //     target_memory_type,
    //     unstable_print_debug,
    //     dwarf,
    // )
    // .wrap_err("Failed to adjust merged Wasm")?;
    // tmp_files.push(ret.to_string());

    // let ret = if target_memory_type.is_single() {
    //     println!("Generating single memory Merged Wasm...");
    //     // let ret = building::optimize_wasm(&ret, &["--multi-memory-lowering"], true, dwarf)?;
    //     let ret = building::optimize_wasm(
    //         &ret,
    //         // &["--multi-memory-lowering-with-bounds-checks"],
    //         &["--multi-memory-lowering"],
    //         true,
    //         dwarf,
    //     )?;
    //     tmp_files.push(ret.to_string());
    //     ret
    // } else {
    //     ret
    // };

    // println!("Optimizing Merged Wasm...");
    // let ret = building::optimize_wasm(&ret, &[], false, dwarf)
    //     .wrap_err("Failed to optimize merged Wasm")?;
    // tmp_files.push(ret.to_string());

    // let ret = if target_memory_type.is_single() {
    //     println!("Directing process {target_memory_type} memory Merged Wasm...");
    //     let ret = director::director(&ret, &wasm_paths, threads, unstable_print_debug, dwarf)?;
    //     tmp_files.push(ret.to_string());
    //     ret
    // } else {
    //     ret
    // };

    // println!("Translating Wasm to Component...");
    // let component = building::wasm_to_component(&ret, &wasm_names)
    //     .wrap_err("Failed to translate Wasm to Component")?;
    // tmp_files.push(component.to_string());

    // println!("Translating Component to JS...");
    // let binary = std::fs::read(&component).wrap_err("Failed to read component")?;
    // let transpiled = parsed_args
    //     .transpile_to_js(&binary, &vfs_name)
    //     .wrap_err("Failed to transpile to JS")?;

    // let mut core_wasm = None;
    // let mut core_wasm_name = None;
    // for (name, data) in transpiled.files.iter() {
    //     let name = camino::Utf8PathBuf::from(name);
    //     let file_name = out_dir.join(&name);
    //     if std::fs::metadata(&file_name).is_ok() {
    //         std::fs::remove_file(&file_name)
    //             .wrap_err_with(|| eyre::eyre!("Failed to remove existing file: {file_name}"))?;
    //     }
    //     if name.as_str().ends_with(".core.wasm") {
    //         let file_name = camino::Utf8PathBuf::from(file_name);
    //         std::fs::write(&file_name, &data)
    //             .wrap_err_with(|| eyre::eyre!("Failed to write core wasm file: {file_name}"))?;
    //         core_wasm = Some(file_name);
    //         core_wasm_name = Some(name);
    //     } else {
    //         if let Some(parent) = name.parent() {
    //             if !parent.as_str().is_empty() {
    //                 let dir = name.ancestors().nth(1).wrap_err_with(|| {
    //                     eyre::eyre!("Failed to get parent directory: {}", name)
    //                 })?;
    //                 let joined_dir = out_dir.join(dir);
    //                 if !std::fs::metadata(&joined_dir).is_ok() {
    //                     if dir.as_str() != "interfaces" {
    //                         log::warn!("Creating directory: {joined_dir}");
    //                     }
    //                     std::fs::create_dir_all(&joined_dir).wrap_err_with(|| {
    //                         eyre::eyre!("Failed to create directory: {joined_dir}")
    //                     })?;
    //                 }
    //             }
    //         }
    //         std::fs::write(&file_name, &data)
    //             .wrap_err_with(|| eyre::eyre!("Failed to write transpiled file: {file_name}"))?;
    //     }
    // }

    // let core_wasm = core_wasm
    //     .as_ref()
    //     .ok_or_else(|| eyre::eyre!("Failed to find core wasm"))?;

    // let core_wasm_name = core_wasm_name
    //     .as_ref()
    //     .ok_or_else(|| eyre::eyre!("Failed to find core wasm name"))?;

    // println!("Optimizing core Wasm...");
    // let core_wasm_opt = building::optimize_wasm(&core_wasm.into(), &[], false, dwarf)
    //     .wrap_err("Failed to optimize core Wasm")?;

    // tmp_files.push(core_wasm.to_string());

    // let (core_wasm_opt, mem_size) = if threads || unstable_print_debug {
    //     let (core_wasm_opt_adjusted_opt, mem_size) = if threads {
    //         println!("Adjusting core Wasm...");
    //         let (core_wasm_opt_adjusted, mem_size) =
    //             threads::adjust_core_wasm(&core_wasm_opt, threads, dwarf)
    //                 .wrap_err("Failed to adjust core Wasm")?;
    //         println!("Optimizing core Wasm...");
    //         let core_wasm_opt_adjusted_opt =
    //             building::optimize_wasm(&core_wasm_opt_adjusted, &[], false, dwarf)
    //                 .wrap_err("Failed to optimize core Wasm")?;
    //         tmp_files.push(core_wasm_opt.to_string());
    //         tmp_files.push(core_wasm_opt_adjusted.to_string());
    //         (core_wasm_opt_adjusted_opt, mem_size)
    //     } else {
    //         (core_wasm_opt, None)
    //     };

    //     if unstable_print_debug {
    //         let core_wasm_opt_adjusted_opt_debug =
    //             core_wasm_opt_adjusted_opt.with_extension("debug.wasm");

    //         tmp_files.push(core_wasm_opt_adjusted_opt.to_string());

    //         let mut module = walrus::Module::load(&core_wasm_opt_adjusted_opt, dwarf)?;

    //         debug::generate_debug_call_function(&mut module)
    //             .wrap_err("Failed to generate debug_call_function")?;

    //         module
    //             .emit_wasm_file(&core_wasm_opt_adjusted_opt_debug)
    //             .to_eyre()
    //             .wrap_err("Failed to write temporary wasm file")?;

    //         let mut module = walrus::Module::load(&core_wasm_opt_adjusted_opt_debug, dwarf)?;

    //         debug::generate_debug_call_function_last(&mut module)
    //             .wrap_err("Failed to generate debug_blind_print_etc_flag")?;

    //         module
    //             .emit_wasm_file(&core_wasm_opt_adjusted_opt_debug)
    //             .to_eyre()
    //             .wrap_err("Failed to write temporary wasm file")?;

    //         (core_wasm_opt_adjusted_opt_debug, mem_size)
    //     } else {
    //         (core_wasm_opt_adjusted_opt, mem_size)
    //     }
    // } else {
    //     (core_wasm_opt, Some(Vec::new()))
    // };

    // for tmp_file in tmp_files {
    //     std::fs::remove_file(&tmp_file)
    //         .wrap_err_with(|| eyre::eyre!("Failed to remove tmp file: {tmp_file}"))?;
    // }

    // std::fs::rename(&core_wasm_opt, &core_wasm).expect("Failed to rename file");

    // let name = core_wasm_name
    //     .get_file_main_name()
    //     .wrap_err("Failed to get core wasm main name")?;

    // if let Some(mem_size) = mem_size {
    //     test_run::thread::gen_threads_run(name, mem_size, &out_dir);
    // } else {
    //     test_run::gen_test_run(name, out_dir);
    // }

    Ok(())
}

// deno run dist/example_vfs.js
