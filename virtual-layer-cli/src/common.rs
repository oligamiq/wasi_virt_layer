use eyre::Context as _;
use walrus::*;

use crate::util::ResultUtil as _;

pub const WASIP1_FUNC: [&str; 7] = [
    "fd_write",
    "environ_sizes_get",
    "environ_get",
    "proc_exit",
    "random_get",
    "sched_yield",
    "clock_time_get",
];

pub const WASIP1_OP: [&str; 2] = ["memory_store_le", "memory_copy"];

pub struct Wasip1Op {
    fid: FunctionId,
    kind: Wasip1OpKind,
}

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
    ) -> eyre::Result<Self> {
        let name = import.name.as_str();
        let wasm_name = wasm_name.as_ref();

        let name = name
            .strip_prefix("__wasip1_vfs_")
            .ok_or_else(|| eyre::eyre!("Invalid import name: {name}"))?;
        let name = name
            .strip_prefix(&format!("{wasm_name}_"))
            .ok_or_else(|| eyre::eyre!("Invalid import name: {name}"))?;

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
            _ if name.starts_with("memory_copy") => {
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
            _ => eyre::bail!("Invalid import name: {name}"),
        };

        let op = Wasip1Op {
            fid: import_fn_id,
            kind,
        };
        Ok(op)
    }

    pub fn replace(
        self,
        module: &mut walrus::Module,
        wasm_mem: walrus::MemoryId,
        vfs_mem: walrus::MemoryId,
    ) -> eyre::Result<()> {
        let Self { fid, kind } = self;

        module
            .replace_imported_func(fid, |(body, arg_locals)| {
                let mut body = body.func_body();

                match kind {
                    Wasip1OpKind::MemoryStoreLe { value, .. } => {
                        if value != walrus::ValType::I32 {
                            unimplemented!("Unimplemented value type: {value} yet");
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
                        if result != walrus::ValType::I32 {
                            unimplemented!("Unimplemented value type: {result} yet");
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
                }
            })
            .to_eyre()
            .wrap_err_with(|| eyre::eyre!("Failed to replace function"))?;

        Ok(())
    }
}
