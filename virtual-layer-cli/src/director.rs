use std::{fs, path::Path};

use camino::Utf8PathBuf;
use eyre::{Context as _, ContextCompat};

use crate::util::{
    CaminoUtilModule as _, ResultUtil as _, WalrusFID as _, WalrusUtilFuncs, WalrusUtilModule,
};

pub fn director(
    path: &Utf8PathBuf,
    wasm: &[impl AsRef<Path>],
    thread: bool,
    debug: bool,
    dwarf: bool,
) -> eyre::Result<Utf8PathBuf> {
    let mut module = walrus::Module::load(path, dwarf)?;

    let wasm = wasm.iter().map(|p| {
        camino::Utf8PathBuf::from_path_buf(p.as_ref().to_owned())
            .map_err(|_| eyre::eyre!("Invalid path: {}", p.as_ref().display()))
    });

    let module_mem_id = module
        .get_memory_id()
        .to_eyre()
        .wrap_err_with(|| eyre::eyre!("Failed to get memory ID"))?;

    for wasm in wasm {
        let wasm = wasm?;

        let wasm_name = wasm.get_file_main_name().unwrap();

        let trap_export_name = format!("__wasip1_vfs_{wasm_name}_memory_trap_anchor");
        let trap_id = module
            .exports
            .get_func(&trap_export_name)
            .to_eyre()
            .wrap_err_with(|| eyre::eyre!("Failed to get export function"))?;

        module.exports.remove(&trap_export_name).unwrap();

        let trap_body = match &mut module.funcs.get_mut(trap_id).kind {
            walrus::FunctionKind::Local(local_function) => {
                let start_block = local_function.entry_block();
                local_function.block_mut(start_block)
            }
            _ => panic!("Unexpected function kind"),
        };

        // Remove the fake value instruction
        let (store_index, store_info) = trap_body
            .iter()
            .enumerate()
            .find_map(|(i, (instr, _))| {
                if let walrus::ir::Instr::Store(walrus::ir::Store {
                    kind: walrus::ir::StoreKind::I32_8 { atomic: false },
                    memory,
                    arg,
                }) = instr
                {
                    if *memory != module_mem_id {
                        return Some(Err(eyre::eyre!(
                            "Unexpected memory ID: expected {:?}, got {:?}",
                            module_mem_id,
                            *memory
                        )));
                    }
                    Some(Ok((i, arg.to_owned())))
                } else {
                    None
                }
            })
            .wrap_err_with(|| eyre::eyre!("Failed to find store instruction"))??;
        trap_body.remove(store_index + 1);
        trap_body.remove(store_index);
        trap_body.remove(store_index - 1);

        if let Ok(fid) = module.imports.get_func(
            "wasip1-vfs",
            &format!("__wasip1_vfs_{wasm_name}_memory_director"),
        ) {
            module
                .replace_imported_func(fid, |(builder, local_id)| {
                    let mut func_body = builder.func_body();
                    func_body
                        .local_get(local_id[0])
                        .call(trap_id)
                        .i32_const(store_info.offset as i32)
                        .binop(walrus::ir::BinaryOp::I32Add)
                        .return_();
                })
                .to_eyre()
                .wrap_err_with(|| eyre::eyre!("Failed to replace imported function"))?;

            if debug {
                module.exports.add(
                    &format!("__wasip1_vfs_{wasm_name}_memory_director_anchor"),
                    fid,
                );
            }
        }
    }

    // shared global lock_memory_grow
    if thread {
        let global_set_alt = "__wasip1_vfs_memory_grow_global_alt_set".get_fid(&module.exports)?;
        let global_get_alt = "__wasip1_vfs_memory_grow_global_alt_get".get_fid(&module.exports)?;

        let global_id = module
            .globals
            .iter()
            .last()
            .map(|g| g.id())
            .wrap_err_with(|| eyre::eyre!("Failed to get global ID"))?;

        module
            .funcs
            .all_rewrite(
                |instr, _| match instr {
                    walrus::ir::Instr::GlobalSet(walrus::ir::GlobalSet { global })
                        if *global == global_id =>
                    {
                        *instr = walrus::ir::Instr::Call(walrus::ir::Call {
                            func: global_set_alt,
                        });
                    }
                    walrus::ir::Instr::GlobalGet(walrus::ir::GlobalGet { global })
                        if *global == global_id =>
                    {
                        *instr = walrus::ir::Instr::Call(walrus::ir::Call {
                            func: global_get_alt,
                        });
                    }
                    _ => {}
                },
                &[] as &[walrus::FunctionId],
            )
            .wrap_err_with(|| eyre::eyre!("Failed to rewrite global set/get"))?;

        module.globals.delete(global_id);
    }

    let new_path = path.with_extension("directed.wasm");

    if fs::metadata(&new_path).is_ok() {
        fs::remove_file(&new_path).expect("Failed to remove existing file");
    }

    module
        .emit_wasm_file(new_path.clone())
        .to_eyre()
        .wrap_err_with(|| eyre::eyre!("Failed to emit wasm file"))?;

    Ok(new_path)
}
