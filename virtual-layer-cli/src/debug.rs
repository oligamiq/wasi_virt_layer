use eyre::Context as _;
use itertools::Itertools;

use crate::util::{ResultUtil as _, WalrusUtilFuncs as _, WalrusUtilModule as _};

pub fn readjust_debug_call_function(module: &mut walrus::Module) -> eyre::Result<bool> {
    let mut changed = false;

    let debugger = module
        .exports
        .get_func("debug_call_function_start")
        .to_eyre()
        .wrap_err("Failed to get debug_call_function export")?;

    let finalize = module
        .exports
        .get_func("debug_call_function_end")
        .to_eyre()
        .wrap_err("Failed to get debug_call_function_end export")?;
}

pub fn generate_debug_call_function(module: &mut walrus::Module) -> eyre::Result<()> {
    fn get_fid(
        module: &mut walrus::Module,
        name: &str,
    ) -> eyre::Result<Option<walrus::FunctionId>> {
        module
            .exports
            .iter()
            .find(|export| export.name == name)
            .map(|export| {
                let fid = match export.item {
                    walrus::ExportItem::Function(fid) => fid,
                    _ => eyre::bail!("{name} is not a function export"),
                };
                Ok(fid)
            })
            .transpose()
    }

    let name = "debug_call_indirect";
    if let Some(e) = get_fid(module, name)?.map(|fid| {
        module
            .debug_call_indirect(fid)
            .wrap_err("Failed to set debug_call_indirect")?;

        log::info!("{name} function found. Enabling debug feature.");

        eyre::Ok(())
    }) {
        e.wrap_err("Failed to enable debug_call_indirect")?;
    }

    let name = "debug_atomic_wait";
    if let Some(e) = get_fid(module, name)?.map(|fid| {
        use walrus::ValType::{I32, I64};

        log::info!("{name} function found. Enabling debug feature.");

        module
            .gen_inspect(fid, &[I32, I32, I64], &[fid], |instr| match instr {
                walrus::ir::Instr::AtomicWait(_) => Some([]),
                _ => None,
            })
            .wrap_err("Failed to set debug_atomic_wait")?;

        eyre::Ok(())
    }) {
        e.wrap_err("Failed to enable debug_atomic_wait")?;
    }

    let name = "debug_call_function_start";
    if let Some(e) = get_fid(module, name)?.map(|fid| {
        let excludes = [
            "debug_call_indirect",
            "debug_atomic_wait",
            "debug_blind_print_etc_flag",
            "debug_call_function_start",
            "debug_call_function_end",
        ]
        .iter()
        .filter_map(|name| {
            Some(
                get_fid(module, name)
                    .transpose()?
                    .map(|fid| module.funcs.find_children_with(fid))
                    .flatten(),
            )
        })
        .flatten_ok()
        .try_collect::<_, Vec<_>, _>()?;

        let finalize_name = "debug_call_function_end";
        let finalize = get_fid(module, finalize_name)?.unwrap();

        log::info!("{name}, {finalize_name} function found. Enabling debug feature.");

        module
            .gen_inspect_with_finalize(Some(fid), Some(finalize), &[], &[], &excludes, |instr| {
                match instr {
                    walrus::ir::Instr::Call(id) => Some([id.func.index() as i32]),
                    _ => None,
                }
            })
            .wrap_err("Failed to set debug_atomic_wait")?;

        eyre::Ok(())
    }) {
        e.wrap_err("Failed to enable debug_call_function")?;
    }

    Ok(())
}
