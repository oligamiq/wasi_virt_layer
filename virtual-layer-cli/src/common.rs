use eyre::Context as _;
use walrus::*;

use crate::util::{ResultUtil as _, WalrusUtilFuncs, WalrusUtilModule};

#[derive(
    strum::EnumString, strum::VariantArray, strum::VariantNames, PartialEq, strum::Display,
)]
#[strum(serialize_all = "snake_case")]
pub enum Wasip1ABIFunc {
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
    FdFdstatGet,
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

#[derive(
    strum::EnumString, strum::VariantArray, strum::VariantNames, PartialEq, strum::Display,
)]
#[strum(serialize_all = "snake_case")]
pub enum Wasip1ThreadsABIFunc {
    ThreadSpawn,
}

pub struct Wasip1Op {
    fid: FunctionId,
    pub kind: Wasip1OpKind,
}

#[derive(Debug)]
pub enum Wasip1OpKind {
    MainVoid {
        main_void_func_id: FunctionId,
        start_func_id: FunctionId,
    },
    // About start section etc
    Start {
        start_func_id: FunctionId,
    },
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
            .ok_or_else(|| eyre::eyre!("Invalid import name prefix: {name}"))?;
        let name = name
            .strip_prefix(&format!("{wasm_name}_"))
            .ok_or_else(|| eyre::eyre!("Invalid import name main: {name}"))?;

        let import_fn_id = if let ImportKind::Function(fid) = import.kind {
            fid
        } else {
            eyre::bail!("Invalid import kind");
        };

        let kind = match name {
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
        start_fn_id: FunctionId,
        debug: bool,
    ) -> eyre::Result<()> {
        let fake_fn_id = module.add_func(&[], &[walrus::ValType::I32], |func, _| {
            func.func_body().i32_const(0).return_();

            Ok(())
        })?;

        let call_main_void: i32 = module
            .funcs
            .rewrite(
                |instr, _| {
                    if let walrus::ir::Instr::Call(c) = instr {
                        if c.func == main_void_func_id {
                            c.func = fake_fn_id;
                            1
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                },
                start_fn_id,
            )
            .wrap_err("Failed to read main_void calls")?
            .into_iter()
            .sum();
        if call_main_void == 0 {
            let call_count = module
                .funcs
                .flat_read(
                    |instr, _| {
                        if let walrus::ir::Instr::Call(c) = instr {
                            if c.func == main_void_func_id { 1 } else { 0 }
                        } else {
                            0
                        }
                    },
                    start_fn_id,
                )
                .wrap_err("Failed to read main_void calls")?
                .into_iter()
                .count();

            if call_count == 1 {
                log::warn!(
                    "main_void is not called directly in start function, but called in nested function. we replaced once call to a fake function that returns 0."
                );
            } else {
                if call_count > 1 {
                    log::warn!(
                        "main_void is not called directly in start function, and called in nested function. main_void called multiple times in start function, rust's default is once."
                    );
                } else {
                    log::warn!(
                        "main_void is not called in nested start function, we think call_indirect is used. we replaced all calls to a fake function that returns 0."
                    );
                    // Strictly speaking, it should be limited to functions called within start_fn,
                    // but since the main_void function is only called inside start_fn and through export,
                    // it is acceptable to modify it in this function.
                    module
                        .connect_func_alt(main_void_func_id, fake_fn_id, debug)
                        .wrap_err("Failed to rewrite main_void call in start")?;
                }
            }
            let copied_start_fn_id =
                module.nested_copy_func(start_fn_id, &[] as &[FunctionId], true, true)?;
            module
                .funcs
                .flat_rewrite(
                    |instr, _| {
                        if let walrus::ir::Instr::Call(c) = instr {
                            if c.func == main_void_func_id {
                                c.func = fake_fn_id;
                            }
                        }
                    },
                    copied_start_fn_id,
                    false,
                )
                .wrap_err("Failed to read main_void calls")?;
            module.connect_func_alt(start_fn_id, copied_start_fn_id, debug)?;
        } else if call_main_void > 1 {
            log::warn!(
                "main_void called multiple times in start function, rust's default is once. we replaced all calls to a fake function that returns 0."
            );
        }

        module.connect_func_alt(fid, main_void_func_id, debug)?;

        Ok(())
    }

    pub fn replace(self, module: &mut walrus::Module, debug: bool) -> eyre::Result<()> {
        if let Wasip1OpKind::MainVoid {
            main_void_func_id,
            start_func_id,
        } = self.kind
        {
            self.main_void(module, self.fid, main_void_func_id, start_func_id, debug)
                .wrap_err(
                    "Failed to implement main_void wasm memory etc before call main function",
                )?;
        }

        Ok(())
    }
}
