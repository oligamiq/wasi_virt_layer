use std::path::Path;

use camino::Utf8PathBuf;
use eyre::Context as _;

use crate::{
    rewrite::TargetMemoryType,
    util::{CaminoUtilModule as _, ResultUtil as _},
};

pub fn director(
    path: &Utf8PathBuf,
    wasm: &[impl AsRef<Path>],
    memory_type: TargetMemoryType,
) -> eyre::Result<Utf8PathBuf> {
    let mut module = walrus::Module::from_file(path)
        .to_eyre()
        .wrap_err_with(|| eyre::eyre!("Failed to load module"))?;

    let wasm = wasm.iter().map(|p| {
        camino::Utf8PathBuf::from_path_buf(p.as_ref().to_owned())
            .map_err(|_| eyre::eyre!("Invalid path: {}", p.as_ref().display()))
    });
    for wasm in wasm {
        let wasm = wasm?;

        print!("Wasm file: {wasm}");
        let wasm_name = wasm.get_file_main_name().unwrap();

        print!("Wasm name: {wasm_name}");

        let trap_id = module
            .exports
            .get_func(&format!("__wasip1_vfs_{wasm_name}_memory_trap_wrap"))
            .to_eyre()
            .wrap_err_with(|| eyre::eyre!("Failed to get export function"))?;

        let trap_id = match &module.funcs.get(trap_id).kind {
            walrus::FunctionKind::Local(local_function) => {
                let start_block = local_function.entry_block();
                let block = local_function.block(start_block);
                let (instr, _) = block.instrs[1].clone();
                match instr {
                    walrus::ir::Instr::Call(walrus::ir::Call { func }) => func,
                    _ => {
                        eyre::bail!("Unexpected instruction in trap function: {instr:?}");
                    }
                }
            }
            _ => panic!("Unexpected function kind"),
        };

        // example multi memory trap function
        //   (func (;233;) (type 3) (param i32) (result i32)
        //     local.get 0
        //     i32.const 0
        //     i32.store 1 align=1
        //     i32.const 0
        //     return
        //   )
        //   (func (;269;) (type 7) (param i32) (result i32)
        //     global.get 2
        //     local.get 0
        //     i32.add
        //     i32.const 0
        //     i32.store align=1
        //     i32.const 0
        //   )
        let trap_body = match &mut module.funcs.get_mut(trap_id).kind {
            walrus::FunctionKind::Local(local_function) => {
                let start_block = local_function.entry_block();
                local_function.block_mut(start_block)
            }
            _ => panic!("Unexpected function kind"),
        };
        // Remove the fake value instruction
        let (store_index, (store_info, _)) = trap_body
            .iter()
            .enumerate()
            .find(|(_, (instr, _))| {
                matches!(
                    instr,
                    walrus::ir::Instr::Store(walrus::ir::Store {
                        kind: walrus::ir::StoreKind::I32 { atomic: false },
                        ..
                    })
                )
            })
            .expect("Failed to find store instruction");
        trap_body.remove(store_index + 1);
        trap_body.remove(store_index);
        trap_body.remove(store_index - 1);

        let fid = module
            .imports
            .get_func(
                "wasip1-vfs",
                &format!("__wasip1_vfs_{wasm_name}_memory_director"),
            )
            .to_eyre()
            .wrap_err_with(|| eyre::eyre!("Failed to get import function"))?;

        module
            .replace_imported_func(fid, |(builder, local_id)| {
                let mut func_body = builder.func_body();
            })
            .to_eyre()
            .wrap_err_with(|| eyre::eyre!("Failed to replace imported function"))?;
    }

    Ok(path.clone())
}

pub fn director_single(path: &Utf8PathBuf, wasm: impl AsRef<Path>) -> eyre::Result<Utf8PathBuf> {
    Ok(path.clone())
}

pub fn director_multi(path: &Utf8PathBuf, wasm: impl AsRef<Path>) -> eyre::Result<Utf8PathBuf> {
    Ok(path.clone())
}
