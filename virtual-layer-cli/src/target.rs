use std::fs;

use camino::Utf8PathBuf;
use eyre::Context as _;
use walrus::*;

use crate::{
    common::WASIP1_FUNC,
    util::{CaminoUtilModule as _, ResultUtil as _, WalrusUtilModule},
};

pub fn adjust_target_wasm(path: &Utf8PathBuf) -> eyre::Result<Utf8PathBuf> {
    let mut module = walrus::Module::from_file(path)
        .to_eyre()
        .wrap_err_with(|| eyre::eyre!("Failed to load module"))?;

    // unsafe extern "C" fn __wasip1_vfs_flag_vfs_memory(ptr: *mut u8, src: *mut u8) {
    //     unsafe { core::ptr::copy_nonoverlapping(src, ptr, 1) };
    // }
    let id = module.add_func(&[ValType::I32, ValType::I32], &[], |memories, builder, arg_locals| {
        let mut func_body = builder.func_body();

        if memories.is_empty() {
            return Err(eyre::eyre!("No memories found"));
        }

        // todo!(); check wasi func's access
        if memories.len() > 1 {
            return Err(eyre::eyre!("Multiple memories found. This is not supported yet. If you need this, please open an issue."));
        }

        let memory_id = memories.iter().next().unwrap().id();

        func_body.local_get(arg_locals[0]).local_get(arg_locals[1])
            .load(memory_id, ir::LoadKind::I32_8 { kind: ir::ExtendedLoad::ZeroExtend }, ir::MemArg {
                offset: 0,
                align: 0,
            })
            .store(memory_id, ir::StoreKind::I32_8 {
                atomic: false,
            }, ir::MemArg {
                offset: 0,
                align: 0,
            });

        func_body.return_();

        Ok(())
    })?;

    module.exports.add(
        &format!(
            "__wasip1_vfs_flag_{}_memory",
            path.get_file_main_name().unwrap()
        ),
        id,
    );

    let rewrite_exports = ["_start", "__main_void", "memory"];

    module
        .exports
        .iter_mut()
        .filter(|export| rewrite_exports.contains(&export.name.as_str()))
        .for_each(|export| {
            export.name = format!(
                "__wasip1_vfs_{}_{}",
                path.get_file_main_name().unwrap(),
                export.name
            );
        });

    module
        .imports
        .iter_mut()
        .filter(|import| {
            WASIP1_FUNC.contains(&import.name.as_str()) && import.module == "wasi_snapshot_preview1"
        })
        .for_each(|import| {
            import.name = format!(
                "__wasip1_vfs_{}_{}",
                path.get_file_main_name().unwrap(),
                import.name
            );
        });

    let new_path = path.with_extension("adjusted.wasm");

    if fs::metadata(&new_path).is_ok() {
        fs::remove_file(&new_path).expect("Failed to remove existing file");
    }
    module
        .emit_wasm_file(new_path.clone())
        .to_eyre()
        .wrap_err_with(|| eyre::eyre!("Failed to emit wasm file"))?;

    Ok(new_path)
}
