use crate::{
    instrs::InstrRewrite as _,
    util::{WalrusFID as _, WalrusUtilModule as _},
};

// 0: Failed to load Wasm file: ./dist\threads_vfs.core.opt.adjusted.wasm
// 1: failed to parse global section
// 2: malformed mutability -- or shared globals require the shared-everything-threads proposal (at offset 0x49f)
//
// The Globals causing errors during memory expansion are those generated
// by wasm-opt --multi-memory-lowering,
// so for now we will only address these.
// When a newly created thread is executed,
// it will use the always-executable VFS code and memory,
// which are based on an address that never changes,
// and perform operations on them atomically.
// Operations on Global variables are replaced,
// and before memory unification,
// memory.grow is modified to be an atomic operation.
// Since this Global variable should only be modified internally,
// this approach should be sufficient.
// module
//     .globals
//     .iter()
//     .map(|g| g.id())
//     .collect::<Vec<_>>()
//     .iter()
//     .for_each(|g| {
//         let g = module.globals.get_mut(*g);
//         if let walrus::GlobalKind::Local(_) = g.kind {
//             if g.mutable {
//                 g.shared = true;
//             }
//         }
//     });

pub fn gen_custom_locker(
    module: &mut walrus::Module,
    mem_id: walrus::MemoryId,
) -> eyre::Result<walrus::FunctionId> {
    let alt_id =
        ("wasip1-vfs_single_memory", "__wasip1_vfs_memory_grow_alt").get_fid(&module.imports)?;
    let base_locker = "__wasip1_vfs_memory_grow_locker".get_fid(&module.exports)?;

    let locker_id = module.copy_func(base_locker)?;
    module.exports.add(
        &format!("__wasip1_vfs_memory_grow_locker_{}", mem_id.index()),
        locker_id,
    );
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

pub fn remove_gen_custom_locker_base(module: &mut walrus::Module, debug: bool) -> eyre::Result<()> {
    use walrus::ir::*;

    let alt_id =
        ("wasip1-vfs_single_memory", "__wasip1_vfs_memory_grow_alt").get_fid(&module.imports)?;
    let base_locker = "__wasip1_vfs_memory_grow_locker".get_fid(&module.exports)?;
    if !debug {
        module.funcs.delete(base_locker);
        module.funcs.delete(alt_id);

        module
            .exports
            .remove("__wasip1_vfs_memory_grow_locker")
            .unwrap();
    } else {
        let mem_id = module.memories.iter().next().unwrap().id();

        module
            .funcs
            .get_mut(base_locker)
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
    }

    module
        .imports
        .remove("wasip1-vfs_single_memory", "__wasip1_vfs_memory_grow_alt")
        .unwrap();

    Ok(())
}
