use std::{
    borrow::Borrow,
    path::{Path, PathBuf},
};

use eyre::Context as _;
use itertools::Itertools;
use walrus::{ir::InstrSeqId, *};

use crate::instrs::{InstrRead, InstrRewrite as _};

pub(crate) trait WalrusUtilImport {
    fn find_mut(&mut self, module: impl AsRef<str>, name: impl AsRef<str>) -> Option<&mut Import>;
    fn swap_import(
        &mut self,
        old_module: impl AsRef<str>,
        old_name: impl AsRef<str>,
        new_module: impl AsRef<str>,
        new_name: impl AsRef<str>,
    ) -> eyre::Result<()>
    where
        Self: Sized,
    {
        let old_module = old_module.as_ref();
        let old_name = old_name.as_ref();
        let new_module = new_module.as_ref();
        let new_name = new_name.as_ref();

        let old_import = self
            .find_mut(old_module, old_name)
            .ok_or_else(|| eyre::eyre!("Import {}.{} not found", old_module, old_name))?;

        old_import.module = "archived".to_string();

        self.find_mut(new_module, new_name).map(|import| {
            import.module = old_module.to_string();
            import.name = old_name.to_string();
        });

        let old_import = self
            .find_mut("archived", old_name)
            .ok_or_else(|| eyre::eyre!("Import archived.{} not found", old_name))?;

        old_import.module = new_module.to_string();
        old_import.name = new_name.to_string();

        Ok(())
    }
}

pub(crate) trait WalrusUtilFuncs {
    /// Find children flat functions
    fn find_children(&self, fid: impl Borrow<FunctionId>) -> eyre::Result<Vec<FunctionId>>;

    /// Find children flat functions with self
    fn find_children_with(&self, fid: impl Borrow<FunctionId>) -> eyre::Result<Vec<FunctionId>> {
        let fid = *fid.borrow();
        let mut children = self.find_children(fid)?;
        children.insert(0, fid);
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
}

#[allow(dead_code)]
pub(crate) trait WalrusUtilModule {
    /// connect function from import to export
    /// export will be removed
    /// and import will be replaced with the export function
    fn connect_func(
        &mut self,
        import_module: impl AsRef<str>,
        import_name: impl AsRef<str>,
        export_name: impl AsRef<str>,
    ) -> eyre::Result<()>;

    fn connect_func_without_remove(
        &mut self,
        import_module: impl AsRef<str>,
        import_name: impl AsRef<str>,
        export_name: impl AsRef<str>,
    ) -> eyre::Result<()>;

    fn connect_func_inner(
        &mut self,
        fid: impl Borrow<FunctionId>,
        export_id: impl Borrow<FunctionId>,
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
    fn get_target_memory_id(&mut self, name: impl AsRef<str>) -> eyre::Result<MemoryId>;

    fn create_memory_anchor(
        &mut self,
        name: impl AsRef<str>,
        memory_hint: Option<usize>,
    ) -> eyre::Result<()>;

    fn get_global_anchor(&mut self, name: impl AsRef<str>) -> eyre::Result<Vec<GlobalId>>;

    fn create_global_anchor(&mut self, name: impl AsRef<str>) -> eyre::Result<()>;

    /// Return all functions that call functions in this fid
    fn get_using_func(
        &self,
        fid: impl Borrow<FunctionId>,
    ) -> eyre::Result<Vec<(FunctionId, InstrSeqId, usize)>>;

    fn renew_id_on_table(
        &mut self,
        old_id: impl Borrow<FunctionId>,
        new_id: impl Borrow<FunctionId>,
    ) -> eyre::Result<()>
    where
        Self: Sized;

    fn fid_pos_on_table(&self, fid: impl Borrow<FunctionId>)
    -> eyre::Result<Vec<(TableId, usize)>>;

    fn renew_call_fn(
        &mut self,
        old_id: impl Borrow<FunctionId>,
        new_id: impl Borrow<FunctionId>,
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

    fn check_function_type(
        &self,
        before: impl Borrow<FunctionId>,
        after: impl Borrow<FunctionId>,
    ) -> eyre::Result<()>
    where
        Self: Sized;

    #[allow(dead_code)]
    fn debug_call_indirect(&mut self, id: impl Borrow<FunctionId>) -> eyre::Result<()>
    where
        Self: Sized;

    #[allow(dead_code)]
    fn gen_inspect<const N: usize>(
        &mut self,
        inspector: impl Borrow<FunctionId>,
        params: &[ValType],
        exclude: &[impl Borrow<FunctionId>],
        filter: impl FnMut(&ir::Instr) -> Option<[i32; N]>,
    ) -> eyre::Result<()>
    where
        Self: Sized;

    #[allow(dead_code)]
    fn gen_finalize<const N: usize>(
        &mut self,
        finalize: impl Borrow<FunctionId>,
        params: &[ValType],
        exclude: &[impl Borrow<FunctionId>],
        filter: impl FnMut(&ir::Instr) -> Option<[i32; N]>,
    ) -> eyre::Result<()>
    where
        Self: Sized;

    #[allow(dead_code)]
    fn gen_inspect_with_finalize<const N: usize>(
        &mut self,
        inspector: Option<impl Borrow<FunctionId>>,
        finalize: Option<impl Borrow<FunctionId>>,
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
}

impl WalrusUtilImport for ModuleImports {
    fn find_mut(&mut self, module: impl AsRef<str>, name: impl AsRef<str>) -> Option<&mut Import> {
        let import_id = self
            .iter()
            .find(|import| import.module == module.as_ref() && import.name == name.as_ref())?
            .id();

        Some(self.get_mut(import_id))
    }
}

impl WalrusUtilModule for walrus::Module {
    fn connect_func_inner(
        &mut self,
        fid: impl Borrow<FunctionId>,
        export_id: impl Borrow<FunctionId>,
        is_delete: bool,
    ) -> eyre::Result<()> {
        let fid = *fid.borrow();
        let export_id = *export_id.borrow();

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

    // only debug
    fn connect_func_without_remove(
        &mut self,
        import_module: impl AsRef<str>,
        import_name: impl AsRef<str>,
        export_name: impl AsRef<str>,
    ) -> eyre::Result<()> {
        let fid = self
            .imports
            .get_func(import_module, &import_name)
            .to_eyre()
            .wrap_err_with(|| eyre::eyre!("import {} not found", import_name.as_ref()))?;

        let export_id = self
            .exports
            .get_func(&export_name)
            .to_eyre()
            .wrap_err_with(|| eyre::eyre!("export {} not found", export_name.as_ref()))?;

        self.connect_func_inner(fid, export_id, false)
            .wrap_err("Failed to connect func")?;

        Ok(())
    }

    fn connect_func(
        &mut self,
        import_module: impl AsRef<str>,
        import_name: impl AsRef<str>,
        export_name: impl AsRef<str>,
    ) -> eyre::Result<()> {
        let fid = self
            .imports
            .get_func(import_module, &import_name)
            .to_eyre()
            .wrap_err_with(|| eyre::eyre!("import {} not found", import_name.as_ref()))?;

        let export_id = self
            .exports
            .get_func(&export_name)
            .to_eyre()
            .wrap_err_with(|| eyre::eyre!("export {} not found", export_name.as_ref()))?;

        self.connect_func_inner(fid, export_id, true)
            .wrap_err("Failed to connect func")?;

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

        Ok(builder.finish(vec![], &mut self.funcs))
    }

    /// if vfs, get vfs memory_id
    fn get_target_memory_id(&mut self, name: impl AsRef<str>) -> eyre::Result<MemoryId> {
        let anchor_name = format!("__wasip1_vfs_flag_{}_memory", name.as_ref());

        let anchor_func_id = self
            .exports
            .get_func(&anchor_name)
            .to_eyre()
            .wrap_err_with(|| eyre::eyre!("anchor {} not found", anchor_name))?;

        self.exports
            .remove(&anchor_name)
            .to_eyre()
            .wrap_err_with(|| eyre::eyre!("Failed to remove anchor export"))?;

        let anchor_body = &self.funcs.get(anchor_func_id).kind;
        if let FunctionKind::Local(local_func) = anchor_body {
            let entry_id = local_func.entry_block();
            let func_body = local_func.block(entry_id);
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
                    Some(_) => Err(eyre::eyre!(
                        "Anchor access double memory, cannot determine memory id"
                    )),
                })?
                .ok_or_else(|| eyre::eyre!("Memory not found"));

            memory_id
        } else {
            Err(eyre::eyre!(
                "anchor (local function) {} not found",
                anchor_name
            ))
        }
    }

    fn create_memory_anchor(
        &mut self,
        name: impl AsRef<str>,
        memory_hint: Option<usize>,
    ) -> eyre::Result<()> {
        let name = name.as_ref();

        let memories = self
            .memories
            .iter()
            .map(|memory| memory.id())
            .collect::<Vec<_>>();

        if memories.is_empty() {
            return Err(eyre::eyre!("No memories found"));
        }

        // After calling environ_sizes_get,
        // identify the memory using the memory referenced
        // by the code trying to read the pointer
        let memory_id = if memories.len() > 1 && memory_hint.is_none() {
            let gen_memory_id = || -> eyre::Result<MemoryId> {
                // environ_sizes_get
                let import_id = self
                    .imports
                    .get_func("wasi_snapshot_preview1", "environ_sizes_get")
                    .to_eyre()
                    .wrap_err_with(|| eyre::eyre!("Failed to get environ_sizes_get"))?;

                let using_funcs = self.get_using_func(import_id)?;

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
        let anchor_name = format!("__wasip1_vfs_flag_{}_global", name.as_ref());

        let anchor_func_id = self
            .exports
            .get_func(&anchor_name)
            .to_eyre()
            .wrap_err_with(|| eyre::eyre!("anchor {} not found", anchor_name))?;

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
                "anchor (local function) {} not found",
                anchor_name
            ))
        }
    }

    fn create_global_anchor(&mut self, name: impl AsRef<str>) -> eyre::Result<()> {
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
            .add(&format!("__wasip1_vfs_flag_{}_global", name.as_ref()), id);

        Ok(())
    }

    fn get_using_func(
        &self,
        fid: impl Borrow<FunctionId>,
    ) -> eyre::Result<Vec<(FunctionId, InstrSeqId, usize)>> {
        let fid = *fid.borrow();

        Ok(self
            .funcs
            .iter_local()
            .flat_map(|(id, func)| {
                func.read(|instr, place| {
                    if let walrus::ir::Instr::Call(walrus::ir::Call { func }) = instr {
                        if fid == *func {
                            return Some((id, place));
                        }
                    }
                    None
                })
                .unwrap()
                .into_iter()
                .filter_map(|v| v)
                .map(|(a, (b, c))| (a, c, b))
            })
            .collect::<Vec<_>>())
    }

    fn renew_id_on_table(
        &mut self,
        old_id: impl Borrow<FunctionId>,
        new_id: impl Borrow<FunctionId>,
    ) -> eyre::Result<()>
    where
        Self: Sized,
    {
        let old_id = *old_id.borrow();
        let new_id = *new_id.borrow();

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

    fn fid_pos_on_table(
        &self,
        fid: impl Borrow<FunctionId>,
    ) -> eyre::Result<Vec<(TableId, usize)>> {
        let fid = *fid.borrow();

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

    // this is broken
    // fn renew_call_fn_in_the_fn(
    //     &mut self,
    //     old_id: impl Borrow<FunctionId>,
    //     new_id: impl Borrow<FunctionId>,
    //     fn_id: impl Borrow<FunctionId>,
    // ) -> eyre::Result<()>
    // where
    //     Self: Sized,
    // {
    //     use walrus::ir::*;
    //     let f_ty_id = self.funcs.get(old_id).ty();
    //     let f_ty_id_params = self.types.get(f_ty_id).params().to_vec();
    //     let f_ty_id_results = self.types.get(f_ty_id).results().to_vec();

    //     // check new_id type and old_id type
    //     self.check_function_type(old_id, new_id)
    //         .wrap_err("Function types do not match on renew call fn that nesting this function")?;

    //     let fid_pos_on_table = self
    //         .fid_pos_on_table(old_id)
    //         .wrap_err("Failed to get fid pos on table")?;

    //     let using_tables = self
    //         .funcs
    //         .flat_read(
    //             |instr, _| {
    //                 if let Instr::CallIndirect(call) = instr {
    //                     if fid_pos_on_table.iter().any(|(tid, _)| *tid == call.table) {
    //                         let ty = self.types.get(call.ty);
    //                         if f_ty_id_params == ty.params() && f_ty_id_results == ty.results() {
    //                             return Some(call.table);
    //                         }
    //                     }
    //                 }
    //                 None
    //             },
    //             fn_id,
    //         )
    //         .wrap_err("Failed to read using tables")?
    //         .into_iter()
    //         .filter_map(|x| x)
    //         .collect::<std::collections::HashSet<_>>()
    //         .into_iter()
    //         .filter_map(|table| {
    //             let fid = fid_pos_on_table
    //                 .iter()
    //                 .filter(|(tid, _)| *tid == table)
    //                 .map(|(_, pos)| *pos as i32)
    //                 .collect::<Vec<_>>();
    //             if fid.is_empty() {
    //                 return None;
    //             }
    //             if fid.len() > 1 {
    //                 log::warn!("Multiple fid pos found on table, why? using the first one");
    //             }
    //             let fid = fid[0];
    //             Some((table, fid))
    //         })
    //         .map(|(table, fid)| {
    //             use walrus::*;

    //             let params_ty = core::iter::once(ValType::I32)
    //                 .chain(f_ty_id_params.clone())
    //                 .collect::<Vec<_>>();
    //             let results_ty = f_ty_id_results.clone();

    //             let new_id = self.gen_new_function(&params_ty, &results_ty, |func, args| {
    //                 func.func_body()
    //                     .local_get(args[0])
    //                     .i32_const(fid)
    //                     .binop(BinaryOp::I32Eq)
    //                     .if_else(
    //                         ValType::I32,
    //                         |cons| {
    //                             for arg in args.iter().skip(1) {
    //                                 cons.local_get(*arg);
    //                             }
    //                             cons.call(new_id).return_();
    //                         },
    //                         |els| {
    //                             for arg in args.iter().skip(1) {
    //                                 els.local_get(*arg);
    //                             }
    //                             els.call_indirect(f_ty_id, table);
    //                         },
    //                     );

    //                 Ok(())
    //             })?;

    //             Ok((table, new_id))
    //         })
    //         .collect::<eyre::Result<std::collections::HashMap<_, _>>>()?;

    //     self.funcs
    //         .flat_rewrite(
    //             |instr, _| {
    //                 if let Some(call) = instr.call_mut() {
    //                     if call.func == old_id {
    //                         call.func = new_id;
    //                     }
    //                 }
    //                 if let Some(call) = instr.call_indirect_mut() {
    //                     let ty = self.types.get(call.ty);
    //                     if f_ty_id_params == ty.params() && f_ty_id_results == ty.results() {
    //                         if let Some(new_id) = using_tables.get(&call.table).cloned() {
    //                             log::info!(
    //                                 "Rewriting call_indirect to direct call in function {:?}. Old: {:?}, New: {:?}",
    //                                 fn_id,
    //                                 call,
    //                                 new_id
    //                             );
    //                             *instr = Instr::Call(Call { func: new_id });
    //                         }
    //                     }
    //                 }
    //             },
    //             fn_id,
    //         )
    //         .wrap_err("Failed to renew function")?;

    //     Ok(())
    // }

    fn renew_call_fn(
        &mut self,
        old_id: impl Borrow<FunctionId>,
        new_id: impl Borrow<FunctionId>,
    ) -> eyre::Result<()>
    where
        Self: Sized,
    {
        let old_id = *old_id.borrow();
        let new_id = *new_id.borrow();

        for (id, _, _) in self
            .get_using_func(old_id)
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

    fn check_function_type(
        &self,
        before: impl Borrow<FunctionId>,
        after: impl Borrow<FunctionId>,
    ) -> eyre::Result<()>
    where
        Self: Sized,
    {
        let before = *before.borrow();
        let after = *after.borrow();

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
    fn debug_call_indirect(&mut self, id: impl Borrow<FunctionId>) -> eyre::Result<()>
    where
        Self: Sized,
    {
        let id = *id.borrow();

        // check id type
        if self.types.get(self.funcs.get(id).ty()).params() != [ValType::I32, ValType::I32]
            || self.types.get(self.funcs.get(id).ty()).results() != []
        {
            eyre::bail!("Function type must be (i32, i32) -> ()");
        }

        let ids = self.funcs.find_children_with(id)?;

        let tables = self
            .funcs
            .iter_local()
            .filter(|(fid, _)| !ids.contains(fid))
            .map(|(fid, fn_)| {
                fn_.read(|instr, pos| {
                    if let walrus::ir::Instr::CallIndirect(call) = instr {
                        Some((call.table, (fid, pos)))
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

    fn gen_inspect<const N: usize>(
        &mut self,
        inspector: impl Borrow<FunctionId>,
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

    fn gen_finalize<const N: usize>(
        &mut self,
        finalize: impl Borrow<FunctionId>,
        results: &[ValType],
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
            results,
            exclude,
            filter,
        )
    }

    fn gen_inspect_with_finalize<const N: usize>(
        &mut self,
        inspector: Option<impl Borrow<FunctionId>>,
        finalize: Option<impl Borrow<FunctionId>>,
        params: &[ValType],
        results: &[ValType],
        exclude: &[impl Borrow<FunctionId>],
        mut filter: impl FnMut(&ir::Instr) -> Option<[i32; N]>,
    ) -> eyre::Result<()>
    where
        Self: Sized,
    {
        let inspector: Option<FunctionId> = inspector.map(|id| *id.borrow());
        let finalize: Option<FunctionId> = finalize.map(|id| *id.borrow());

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
            .map(|f| self.funcs.find_children_with(f))
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
            .map(|fid| self.funcs.find_children_with(fid))
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
}

impl WalrusUtilFuncs for walrus::ModuleFunctions {
    fn find_children(&self, fid: impl Borrow<FunctionId>) -> eyre::Result<Vec<FunctionId>> {
        let fid = *fid.borrow();

        let mut children = vec![];
        let mut stack = vec![fid];
        while let Some(fid) = stack.pop() {
            match &self.get(fid).kind {
                FunctionKind::Local(imported_function) => {
                    imported_function.read(|instr, _place| {
                        if let walrus::ir::Instr::Call(walrus::ir::Call { func }) = instr {
                            if !children.contains(func) {
                                children.push(*func);
                                stack.push(*func);
                            }
                        }
                    })?;
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
    ) -> eyre::Result<Vec<T>>
    where
        Self: Sized,
    {
        self.find_children_with(fid)?
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
        self.find_children_with(fid)?
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
