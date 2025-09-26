use std::fs;

use camino::Utf8PathBuf;
use eyre::Context as _;

use crate::{
    common::Wasip1SnapshotPreview1Func,
    rewrite::TargetMemoryType,
    shared_global, threads,
    util::{
        CaminoUtilModule as _, ResultUtil as _, WalrusUtilFuncs as _, WalrusUtilImport,
        WalrusUtilModule,
    },
};

pub fn adjust_target_wasm(
    path: &Utf8PathBuf,
    memory_hint: Option<usize>,
    threads: bool,
    debug: bool,
    dwarf: bool,
    mem_type: TargetMemoryType,
    has_debug_call_memory_grow: bool,
) -> eyre::Result<Utf8PathBuf> {
    let name = path
        .get_file_main_name()
        .unwrap_or_else(|| panic!("Failed to get file name from {path}"));

    let mut module = walrus::Module::load(path, dwarf)?;

    if threads {
        threads::remove_unused_threads_function(&mut module)
            .wrap_err("Failed to remove unused threads function")?;
    }

    module.create_memory_anchor(&name, memory_hint)?;

    module
        .create_global_anchor(&name)
        .wrap_err("Failed to create global anchor")?;

    let rewrite_exports = ["_start", "__main_void", "memory"];

    module
        .exports
        .iter_mut()
        .filter(|export| rewrite_exports.contains(&export.name.as_str()))
        .for_each(|export| {
            export.name = format!("__wasip1_vfs_{}_{}", &name, export.name);
        });

    module
        .imports
        .iter_mut()
        .filter(|import| {
            <Wasip1SnapshotPreview1Func as strum::VariantNames>::VARIANTS
                .contains(&import.name.as_str())
                && import.module == "wasi_snapshot_preview1"
        })
        .for_each(|import| {
            import.name = format!("__wasip1_vfs_{name}_{}", import.name);
        });

    // threads
    if threads {
        module
            .imports
            .find_mut(("wasi", "thread-spawn"))
            .ok()
            .map(|import| {
                import.name = format!("__wasip1_vfs_wasi_thread_spawn_{name}");
            });

        module
            .exports
            .iter_mut()
            .find(|export| export.name == "wasi_thread_start")
            .map(|export| {
                export.name = format!("__wasip1_vfs_wasi_thread_start_{name}");
            });

        if matches!(mem_type, TargetMemoryType::Single) {
            shared_global::lock_memory_grow(&mut module, &name)
                .wrap_err("Failed to wrap memory.grow by lock instructions")?;
        }

        if debug && has_debug_call_memory_grow {
            let func_ty = module.types.add(
                &[
                    walrus::ValType::I32,
                    walrus::ValType::I32,
                    walrus::ValType::I32,
                ],
                &[],
            );
            let (id, _) = module.add_import_func(
                "wasip1-vfs_debug",
                "debug_call_memory_grow_import",
                func_ty,
            );
            let (id2, _) = module.add_import_func(
                "wasip1-vfs_debug",
                "debug_call_memory_grow_pre_import",
                func_ty,
            );

            module
                .gen_inspect_with_finalize(
                    Some(id),
                    Some(id2),
                    &[walrus::ValType::I32],
                    &[walrus::ValType::I32],
                    &module.funcs.find_children_with(id).unwrap(),
                    |instr| {
                        if let walrus::ir::Instr::MemoryGrow(walrus::ir::MemoryGrow {
                            memory: _,
                            ..
                        }) = instr
                        {
                            static mut I: i32 = 117;
                            unsafe { I += 1 };
                            println!("Rewriting memory.grow to call debug function");
                            Some([1, unsafe { I }])
                        } else {
                            None
                        }
                    },
                )
                .unwrap();
        }
    }

    let new_path = path.with_extension("adjusted.wasm");

    if fs::metadata(&new_path).is_ok() {
        fs::remove_file(&new_path).expect("Failed to remove existing file");
    }
    module
        .emit_wasm_file(new_path.clone())
        .to_eyre()
        .wrap_err("Failed to emit wasm file")?;

    Ok(new_path)
}
