use crate::{
    instrs::InstrRewrite as _,
    util::{WalrusFID as _, WalrusUtilModule as _},
};

pub fn lock_memory_grow(module: &mut walrus::Module, name: impl AsRef<str>) -> eyre::Result<()> {
    let name = name.as_ref();

    for (i, mem) in module
        .memories
        .iter()
        .map(|m| m.id())
        .collect::<Vec<_>>()
        .into_iter()
        .enumerate()
    {
        use walrus::ir::*;

        let memory_grow_locker_ty = module
            .types
            .add(&[walrus::ValType::I32], &[walrus::ValType::I32]);
        let memory_grow_locker_id = module
            .add_import_func(
                "wasip1-vfs_single_memory",
                &format!("__wasip1_vfs_memory_grow_{name}_locker_{i}"),
                memory_grow_locker_ty,
            )
            .0;

        let anchor_name = format!("__wasip1_vfs_memory_grow_{name}_locker_{i}_anchor");
        let anchor = module.add_func(&[], &[walrus::ValType::I32], |builder, _| {
            let mut func_body = builder.func_body();
            func_body.load(
                mem,
                LoadKind::I32 { atomic: false },
                MemArg {
                    align: 0,
                    offset: 0,
                },
            );
            Ok(())
        })?;

        module.exports.add(&anchor_name, anchor);

        module.funcs.iter_local_mut().for_each(|(_, f)| {
            f.builder_mut()
                .func_body()
                .rewrite(|instr, _| {
                    if let Instr::MemoryGrow(MemoryGrow { memory: m }) = instr {
                        if *m == mem {
                            *instr = Instr::Call(Call {
                                func: memory_grow_locker_id,
                            });
                        }
                    }
                })
                .unwrap();
        });
    }

    Ok(())
}

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
