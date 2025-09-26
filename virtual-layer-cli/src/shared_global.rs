use crate::{
    instrs::InstrRewrite as _,
    util::{WalrusFID as _, WalrusUtilModule as _},
};

pub fn gen_custom_locker(
    module: &mut walrus::Module,
    mem_id: walrus::MemoryId,
) -> eyre::Result<walrus::FunctionId> {
    let alt_id =
        ("wasip1-vfs_single_memory", "__wasip1_vfs_memory_grow_alt").get_fid(&module.imports)?;
    let base_locker = "__wasip1_vfs_memory_grow_locker".get_fid(&module.exports)?;

    let locker_id = module.copy_func(base_locker)?;
    let locker = module.funcs.get_mut(locker_id);

    use walrus::ir::*;

    locker
        .kind
        .unwrap_local_mut()
        .builder_mut()
        .func_body()
        .rewrite(|instr, _| {
            if let Instr::Call(Call { func }) = instr {
                if *func == alt_id {
                    *instr = Instr::MemoryGrow(MemoryGrow { memory: mem_id });
                }
            }
        })?;

    Ok(locker_id)
}
