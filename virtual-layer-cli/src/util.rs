use std::{
    borrow::Borrow,
    collections::HashMap,
    fmt::Debug,
    path::{Path, PathBuf},
};

use eyre::{Context as _, ContextCompat as _};
use itertools::Itertools;
use walrus::{ir::InstrSeqId, *};

use crate::instrs::{InstrRead, InstrRewrite as _};

#[allow(dead_code)]
pub(crate) trait WalrusUtilImport: Debug {
    fn find_mut<A>(&mut self, as_fn: impl WalrusFID<A>) -> eyre::Result<&mut Import>;

    /// Swap two imports but if other not found, skip
    /// This is useful when you want to swap imports that may not exist
    fn may_swap_import<A>(
        &mut self,
        one: impl WalrusFID<A>,
        other: (impl AsRef<str>, impl AsRef<str>),
    ) -> eyre::Result<()>
    where
        Self: Sized,
    {
        let other_module = other.0.as_ref();
        let other_name = other.1.as_ref();

        let one_import = self
            .find_mut(one)
            .wrap_err_with(|| eyre::eyre!("One Import {} not found", one.as_str()))?;

        let one_module = one_import.module.clone();
        let one_name = one_import.name.clone();

        one_import.module = "archived".to_string();

        self.find_mut((other_module, other_name))
            .ok()
            .map(|import| {
                import.module = one_module;
                import.name = one_name.clone();
            });

        let one_import = self.find_mut(("archived", &one_name)).unwrap();

        one_import.module = other_module.to_string();
        one_import.name = other_name.to_string();

        Ok(())
    }

    fn swap_import<A, B>(
        &mut self,
        one: impl WalrusFID<A>,
        other: impl WalrusFID<B>,
    ) -> eyre::Result<()>
    where
        Self: Sized,
    {
        let (other_module, other_name) = {
            let other_import = self
                .find_mut(other)
                .wrap_err_with(|| eyre::eyre!("Other Import {} not found", other.as_str()))?;
            (other_import.module.clone(), other_import.name.clone())
        };

        let one_import = self
            .find_mut(one)
            .wrap_err_with(|| eyre::eyre!("One Import {} not found", one.as_str()))?;

        let one_module = one_import.module.clone();
        let one_name = one_import.name.clone();

        one_import.module = "archived".to_string();

        let other_import = self.find_mut(other).unwrap();

        other_import.module = one_module;
        other_import.name = one_name.clone();

        let one_import = self.find_mut(("archived", &one_name)).unwrap();

        one_import.module = other_module;
        one_import.name = other_name.clone();

        Ok(())
    }
}

pub(crate) trait WalrusUtilExport: Debug {
    fn erase<A>(&mut self, as_fn: impl WalrusFID<A>) -> eyre::Result<()>;
    fn erase_with<A>(&mut self, as_fn: impl WalrusFID<A>, debug: bool) -> eyre::Result<()> {
        if !debug { self.erase(as_fn) } else { Ok(()) }
    }
}

pub(crate) trait WalrusUtilFuncs {
    /// Find children flat functions
    fn find_children(
        &self,
        fid: impl Borrow<FunctionId>,
        allow_call_indirect: bool,
    ) -> eyre::Result<Vec<FunctionId>>;

    /// Find children flat functions with self
    fn find_children_with(
        &self,
        fid: impl Borrow<FunctionId>,
        allow_call_indirect: bool,
    ) -> eyre::Result<Vec<FunctionId>> {
        let fid = *fid.borrow();
        let mut children = self.find_children(fid, allow_call_indirect)?;
        if !children.contains(&fid) {
            children.insert(0, fid);
        }
        Ok(children)
    }

    /// call rewrite on function
    fn rewrite<T>(
        &mut self,
        find: impl FnMut(&mut ir::Instr, (usize, InstrSeqId)) -> T,
        fid: impl Borrow<FunctionId>,
    ) -> eyre::Result<Vec<T>>
    where
        Self: Sized;

    /// call rewrite on children functions
    fn flat_rewrite<T>(
        &mut self,
        find: impl FnMut(&mut ir::Instr, (usize, InstrSeqId)) -> T,
        fid: impl Borrow<FunctionId>,
        allow_call_indirect: bool,
    ) -> eyre::Result<Vec<T>>
    where
        Self: Sized;

    fn read<T>(
        &self,
        find: impl FnMut(&ir::Instr, (usize, InstrSeqId)) -> T,
        fid: impl Borrow<FunctionId>,
    ) -> eyre::Result<Vec<T>>
    where
        Self: Sized;

    fn flat_read<T>(
        &self,
        find: impl FnMut(&ir::Instr, (usize, InstrSeqId)) -> T,
        fid: impl Borrow<FunctionId>,
    ) -> eyre::Result<Vec<T>>
    where
        Self: Sized;

    fn all_rewrite<T>(
        &mut self,
        find: impl FnMut(&mut ir::Instr, (usize, InstrSeqId)) -> T,
        exclude: &[impl Borrow<FunctionId>],
    ) -> eyre::Result<Vec<T>>
    where
        Self: Sized;

    fn all_read<T>(
        &self,
        find: impl FnMut(&ir::Instr, (usize, InstrSeqId)) -> T,
        exclude: &[impl Borrow<FunctionId>],
    ) -> eyre::Result<Vec<T>>
    where
        Self: Sized;
}

#[allow(dead_code)]
pub(crate) trait WalrusUtilModule {
    /// connect function from import to export
    /// export will be removed
    /// and import will be replaced with the export function
    fn connect_func<A, B>(
        &mut self,
        import: impl WalrusFID<A>,
        export: impl WalrusFID<B>,
    ) -> eyre::Result<()> {
        self.connect_func_with_is_delete(import, export, true)
    }

    fn connect_func_alt<A, B>(
        &mut self,
        import: impl WalrusFID<A>,
        export: impl WalrusFID<B>,
        is_delete: bool,
    ) -> eyre::Result<()>;

    fn connect_func_without_remove<A, B>(
        &mut self,
        import: impl WalrusFID<A>,
        export: impl WalrusFID<B>,
    ) -> eyre::Result<()> {
        self.connect_func_with_is_delete(import, export, false)
    }

    fn connect_func_with_is_delete<A, B>(
        &mut self,
        import: impl WalrusFID<A>,
        export: impl WalrusFID<B>,
        is_delete: bool,
    ) -> eyre::Result<()>;

    /// add fake function to the module
    /// and return the function id
    fn add_func(
        &mut self,
        params: &[ValType],
        results: &[ValType],
        fn_: impl FnMut(&mut FunctionBuilder, &Vec<LocalId>) -> eyre::Result<()>,
    ) -> eyre::Result<FunctionId>;

    /// get the memory id from target name
    /// and remove anchor
    fn get_target_memory_id(
        &mut self,
        name: impl AsRef<str>,
        remove: bool,
    ) -> eyre::Result<MemoryId>;

    fn find_used_memory_id(&self, memory_hint: Option<usize>) -> eyre::Result<MemoryId>;

    fn create_memory_anchor(
        &mut self,
        name: impl AsRef<str>,
        memory_id: MemoryId,
    ) -> eyre::Result<()>;

    fn get_global_anchor(&mut self, name: impl AsRef<str>) -> eyre::Result<Vec<GlobalId>>;

    fn create_global_anchor(&mut self, name: impl AsRef<str>) -> eyre::Result<()>;

    /// Return all functions that call functions in this fid
    fn get_using_func<A>(
        &self,
        as_fn: impl WalrusFID<A>,
        allow_call_indirect: bool,
    ) -> eyre::Result<Vec<(FunctionId, InstrSeqId, usize)>>;

    fn renew_id_on_table<A, B>(
        &mut self,
        old: impl WalrusFID<A>,
        new: impl WalrusFID<B>,
    ) -> eyre::Result<()>
    where
        Self: Sized;

    fn fid_pos_on_table<A>(&self, as_fn: impl WalrusFID<A>) -> eyre::Result<Vec<(TableId, usize)>>;

    fn renew_call_fn<A, B>(
        &mut self,
        old: impl WalrusFID<A>,
        new: impl WalrusFID<B>,
    ) -> eyre::Result<()>
    where
        Self: Sized;

    // this is broken
    // fn renew_call_fn_in_the_fn(
    //     &mut self,
    //     old_id: impl Borrow<FunctionId>,
    //     new_id: impl Borrow<FunctionId>,
    //     fn_id: impl Borrow<FunctionId>,
    // ) -> eyre::Result<()>
    // where
    //     Self: Sized;

    fn gen_new_function(
        &mut self,
        params: &[ValType],
        results: &[ValType],
        fn_: impl FnOnce(&mut FunctionBuilder, &Vec<LocalId>) -> eyre::Result<()>,
    ) -> eyre::Result<FunctionId>
    where
        Self: Sized;

    fn check_function_type<A, B>(
        &self,
        before: impl WalrusFID<A>,
        after: impl WalrusFID<B>,
    ) -> eyre::Result<()>
    where
        Self: Sized;

    #[allow(dead_code)]
    fn debug_call_indirect<A>(&mut self, debugger: impl WalrusFID<A>) -> eyre::Result<()>
    where
        Self: Sized;

    #[allow(dead_code)]
    fn gen_inspect<const N: usize, A>(
        &mut self,
        inspector: impl WalrusFID<A>,
        params: &[ValType],
        exclude: &[impl Borrow<FunctionId>],
        filter: impl FnMut(&ir::Instr) -> Option<[i32; N]>,
    ) -> eyre::Result<()>
    where
        Self: Sized,
    {
        self.gen_inspect_with_finalize(
            Some(inspector),
            None::<FunctionId>,
            params,
            &[],
            exclude,
            filter,
        )
    }

    #[allow(dead_code)]
    fn gen_finalize<const N: usize, A>(
        &mut self,
        finalize: impl WalrusFID<A>,
        params: &[ValType],
        exclude: &[impl Borrow<FunctionId>],
        filter: impl FnMut(&ir::Instr) -> Option<[i32; N]>,
    ) -> eyre::Result<()>
    where
        Self: Sized,
    {
        self.gen_inspect_with_finalize(
            None::<FunctionId>,
            Some(finalize),
            &[],
            params,
            exclude,
            filter,
        )
    }

    #[allow(dead_code)]
    fn gen_inspect_with_finalize<const N: usize, A, B>(
        &mut self,
        inspector: Option<impl WalrusFID<A>>,
        finalize: Option<impl WalrusFID<B>>,
        params: &[ValType],
        results: &[ValType],
        exclude: &[impl Borrow<FunctionId>],
        filter: impl FnMut(&ir::Instr) -> Option<[i32; N]>,
    ) -> eyre::Result<()>
    where
        Self: Sized;

    #[allow(dead_code)]
    fn assert_i32_const(
        &mut self,
        val: i32,
    ) -> eyre::Result<impl FnMut(&mut walrus::InstrSeqBuilder) -> eyre::Result<()> + 'static>;

    fn load(path: impl AsRef<Path>, dwarf: bool) -> eyre::Result<Self>
    where
        Self: Sized;

    fn copy_func<A>(&mut self, from: impl WalrusFID<A>) -> eyre::Result<walrus::FunctionId>
    where
        Self: Sized;

    // This method copies functions by copying the functions called internally.
    // It is used to rewrite the internal instructions of functions called under specific conditions.
    // Note: that calls_indirect may throw errors.
    fn nested_copy_func<A>(
        &mut self,
        from: impl WalrusFID<A>,
        exclude: &[impl Borrow<FunctionId>],
        allow_import_func: bool,
        allow_call_indirect: bool,
    ) -> eyre::Result<walrus::FunctionId>
    where
        Self: Sized;
}

impl WalrusUtilImport for ModuleImports {
    fn find_mut<A>(&mut self, as_fn: impl WalrusFID<A>) -> eyre::Result<&mut Import> {
        let fid = as_fn.get_fid(self)?;

        let import_id = self.get_imported_func(fid).unwrap().id();

        Ok(self.get_mut(import_id))
    }
}

impl WalrusUtilExport for ModuleExports {
    fn erase<A>(&mut self, as_fn: impl WalrusFID<A>) -> eyre::Result<()> {
        let fid = as_fn.get_fid(self)?;

        let export_id = self
            .iter()
            .find(|f| {
                if let walrus::ExportItem::Function(f) = f.item {
                    f == fid
                } else {
                    false
                }
            })
            .map(|f| f.id())
            .ok_or_else(|| eyre::eyre!("Export not found: {}", as_fn.as_str()))?;

        self.delete(export_id);

        Ok(())
    }
}

impl WalrusUtilModule for walrus::Module {
    fn connect_func_with_is_delete<A, B>(
        &mut self,
        import: impl WalrusFID<A>,
        export: impl WalrusFID<B>,
        is_delete: bool,
    ) -> eyre::Result<()> {
        let fid = import.get_fid(&self.imports)?;
        let export_id = export.get_fid(&self.exports)?;

        self.check_function_type(fid, export_id)
            .wrap_err("Function types do not match on connect func inner")?;

        self.replace_imported_func(fid, |(builder, arg_locals)| {
            let mut func_body = builder.func_body();

            for local in arg_locals {
                func_body.local_get(*local);
            }
            func_body.call(export_id);
            func_body.return_();
        })
        .to_eyre()
        .wrap_err_with(|| eyre::eyre!("Failed to replace imported function"))?;

        let export_id = self
            .exports
            .iter()
            .find(|f| {
                if let walrus::ExportItem::Function(f) = f.item {
                    f == export_id
                } else {
                    false
                }
            })
            .map(|f| f.id())
            .ok_or_else(|| eyre::eyre!("Export not found"))?;

        if is_delete {
            self.exports.delete(export_id);
        }

        Ok(())
    }

    fn add_func(
        &mut self,
        params: &[ValType],
        results: &[ValType],
        fn_: impl FnOnce(&mut FunctionBuilder, &Vec<LocalId>) -> eyre::Result<()>,
    ) -> eyre::Result<FunctionId> {
        let mut builder = FunctionBuilder::new(&mut self.types, params, results);

        let args = params
            .iter()
            .map(|ty| self.locals.add(*ty))
            .collect::<Vec<_>>();

        fn_(&mut builder, &args)?;

        Ok(builder.finish(args, &mut self.funcs))
    }

    /// if vfs, get vfs memory_id
    fn get_target_memory_id(
        &mut self,
        name: impl AsRef<str>,
        remove: bool,
    ) -> eyre::Result<MemoryId> {
        let name = name.as_ref();

        let anchor_func_id = format!("__wasip1_vfs_flag_{name}_memory").get_fid(&self.exports)?;

        self.exports.erase_with(anchor_func_id, !remove)?;

        let anchor_body = &self.funcs.get(anchor_func_id).kind;

        let local_func = anchor_body.unwrap_local();

        let func_body = local_func.block(local_func.entry_block());
        let memory_id = func_body
            .iter()
            .map(|(block, _)| block)
            .filter_map(|block| match block {
                ir::Instr::Load(ir::Load { memory, .. }) => Some(*memory),
                ir::Instr::Store(ir::Store { memory, .. }) => Some(*memory),
                _ => None,
            })
            .fold(Ok(Option::<MemoryId>::None), |a, b| match a? {
                Some(a) if a == b => Ok(Some(a)),
                None => Ok(Some(b)),
                Some(_) => eyre::bail!("Anchor access double memory, cannot determine memory id"),
            })?
            .ok_or_else(|| eyre::eyre!("Memory not found"))?;

        Ok(memory_id)
    }

    fn find_used_memory_id(&self, memory_hint: Option<usize>) -> eyre::Result<MemoryId> {
        let memories = self
            .memories
            .iter()
            .map(|memory| memory.id())
            .collect::<Vec<_>>();

        if memories.is_empty() {
            eyre::bail!("No memories found");
        }

        // After calling environ_sizes_get,
        // identify the memory using the memory referenced
        // by the code trying to read the pointer
        let memory_id = if memories.len() > 1 && memory_hint.is_none() {
            let gen_memory_id = || -> eyre::Result<MemoryId> {
                // environ_sizes_get
                let import_id =
                    ("wasi_snapshot_preview1", "environ_sizes_get").get_fid(&self.imports)?;

                let using_funcs = self.get_using_func(import_id, true)?;

                let ret_mem_id = std::sync::Arc::new(std::sync::Mutex::new(None));

                for (fid, _, _) in using_funcs {
                    let arg_ptr =
                        std::sync::Arc::new(std::sync::Mutex::new(Option::<Vec<u32>>::None));
                    let arg_ptr_c = arg_ptr.clone();

                    let ret_mem_id_c = ret_mem_id.clone();

                    let mut interpreter = walrus_simple_interpreter::Interpreter::new(self)
                        .to_eyre()
                        .wrap_err_with(|| eyre::eyre!("Failed to create interpreter"))?;

                    interpreter.set_interrupt_handler_mem(move |_, _, _, (id, address, _, ty)| {

                    if matches!(ty, walrus_simple_interpreter::MemoryAccessType::Load) {
                        if let Some(v) = arg_ptr_c.lock().unwrap().as_ref() {
                            if v.contains(&address) {
                                if let Some(mem_id) = ret_mem_id_c.clone().lock().unwrap().as_ref() {
                                    if *mem_id != id {
                                        return Err(anyhow::anyhow!(
                                            "Memory access double memory, cannot determine memory id"
                                        ));
                                    }
                                } else {
                                    ret_mem_id_c.clone().lock().unwrap().replace(id);
                                }
                            }
                        }
                    }

                    Ok(())
                });

                    let memories = memories.clone();

                    interpreter.add_function("environ_sizes_get", move |interpreter, args| {
                        let args = args
                            .iter()
                            .map(|arg| {
                                if let ir::Value::I32(arg) = arg {
                                    Ok(*arg as u32)
                                } else {
                                    Err(anyhow::anyhow!("Invalid argument type"))
                                }
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        interpreter.mem_set_i32(memories[0], args[0], 0)?;
                        interpreter.mem_set_i32(memories[0], args[1], 0)?;

                        arg_ptr.lock().unwrap().replace(args);

                        Ok(vec![ir::Value::I32(0)])
                    });

                    let args = self
                        .types
                        .get(self.funcs.get(fid).ty())
                        .params()
                        .iter()
                        .map(|ty| ty.normal())
                        .collect::<eyre::Result<Vec<_>>>()
                        .wrap_err_with(|| eyre::eyre!("Failed to get function args"))?;
                    if let Err(e) = interpreter.call(fid, self, &args).to_eyre() {
                        if ret_mem_id.lock().unwrap().is_none() {
                            eprintln!("Error: {e}");
                        }
                    }
                }

                if let Some(mem_id) = ret_mem_id.lock().unwrap().as_ref() {
                    Ok(*mem_id)
                } else {
                    return Err(eyre::eyre!("Memory not found"));
                }
            };
            gen_memory_id().wrap_err_with(|| {
                eyre::eyre!("Failed to detect memory id. You can use memory hint.")
            })?
        } else if let Some(memory_hint) = memory_hint {
            if memory_hint >= memories.len() {
                return Err(eyre::eyre!(
                    "Memory hint {} is out of bounds for memories: {:?}",
                    memory_hint,
                    memories
                ));
            }
            memories[memory_hint]
        } else {
            memories[0]
        };

        Ok(memory_id)
    }

    fn create_memory_anchor(
        &mut self,
        name: impl AsRef<str>,
        memory_id: MemoryId,
    ) -> eyre::Result<()> {
        let name = name.as_ref();

        // unsafe extern "C" fn __wasip1_vfs_flag_vfs_memory(ptr: *mut u8, src: *mut u8) {
        //     unsafe { core::ptr::copy_nonoverlapping(src, ptr, 1) };
        // }
        let id = self.add_func(&[ValType::I32, ValType::I32], &[], |builder, arg_locals| {
            let mut func_body = builder.func_body();

            func_body
                .local_get(arg_locals[0])
                .local_get(arg_locals[1])
                .load(
                    memory_id,
                    ir::LoadKind::I32_8 {
                        kind: ir::ExtendedLoad::ZeroExtend,
                    },
                    ir::MemArg {
                        offset: 0,
                        align: 0,
                    },
                )
                .store(
                    memory_id,
                    ir::StoreKind::I32_8 { atomic: false },
                    ir::MemArg {
                        offset: 0,
                        align: 0,
                    },
                );

            func_body.return_();

            Ok(())
        })?;

        self.exports
            .add(&format!("__wasip1_vfs_flag_{name}_memory"), id);

        Ok(())
    }

    fn get_global_anchor(&mut self, name: impl AsRef<str>) -> eyre::Result<Vec<GlobalId>> {
        let name = name.as_ref();
        let anchor_name = format!("__wasip1_vfs_flag_{name}_global");

        let anchor_func_id = self
            .exports
            .get_func(&anchor_name)
            .to_eyre()
            .wrap_err_with(|| eyre::eyre!("anchor {anchor_name} not found"))?;

        self.exports
            .remove(&anchor_name)
            .to_eyre()
            .wrap_err_with(|| eyre::eyre!("Failed to remove anchor export"))?;

        let anchor_body = &self.funcs.get(anchor_func_id).kind;
        if let FunctionKind::Local(local_func) = anchor_body {
            let entry_id = local_func.entry_block();
            let func_body = local_func.block(entry_id);
            let global_ids = func_body
                .iter()
                .map(|(block, _)| block)
                .filter_map(|block| match block {
                    ir::Instr::GlobalSet(ir::GlobalSet { global, .. }) => Some(*global),
                    _ => None,
                })
                .collect::<Vec<_>>();

            Ok(global_ids)
        } else {
            Err(eyre::eyre!(
                "anchor (local function) {anchor_name} not found",
            ))
        }
    }

    fn create_global_anchor(&mut self, name: impl AsRef<str>) -> eyre::Result<()> {
        let name = name.as_ref();

        let global_ids = self
            .globals
            .iter()
            .map(|global| (global.id(), global.ty))
            .collect::<Vec<_>>();
        let id = self.add_func(&[], &[], |builder, _| {
            let mut func_body = builder.func_body();

            for (id, ty) in global_ids.iter() {
                func_body
                    .const_(
                        ty.normal()
                            .wrap_err_with(|| eyre::eyre!("Failed to get global type"))?,
                    )
                    .global_set(*id);
            }

            func_body.return_();

            Ok(())
        })?;

        self.exports
            .add(&format!("__wasip1_vfs_flag_{name}_global"), id);

        Ok(())
    }

    fn get_using_func<A>(
        &self,
        as_fn: impl WalrusFID<A>,
        allow_call_indirect: bool,
    ) -> eyre::Result<Vec<(FunctionId, InstrSeqId, usize)>> {
        let fid = as_fn.get_fid(self)?;

        self.funcs
            .iter_local()
            .map(|(id, func)| {
                func.read(|instr, place| {
                    use walrus::ir::*;
                    match instr {
                        Instr::Call(Call { func }) | Instr::ReturnCall(ReturnCall { func })
                            if fid == *func =>
                        {
                            Ok(Some((id, place)))
                        }
                        Instr::CallIndirect(CallIndirect { table: _, ty: _ })
                            if !allow_call_indirect =>
                        {
                            eyre::bail!("call_indirect is not supported in get_using_func");
                        }
                        _ => Ok(None),
                    }
                })
                .and_then(|v| {
                    v.into_iter()
                        .filter_map_ok(|v| v)
                        .map_ok(|(a, (b, c))| (a, c, b))
                        .collect::<eyre::Result<Vec<_>>>()
                })
            })
            .flatten_ok()
            .collect::<eyre::Result<Vec<_>>>()
    }

    fn renew_id_on_table<A, B>(
        &mut self,
        old: impl WalrusFID<A>,
        new: impl WalrusFID<B>,
    ) -> eyre::Result<()>
    where
        Self: Sized,
    {
        let old_id = old.get_fid(self)?;
        let new_id = new.get_fid(self)?;

        self.check_function_type(old_id, new_id)
            .wrap_err("Function types do not match on renew id on table")?;

        for table in self.tables.iter_mut() {
            for elem in &table.elem_segments {
                let elem = self.elements.get_mut(*elem);
                if let walrus::ElementKind::Active {
                    table: table_id, ..
                } = elem.kind
                {
                    if table_id != table.id() {
                        unreachable!();
                    }
                } else {
                    unreachable!();
                }
                match &mut elem.items {
                    walrus::ElementItems::Functions(ids) => {
                        ids.iter_mut().for_each(|id| {
                            if *id == old_id {
                                log::info!(
                                    "Rewriting function id on table. Old: {:?}, New: {:?}",
                                    old_id,
                                    new_id
                                );
                                *id = new_id;
                            }
                        });
                    }
                    walrus::ElementItems::Expressions(..) => unimplemented!(),
                }
            }
        }

        Ok(())
    }

    fn fid_pos_on_table<A>(&self, fid: impl WalrusFID<A>) -> eyre::Result<Vec<(TableId, usize)>> {
        let fid = fid.get_fid(self)?;

        let mut positions = vec![];
        for table in self.tables.iter() {
            for elem in &table.elem_segments {
                let elem = self.elements.get(*elem);
                if let walrus::ElementKind::Active {
                    table: table_id, ..
                } = elem.kind
                {
                    if table_id != table.id() {
                        unreachable!();
                    }
                } else {
                    unreachable!();
                }
                match &elem.items {
                    walrus::ElementItems::Functions(ids) => {
                        ids.iter().copied().enumerate().for_each(|(i, id)| {
                            if id == fid {
                                positions.push((table.id(), i));
                            }
                        });
                    }
                    walrus::ElementItems::Expressions(..) => unimplemented!(),
                }
            }
        }
        Ok(positions)
    }

    fn renew_call_fn<A, B>(
        &mut self,
        old: impl WalrusFID<A>,
        new: impl WalrusFID<B>,
    ) -> eyre::Result<()>
    where
        Self: Sized,
    {
        let old_id = old.get_fid(self)?;
        let new_id = new.get_fid(self)?;

        for (id, _, _) in self
            .get_using_func(old_id, true)
            .wrap_err("Failed to get using func")?
        {
            self.funcs
                .rewrite(
                    |instr, _| {
                        if let walrus::ir::Instr::Call(call) = instr {
                            if call.func == old_id {
                                call.func = new_id;
                            }
                        }
                    },
                    id,
                )
                .wrap_err("Failed to renew function call")?;
        }

        self.renew_id_on_table(old_id, new_id)?;

        // if old function is imported
        if let walrus::FunctionKind::Import(import) = &self.funcs.get(old_id).kind {
            self.imports.delete(import.import);
        }
        // renew export
        self.exports
            .iter_mut()
            .filter(|export| match export.item {
                walrus::ExportItem::Function(f) if f == old_id => true,
                _ => false,
            })
            .for_each(|export| {
                export.item = walrus::ExportItem::Function(new_id);
            });
        self.funcs.delete(old_id);

        Ok(())
    }

    fn gen_new_function(
        &mut self,
        params: &[ValType],
        results: &[ValType],
        fn_: impl FnOnce(&mut FunctionBuilder, &Vec<LocalId>) -> eyre::Result<()>,
    ) -> eyre::Result<FunctionId>
    where
        Self: Sized,
    {
        let args = params
            .iter()
            .map(|ty| self.locals.add(*ty))
            .collect::<Vec<_>>();

        let mut func = FunctionBuilder::new(&mut self.types, &params, &results);

        fn_(&mut func, &args)?;

        Ok(func.finish(args, &mut self.funcs))
    }

    fn check_function_type<A, B>(
        &self,
        before: impl WalrusFID<A>,
        after: impl WalrusFID<B>,
    ) -> eyre::Result<()>
    where
        Self: Sized,
    {
        let before = before.get_fid(self)?;
        let after = after.get_fid(self)?;

        let a_ty = self.funcs.get(before).ty();
        let a_ty_params = self.types.get(a_ty).params();
        let a_ty_results = self.types.get(a_ty).results();

        let b_ty = self.funcs.get(after).ty();
        let b_ty_params = self.types.get(b_ty).params();
        let b_ty_results = self.types.get(b_ty).results();

        if a_ty_params != b_ty_params || a_ty_results != b_ty_results {
            eyre::bail!(
                "Function types do not match. Before: {a_ty_params:?} -> {a_ty_results:?}, After: {b_ty_params:?} -> {b_ty_results:?}"
            );
        }

        Ok(())
    }

    // Insert a specific function into every call_indirect within all functions.
    // The type of the received function is fn (table_id, pos);
    fn debug_call_indirect<A>(&mut self, id: impl WalrusFID<A>) -> eyre::Result<()>
    where
        Self: Sized,
    {
        let id = id.get_fid(self)?;

        // check id type
        if self.types.get(self.funcs.get(id).ty()).params() != [ValType::I32, ValType::I32]
            || self.types.get(self.funcs.get(id).ty()).results() != []
        {
            eyre::bail!("Function type must be (i32, i32) -> ()");
        }

        let ids = self.funcs.find_children_with(id, false)?;

        let tables = self
            .funcs
            .iter_local()
            .filter(|(fid, _)| !ids.contains(fid))
            .map(|(fid, fn_)| {
                fn_.read(|instr, pos| {
                    if let walrus::ir::Instr::CallIndirect(call) = instr {
                        Some((call.table, (fid, pos)))
                    } else if let walrus::ir::Instr::ReturnCallIndirect(..) = instr {
                        unimplemented!("return_call_indirect is not supported yet")
                    } else {
                        None
                    }
                })
            })
            .flatten_ok()
            .filter_map_ok(|x| x)
            .collect::<eyre::Result<Vec<_>>>()?;

        let table_fns = tables
            .iter()
            .map(|(table, _)| *table)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .map(|table| {
                // As if nothing had happened, it passes the value again.
                let middle_fn_id =
                    self.add_func(&[ValType::I32], &[ValType::I32], |builder, args| {
                        builder
                            .func_body()
                            .i32_const(table.index() as i32)
                            .local_get(args[0])
                            .call(id)
                            .local_get(args[0])
                            .return_();
                        Ok(())
                    })?;

                Ok((table, middle_fn_id))
            })
            .collect::<eyre::Result<std::collections::HashMap<_, _>>>()?;

        for (tid, (fid, (pos, seq_id))) in tables
            .into_iter()
            .sorted_by(
                |(_, (fid_a, (pos_a, seq_id_a))), (_, (fid_b, (pos_b, seq_id_b)))| match fid_a
                    .cmp(&fid_b)
                {
                    std::cmp::Ordering::Equal => match seq_id_a.cmp(&seq_id_b) {
                        std::cmp::Ordering::Equal => pos_a.cmp(&pos_b),
                        other => other,
                    },
                    other => other,
                },
            )
            .rev()
        {
            match self.funcs.get_mut(fid).kind {
                FunctionKind::Local(ref mut local_func) => {
                    if let Some(walrus::ir::Instr::CallIndirect(walrus::ir::CallIndirect {
                        table,
                        ..
                    })) = local_func
                        .builder_mut()
                        .instr_seq(seq_id)
                        .instrs()
                        .get(pos)
                        .map(|(instr, _)| instr)
                    {
                        if *table != tid {
                            eyre::bail!("Table id mismatch");
                        }
                    } else {
                        eyre::bail!("Instruction at position is not call_indirect");
                    }
                    local_func
                        .builder_mut()
                        .instr_seq(seq_id)
                        .call_at(pos, table_fns[&tid]);
                }
                _ => unreachable!(),
            }
        }

        Ok(())
    }

    fn gen_inspect_with_finalize<const N: usize, A, B>(
        &mut self,
        inspector: Option<impl WalrusFID<A>>,
        finalize: Option<impl WalrusFID<B>>,
        params: &[ValType],
        results: &[ValType],
        exclude: &[impl Borrow<FunctionId>],
        mut filter: impl FnMut(&ir::Instr) -> Option<[i32; N]>,
    ) -> eyre::Result<()>
    where
        Self: Sized,
    {
        let inspector: Option<FunctionId> = inspector.map(|id| id.get_fid(self)).transpose()?;
        let finalize: Option<FunctionId> = finalize.map(|id| id.get_fid(self)).transpose()?;

        // check inspector type
        let check_inspector = |params: &[ValType], name: &str, fid| {
            if self.types.get(self.funcs.get(fid).ty()).params()
                != params
                    .iter()
                    .cloned()
                    .chain(std::iter::repeat(ValType::I32).take(N))
                    .collect::<Vec<_>>()
            {
                eyre::bail!("{name} function type must be ({params:?}) -> ()",);
            }

            Ok(())
        };

        if let Some(inspector) = inspector {
            check_inspector(params, "Inspector", inspector)?;
        }

        if let Some(finalize) = finalize {
            check_inspector(results, "Finalize", finalize)?;
        }

        let exclude = [inspector, finalize]
            .iter()
            .filter_map(|id| *id)
            .map(|f| self.funcs.find_children_with(f, false))
            .flatten_ok()
            .chain(exclude.iter().map(|id| Ok(*id.borrow())))
            .collect::<eyre::Result<std::collections::HashSet<_>>>()
            .wrap_err("Failed to find exclude functions")?;

        let instrs = self
            .funcs
            .iter_local()
            .filter(|(fid, _)| !exclude.contains(fid))
            .map(|(fid, fn_)| {
                fn_.read(|instr, pos| {
                    if let Some(ret) = filter(instr) {
                        Some((ret, fid, pos))
                    } else {
                        None
                    }
                })
            })
            .flatten_ok()
            .filter_map_ok(|x| x)
            .collect::<eyre::Result<Vec<_>>>()?;

        let instrs_set = instrs
            .iter()
            .map(|(ret, _, _)| *ret)
            .collect::<std::collections::HashSet<_>>();

        let mut group_by_fns = |fns: Option<FunctionId>, params: &[ValType]| {
            fns.map(|fns| {
                instrs_set
                    .iter()
                    .map(|ret| {
                        let middle_fn_id = self.add_func(&params, &params, |builder, args| {
                            let mut func_body = builder.func_body();
                            for ret in ret {
                                func_body.i32_const(*ret);
                            }
                            for arg in args {
                                func_body.local_get(*arg);
                            }
                            func_body.call(fns);
                            for arg in args {
                                func_body.local_get(*arg);
                            }
                            func_body.return_();
                            Ok(())
                        })?;
                        Ok((ret, middle_fn_id))
                    })
                    .collect::<eyre::Result<std::collections::HashMap<_, _>>>()
            })
            .transpose()
        };

        let group_inspector_fns = group_by_fns(inspector, params)?;
        let group_finalize_fns = group_by_fns(finalize, results)?;

        let ids = [inspector, finalize]
            .iter()
            .filter_map(|id| *id)
            .map(|fid| self.funcs.find_children_with(fid, false))
            .flatten_ok()
            .collect::<eyre::Result<std::collections::HashSet<_>>>()
            .wrap_err("Failed to find exclude functions")?;

        for (ret, fid, (pos, seq_id)) in instrs
            .into_iter()
            .sorted_by(
                |(_, fid_a, (pos_a, seq_id_a)), (_, fid_b, (pos_b, seq_id_b))| match fid_a
                    .cmp(&fid_b)
                {
                    std::cmp::Ordering::Equal => match seq_id_a.cmp(&seq_id_b) {
                        std::cmp::Ordering::Equal => pos_a.cmp(&pos_b),
                        other => other,
                    },
                    other => other,
                },
            )
            .rev()
        {
            if ids.contains(&fid) {
                continue;
            }

            match self.funcs.get_mut(fid).kind {
                FunctionKind::Local(ref mut local_func) => {
                    let mut instr_seq = local_func.builder_mut().instr_seq(seq_id);
                    let instr = instr_seq
                        .instrs()
                        .get(pos)
                        .map(|(instr, _)| instr)
                        .ok_or_else(|| eyre::eyre!("Instruction at position not found"))?;

                    if filter(instr).is_none() {
                        eyre::bail!("Instruction at position does not match filter");
                    }

                    if let Some(_) = finalize {
                        instr_seq.call_at(pos + 1, group_finalize_fns.as_ref().unwrap()[&ret]);
                    }
                    if let Some(_) = inspector {
                        instr_seq.call_at(pos, group_inspector_fns.as_ref().unwrap()[&ret]);
                    }
                }
                _ => unreachable!(),
            }
        }

        Ok(())
    }

    fn assert_i32_const(
        &mut self,
        val: i32,
    ) -> eyre::Result<impl FnMut(&mut walrus::InstrSeqBuilder) -> eyre::Result<()> + 'static> {
        use walrus::ValType::I32;
        let fid = self.add_func(&[I32], &[I32], |builder, args| {
            builder
                .func_body()
                .i32_const(val)
                .local_get(args[0])
                .binop(walrus::ir::BinaryOp::I32Eq)
                .if_else(
                    None,
                    |cons| {
                        cons.local_get(args[0]);
                        cons.return_();
                    },
                    |els| {
                        els.unreachable();
                    },
                );
            Ok(())
        })?;

        Ok(
            move |func_body: &mut walrus::InstrSeqBuilder| -> eyre::Result<()> {
                func_body.call(fid);
                Ok(())
            },
        )
    }

    fn load(path: impl AsRef<Path>, dwarf: bool) -> eyre::Result<Self>
    where
        Self: Sized,
    {
        let mut config = walrus::ModuleConfig::new();
        config.generate_dwarf(dwarf);
        let module = walrus::Module::from_file_with_config(path.as_ref(), &config)
            .to_eyre()
            .wrap_err_with(|| {
                eyre::eyre!("Failed to load Wasm file: {}", path.as_ref().display())
            })?;
        Ok(module)
    }

    fn connect_func_alt<A, B>(
        &mut self,
        import: impl WalrusFID<A>,
        export: impl WalrusFID<B>,
        is_debug: bool,
    ) -> eyre::Result<()> {
        let export = export.get_fid(&self.exports)?;
        self.renew_call_fn(import, export)?;

        if !is_debug {
            let eid = self
                .exports
                .iter()
                .find(|e| {
                    if let walrus::ExportItem::Function(f) = e.item {
                        f == export
                    } else {
                        false
                    }
                })
                .map(|e| e.id())
                .unwrap();

            self.exports.delete(eid);
        }

        Ok(())
    }

    fn copy_func<A>(&mut self, from: impl WalrusFID<A>) -> eyre::Result<walrus::FunctionId>
    where
        Self: Sized,
    {
        let from = from.get_fid(self)?;

        let func_base = self.funcs.get(from);
        let ty_base = func_base.ty();
        let types_base = self.types.get(ty_base);
        let params_base = types_base.params().to_vec();
        let results_base = types_base.results().to_vec();
        let local_func_base = func_base.kind.unwrap_local();

        let local_ids_base = local_func_base
            .args
            .iter()
            .copied()
            .chain(
                local_func_base
                    .read(|instr, _| match instr {
                        walrus::ir::Instr::LocalGet(walrus::ir::LocalGet { local }) => Some(*local),
                        walrus::ir::Instr::LocalSet(walrus::ir::LocalSet { local }) => Some(*local),
                        walrus::ir::Instr::LocalTee(walrus::ir::LocalTee { local }) => Some(*local),
                        _ => None,
                    })?
                    .into_iter()
                    .filter_map(|x| x)
                    .filter(|id| !local_func_base.args.contains(id))
                    .collect::<std::collections::HashSet<_>>()
                    .into_iter(),
            )
            .collect::<Vec<_>>();

        let locals_base = local_ids_base
            .iter()
            .map(|ty| self.locals.get(*ty).ty())
            .collect::<Vec<_>>();

        let entry_base = local_func_base.entry_block();
        let mut instrs_base = local_func_base
            .read(|instr, (pos, id)| (pos, id, instr.clone()))?
            .into_iter()
            .into_group_map_by(|(_, id, _)| *id)
            .into_iter()
            .map(|(id, vec)| {
                let instrs = vec
                    .into_iter()
                    .map(|(pos, _, instr)| (pos, instr))
                    .sorted_by_key(|(pos, _)| *pos)
                    .enumerate()
                    .inspect(|(i, (pos, _))| assert_eq!(i, pos))
                    .map(|(_, (_, instr))| instr)
                    .collect::<Vec<_>>();

                (id, instrs)
            })
            .collect::<HashMap<_, _>>();

        let mut builder = FunctionBuilder::new(&mut self.types, &params_base, &results_base);

        let new_locals = locals_base
            .iter()
            .map(|ty| self.locals.add(*ty))
            .collect::<Vec<_>>();

        let new_args = new_locals
            .iter()
            .copied()
            .take(local_func_base.args.len())
            .collect::<Vec<_>>();

        let local_map = local_ids_base
            .iter()
            .zip(new_locals.iter())
            .map(|(a, b)| (*a, *b))
            .collect::<HashMap<_, _>>();

        let new_body_id = builder.func_body_id();

        let mut seq_map = instrs_base
            .keys()
            .filter(|id| **id != entry_base)
            .map(|id| {
                (
                    *id,
                    builder
                        .dangling_instr_seq(local_func_base.block(*id).ty)
                        .id(),
                )
            })
            .chain(std::iter::once((entry_base, new_body_id)))
            .collect::<HashMap<_, _>>();

        use walrus::ir::*;

        for (seq, instrs_base) in instrs_base.drain() {
            instrs_base
                .iter()
                .map(|instr| match instr {
                    Instr::Block(Block { seq }) => {
                        vec![seq]
                    }
                    Instr::Loop(Loop { seq }) => {
                        vec![seq]
                    }
                    Instr::Br(Br { block }) => {
                        vec![block]
                    }
                    Instr::BrIf(BrIf { block }) => {
                        vec![block]
                    }
                    Instr::IfElse(IfElse {
                        alternative,
                        consequent,
                    }) => {
                        vec![alternative, consequent]
                    }
                    Instr::BrTable(BrTable { blocks, default }) => blocks
                        .iter()
                        .chain(std::iter::once(default))
                        .collect::<Vec<_>>(),
                    _ => Vec::new(),
                })
                .flatten()
                .for_each(|b| {
                    if !seq_map.contains_key(&b) {
                        let blank_seq = builder
                            .dangling_instr_seq(local_func_base.block(*b).ty)
                            .id();
                        seq_map.insert(*b, blank_seq);
                    }
                });

            let mut now_seq = builder.instr_seq(seq_map[&seq]);

            for instr in instrs_base {
                match instr {
                    Instr::Block(Block { seq }) => {
                        now_seq.instr(Instr::Block(Block { seq: seq_map[&seq] }));
                    }
                    Instr::Loop(Loop { seq }) => {
                        now_seq.instr(Instr::Loop(Loop { seq: seq_map[&seq] }));
                    }
                    Instr::Br(Br { block }) => {
                        now_seq.instr(Instr::Br(Br {
                            block: seq_map[&block],
                        }));
                    }
                    Instr::BrIf(BrIf { block }) => {
                        now_seq.instr(Instr::BrIf(BrIf {
                            block: seq_map[&block],
                        }));
                    }
                    Instr::IfElse(IfElse {
                        consequent,
                        alternative,
                    }) => {
                        now_seq.instr(Instr::IfElse(IfElse {
                            consequent: seq_map[&consequent],
                            alternative: seq_map[&alternative],
                        }));
                    }
                    Instr::BrTable(BrTable { blocks, default }) => {
                        now_seq.instr(Instr::BrTable(BrTable {
                            blocks: blocks
                                .iter()
                                .map(|b| seq_map[b])
                                .collect::<Vec<_>>()
                                .into_boxed_slice(),
                            default: seq_map[&default],
                        }));
                    }
                    Instr::LocalGet(LocalGet { local }) => {
                        now_seq.instr(Instr::LocalGet(LocalGet {
                            local: local_map[&local],
                        }));
                    }
                    Instr::LocalSet(LocalSet { local }) => {
                        now_seq.instr(Instr::LocalSet(LocalSet {
                            local: local_map[&local],
                        }));
                    }
                    Instr::LocalTee(LocalTee { local }) => {
                        now_seq.instr(Instr::LocalTee(LocalTee {
                            local: local_map[&local],
                        }));
                    }
                    _ => {
                        now_seq.instr(instr);
                    }
                }
            }
        }

        Ok(builder.finish(new_args, &mut self.funcs))
    }

    fn nested_copy_func<A>(
        &mut self,
        from: impl WalrusFID<A>,
        exclude: &[impl Borrow<FunctionId>],
        allow_import_func: bool,
        allow_call_indirect: bool,
    ) -> eyre::Result<walrus::FunctionId>
    where
        Self: Sized,
    {
        let from = from.get_fid(self)?;
        let exclude = exclude.iter().map(|e| *e.borrow()).collect::<Vec<_>>();

        let mut fid_map: HashMap<FunctionId, FunctionId> = HashMap::new();

        if exclude.contains(&from) {
            return Ok(from);
        }

        let fids = self.funcs.find_children_with(from, allow_call_indirect)?;

        for fid in fids {
            if exclude.contains(&fid) {
                continue;
            }
            if fid_map.contains_key(&fid) {
                unreachable!();
            }

            let func = self.funcs.get(fid);
            match &func.kind {
                FunctionKind::Import(import) => {
                    if !allow_import_func {
                        let import = self.imports.get(import.import);
                        eyre::bail!("Import function found: {:?}", import);
                    }
                    fid_map.insert(fid, fid);
                }
                FunctionKind::Local(_) => {
                    let new_fid = self.copy_func(fid)?;
                    fid_map.insert(fid, new_fid);
                }
                _ => {
                    eyre::bail!("Unknown function kind: {:?}", func.kind);
                }
            }
        }

        for (old_fid, new_fid) in fid_map.iter() {
            if *old_fid == from || exclude.contains(old_fid) || *new_fid == *old_fid {
                continue;
            }
            let local = self.funcs.get_mut(*new_fid).kind.unwrap_local_mut();
            local
                .builder_mut()
                .func_body()
                .rewrite(|instr, _| {
                    use walrus::ir::*;
                    match instr {
                        Instr::Call(Call { func, .. })
                        | Instr::ReturnCall(ReturnCall { func, .. }) => {
                            if *func == *old_fid {
                                *func = fid_map[old_fid];
                            }
                        }
                        Instr::CallIndirect(call) if !allow_call_indirect => {
                            eyre::bail!("Call indirect found: {:?}", call);
                        }
                        Instr::ReturnCallIndirect(call) if !allow_call_indirect => {
                            eyre::bail!("Return call indirect found: {:?}", call);
                        }
                        _ => {}
                    }
                    Ok(())
                })
                .wrap_err("Failed to renew function call")?;
        }

        Ok(fid_map[&from])
    }
}

impl WalrusUtilFuncs for walrus::ModuleFunctions {
    fn find_children(
        &self,
        fid: impl Borrow<FunctionId>,
        allow_call_indirect: bool,
    ) -> eyre::Result<Vec<FunctionId>> {
        let fid = *fid.borrow();

        let mut children = vec![];
        let mut stack = vec![fid];
        while let Some(fid) = stack.pop() {
            match &self.get(fid).kind {
                FunctionKind::Local(imported_function) => {
                    imported_function
                        .read(|instr, _place| {
                            use walrus::ir::*;
                            match instr {
                                Instr::Call(Call { func })
                                | Instr::ReturnCall(ReturnCall { func, .. }) => {
                                    if !children.contains(func) {
                                        children.push(*func);
                                        stack.push(*func);
                                    }
                                }
                                Instr::CallIndirect(call) if !allow_call_indirect => {
                                    eyre::bail!("Call indirect found: {:?}", call);
                                }
                                Instr::ReturnCallIndirect(call) if !allow_call_indirect => {
                                    eyre::bail!("Return call indirect found: {:?}", call);
                                }
                                _ => {}
                            }
                            Ok(())
                        })?
                        .into_iter()
                        .collect::<eyre::Result<Vec<_>>>()?;
                }
                _ => {}
            }
        }
        Ok(children)
    }

    fn rewrite<T>(
        &mut self,
        find: impl FnMut(&mut ir::Instr, (usize, InstrSeqId)) -> T,
        fid: impl Borrow<FunctionId>,
    ) -> eyre::Result<Vec<T>>
    where
        Self: Sized,
    {
        let fid = *fid.borrow();

        let func = self.get_mut(fid);
        if let FunctionKind::Local(local_func) = &mut func.kind {
            local_func.builder_mut().func_body().rewrite(find)
        } else {
            eyre::bail!("Function is not local");
        }
    }

    fn read<T>(
        &self,
        mut find: impl FnMut(&ir::Instr, (usize, InstrSeqId)) -> T,
        fid: impl Borrow<FunctionId>,
    ) -> eyre::Result<Vec<T>>
    where
        Self: Sized,
    {
        let fid = *fid.borrow();

        let func = self.get(fid);
        if let FunctionKind::Local(local_func) = &func.kind {
            local_func.read(&mut find)
        } else {
            eyre::bail!("Function is not local");
        }
    }

    fn flat_rewrite<T>(
        &mut self,
        mut find: impl FnMut(&mut ir::Instr, (usize, InstrSeqId)) -> T,
        fid: impl Borrow<FunctionId>,
        allow_call_indirect: bool,
    ) -> eyre::Result<Vec<T>>
    where
        Self: Sized,
    {
        self.find_children_with(fid, allow_call_indirect)?
            .into_iter()
            .filter(|fid| {
                if let walrus::FunctionKind::Local(_) = self.get(*fid).kind {
                    true
                } else {
                    false
                }
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|fid| self.rewrite(&mut find, fid))
            .flatten_ok()
            .collect()
    }

    fn flat_read<T>(
        &self,
        mut find: impl FnMut(&ir::Instr, (usize, InstrSeqId)) -> T,
        fid: impl Borrow<FunctionId>,
    ) -> eyre::Result<Vec<T>>
    where
        Self: Sized,
    {
        self.find_children_with(fid, false)?
            .into_iter()
            .filter(|fid| {
                if let walrus::FunctionKind::Local(_) = self.get(*fid).kind {
                    true
                } else {
                    false
                }
            })
            .map(|fid| self.read(&mut find, fid))
            .flatten_ok()
            .collect()
    }

    fn all_read<T>(
        &self,
        mut find: impl FnMut(&ir::Instr, (usize, InstrSeqId)) -> T,
        exclude: &[impl Borrow<FunctionId>],
    ) -> eyre::Result<Vec<T>>
    where
        Self: Sized,
    {
        let exclude = exclude.iter().map(|e| *e.borrow()).collect::<Vec<_>>();
        self.iter_local()
            .filter(|(fid, _)| !exclude.contains(fid))
            .map(|(fid, _)| self.read(&mut find, fid))
            .flatten_ok()
            .collect()
    }

    fn all_rewrite<T>(
        &mut self,
        mut find: impl FnMut(&mut ir::Instr, (usize, InstrSeqId)) -> T,
        exclude: &[impl Borrow<FunctionId>],
    ) -> eyre::Result<Vec<T>>
    where
        Self: Sized,
    {
        let exclude = exclude.iter().map(|e| *e.borrow()).collect::<Vec<_>>();
        self.iter_local()
            .filter(|(fid, _)| !exclude.contains(fid))
            .map(|(fid, _)| fid)
            .collect::<Vec<_>>()
            .into_iter()
            .map(|fid| self.rewrite(&mut find, fid))
            .flatten_ok()
            .collect()
    }
}

pub trait CaminoUtilModule {
    fn get_file_main_name(&self) -> Option<String>;
}

impl CaminoUtilModule for camino::Utf8Path {
    fn get_file_main_name(&self) -> Option<String> {
        let binding = self.file_name().unwrap().split(".").collect::<Vec<_>>();
        let file_name_poss = binding.iter().rev();
        let mut file_name = None;
        for name in file_name_poss {
            if *name == "opt" || *name == "adjusted" || *name == "wasm" || *name == "core" {
                continue;
            }
            file_name = Some(name);
            break;
        }

        file_name.map(|s| s.to_string())
    }
}

impl CaminoUtilModule for PathBuf {
    fn get_file_main_name(&self) -> Option<String> {
        camino::Utf8Path::new(self.to_str().unwrap()).get_file_main_name()
    }
}

impl CaminoUtilModule for Path {
    fn get_file_main_name(&self) -> Option<String> {
        camino::Utf8Path::new(self.to_str().unwrap()).get_file_main_name()
    }
}

pub trait ResultUtil<T> {
    fn to_eyre(self) -> eyre::Result<T>;
}

// https://github.com/eyre-rs/eyre/issues/31
impl<T> ResultUtil<T> for anyhow::Result<T> {
    fn to_eyre(self) -> eyre::Result<T> {
        self.map_err(|e| {
            eyre::eyre!(Box::<dyn std::error::Error + Send + Sync + 'static>::from(
                e
            ))
        })
    }
}

impl<T, I: Iterator> ResultUtil<T> for Result<T, itertools::ExactlyOneError<I>> {
    fn to_eyre(self) -> eyre::Result<T> {
        self.map_err(|e| eyre::eyre!(e.to_string()))
    }
}

pub trait Normal<T> {
    fn normal(self) -> eyre::Result<T>;
}

impl Normal<walrus::ir::Value> for walrus::ValType {
    fn normal(self) -> eyre::Result<walrus::ir::Value> {
        match self {
            walrus::ValType::I32 => Ok(walrus::ir::Value::I32(0)),
            walrus::ValType::I64 => Ok(walrus::ir::Value::I64(0)),
            walrus::ValType::F32 => Ok(walrus::ir::Value::F32(0.0)),
            walrus::ValType::F64 => Ok(walrus::ir::Value::F64(0.0)),
            walrus::ValType::V128 => Err(eyre::eyre!("V128 not supported")),
            ValType::Ref(_) => Err(eyre::eyre!("Ref not supported")),
        }
    }
}

pub const CORE_MODULE_ROOT: &str = "wasip1-vfs:host/virtual-file-system-wasip1-core";
pub const THREADS_MODULE_ROOT: &str = "wasip1-vfs:host/virtual-file-system-wasip1-threads-import";

pub trait WalrusFID<Marker>: Copy {
    fn get_fid(self, assist: &impl WalrusFIDAssister) -> eyre::Result<FunctionId>;
    fn find_fid(self, assist: &impl WalrusFIDAssister) -> Option<FunctionId>;
    fn as_str(self) -> String;
}

pub trait WalrusFIDAssister {
    fn get_fid_by_fid(&self, fid: FunctionId) -> eyre::Result<FunctionId>;
    fn find_fid_by_fid(&self, fid: FunctionId) -> Option<FunctionId>;
    fn get_fid_by_name(&self, name: &str) -> eyre::Result<FunctionId>;
    fn find_fid_by_name(&self, name: &str) -> Option<FunctionId>;
    fn get_fid_by_double_name(&self, module: &str, name: &str) -> eyre::Result<FunctionId>;
    fn find_fid_by_double_name(&self, module: &str, name: &str) -> Option<FunctionId>;
}

pub struct FunctionIdMarker;
pub struct StrMarker;
pub struct DoubleStrMarker;

impl<B: Borrow<FunctionId> + Copy> WalrusFID<FunctionIdMarker> for B {
    fn as_str(self) -> String {
        format!("{:?}", self.borrow())
    }

    fn get_fid(self, assist: &impl WalrusFIDAssister) -> eyre::Result<FunctionId> {
        let fid = *self.borrow();
        assist
            .get_fid_by_fid(fid)
            .wrap_err_with(|| eyre::eyre!("FunctionId {:?} not found in get_fid", fid))
    }

    fn find_fid(self, assist: &impl WalrusFIDAssister) -> Option<FunctionId> {
        let fid = *self.borrow();
        assist.find_fid_by_fid(fid)
    }
}

impl<S: AsRef<str> + Copy> WalrusFID<StrMarker> for S {
    fn as_str(self) -> String {
        self.as_ref().to_string()
    }

    fn get_fid(self, assist: &impl WalrusFIDAssister) -> eyre::Result<FunctionId> {
        let name = self.as_ref();
        assist
            .get_fid_by_name(name)
            .wrap_err_with(|| eyre::eyre!("Function name {name} not found in get_fid"))
    }

    fn find_fid(self, assist: &impl WalrusFIDAssister) -> Option<FunctionId> {
        let name = self.as_ref();
        assist.find_fid_by_name(name)
    }
}

impl<S1: AsRef<str> + Copy, S2: AsRef<str> + Copy> WalrusFID<DoubleStrMarker> for (S1, S2) {
    fn as_str(self) -> String {
        format!("{}.{}", self.0.as_ref(), self.1.as_ref())
    }

    fn get_fid(self, assist: &impl WalrusFIDAssister) -> eyre::Result<FunctionId> {
        let module = self.0.as_ref();
        let name = self.1.as_ref();
        assist
            .get_fid_by_double_name(module, name)
            .wrap_err_with(|| eyre::eyre!("Function name {module}.{name} not found in get_fid"))
    }

    fn find_fid(self, assist: &impl WalrusFIDAssister) -> Option<FunctionId> {
        let module = self.0.as_ref();
        let name = self.1.as_ref();
        assist.find_fid_by_double_name(module, name)
    }
}

impl WalrusFIDAssister for walrus::Module {
    fn get_fid_by_fid(&self, fid: FunctionId) -> eyre::Result<FunctionId> {
        if self.funcs.iter().any(|f| f.id() == fid) {
            Ok(fid)
        } else {
            eyre::bail!("FunctionId {:?} not found in get_fid_by_fid", fid);
        }
    }

    fn find_fid_by_fid(&self, fid: FunctionId) -> Option<FunctionId> {
        if self.funcs.iter().any(|f| f.id() == fid) {
            Some(fid)
        } else {
            None
        }
    }

    fn find_fid_by_name(&self, name: &str) -> Option<FunctionId> {
        if let Ok(id) = self.exports.get_fid_by_name(name) {
            Some(id)
        } else {
            self.imports.find_fid_by_name(name)
        }
    }

    fn get_fid_by_name(&self, name: &str) -> eyre::Result<FunctionId> {
        if let Ok(id) = self.exports.get_fid_by_name(name) {
            Ok(id)
        } else {
            self.imports.get_fid_by_name(name)
        }
    }

    fn get_fid_by_double_name(&self, module: &str, name: &str) -> eyre::Result<FunctionId> {
        self.imports.get_fid_by_double_name(module, name)
    }

    fn find_fid_by_double_name(&self, module: &str, name: &str) -> Option<FunctionId> {
        self.imports.find_fid_by_double_name(module, name)
    }
}

impl WalrusFIDAssister for walrus::ModuleImports {
    fn get_fid_by_fid(&self, fid: FunctionId) -> eyre::Result<FunctionId> {
        if self.iter().any(|im| match im.kind {
            walrus::ImportKind::Function(f) if f == fid => true,
            _ => false,
        }) {
            Ok(fid)
        } else {
            eyre::bail!("FunctionId {fid:?} not found in get_fid_by_fid");
        }
    }

    fn find_fid_by_fid(&self, fid: FunctionId) -> Option<FunctionId> {
        if self.iter().any(|im| match im.kind {
            walrus::ImportKind::Function(f) if f == fid => true,
            _ => false,
        }) {
            Some(fid)
        } else {
            None
        }
    }

    fn find_fid_by_name(&self, name: &str) -> Option<FunctionId> {
        self.iter()
            .filter_map(|im| match im.kind {
                walrus::ImportKind::Function(fid) if im.name == name => Some(fid),
                _ => None,
            })
            .exactly_one()
            .ok()
    }

    fn get_fid_by_name(&self, name: &str) -> eyre::Result<FunctionId> {
        self.iter()
            .filter_map(|im| match im.kind {
                walrus::ImportKind::Function(fid) if im.name == name => Some(fid),
                _ => None,
            })
            .exactly_one()
            .to_eyre()
            .wrap_err_with(|| {
                eyre::eyre!("Multiple or no function name {name} found in get_fid_by_name")
            })
    }

    fn get_fid_by_double_name(&self, module: &str, name: &str) -> eyre::Result<FunctionId> {
        self.iter()
            .filter_map(|im| match im.kind {
                walrus::ImportKind::Function(fid) if im.name == name && im.module == module => {
                    Some(fid)
                }
                _ => None,
            })
            .exactly_one()
            .to_eyre()
            .wrap_err_with(|| {
                eyre::eyre!("Function name {module}.{name} not found in get_fid_by_double_name")
            })
    }

    fn find_fid_by_double_name(&self, module: &str, name: &str) -> Option<FunctionId> {
        self.iter()
            .filter_map(|im| match im.kind {
                walrus::ImportKind::Function(fid) if im.name == name && im.module == module => {
                    Some(fid)
                }
                _ => None,
            })
            .exactly_one()
            .ok()
    }
}

impl WalrusFIDAssister for ModuleExports {
    fn get_fid_by_fid(&self, fid: FunctionId) -> eyre::Result<FunctionId> {
        if self.iter().any(|ex| match ex.item {
            walrus::ExportItem::Function(f) if f == fid => true,
            _ => false,
        }) {
            Ok(fid)
        } else {
            eyre::bail!("FunctionId {:?} not found in get_fid_by_fid", fid);
        }
    }

    fn find_fid_by_fid(&self, fid: FunctionId) -> Option<FunctionId> {
        if self.iter().any(|ex| match ex.item {
            walrus::ExportItem::Function(f) if f == fid => true,
            _ => false,
        }) {
            Some(fid)
        } else {
            None
        }
    }

    fn find_fid_by_name(&self, name: &str) -> Option<FunctionId> {
        self.iter().find_map(|ex| match ex.item {
            walrus::ExportItem::Function(fid) if ex.name == name => Some(fid),
            _ => None,
        })
    }

    fn get_fid_by_name(&self, name: &str) -> eyre::Result<FunctionId> {
        self.iter()
            .find_map(|ex| match ex.item {
                walrus::ExportItem::Function(fid) if ex.name == name => Some(fid),
                _ => None,
            })
            .wrap_err_with(|| eyre::eyre!("Function name {name} not found in get_fid_by_name"))
    }

    fn get_fid_by_double_name(&self, _: &str, _: &str) -> eyre::Result<FunctionId> {
        panic!("Module name is not stored in exports, cannot get by double name");
    }

    fn find_fid_by_double_name(&self, _: &str, _: &str) -> Option<FunctionId> {
        panic!("Module name is not stored in exports, cannot find by double name");
    }
}

// pub fn init_data_set(buff: &mut walrus::ModuleData, offset: u32, data: &[u8]) -> eyre::Result<()> {
//     let data_ids = buff.iter().map(|data| data.id()).collect::<Vec<_>>();

//     for id in data_ids {
//         let data = buff.get_mut(id);
//         if let walrus::DataKind::Active(walrus::ActiveData {
//             memory: _,
//             offset: walrus::ir::Value::I32(current_offset),
//             ..
//         }) = &data.kind
//         {
//             let current_offset = *current_offset as u32;
//             if current_offset <= offset && offset < current_offset + data.value.len() as u32 {
//                 let start = (offset - current_offset) as usize;
//                 let end = std::cmp::min(start + data.value.len(), start + data.len());
//                 data.value[start..end].copy_from_slice(&data[..(end - start)]);
//                 return Ok(());
//             }
//         }
//     }

//     Ok(())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_file_main_name() {
        let path = camino::Utf8Path::new("name.opt.adjusted.wasm");
        let file_name = path.get_file_main_name();
        assert_eq!(file_name.unwrap(), "name");
    }
}
