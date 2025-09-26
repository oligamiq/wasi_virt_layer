use eyre::{Context, ContextCompat};
use rewrite::adjust_wasm;
use util::CaminoUtilModule as _;

use crate::{
    rewrite::{TargetMemoryType, TomlRestorers, adjust_target_feature, get_target_feature},
    util::{ResultUtil as _, WalrusUtilModule},
};

pub mod adjust;
pub mod args;
pub mod building;
pub mod common;
pub mod debug;
pub mod director;
pub mod down_color;
pub mod instrs;
pub mod is_valid;
pub mod merge;
pub mod rewrite;
pub mod shared_global;
pub mod target;
pub mod test_run;
pub mod threads;
pub mod util;

pub fn main(args: impl IntoIterator<Item = impl Into<String>>) -> eyre::Result<()> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();
    color_eyre::install()?;

    let mut tmp_files = Vec::new();
    let mut toml_restores = TomlRestorers::new();

    let parsed_args = args::Args::new(args);

    let out_dir = camino::Utf8PathBuf::from(&parsed_args.out_dir);

    let manifest_path = parsed_args.get_manifest_path();
    let cargo_metadata = {
        let mut metadata_command = cargo_metadata::MetadataCommand::new();
        if let Some(manifest_path) = manifest_path {
            metadata_command.manifest_path(manifest_path);
        }
        metadata_command.exec().unwrap()
    };
    let building_crate = building::get_building_crate(&cargo_metadata, &parsed_args.package)?;

    if let Some(target_memory_type) = parsed_args.target_memory_type {
        toml_restores.push(adjust_target_feature(
            &cargo_metadata,
            &building_crate,
            target_memory_type == TargetMemoryType::Multi,
            "multi_memory",
        )?);
    }

    if let Some(threads) = parsed_args.threads {
        toml_restores.push(adjust_target_feature(
            &cargo_metadata,
            &building_crate,
            threads,
            "threads",
        )?);
    }

    if let Some(dwarf) = parsed_args.dwarf {
        toml_restores.push(rewrite::set_dwarf(&cargo_metadata, &building_crate, dwarf)?);
    }
    let dwarf = parsed_args.dwarf.unwrap_or(false);

    let threads = parsed_args
        .threads
        .unwrap_or(get_target_feature(&building_crate, "threads")?);

    println!("Compiling {}", building_crate.name);

    // todo!();
    let ret = building::build_vfs(
        manifest_path.clone(),
        &parsed_args.package,
        building_crate.clone(),
        threads,
    )
    .wrap_err_with(|| eyre::eyre!("Failed to build VFS: {}", building_crate.name))?;

    toml_restores.restore()?;

    println!("Optimizing VFS Wasm...");
    let ret =
        building::optimize_wasm(&ret, &[], false, dwarf).wrap_err("Failed to optimize Wasm")?;

    let print_debug = debug::has_debug(&walrus::Module::load(&ret, dwarf)?);

    println!("Adjusting VFS Wasm...");
    let (ret, target_memory_type, has_debug_call_memory_grow) =
        adjust_wasm(&ret, &parsed_args.wasm, threads, print_debug, dwarf)
            .wrap_err("Failed to adjust Wasm")?;

    println!("Optimizing VFS Wasm...");
    let ret =
        building::optimize_wasm(&ret, &[], false, dwarf).wrap_err("Failed to optimize Wasm")?;

    println!("Generated VFS: {ret}");

    println!("Remove existing output directory...");
    if std::fs::metadata(&out_dir).is_ok() {
        std::fs::remove_dir_all(&out_dir).expect("Failed to remove existing directory");
    }
    std::fs::create_dir_all(&out_dir).expect("Failed to create output directory");

    println!("Preparing target Wasm...");
    let (wasm_paths, wasm_names) = parsed_args
        .wasm
        .iter()
        .zip(
            parsed_args
                .wasm_memory_hint
                .iter()
                .map(|h| Some(*h))
                .chain(std::iter::repeat(None)),
        )
        .map(|(old_wasm, memory_hint)| {
            let file_name = old_wasm.file_name().unwrap();
            let wasm = format!("{out_dir}/{file_name}");
            std::fs::copy(old_wasm, &wasm)
                .wrap_err_with(|| eyre::eyre!("Failed to find Wasm file {old_wasm}"))?;
            let name = old_wasm.get_file_main_name().unwrap();
            println!("Optimizing target Wasm [{name}]...");
            tmp_files.push(wasm.to_string());
            let wasm = building::optimize_wasm(&wasm.into(), &[], false, dwarf)
                .wrap_err("Failed to optimize Wasm")?;
            tmp_files.push(wasm.to_string());
            println!("Adjusting target Wasm [{name}]...");
            let wasm = target::adjust_target_wasm(
                &wasm,
                memory_hint,
                threads,
                print_debug,
                dwarf,
                target_memory_type,
                has_debug_call_memory_grow,
            )
            .wrap_err("Failed to adjust Wasm")?;
            tmp_files.push(wasm.to_string());
            Ok((wasm, name))
        })
        .collect::<eyre::Result<(Vec<_>, Vec<_>)>>()?;

    println!("Merging Wasm...");

    let output = format!("{out_dir}/merged.wasm");
    if std::fs::metadata(&output).is_ok() {
        std::fs::remove_file(&output).expect("Failed to remove existing file");
    }
    merge::merge(&ret, &wasm_paths, &output, dwarf).wrap_err("Failed to merge Wasm")?;
    tmp_files.push(output.clone());

    println!("Optimizing Merged Wasm...");
    let ret = building::optimize_wasm(&output.clone().into(), &[], false, dwarf)
        .wrap_err("Failed to optimize merged Wasm")?;
    tmp_files.push(ret.to_string());

    println!("Adjusting Merged Wasm...");
    let ret = adjust::adjust_merged_wasm(
        &ret,
        &wasm_paths,
        threads,
        target_memory_type,
        print_debug,
        dwarf,
    )
    .wrap_err("Failed to adjust merged Wasm")?;
    tmp_files.push(ret.to_string());

    let ret = if matches!(target_memory_type, TargetMemoryType::Single) {
        println!("Generating single memory Merged Wasm...");
        // let ret = building::optimize_wasm(&ret, &["--multi-memory-lowering"], true, dwarf)?;
        let ret = building::optimize_wasm(
            &ret,
            // &["--multi-memory-lowering-with-bounds-checks"],
            &["--multi-memory-lowering"],
            true,
            dwarf,
        )?;
        tmp_files.push(ret.to_string());
        ret
    } else {
        ret
    };

    println!("Optimizing Merged Wasm...");
    let ret = building::optimize_wasm(&ret, &[], false, dwarf)
        .wrap_err("Failed to optimize merged Wasm")?;
    tmp_files.push(ret.to_string());

    let ret = if matches!(target_memory_type, TargetMemoryType::Single) {
        println!("Directing process {target_memory_type} memory Merged Wasm...");
        let ret = director::director(&ret, &wasm_paths, print_debug, dwarf)?;
        tmp_files.push(ret.to_string());
        ret
    } else {
        ret
    };

    println!("Translating Wasm to Component...");
    let component = building::wasm_to_component(&ret, &wasm_names)
        .wrap_err("Failed to translate Wasm to Component")?;
    tmp_files.push(component.to_string());

    println!("Translating Component to JS...");
    let binary = std::fs::read(&component).wrap_err("Failed to read component")?;
    let transpiled = parsed_args
        .transpile_to_js(&binary, &building_crate.name)
        .wrap_err("Failed to transpile to JS")?;

    let mut core_wasm = None;
    let mut core_wasm_name = None;
    for (name, data) in transpiled.files.iter() {
        let name = camino::Utf8PathBuf::from(name);
        let file_name = out_dir.join(&name);
        if std::fs::metadata(&file_name).is_ok() {
            std::fs::remove_file(&file_name)
                .wrap_err_with(|| eyre::eyre!("Failed to remove existing file: {file_name}"))?;
        }
        if name.as_str().ends_with(".core.wasm") {
            let file_name = camino::Utf8PathBuf::from(file_name);
            std::fs::write(&file_name, &data)
                .wrap_err_with(|| eyre::eyre!("Failed to write core wasm file: {file_name}"))?;
            core_wasm = Some(file_name);
            core_wasm_name = Some(name);
        } else {
            if let Some(parent) = name.parent() {
                if !parent.as_str().is_empty() {
                    let dir = name.ancestors().nth(1).wrap_err_with(|| {
                        eyre::eyre!("Failed to get parent directory: {}", name)
                    })?;
                    let joined_dir = out_dir.join(dir);
                    if !std::fs::metadata(&joined_dir).is_ok() {
                        if dir.as_str() != "interfaces" {
                            log::warn!("Creating directory: {joined_dir}");
                        }
                        std::fs::create_dir_all(&joined_dir).wrap_err_with(|| {
                            eyre::eyre!("Failed to create directory: {joined_dir}")
                        })?;
                    }
                }
            }
            std::fs::write(&file_name, &data)
                .wrap_err_with(|| eyre::eyre!("Failed to write transpiled file: {file_name}"))?;
        }
    }

    let core_wasm = core_wasm
        .as_ref()
        .ok_or_else(|| eyre::eyre!("Failed to find core wasm"))?;

    let core_wasm_name = core_wasm_name
        .as_ref()
        .ok_or_else(|| eyre::eyre!("Failed to find core wasm name"))?;

    println!("Optimizing core Wasm...");
    let core_wasm_opt = building::optimize_wasm(&core_wasm.into(), &[], false, dwarf)
        .wrap_err("Failed to optimize core Wasm")?;

    tmp_files.push(core_wasm.to_string());

    let (core_wasm_opt, mem_size) = if threads || print_debug {
        let (core_wasm_opt_adjusted_opt, mem_size) = if threads {
            println!("Adjusting core Wasm...");
            let (core_wasm_opt_adjusted, mem_size) =
                threads::adjust_core_wasm(&core_wasm_opt, threads, dwarf)
                    .wrap_err("Failed to adjust core Wasm")?;
            println!("Optimizing core Wasm...");
            let core_wasm_opt_adjusted_opt =
                building::optimize_wasm(&core_wasm_opt_adjusted, &[], false, dwarf)
                    .wrap_err("Failed to optimize core Wasm")?;
            tmp_files.push(core_wasm_opt.to_string());
            tmp_files.push(core_wasm_opt_adjusted.to_string());
            (core_wasm_opt_adjusted_opt, mem_size)
        } else {
            (core_wasm_opt, None)
        };

        if print_debug {
            let core_wasm_opt_adjusted_opt_debug =
                core_wasm_opt_adjusted_opt.with_extension("debug.wasm");

            tmp_files.push(core_wasm_opt_adjusted_opt.to_string());

            let mut module = walrus::Module::load(&core_wasm_opt_adjusted_opt, dwarf)?;

            debug::generate_debug_call_function(&mut module)
                .wrap_err("Failed to generate debug_call_function")?;

            module
                .emit_wasm_file(&core_wasm_opt_adjusted_opt_debug)
                .to_eyre()
                .wrap_err("Failed to write temporary wasm file")?;

            let mut module = walrus::Module::load(&core_wasm_opt_adjusted_opt_debug, dwarf)?;

            debug::generate_debug_call_function_last(&mut module)
                .wrap_err("Failed to generate debug_blind_print_etc_flag")?;

            module
                .emit_wasm_file(&core_wasm_opt_adjusted_opt_debug)
                .to_eyre()
                .wrap_err("Failed to write temporary wasm file")?;

            (core_wasm_opt_adjusted_opt_debug, mem_size)
        } else {
            (core_wasm_opt_adjusted_opt, mem_size)
        }
    } else {
        (core_wasm_opt, Some(Vec::new()))
    };

    for tmp_file in tmp_files {
        std::fs::remove_file(&tmp_file)
            .wrap_err_with(|| eyre::eyre!("Failed to remove tmp file: {tmp_file}"))?;
    }

    std::fs::rename(&core_wasm_opt, &core_wasm).expect("Failed to rename file");

    let name = core_wasm_name
        .get_file_main_name()
        .wrap_err("Failed to get core wasm main name")?;

    if let Some(mem_size) = mem_size {
        test_run::thread::gen_threads_run(name, mem_size, &out_dir);
    } else {
        test_run::gen_test_run(name, out_dir);
    }

    Ok(())
}

// deno run dist/example_vfs.js
