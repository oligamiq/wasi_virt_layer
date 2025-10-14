use eyre::Context as _;
use walrus::*;

use crate::{
    generator::{Generator, GeneratorCtx, ModuleExternal},
    instrs::InstrRewrite as _,
    util::{
        NAMESPACE, ResultUtil as _, WalrusFID, WalrusUtilExport as _, WalrusUtilFuncs as _,
        WalrusUtilImport as _, WalrusUtilModule as _,
    },
};

/// To enable the reset function,
/// a memory area shall be provided
/// to retain memory information at startup.
pub struct VFSExternalMemoryManager {
    external_size: usize,
    current_size: usize, // * 64KiB
    mem_id: MemoryId,
}

impl VFSExternalMemoryManager {
    pub fn new(module: &mut walrus::Module) -> Self {
        let mem_id = module.memories.add_local(true, false, 0, None, None);

        Self {
            external_size: 0,
            current_size: 0,
            mem_id,
        }
    }

    pub fn memory_id(&self) -> MemoryId {
        self.mem_id
    }

    pub fn alloc(&mut self, size: usize) -> usize {
        let ptr = self.current_size * 64 * 1024 + self.external_size;
        self.external_size += size;

        ptr
    }

    pub fn flush(mut self, module: &mut walrus::Module, threads: bool) -> eyre::Result<MemoryId> {
        let external_size = (0..=0x10000)
            .find(|i| *i * 64 * 1024 >= self.external_size)
            .ok_or_else(|| eyre::eyre!("Failed to find external size in 0..=0x10000"))?;

        self.current_size += external_size;

        let mem = module.memories.get_mut(self.mem_id);

        mem.initial = self.current_size as u64;
        mem.shared = threads;

        if threads {
            mem.maximum = Some(mem.initial);
        }

        Ok(self.mem_id)
    }
}

#[derive(Debug, Default)]
pub struct ResetFunc;

impl Generator for ResetFunc {
    fn post_combine(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        let mut mem_manager = VFSExternalMemoryManager::new(module);

        let wasm_mem = ctx.vfs_used_memory_id.unwrap();

        let initializers = module
            .add_func(&[], &[], |_, _| Ok(()))
            .wrap_err_with(|| eyre::eyre!("Failed to add initializer function"))?;

        for wasm in &ctx.target_names {
            let reset_name = format!("__wasip1_vfs_{wasm}_reset");

            if let Some(reset) = (NAMESPACE, &reset_name).get_fid(&module.imports).ok() {
                let global = ctx.target_used_global_id.as_ref().unwrap()[wasm]
                .iter()
                .copied()
                .map(|g| module.globals.get(g))
                .filter(|g| g.mutable)
                .filter_map(|g| {
                    if let GlobalKind::Local(ConstExpr::Value(v)) = g.kind {
                        Some((g.id(), v.clone()))
                    } else {
                        log::warn!(
                            "Global segment {:?} is not a value, we support only local variables",
                            g.kind
                        );
                        None
                    }
                })
                .collect::<Box<_>>();

                let data_range = module
                    .data
                    .iter()
                    .filter_map(|data| {
                        match data.kind {
                            DataKind::Active { memory, offset } if memory == wasm_mem => {
                                if let ConstExpr::Value(v) = offset {
                                    if let ir::Value::I32(offset) = v {
                                        Some((offset, data.value.len()))
                                    } else {
                                        log::warn!(
                                            "Data segment {:?} is not i32, we support only i32",
                                            offset
                                        );
                                        None
                                    }
                                } else {
                                    log::warn!(
                                        "Data segment {:?} is not a value, we support only i32",
                                        offset
                                    );
                                    None
                                }
                            }
                            // Passive is across memories so ignore on now
                            _ => None,
                        }
                    })
                    .collect::<Box<_>>();

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
                    .collect::<Box<_>>();

                let mem_init = data_range
                    .into_iter()
                    .map(|(offset, len)| (offset, len, mem_manager.alloc(len)))
                    .collect::<Box<_>>();

                let reset_area_mem_id = mem_manager.memory_id();

                let start_section_id = module.start.clone();

                module
                    .replace_imported_func(reset, |(builder, _)| {
                        let mut body = builder.func_body();

                        for (id, value) in global.iter() {
                            body.const_(*value).global_set(*id);
                        }
                        for (start, end) in zero_range.iter() {
                            // ptr
                            body.i32_const(*start)
                                // value
                                .i32_const(0);

                            // len
                            if let Some(end) = end {
                                body.i32_const(*end - *start);
                            } else {
                                body.memory_size(wasm_mem);

                                // asserter.as_mut().unwrap()(&mut body).unwrap();

                                body.i32_const(64 * 1024)
                                    .binop(ir::BinaryOp::I32Mul)
                                    .i32_const(*start)
                                    .binop(ir::BinaryOp::I32Sub);
                            }
                            body.memory_fill(wasm_mem);
                        }
                        for (mem_offset, mem_len, mem_ptr) in mem_init.iter() {
                            body.i32_const(*mem_offset) // dst
                                .i32_const(*mem_ptr as i32) // src
                                .i32_const(*mem_len as i32) // len
                                .memory_copy(reset_area_mem_id, wasm_mem);
                        }

                        if let Some(start_section_id) = start_section_id {
                            body.call(start_section_id);
                        }
                    })
                    .to_eyre()
                    .wrap_err_with(|| eyre::eyre!("Failed to replace reset function for {wasm}"))?;

                let mut func_body = module
                    .funcs
                    .get_mut(initializers)
                    .kind
                    .unwrap_local_mut()
                    .builder_mut()
                    .func_body();

                for (offset, len, ptr) in mem_init {
                    func_body
                        .i32_const(ptr as i32) // dst
                        .i32_const(offset) // src
                        .i32_const(len as i32) // len
                        .memory_copy(wasm_mem, reset_area_mem_id);
                }
            }
        }

        let _ = mem_manager.flush(module, ctx.threads)?;

        // Saves the memory state upon initial startup.
        // As the start section is also invoked when spawning threads,
        // ensure it is called only once if threads are enabled.
        let init_id = if ctx.threads {
            let reset_on_thread = "__wasip1_vfs_reset_on_thread".get_fid(&module.exports)?;
            let reset_on_thread_once =
                (NAMESPACE, "__wasip1_vfs_reset_on_thread_once").get_fid(&module.imports)?;

            module.imports.erase(reset_on_thread_once)?;

            module.exports.erase(reset_on_thread)?;
            module.funcs.delete(reset_on_thread);

            // module.renew_call_fn(reset_on_thread_once, initializers)?;

            // reset_on_thread

            initializers
        } else {
            initializers
        };

        let old_start = module.start.clone();
        let new_start = module
            .add_func(&[], &[], |builder, _| {
                let mut body = builder.func_body();
                body.call(init_id);
                if let Some(old_start) = old_start {
                    body.call(old_start);
                }
                Ok(())
            })
            .wrap_err_with(|| eyre::eyre!("Failed to add new start function"))?;

        module.start = Some(new_start);

        if ctx.unstable_print_debug {
            if let Some(start) = module.start {
                module.exports.add("__wasip1_vfs_start_init_old", start);
            }
        }

        // memory_init(memory, data)
        // fn(&mut self, Id<Memory>, Id<Data>)
        // data_drop(&mut self, data: DataId)
        // so we remove all data_drop sections.
        // Prevent the active segment from being deleted
        // so that it can be called upon as many times as required.
        module
            .funcs
            .iter_mut()
            .map(|func| {
                match &mut func.kind {
                    walrus::FunctionKind::Local(l) => {
                        l.builder_mut()
                            .func_body()
                            .retain(|instr, _| !instr.is_data_drop());
                    }
                    _ => {}
                }
                Ok(())
            })
            .collect::<eyre::Result<Vec<_>>>()?;

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct StartFunc;

impl Generator for StartFunc {
    fn pre_target(
        &mut self,
        module: &mut walrus::Module,
        _: &GeneratorCtx,
        external: &ModuleExternal,
    ) -> eyre::Result<()> {
        let id = "_start".get_fid(&module.exports)?;

        module
            .exports
            .get_mut(module.exports.get_exported_func(id).unwrap().id())
            .name = format!("__wasip1_vfs_{wasm}__start", wasm = external.name);

        Ok(())
    }

    fn post_combine(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        for wasm in &ctx.target_names {
            module.renew_call_fn(
                (NAMESPACE, &format!("__wasip1_vfs_{wasm}__start")).get_fid(&module.imports)?,
                ctx.start_func_id.as_ref().unwrap()[wasm],
                // Export already removed by StartFuncIdVisitor
            )?;
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct MainVoidFunc;

impl Generator for MainVoidFunc {
    fn pre_target(
        &mut self,
        module: &mut walrus::Module,
        _: &GeneratorCtx,
        external: &ModuleExternal,
    ) -> eyre::Result<()> {
        let id = "__main_void".get_fid(&module.exports)?;

        module
            .exports
            .get_mut(module.exports.get_exported_func(id).unwrap().id())
            .name = format!("__wasip1_vfs_{wasm}___main_void", wasm = external.name);

        Ok(())
    }

    fn post_combine(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        for wasm in &ctx.target_names {
            if let Some(fid) = (NAMESPACE, &format!("__wasip1_vfs_{wasm}___main_void"))
                .get_fid(&module.imports)
                .ok()
            {
                let main_void_func_id =
                    format!("__wasip1_vfs_{wasm}___main_void").get_fid(&module.exports)?;
                let start_fn_id = ctx.start_func_id.as_ref().unwrap()[wasm];

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
                                .connect_func_alt(
                                    main_void_func_id,
                                    fake_fn_id,
                                    ctx.unstable_print_debug,
                                )
                                .wrap_err("Failed to rewrite main_void call in start")?;
                        }
                    }
                    let start_fn_id =
                        module.nested_copy_func(start_fn_id, &[start_fn_id], true, true)?;
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
                            start_fn_id,
                            false,
                        )
                        .wrap_err("Failed to read main_void calls")?;
                } else if call_main_void > 1 {
                    log::warn!(
                        "main_void called multiple times in start function, rust's default is once. we replaced all calls to a fake function that returns 0."
                    );
                }

                module.connect_func_alt(fid, main_void_func_id, ctx.unstable_print_debug)?;
            }
        }

        Ok(())
    }
}
