use eyre::Context as _;
use walrus::*;

use crate::{
    instrs::InstrRewrite,
    util::{ResultUtil as _, WalrusUtilModule},
};

#[derive(
    strum::EnumString, strum::VariantArray, strum::VariantNames, PartialEq, strum::Display,
)]
#[strum(serialize_all = "snake_case")]
pub enum Wasip1SnapshotPreview1Func {
    EnvironSizesGet,
    EnvironGet,
    ProcExit,
    RandomGet,
    SchedYield,
    ClockTimeGet,
    ClockResGet,
    FdAdvise,
    FdAllocate,
    FdDatasync,
    FdFdstatSetFlags,
    FdFdstatSetRights,
    FdWrite,
    FdPwrite,
    FdReaddir,
    FdClose,
    FdPrestatGet,
    FdPrestatDirName,
    FdFilestatGet,
    FdRead,
    FdPread,
    FdFilestatSetSize,
    FdFilestatSetTimes,
    FdRenumber,
    FdSeek,
    FdSync,
    FdTell,
    PathCreateDirectory,
    PathFilestatGet,
    PathFilestatSetTimes,
    PathLink,
    PathReadlink,
    PathRemoveDirectory,
    PathRename,
    PathOpen,
    PathSymlink,
    PathUnlinkFile,
    PollOneoff,
    ArgsGet,
    ArgsSizesGet,
    SockAccept,
    SockRecv,
    SockSend,
    SockShutdown,
}

pub struct Wasip1Op {
    fid: FunctionId,
    pub kind: Wasip1OpKind,
}

pub struct VFSExternalMemoryManager {
    pub vfs_memory_id: walrus::MemoryId,
    pub external_size: usize,
    pub current_size: usize, // * 64KiB
}

impl VFSExternalMemoryManager {
    pub fn new(vfs_memory_id: walrus::MemoryId, module: &walrus::Module) -> Self {
        let current_size = module.memories.get(vfs_memory_id).initial as usize;

        Self {
            vfs_memory_id,
            external_size: 0,
            current_size,
        }
    }

    pub fn alloc(&mut self, size: usize) -> usize {
        let ptr = self.current_size * 64 * 1024 + self.external_size;
        self.external_size += size;

        ptr
    }

    pub fn flush(mut self, module: &mut walrus::Module) -> eyre::Result<()> {
        let external_size = (0..100)
            .find(|i| *i * 64 * 1024 >= self.external_size)
            .ok_or_else(|| eyre::eyre!("Failed to find external size"))?;

        self.current_size += external_size;

        let memory = module.memories.get_mut(self.vfs_memory_id);
        memory.initial = self.current_size as u64;

        Ok(())
    }
}

#[derive(Debug)]
pub enum Wasip1OpKind {
    MemoryStoreLe {
        offset: walrus::ValType,
        value: walrus::ValType,
    },
    MemoryCopy {
        offset: walrus::ValType,
        src: walrus::ValType,
        len: walrus::ValType,
    },
    MemoryCopyTo {
        offset: walrus::ValType,
        src: walrus::ValType,
        len: walrus::ValType,
    },
    MemoryLoadLe {
        offset: walrus::ValType,
        result: walrus::ValType,
    },
    MainVoid {
        main_void_func_id: FunctionId,
        start_func_id: FunctionId,
    },
    Start {
        start_func_id: FunctionId,
    },
    Reset {
        global: Box<[(walrus::GlobalId, walrus::ir::Value)]>,
        zero_range: Box<[(i32, Option<i32>)]>,
        mem_init: Box<[(i32, usize, usize)]>,
    },
    MemoryTrap {},
    Skip,
}

macro_rules! assert_ptr {
    ($ptr:expr) => {
        if { $ptr } != walrus::ValType::I32 {
            let ptr = $ptr;
            eyre::bail!("Invalid pointer type, expected i32. Got {ptr}");
        }
    };
}

macro_rules! check_len {
    ($params:expr, $len:expr) => {
        if { $params.len() } != { $len } {
            let len = $len;
            eyre::bail!(
                "Invalid params length, expected {len}. Got {}",
                { $params }.len()
            );
        }
    };
}

macro_rules! assert_len {
    ($len:expr) => {
        if { $len } != walrus::ValType::I32 {
            let len = $len;
            eyre::bail!("Invalid length type, expected i32. Got {len}");
        }
    };
}

impl Wasip1Op {
    pub fn parse(
        module: &walrus::Module,
        import: &walrus::Import,
        wasm_name: impl AsRef<str>,
        mem_manager: &mut VFSExternalMemoryManager,
        wasm_mem: walrus::MemoryId,
        wasm_global: Vec<walrus::GlobalId>,
    ) -> eyre::Result<Self> {
        let name = import.name.as_str();
        let wasm_name = wasm_name.as_ref();

        let name = name
            .strip_prefix("__wasip1_vfs_")
            .ok_or_else(|| eyre::eyre!("Invalid import name prefix: {name}"))?;
        let name = name
            .strip_prefix(&format!("{wasm_name}_"))
            .ok_or_else(|| eyre::eyre!("Invalid import name main: {name}"))?;

        let import_fn_id = if let ImportKind::Function(fid) = import.kind {
            fid
        } else {
            eyre::bail!("Invalid import kind");
        };

        let func = module.funcs.get(import_fn_id);

        let ty = module.types.get(func.ty());

        let kind = match name {
            _ if name.starts_with("memory_store_le") => {
                fn memory_store_le(params: &[ValType]) -> eyre::Result<Wasip1OpKind> {
                    check_len!(params, 2);
                    assert_ptr!(params[0]);
                    Ok(Wasip1OpKind::MemoryStoreLe {
                        offset: params[0],
                        value: params[1],
                    })
                }
                memory_store_le(ty.params())
                    .wrap_err_with(|| eyre::eyre!("Invalid memory_store_le params"))?
            }
            _ if name.starts_with("memory_copy_from") => {
                fn memory_copy(params: &[ValType]) -> eyre::Result<Wasip1OpKind> {
                    check_len!(params, 3);
                    assert_ptr!(params[0]);
                    assert_ptr!(params[1]);
                    assert_len!(params[2]);
                    Ok(Wasip1OpKind::MemoryCopy {
                        offset: params[0],
                        src: params[1],
                        len: params[2],
                    })
                }
                memory_copy(ty.params())
                    .wrap_err_with(|| eyre::eyre!("Invalid memory_copy params"))?
            }
            _ if name.starts_with("memory_copy_to") => {
                fn memory_copy_to(params: &[ValType]) -> eyre::Result<Wasip1OpKind> {
                    check_len!(params, 3);
                    assert_ptr!(params[0]);
                    assert_ptr!(params[1]);
                    assert_len!(params[2]);
                    Ok(Wasip1OpKind::MemoryCopyTo {
                        offset: params[0],
                        src: params[1],
                        len: params[2],
                    })
                }
                memory_copy_to(ty.params())
                    .wrap_err_with(|| eyre::eyre!("Invalid memory_copy_to params"))?
            }
            _ if name.starts_with("memory_load_le") => {
                fn memory_load_le(
                    params: &[ValType],
                    results: &[ValType],
                ) -> eyre::Result<Wasip1OpKind> {
                    check_len!(params, 1);
                    assert_ptr!(params[0]);
                    check_len!(results, 1);
                    Ok(Wasip1OpKind::MemoryLoadLe {
                        offset: params[0],
                        result: results[0],
                    })
                }
                memory_load_le(ty.params(), ty.results())
                    .wrap_err_with(|| eyre::eyre!("Invalid memory_load_le params"))?
            }
            _ if name.starts_with("__main_void") => {
                let main_void_func_id = module
                    .exports
                    .get_func(&format!("__wasip1_vfs_{wasm_name}___main_void"))
                    .to_eyre()
                    .wrap_err_with(|| eyre::eyre!("Failed to get main_void function"))?;

                let start_func_id = module
                    .exports
                    .get_func(&format!("__wasip1_vfs_{wasm_name}__start"))
                    .to_eyre()
                    .wrap_err_with(|| eyre::eyre!("Failed to get start function"))?;

                Wasip1OpKind::MainVoid {
                    main_void_func_id,
                    start_func_id,
                }
            }
            _ if name.starts_with("_start") => {
                let start_func_id = module
                    .exports
                    .get_func(&format!("__wasip1_vfs_{wasm_name}__start"))
                    .to_eyre()
                    .wrap_err_with(|| eyre::eyre!("Failed to get start function"))?;

                Wasip1OpKind::Start { start_func_id }
            }
            _ if name.starts_with("reset") => {
                let global = module
                    .globals
                    .iter()
                    .filter(|g| wasm_global.contains(&g.id()))
                    .filter(|global| global.mutable)
                    .filter_map(|global| {
                        if let GlobalKind::Local(ConstExpr::Value(v)) = global.kind {
                            Some((global.id(), v.clone()))
                        } else {
                            log::warn!("Global segment {:?} is not a value", global.kind);
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .into_boxed_slice();

                let data_range = module
                    .data
                    .iter()
                    .filter_map(|data| match data.kind {
                        walrus::DataKind::Active { memory, offset } => {
                            if memory == wasm_mem {
                                if let ConstExpr::Value(v) = offset {
                                    if let ir::Value::I32(offset) = v {
                                        Some((offset, data.value.len()))
                                    } else {
                                        log::warn!("Data segment {:?} is not i32", offset);
                                        None
                                    }
                                } else {
                                    log::warn!("Data segment {:?} is not a value", offset);
                                    None
                                }
                            } else {
                                None
                            }
                        }
                        _ => {
                            log::warn!("Data segment passive is not supported");
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                let zero_range = std::iter::once(Some(0i32))
                    .chain(
                        data_range
                            .iter()
                            .flat_map(|(offset, len)| [Some(*offset), Some(*offset + *len as i32)]),
                    )
                    .chain(std::iter::once(None))
                    .collect::<Vec<_>>()
                    .chunks(2)
                    .map(|chunk| (chunk[0].unwrap(), chunk[1]))
                    .collect::<Vec<_>>()
                    .into_boxed_slice();

                let mem_init = data_range
                    .into_iter()
                    .map(|(offset, len)| (offset, len, mem_manager.alloc(len)))
                    .collect::<Vec<_>>()
                    .into_boxed_slice();

                Wasip1OpKind::Reset {
                    global,
                    zero_range,
                    mem_init,
                }
            }
            _ if name.starts_with("memory_trap") => Wasip1OpKind::MemoryTrap {},
            _ if name.starts_with("memory_director") => Wasip1OpKind::Skip {},
            _ => eyre::bail!("Invalid import name: {name}"),
        };

        let op = Wasip1Op {
            fid: import_fn_id,
            kind,
        };
        Ok(op)
    }

    pub fn main_void(
        &self,
        module: &mut walrus::Module,
        fid: FunctionId,
        main_void_func_id: FunctionId,
        start_func_id: FunctionId,
    ) -> eyre::Result<()> {
        let start_func = module.funcs.get_mut(start_func_id);
        if let walrus::FunctionKind::Local(func) = &mut start_func.kind {
            func.builder_mut().func_body().rewrite(|instr, _| {
                if let ir::Instr::Call(call) = instr {
                    if call.func == main_void_func_id {
                        *instr = ir::Instr::Const(ir::Const {
                            value: ir::Value::I32(0),
                        })
                    }
                }
            })?;
        } else {
            eyre::bail!("Invalid start function kind");
        }

        module.connect_func_inner(fid, main_void_func_id)?;

        Ok(())
    }

    pub fn start(
        &self,
        module: &mut walrus::Module,
        fid: FunctionId,
        start_func_id: FunctionId,
        wasm_mem: walrus::MemoryId,
        vfs_mem: walrus::MemoryId,
        is_reset_contain: Option<&Wasip1Op>,
    ) -> eyre::Result<()> {
        module
            .replace_imported_func(fid, |(builder, arg_locals)| {
                let mut func_body = builder.func_body();

                if let Some(reset) = is_reset_contain {
                    if let Wasip1OpKind::Reset { mem_init, .. } = &reset.kind {
                        for (offset, len, ptr) in mem_init {
                            func_body
                                .i32_const(*ptr as i32)
                                .i32_const(*offset)
                                .i32_const(*len as i32)
                                .memory_copy(wasm_mem, vfs_mem);
                        }
                    } else {
                        unreachable!();
                    }
                }

                for local in arg_locals {
                    func_body.local_get(*local);
                }
                func_body.call(start_func_id);
                func_body.return_();
            })
            .to_eyre()
            .wrap_err_with(|| eyre::eyre!("Failed to replace imported function"))?;

        let export_id = module
            .exports
            .iter()
            .find(|f| {
                if let walrus::ExportItem::Function(f) = f.item {
                    f == start_func_id
                } else {
                    false
                }
            })
            .map(|f| f.id())
            .ok_or_else(|| eyre::eyre!("Export not found"))?;

        module.exports.delete(export_id);

        Ok(())
    }

    pub fn replace(
        self,
        module: &mut walrus::Module,
        wasm_mem: walrus::MemoryId,
        vfs_mem: walrus::MemoryId,
        is_reset_contain: Option<&Wasip1Op>,
    ) -> eyre::Result<()> {
        // if matches!(self.kind, Wasip1OpKind::MainVoid) {
        //     self.main_void(module, self.fid)?;
        // }

        if let Wasip1OpKind::MainVoid {
            main_void_func_id,
            start_func_id,
        } = self.kind
        {
            self.main_void(module, self.fid, main_void_func_id, start_func_id)?;
        } else if let Wasip1OpKind::Start { start_func_id } = self.kind {
            self.start(
                module,
                self.fid,
                start_func_id,
                wasm_mem,
                vfs_mem,
                is_reset_contain,
            )?;
        } else if let Wasip1OpKind::Skip = self.kind {
        } else {
            let Self { fid, kind } = self;

            module
                .replace_imported_func(fid, |(body, arg_locals)| {
                    let mut body = body.func_body();

                    match &kind {
                        Wasip1OpKind::MemoryStoreLe { value, .. } => {
                            if *value != walrus::ValType::I32 {
                                todo!("Unimplemented value type: {value} yet");
                            }

                            body.local_get(arg_locals[0])
                                .local_get(arg_locals[1])
                                .store(
                                    wasm_mem,
                                    ir::StoreKind::I32 { atomic: false },
                                    ir::MemArg {
                                        align: 0,
                                        offset: 0,
                                    },
                                )
                                .return_();
                        }
                        Wasip1OpKind::MemoryCopy { .. } => {
                            body.local_get(arg_locals[0])
                                .local_get(arg_locals[1])
                                .local_get(arg_locals[2])
                                .memory_copy(vfs_mem, wasm_mem)
                                .return_();
                        }
                        Wasip1OpKind::MemoryCopyTo { .. } => {
                            body.local_get(arg_locals[0])
                                .local_get(arg_locals[1])
                                .local_get(arg_locals[2])
                                .memory_copy(wasm_mem, vfs_mem)
                                .return_();
                        }
                        Wasip1OpKind::MemoryLoadLe { result, .. } => {
                            if *result != walrus::ValType::I32 {
                                todo!("Unimplemented value type: {result} yet");
                            }

                            body.local_get(arg_locals[0])
                                .load(
                                    wasm_mem,
                                    ir::LoadKind::I32 { atomic: false },
                                    ir::MemArg {
                                        offset: 0,
                                        align: 0,
                                    },
                                )
                                .return_();
                        }
                        Wasip1OpKind::MainVoid { .. } => unreachable!(),
                        Wasip1OpKind::Start { .. } => unreachable!(),
                        Wasip1OpKind::Reset {
                            global,
                            zero_range,
                            mem_init,
                        } => {
                            for global in global.iter() {
                                let (id, value) = global;
                                body.const_(*value).global_set(*id);
                            }
                            for (start, end) in zero_range.iter() {
                                body.i32_const(*start).i32_const(0);

                                if let Some(end) = end {
                                    body.i32_const(*end - *start);
                                } else {
                                    body.memory_size(wasm_mem)
                                        .i32_const(64 * 1024)
                                        .binop(ir::BinaryOp::I32Mul)
                                        .i32_const(*start)
                                        .binop(ir::BinaryOp::I32Sub);
                                }
                                body.memory_fill(wasm_mem);
                            }
                            for (mem_offset, mem_len, mem_ptr) in mem_init.iter() {
                                body.i32_const(*mem_offset)
                                    .i32_const(*mem_ptr as i32)
                                    .i32_const(*mem_len as i32)
                                    .memory_copy(vfs_mem, wasm_mem);
                            }
                            body.return_();
                        }
                        Wasip1OpKind::MemoryTrap { .. } => {
                            body.local_get(arg_locals[0])
                                .i32_const(0) // fake value
                                .store(
                                    wasm_mem,
                                    ir::StoreKind::I32_8 { atomic: false },
                                    ir::MemArg {
                                        align: 0,
                                        offset: 0,
                                    },
                                )
                                .i32_const(0) // fake return value
                                .return_();
                        }
                        Wasip1OpKind::Skip => {
                            unreachable!();
                        }
                    }
                })
                .to_eyre()
                .wrap_err_with(|| eyre::eyre!("Failed to replace function: {:?}", kind))?;
        }

        Ok(())
    }
}
