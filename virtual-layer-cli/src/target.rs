use std::fs;

use camino::Utf8PathBuf;
use walrus::*;

use crate::util::{CaminoUtilModule as _, WalrusUtilModule};

pub fn adjust_target_wasm(path: &Utf8PathBuf) -> anyhow::Result<Utf8PathBuf> {
    let mut module = walrus::Module::from_file(path)?;

    // unsafe extern "C" fn __wasip1_vfs_flag_vfs_memory(ptr: *mut u8, src: *mut u8) {
    //     unsafe { core::ptr::copy_nonoverlapping(src, ptr, 1) };
    // }

    let id = module.add_func(&[ValType::I32, ValType::I32], &[], |memories, builder, arg_locals| {
        let mut func_body = builder.func_body();

        if memories.is_empty() {
            return Err(anyhow::anyhow!("No memories found"));
        }

        if memories.len() > 1 {
            return Err(anyhow::anyhow!("Multiple memories found. This is not supported yet. If you need this, please open an issue."));
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

    let new_path = path.with_extension("adjusted.wasm");

    if fs::metadata(&new_path).is_ok() {
        fs::remove_file(&new_path).expect("Failed to remove existing file");
    }
    module.emit_wasm_file(new_path.clone())?;

    Ok(new_path)
}
