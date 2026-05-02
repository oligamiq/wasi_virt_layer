use std::{
    borrow::Borrow,
    collections::HashMap,
    fmt::Debug,
    path::{Path, PathBuf},
    str::FromStr,
    sync::atomic::AtomicUsize,
};

use compact_str::CompactString;
use eyre::{Context as _, ContextCompat as _};
use itertools::Itertools;
use walrus::{ir::InstrSeqId, *};

use crate::{
    args::TargetMemoryType,
    instrs::{InstrRead, InstrRewrite as _},
    unique_name::UniqueNameMarker,
};

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

    fn erase<A>(&mut self, as_fn: impl WalrusFID<A>) -> eyre::Result<()>;
}

pub(crate) trait WalrusUtilExport: Debug {
    /// As it deletes based on the fid, it may involve functions that export the same function.
    fn erase<A>(&mut self, as_fn: impl WalrusFID<A>) -> eyre::Result<()>;
    /// As it deletes based on the fid, it may involve functions that export the same function.
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
    ) -> eyre::Result<()>;

    fn connect_func_alt_with_remove_export<A>(
        &mut self,
        import: impl WalrusFID<A>,
        export: impl AsRef<str>,
        is_debug: bool,
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
    fn get_memory_anchor(&mut self, name: impl AsRef<str>, remove: bool) -> eyre::Result<MemoryId> {
        self.get_memory_anchor_with_info::<String>(name, remove)
            .map(|(mem_id, _)| mem_id)
    }

    fn get_memory_anchor_with_info<T: ToString + std::str::FromStr>(
        &mut self,
        name: impl AsRef<str>,
        remove: bool,
    ) -> eyre::Result<(MemoryId, Option<T>)>
    where
        <T as std::str::FromStr>::Err: Debug;

    fn find_used_memory_id(&self, memory_hint: Option<usize>) -> eyre::Result<MemoryId>;

    fn flatten_tables(&mut self) -> eyre::Result<()>;

    /// create memory anchor function
    fn create_memory_anchor(
        &mut self,
        name: impl AsRef<str>,
        memory_id: MemoryId,
    ) -> eyre::Result<MemoryId> {
        self.create_memory_anchor_with_info(name, memory_id, None::<String>)?;

        Ok(memory_id)
    }

    fn create_memory_anchor_with_info(
        &mut self,
        name: impl AsRef<str>,
        memory_id: MemoryId,
        with_info: Option<impl ToString + FromStr>,
    ) -> eyre::Result<()>;

    fn get_memory_type(&mut self, is_remove: bool) -> eyre::Result<TargetMemoryType>;

    fn get_global_anchor(&mut self, name: impl AsRef<str>) -> eyre::Result<Box<[GlobalId]>>;

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

    fn fid_pos_on_table<A>(
        &self,
        as_fn: impl WalrusFID<A>,
    ) -> eyre::Result<Box<[(TableId, usize)]>>;

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

    /// This method copies functions by copying the functions called internally.
    /// It is used to rewrite the internal instructions of functions called under specific conditions.
    /// Note: that calls_indirect may throw errors.
    /// If you include yourself in the exclude list, only the function being called will be copied.
    /// Note: that even if a function not included in the exclude list is called, that call will not be updated.
    fn nested_copy_func<A>(
        &mut self,
        from: impl WalrusFID<A>,
        exclude: &[impl Borrow<FunctionId>],
        allow_import_func: bool,
        allow_call_indirect: bool,
    ) -> eyre::Result<walrus::FunctionId>
    where
        Self: Sized;

    fn renew_export<A, B>(
        &mut self,
        old: impl WalrusFID<A>,
        new: impl WalrusFID<B>,
    ) -> eyre::Result<()>
    where
        Self: Sized;

    fn save_info(
        &mut self,
        salt: impl AsRef<str>,
        info: impl ToString + FromStr,
    ) -> eyre::Result<()>;

    fn load_info<T: ToString + FromStr>(&mut self, salt: impl AsRef<str>) -> eyre::Result<T>
    where
        <T as std::str::FromStr>::Err: Debug;
}

impl WalrusUtilImport for ModuleImports {
    fn find_mut<A>(&mut self, as_fn: impl WalrusFID<A>) -> eyre::Result<&mut Import> {
        let fid = as_fn.get_fid(self)?;

        let import_id = self.get_imported_func(fid).unwrap().id();

        Ok(self.get_mut(import_id))
    }

    fn erase<A>(&mut self, as_fn: impl WalrusFID<A>) -> eyre::Result<()> {
        let fid = as_fn.get_fid(self)?;
        let import_id = self
            .iter()
            .find(|f| {
                if let walrus::ImportKind::Function(f) = f.kind {
                    f == fid
                } else {
                    false
                }
            })
            .map(|f| f.id())
            .ok_or_else(|| eyre::eyre!("Import not found: {}", as_fn.as_str()))?;

        self.delete(import_id);

        Ok(())
    }
}

impl WalrusUtilExport for ModuleExports {
    fn erase<A>(&mut self, as_fn: impl WalrusFID<A>) -> eyre::Result<()> {
        let fid = as_fn.get_fid(self)?;

        let export_id = self
            .iter()
            .filter(|f| matches!(f.item, walrus::ExportItem::Function(f) if f == fid))
            .map(|f| f.id())
            .exactly_one()
            .to_eyre()
            .wrap_err("Expected exactly one export for function")?;

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
    fn get_memory_anchor_with_info<T: ToString + std::str::FromStr>(
        &mut self,
        name: impl AsRef<str>,
        remove: bool,
    ) -> eyre::Result<(MemoryId, Option<T>)>
    where
        <T as std::str::FromStr>::Err: Debug,
    {
        let name = name.as_ref();

        let anchor_name = self
            .exports
            .iter()
            .find_map(|export| {
                if matches!(export.item, walrus::ExportItem::Function(_))
                    && export
                        .name
                        .starts_with(&format!("__wasip1_vfs_flag_{name}_memory"))
                {
                    Some(export.name.clone())
                } else {
                    None
                }
            })
            .ok_or_else(|| eyre::eyre!("Memory anchor function not found"))?;
        let anchor_func_id = anchor_name.get_fid(&self.exports).unwrap();

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

        let anchor_name_ex = anchor_name
            .strip_prefix(&format!("__wasip1_vfs_flag_{name}_memory"))
            .unwrap();
        let with_info = if anchor_name_ex.starts_with("_with_") {
            Some(
                anchor_name_ex
                    .strip_prefix("_with_")
                    .unwrap()
                    .parse::<T>()
                    .map_err(|e| eyre::eyre!("{e:?}"))
                    .wrap_err_with(|| {
                        eyre::eyre!("Failed to parse memory anchor info: {anchor_name_ex}")
                    })?,
            )
        } else {
            None
        };

        Ok((memory_id, with_info))
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
                        std::sync::Arc::new(std::sync::Mutex::new(Option::<Vec<u64>>::None));
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
                                    Ok(*arg as u64)
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

    fn create_memory_anchor_with_info(
        &mut self,
        name: impl AsRef<str>,
        memory_id: MemoryId,
        with_info: Option<impl ToString + FromStr>,
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

        let ex_name = if let Some(with_info) = with_info {
            format!(
                "__wasip1_vfs_flag_{name}_memory_with_{}",
                with_info.to_string()
            )
        } else {
            format!("__wasip1_vfs_flag_{name}_memory")
        };
        self.exports.add(&ex_name, id);

        Ok(())
    }

    fn get_global_anchor(&mut self, name: impl AsRef<str>) -> eyre::Result<Box<[GlobalId]>> {
        let name = name.as_ref();
        let anchor_name = format!("__wasip1_vfs_flag_{name}_global");

        let anchor_func_id = anchor_name.get_fid(&self.exports)?;

        self.exports.erase(anchor_func_id)?;

        let anchor_body = &self.funcs.get(anchor_func_id).kind;
        if let FunctionKind::Local(local_func) = anchor_body {
            let entry_id = local_func.entry_block();
            let func_body = local_func.block(entry_id);
            let global_ids = func_body
                .iter()
                .map(|(block, _)| block)
                .filter_map(|block| match block {
                    ir::Instr::GlobalSet(ir::GlobalSet { global, .. })
                    | ir::Instr::GlobalGet(ir::GlobalGet { global, .. }) => Some(*global),
                    _ => None,
                })
                .collect::<Box<_>>();

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
            .map(|global| (global.id(), global.ty, global.mutable))
            .collect::<Vec<_>>();

        let results = global_ids
            .iter()
            .filter(|(_, _, mutable)| !*mutable)
            .map(|(_, ty, _)| ty)
            .cloned()
            .collect::<Vec<_>>();

        let id = self.add_func(&[], &results, |builder, _| {
            let mut func_body = builder.func_body();

            for (id, ty, mutable) in global_ids.iter() {
                if *mutable {
                    func_body
                        .const_(
                            ty.normal()
                                .wrap_err_with(|| eyre::eyre!("Failed to get global type"))?,
                        )
                        .global_set(*id);
                } else {
                    func_body.global_get(*id);
                }
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

    fn fid_pos_on_table<A>(&self, fid: impl WalrusFID<A>) -> eyre::Result<Box<[(TableId, usize)]>> {
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
        Ok(positions.into_boxed_slice())
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
    ) -> eyre::Result<()> {
        let err_msg = || {
            format!(
                "Function types do not match on connect func alt: {} -> {}",
                import.as_str(),
                export.as_str()
            )
        };
        let export = export
            .get_fid(&self.exports)
            .wrap_err("Export function not found")
            .wrap_err_with(err_msg)?;

        self.renew_call_fn(import, export)
            .wrap_err("Failed to renew call function")
            .wrap_err_with(err_msg)?;

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

        let fids = self
            .funcs
            .find_children_with(from, allow_call_indirect)
            .wrap_err("Failed to find children functions")?;

        for fid in fids {
            if exclude.contains(&fid) {
                fid_map.insert(fid, fid);
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
            // If included in the exclude list, it is normally ignored; however, in its own case, it is rewritten.
            if (old_fid == new_fid || exclude.contains(new_fid)) && *new_fid != from {
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
                            if let Some(new_func) = fid_map.get(func) {
                                *func = *new_func;
                            }
                        }
                        Instr::CallIndirect(call) if !allow_call_indirect => {
                            eyre::bail!("Call indirect found: {call:?} in nested copy");
                        }
                        Instr::ReturnCallIndirect(call) if !allow_call_indirect => {
                            eyre::bail!("Return call indirect found: {call:?} in nested copy");
                        }
                        _ => {}
                    }
                    Ok(())
                })
                .wrap_err("Failed to renew function call")?;
        }

        Ok(fid_map[&from])
    }

    fn renew_export<A, B>(
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
            .wrap_err("Function types do not match on renew export")?;

        self.exports
            .iter_mut()
            .filter(|export| matches!(export.item, walrus::ExportItem::Function(f) if f == old_id))
            .for_each(|export| {
                export.item = walrus::ExportItem::Function(new_id);
            });

        Ok(())
    }

    fn get_memory_type(&mut self, is_remove: bool) -> eyre::Result<TargetMemoryType> {
        let (target_memory_type, eid) = self
            .exports
            .iter()
            .find(|e| e.name == "__wasip1_vfs_flag_vfs_multi_memory")
            .map(|e| Ok((TargetMemoryType::Multi, e.id())))
            .unwrap_or(
                self.exports
                    .iter()
                    .find(|e| e.name == "__wasip1_vfs_flag_vfs_single_memory")
                    .map(|e| Ok((TargetMemoryType::Single, e.id())))
                    .unwrap_or(Err(eyre::eyre!("No target memory type found"))),
            )?;

        if is_remove {
            self.exports.delete(eid);
        }

        Ok(target_memory_type)
    }

    fn save_info(
        &mut self,
        key: impl AsRef<str>,
        info: impl ToString + FromStr,
    ) -> eyre::Result<()> {
        let blank_fn = self.add_func(&[], &[], |builder, _| {
            builder.func_body().unreachable();
            Ok(())
        })?;

        let name = format!(
            "__wasip1_vfs_flag_info_{}_{}",
            key.as_ref(),
            info.to_string()
        );

        self.exports.add(&name, blank_fn);

        Ok(())
    }

    fn load_info<T: ToString + FromStr>(&mut self, key: impl AsRef<str>) -> eyre::Result<T>
    where
        <T as std::str::FromStr>::Err: Debug,
    {
        let name = format!("__wasip1_vfs_flag_info_{}_", key.as_ref());
        let export = self
            .exports
            .iter()
            .find(|e| e.name.starts_with(&name))
            .ok_or_else(|| eyre::eyre!("No info found"))?;
        let info_str = &export.name[name.len()..];
        let info = info_str
            .parse::<T>()
            .map_err(|e| eyre::eyre!("Failed to parse info: {:?}", e))?;

        self.exports.delete(export.id());

        Ok(info)
    }

    fn connect_func_alt_with_remove_export<A>(
        &mut self,
        import: impl WalrusFID<A>,
        export: impl AsRef<str>,
        is_debug: bool,
    ) -> eyre::Result<()> {
        let export = export.as_ref();
        self.connect_func_alt(import, export)?;
        if is_debug {
            self.exports
                .iter_mut()
                .find(|e| e.name == export)
                .unwrap()
                .name = format!("_____debug_left_{export}");
        } else {
            self.exports.remove(export).unwrap();
        }

        Ok(())
    }

    fn flatten_tables(&mut self) -> eyre::Result<()> {
        let table_ids: Vec<_> = self.tables.iter().map(|t| t.id()).collect();
        if table_ids.len() <= 1 {
            return Ok(());
        }

        log::info!("Flattening {} tables into one...", table_ids.len());

        let target_table_id = table_ids[0];
        let mut current_offset = self.tables.get(target_table_id).initial;

        let mut table_offsets = std::collections::HashMap::new();
        table_offsets.insert(target_table_id, 0);

        for &table_id in &table_ids[1..] {
            let table = self.tables.get(table_id);
            let table_size = table.initial;
            let offset = current_offset;
            table_offsets.insert(table_id, offset);

            current_offset += table_size;
        }

        // 1. Move all element segments to target table
        let elem_ids: Vec<_> = self.elements.iter().map(|e| e.id()).collect();
        for eid in elem_ids {
            let elem = self.elements.get_mut(eid);
            match &mut elem.kind {
                walrus::ElementKind::Active { table, offset } => {
                    if let Some(&off) = table_offsets.get(table) {
                        if off > 0 {
                            *table = target_table_id;
                            // Adjust the offset expression. 
                            // Element segments typically use a single i32.const instruction.
                            match offset {
                                walrus::ConstExpr::Value(walrus::ir::Value::I32(val)) => {
                                    *val += off as i32;
                                }
                                _ => eyre::bail!("Unsupported element offset expression type: only i32.const is supported for table flattening"),
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // 2. Update instructions in all functions
        for func in self.funcs.iter_mut() {
            if let walrus::FunctionKind::Local(local) = &mut func.kind {
                let mut stack = vec![local.entry_block()];
                let mut visited = std::collections::HashSet::new();
                
                while let Some(seq_id) = stack.pop() {
                    if !visited.insert(seq_id) {
                        continue;
                    }

                    // Collect next sequences to visit
                    {
                        let seq = local.block(seq_id);
                        for (instr, _) in &seq.instrs {
                            match instr {
                                walrus::ir::Instr::Block(b) => stack.push(b.seq),
                                walrus::ir::Instr::Loop(l) => stack.push(l.seq),
                                walrus::ir::Instr::IfElse(if_) => {
                                    stack.push(if_.consequent);
                                    stack.push(if_.alternative);
                                }
                                _ => {}
                            }
                        }
                    }

                    // Now modify instructions in the current block
                    let instrs = &mut local.block_mut(seq_id).instrs;
                    let mut i = 0;
                    while i < instrs.len() {
                        let mut injected = Vec::new();
                        let loc = instrs[i].1;

                        match &mut instrs[i].0 {
                            walrus::ir::Instr::CallIndirect(call) => {
                                if let Some(&off) = table_offsets.get(&call.table) {
                                    if off > 0 {
                                        call.table = target_table_id;
                                        injected.push((walrus::ir::Instr::Const(walrus::ir::Const { value: walrus::ir::Value::I32(off as i32) }), loc));
                                        injected.push((walrus::ir::Instr::Binop(walrus::ir::Binop { op: walrus::ir::BinaryOp::I32Add }), loc));
                                    }
                                }
                            }
                            walrus::ir::Instr::TableGet(table) => {
                                if let Some(&off) = table_offsets.get(&table.table) {
                                    if off > 0 {
                                        table.table = target_table_id;
                                        injected.push((walrus::ir::Instr::Const(walrus::ir::Const { value: walrus::ir::Value::I32(off as i32) }), loc));
                                        injected.push((walrus::ir::Instr::Binop(walrus::ir::Binop { op: walrus::ir::BinaryOp::I32Add }), loc));
                                    }
                                }
                            }
                            _ => {}
                        }

                        if !injected.is_empty() {
                            instrs.splice(i..i, injected);
                            i += 2; 
                        }
                        i += 1;
                    }
                }
            }
        }

        // 3. Delete other tables
        for &table_id in &table_ids[1..] {
            self.tables.delete(table_id);
        }

        // 4. Update target table size
        let target_table = self.tables.get_mut(target_table_id);
        target_table.initial = current_offset;
        if let Some(ref mut max) = target_table.maximum {
            *max = std::cmp::max(*max, current_offset);
        }

        Ok(())
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
                        .read(|instr, _| {
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
                                    eyre::bail!("Call indirect found: {call:?} in find children");
                                }
                                Instr::ReturnCallIndirect(call) if !allow_call_indirect => {
                                    eyre::bail!(
                                        "Return call indirect found: {call:?} in find children"
                                    );
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

/// Extension trait for Camino paths to extract the main module file name.
pub trait CaminoUtilModule {
    /// Gets the base compact string name without extension and specific postfixes.
    fn get_file_main_name(&self) -> Option<CompactString>;
}

impl CaminoUtilModule for camino::Utf8Path {
    fn get_file_main_name(&self) -> Option<CompactString> {
        let binding = self.file_name().unwrap().split(".").collect::<Vec<_>>();
        let file_name_poss = binding.iter().rev();
        let mut file_name = None;
        for name in file_name_poss {
            if *name == "opt"
                || *name == "adjusted"
                || *name == "wasm"
                || *name == "core"
                || *name == "component"
            {
                continue;
            }
            file_name = Some(name);
            break;
        }
        let file_name = file_name.map(ToOwned::to_owned).or_else(|| {
            self.file_name()
                .unwrap()
                .split(".")
                .next()
                .as_ref()
                .cloned()
        });

        file_name.map(CompactString::from)
    }
}

impl CaminoUtilModule for PathBuf {
    fn get_file_main_name(&self) -> Option<CompactString> {
        camino::Utf8Path::new(self.to_str().unwrap()).get_file_main_name()
    }
}

impl CaminoUtilModule for Path {
    fn get_file_main_name(&self) -> Option<CompactString> {
        camino::Utf8Path::new(self.to_str().unwrap()).get_file_main_name()
    }
}

/// Utility trait for converting generic or `anyhow` results into `eyre::Result`.
pub trait ResultUtil<T> {
    /// Converts the result into an `eyre::Result`.
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

/// Normalization trait for value types (e.g., getting a zeroed-out value).
pub trait Normal<T> {
    /// Returns the generalized normal or default representation of a type.
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

/// Provides unified Function ID resolution for different markers (like tuples of module/name).
pub trait WalrusFID<Marker>: Copy {
    /// Resolves and retrieves the FunctionId via an assisting context, erroring if not found.
    fn get_fid(self, assist: &impl WalrusFIDAssister) -> eyre::Result<FunctionId>;
    /// Resolves and retrieves the FunctionId via an assisting context optionally.
    fn find_fid(self, assist: &impl WalrusFIDAssister) -> Option<FunctionId>;
    /// Formats the identity marker into a standard string representation.
    fn as_str(self) -> String;
}

/// Provides the actual lookup operations for FID resolution.
pub trait WalrusFIDAssister {
    /// Gets a function ID by its existing ID, failing if it does not exist.
    fn get_fid_by_fid(&self, fid: FunctionId) -> eyre::Result<FunctionId>;
    /// Attempts to find a function ID by its existing ID.
    fn find_fid_by_fid(&self, fid: FunctionId) -> Option<FunctionId>;
    /// Gets a function ID by its name, failing if it does not exist.
    fn get_fid_by_name(&self, name: &str) -> eyre::Result<FunctionId>;
    /// Attempts to find a function ID by its name.
    fn find_fid_by_name(&self, name: &str) -> Option<FunctionId>;
    /// Gets a function ID by its module and name, failing if it does not exist.
    fn get_fid_by_double_name(&self, module: &str, name: &str) -> eyre::Result<FunctionId>;
    /// Attempts to find a function ID by its module and name.
    fn find_fid_by_double_name(&self, module: &str, name: &str) -> Option<FunctionId>;
}

/// Marker for looking up FIDs by `FunctionId` values.
pub struct FunctionIdMarker;
/// Marker for looking up FIDs by string names.
pub struct StrMarker;
/// Marker for looking up FIDs using `UniqueName` constants.
pub struct UniqueMarker;
/// Marker for looking up FIDs by a module-name and item-name tuple.
pub struct DoubleStrMarker;
/// Marker for looking up FIDs using a string module-name and `UniqueName` item.
pub struct StrAndUniqueNameMarker;

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

impl<U: UniqueNameMarker> WalrusFID<UniqueMarker> for U {
    fn as_str(self) -> String {
        self.to_string()
    }

    fn get_fid(self, assist: &impl WalrusFIDAssister) -> eyre::Result<FunctionId> {
        let name = self.to_string();
        assist
            .get_fid_by_name(&name)
            .wrap_err_with(|| eyre::eyre!("Function name {name} not found in get_fid"))
    }

    fn find_fid(self, assist: &impl WalrusFIDAssister) -> Option<FunctionId> {
        let name = self.to_string();
        assist.find_fid_by_name(&name)
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

impl<S: AsRef<str> + Copy, U: UniqueNameMarker> WalrusFID<StrAndUniqueNameMarker> for (S, U) {
    fn as_str(self) -> String {
        format!("{}.{}", self.0.as_ref(), self.1.to_string())
    }

    fn get_fid(self, assist: &impl WalrusFIDAssister) -> eyre::Result<FunctionId> {
        let module = self.0.as_ref();
        let name = self.1.to_string();
        assist
            .get_fid_by_double_name(module, &name)
            .wrap_err_with(|| eyre::eyre!("Function name {module}.{name} not found in get_fid"))
    }

    fn find_fid(self, assist: &impl WalrusFIDAssister) -> Option<FunctionId> {
        let module = self.0.as_ref();
        let name = self.1.to_string();
        assist.find_fid_by_double_name(module, &name)
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

/// A container holding global statics for tracking Wasm Names.
#[derive(Debug)]
pub struct WasmNameHolder(&'static [compact_str::CompactString], &'static AtomicUsize);

impl WasmNameHolder {
    /// Creates a new `WasmNameHolder`, leaking the provided names into static memory.
    pub fn new(strings: Box<[compact_str::CompactString]>) -> Self {
        let count = Box::leak(Box::new(AtomicUsize::new(0)));

        let strings = Box::leak(strings);
        WasmNameHolder(strings, count)
    }

    /// Returns an iterator over the underlying tracked `WasmName` items.
    pub fn iter(&self) -> impl Iterator<Item = WasmName> {
        self.0.iter().map(|s| WasmName::new(s.as_str(), self.1))
    }
}

impl Drop for WasmNameHolder {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.0 as *const _ as *mut [compact_str::CompactString]);
            let count = self.1.load(std::sync::atomic::Ordering::SeqCst);
            let _ = Box::from_raw(self.1 as *const _ as *mut AtomicUsize);
            if count != 0 {
                panic!(
                    "WasmNameHolder dropped while there are still {count} WasmName instances alive"
                );
            }
        }
    }
}

/// Context-aware wrapper for a copied string slice with lifecycle tracking.
pub struct WasmName(&'static str, &'static AtomicUsize);
impl WasmName {
    /// Creates a new tracked `WasmName`.
    pub fn new(s: &'static str, counter: &'static AtomicUsize) -> Self {
        counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        WasmName(s, counter)
    }
}
impl Drop for WasmName {
    fn drop(&mut self) {
        self.1.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
    }
}
impl Clone for WasmName {
    fn clone(&self) -> Self {
        self.1.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        WasmName(self.0, self.1)
    }
}
impl std::hash::Hash for WasmName {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.0 as *const str).hash(state);
    }
}
impl PartialEq for WasmName {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::addr_eq(self.0 as *const _, other.0 as *const _)
    }
}
impl Eq for WasmName {}
impl std::fmt::Debug for WasmName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl std::fmt::Display for WasmName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl AsRef<str> for WasmName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl Borrow<str> for WasmName {
    fn borrow(&self) -> &str {
        &self.0
    }
}

/// Generates a standardized static import string for WASM component architectures.
pub fn gen_component_name(namespace: &str, name: &str) -> String {
    format!("[static]{namespace}.{}-import", name.replace("_", "-"))
}

/// Iterator for enumerating combinations of boolean features.
#[derive(Debug)]
pub struct BitIterator {
    current: FeatureCombinationIteratorInnerBits,
    skip: FeatureCombinationIteratorInnerBits, // if this bit is set, skip this iteration
    kind: u8,
}

impl BitIterator {
    const MAX_KIND: u8 = core::mem::size_of::<FeatureCombinationIteratorInnerBits>() as u8 * 8;

    /// Constructs a new `BitIterator` for a given feature count.
    pub fn new(kind: u8) -> Self {
        if kind >= Self::MAX_KIND {
            panic!("Kind must be between 0 and {}", Self::MAX_KIND - 1);
        }

        BitIterator {
            current: FeatureCombinationIteratorInnerBits::ZERO,
            kind,
            skip: FeatureCombinationIteratorInnerBits::ZERO,
        }
    }

    /// Retrieves the current bit state representation.
    pub fn now(&self) -> FeatureCombinationIteratorInnerBits {
        self.current
    }

    /// Registers a specific bit index to be skipped during iteration.
    pub fn register_skip(&mut self, bit: u8) {
        if bit >= Self::MAX_KIND {
            panic!("Bit must be between 0 and {}", Self::MAX_KIND - 1);
        }
        self.skip.set(bit as usize, true);
    }

    /// Unregisters a raw mask of underlying bits from being skipped.
    pub fn skip_raw(&mut self, mask: FeatureCombinationIteratorInnerBits) {
        self.skip |= mask;
    }

    /// Unregisters a specific skipped bit index.
    pub fn unregister_skip(&mut self, bit: u8) {
        if bit >= Self::MAX_KIND {
            panic!("Bit must be between 0 and {}", Self::MAX_KIND - 1);
        }
        self.skip.set(bit as usize, false);
    }

    /// Unregisters an exact sequence of raw iteration bits.
    pub fn unregister_skip_raw(&mut self, mask: FeatureCombinationIteratorInnerBits) {
        self.skip &= !mask;
    }

    /// Clears any registered skip states.
    pub fn clear_skip(&mut self) {
        self.skip = FeatureCombinationIteratorInnerBits::ZERO;
    }
}

/// Underlying bit representations for the iterator generator.
pub mod bits {
    use bitvec::prelude::*;
    type FeatureCombinationIteratorInnerBitsInner = BitArray<[u64; 2]>;

    #[derive(Copy, Clone, Debug)]
    /// Strongly typed inner bit representation optimized for combination tracking.
    pub struct FeatureCombinationIteratorInnerBits(FeatureCombinationIteratorInnerBitsInner);

    impl core::ops::BitAnd for FeatureCombinationIteratorInnerBits {
        type Output = Self;

        fn bitand(self, rhs: Self) -> Self::Output {
            FeatureCombinationIteratorInnerBits { 0: self.0 & rhs.0 }
        }
    }

    impl core::ops::BitAndAssign for FeatureCombinationIteratorInnerBits {
        fn bitand_assign(&mut self, rhs: Self) {
            self.0 &= rhs.0;
        }
    }

    impl core::ops::BitOr for FeatureCombinationIteratorInnerBits {
        type Output = Self;

        fn bitor(self, rhs: Self) -> Self::Output {
            FeatureCombinationIteratorInnerBits { 0: self.0 | rhs.0 }
        }
    }

    impl core::ops::BitOrAssign for FeatureCombinationIteratorInnerBits {
        fn bitor_assign(&mut self, rhs: Self) {
            self.0 |= rhs.0;
        }
    }

    impl core::ops::BitXor for FeatureCombinationIteratorInnerBits {
        type Output = Self;

        fn bitxor(self, rhs: Self) -> Self::Output {
            FeatureCombinationIteratorInnerBits { 0: self.0 ^ rhs.0 }
        }
    }

    impl core::ops::Not for FeatureCombinationIteratorInnerBits {
        type Output = Self;

        fn not(self) -> Self::Output {
            FeatureCombinationIteratorInnerBits { 0: !self.0 }
        }
    }

    impl core::ops::AddAssign for FeatureCombinationIteratorInnerBits {
        fn add_assign(&mut self, rhs: Self) {
            // arbitrary-precision integer
            let raw_lhs = &mut self.0.data;
            let raw_rhs = &rhs.0.data;
            let mut carry = 0u64;

            for i in 0..raw_lhs.len() {
                let (sum1, carry1) = raw_lhs[i].overflowing_add(raw_rhs[i]);
                let (sum2, carry2) = sum1.overflowing_add(carry);
                raw_lhs[i] = sum2;
                carry = (carry1 as u64) + (carry2 as u64);
            }
        }
    }

    impl core::cmp::PartialOrd for FeatureCombinationIteratorInnerBits {
        fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }

    impl core::cmp::Ord for FeatureCombinationIteratorInnerBits {
        fn cmp(&self, other: &Self) -> core::cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }

    impl core::cmp::PartialEq for FeatureCombinationIteratorInnerBits {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl core::cmp::Eq for FeatureCombinationIteratorInnerBits {}

    impl core::ops::Index<usize> for FeatureCombinationIteratorInnerBits {
        type Output = bool;

        fn index(&self, index: usize) -> &bool {
            &self.0[index]
        }
    }

    impl FeatureCombinationIteratorInnerBits {
        /// Represents zero bits or no combinations active.
        pub const ZERO: Self = FeatureCombinationIteratorInnerBits {
            0: FeatureCombinationIteratorInnerBitsInner::ZERO,
        };
        /// Represents a standard first iteration or bit-start active.
        pub const ONE: Self = Self::from_number(1);

        /// Sets a specific position in the underlying representation.
        pub fn set(&mut self, index: usize, value: bool) {
            self.0.set(index, value);
        }

        /// Checks if this completely represents zeros.
        pub fn is_zero(&self) -> bool {
            self.0 == FeatureCombinationIteratorInnerBitsInner::ZERO
        }

        /// Checks if this is fully saturated.
        pub fn is_full(&self) -> bool {
            self.0.all()
        }

        /// Counts empty zeroes towards the most significant bit.
        pub fn leading_zeros(&self) -> usize {
            self.0.leading_zeros()
        }

        /// Counts empty zeroes towards the least significant bit.
        pub fn trailing_zeros(&self) -> usize {
            self.0.trailing_zeros()
        }

        /// Retrieves memory layout mapping to underlying pointers.
        pub fn as_raw_slice(&self) -> &[u64] {
            self.0.as_raw_slice()
        }

        /// Wraps static number initializations.
        pub const fn from_number(num: u64) -> Self {
            let mut bits: FeatureCombinationIteratorInnerBitsInner =
                FeatureCombinationIteratorInnerBitsInner::ZERO;
            bits.data[0] = num;
            FeatureCombinationIteratorInnerBits { 0: bits }
        }

        /// Generates an instance securely masked to the indicated position.
        pub fn from_one_pos(pos: usize) -> Self {
            let mut bits: FeatureCombinationIteratorInnerBitsInner =
                FeatureCombinationIteratorInnerBitsInner::ZERO;
            bits.set(pos, true);
            FeatureCombinationIteratorInnerBits { 0: bits }
        }

        /// Rapidly progresses to the next combination variant manually.
        pub fn increment(&mut self) {
            // arbitrary-precision integer
            let raw = &mut self.0.data;
            let mut carry = 1u64;

            for i in 0..raw.len() {
                let (new_value, new_carry) = raw[i].overflowing_add(carry);
                raw[i] = new_value;
                carry = if new_carry { 1 } else { 0 };
                if carry == 0 {
                    break;
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::FeatureCombinationIteratorInnerBits;

        #[test]
        fn test_increment() {
            let mut s = FeatureCombinationIteratorInnerBits::ZERO;
            assert_eq!(s.leading_zeros(), 128);
            s.set(1, true);
            println!("{:?}", s.as_raw_slice());
            assert_eq!(s.leading_zeros(), 1);

            let mut a = FeatureCombinationIteratorInnerBits::ONE;
            a.increment();
            assert_eq!(a.as_raw_slice()[0], 2);

            let mut b = FeatureCombinationIteratorInnerBits::from_number(u64::MAX);
            b.increment();
            assert_eq!(b.as_raw_slice()[0], 0);
            assert_eq!(b.as_raw_slice()[1], 1);

            let mut c = FeatureCombinationIteratorInnerBits::from_number(u64::MAX);
            c.set(64, true);
            c.increment();
            assert_eq!(c.as_raw_slice()[0], 0);
            assert_eq!(c.as_raw_slice()[1], 2);
        }

        #[test]
        fn test_add_assign() {
            let mut a = FeatureCombinationIteratorInnerBits::from_number(1);
            let b = FeatureCombinationIteratorInnerBits::from_number(2);
            a += b;
            assert_eq!(a.as_raw_slice()[0], 3);

            let mut c = FeatureCombinationIteratorInnerBits::from_number(u64::MAX);
            let d = FeatureCombinationIteratorInnerBits::from_number(1);
            c += d;
            assert_eq!(c.as_raw_slice()[0], 0);
            assert_eq!(c.as_raw_slice()[1], 1);

            let mut e = FeatureCombinationIteratorInnerBits::from_number(u64::MAX);
            e.set(64, true);
            let f = FeatureCombinationIteratorInnerBits::from_number(1);
            e += f;
            assert_eq!(e.as_raw_slice()[0], 0);
            assert_eq!(e.as_raw_slice()[1], 2);
        }
    }
}
pub use bits::FeatureCombinationIteratorInnerBits;

impl Iterator for BitIterator {
    type Item = FeatureCombinationIteratorInnerBits;

    fn next(&mut self) -> Option<Self::Item> {
        let count = core::mem::size_of::<FeatureCombinationIteratorInnerBits>() * 8
            - self.current.trailing_zeros();
        if count > self.kind as usize {
            None
        } else {
            let result = self.current;
            self.current.increment();

            loop {
                let flag = self.current & self.skip;
                if flag.is_zero() {
                    break;
                } else {
                    self.current +=
                        FeatureCombinationIteratorInnerBits::from_one_pos(flag.leading_zeros());
                }
            }
            Some(result)
        }
    }
}

#[derive(Debug)]
/// Facilitates multi-state combination testing iteration mapping arrays.
pub struct FeatureCombinationIterator<C: Borrow<T>, T: ?Sized> {
    features: Vec<(
        C,
        FeatureCombinationIteratorInnerBits,
        FeatureCombinationIteratorInnerBits,
    )>,
    current: BitIterator,
    __marker: std::marker::PhantomData<T>,
}

impl<'a, T: 'a + ?Sized, B: Borrow<T>, C: Borrow<T>, I: IntoIterator<Item = B>> FromIterator<(C, I)>
    for FeatureCombinationIterator<C, T>
where
    for<'c> &'c T: std::cmp::Eq + std::hash::Hash,
{
    fn from_iter<U: IntoIterator<Item = (C, I)>>(iter: U) -> Self {
        // What T refers to
        let data = iter
            .into_iter()
            .map(|(v, includes)| (v, includes.into_iter().collect::<Vec<_>>()))
            .collect::<Vec<_>>();

        // Referring to T
        let num = {
            let mut counts = HashMap::new();
            // Initialize counts for all items to 0
            for (v, _) in &data {
                counts.insert(v.borrow(), 0isize);
            }
            // Count references
            for (_, inc) in &data {
                for v in inc {
                    if let Some(c) = counts.get_mut(&v.borrow()) {
                        *c += 1;
                    }
                }
            }

            data.iter()
                .map(|(v, _)| -*counts.get(&v.borrow()).unwrap_or(&0))
                .collect::<Vec<_>>()
        };

        let data = data.into_iter().zip(num).collect::<Vec<_>>();

        // TODO!(); fix with behavior change
        // data.sort_by_key(|(_, v)| *v);
        // Remove 'num' from data, but keep dependencies
        let mut data: Vec<(C, Vec<B>)> = data.into_iter().map(|(v, _)| v).collect();

        // Check for non-trivial cycles (mutual references)
        {
            let index_map: std::collections::HashMap<&T, usize> = data
                .iter()
                .enumerate()
                .map(|(i, (v, _))| (v.borrow(), i))
                .collect();

            let mut visited = vec![0u8; data.len()]; // 0: White, 1: Gray, 2: Black
            let mut stack = Vec::new();

            for i in 0..data.len() {
                if visited[i] != 0 {
                    continue;
                }

                stack.push((i, 0));
                visited[i] = 1; // Gray

                while let Some((u, dep_idx)) = stack.last_mut() {
                    let u = *u;
                    let deps = &data[u].1;
                    if *dep_idx < deps.len() {
                        let dep = &deps[*dep_idx];
                        *dep_idx += 1;

                        if let Some(&v) = index_map.get(&dep.borrow()) {
                            if u == v {
                                continue;
                            }
                            if visited[v] == 1 {
                                panic!("Mutual reference detected!");
                            }
                            if visited[v] == 0 {
                                visited[v] = 1;
                                stack.push((v, 0));
                            }
                        }
                    } else {
                        // Finished processing u
                        visited[u] = 2; // Black
                        stack.pop();
                    }
                }
            }
        }

        // Ensure that values referencing themselves are always placed on the right.
        // if sortable, swap with a value that does not reference itself so we check it.
        let mut count = 0i32;
        loop {
            let mut changed = false;
            for i in 0..data.len() {
                let mut swap_idx = None;
                {
                    let (ref_value, ref_includes) = &data[i];
                    if ref_includes
                        .iter()
                        .any(|v| v.borrow() == ref_value.borrow())
                    {
                        // Find a value to swap with
                        for j in (i + 1)..data.len() {
                            let (_, swap_includes) = &data[j];
                            if !swap_includes
                                .iter()
                                .any(|v| v.borrow() == ref_value.borrow())
                            {
                                swap_idx = Some(j);
                                break;
                            }
                        }
                    }
                }

                if let Some(j) = swap_idx {
                    data.swap(i, j);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
            if count.checked_add(1).is_none() {
                break;
            }
            count += 1;
            if count as u128 > 1u128.checked_shl(data.len() as u32).unwrap_or(u128::MAX) {
                panic!("Mutual reference detected!");
            }
        }

        let (data, _): (Vec<_>, Vec<_>) = data.into_iter().map(|v| (v, ())).unzip();

        let len = data.len();

        let map = data
            .iter()
            .enumerate()
            .map(|(i, (v, _))| (v.borrow(), i))
            .collect::<HashMap<&T, usize>>();

        // Rebuild dependents map from sorted data
        let dependents_map = {
            let mut dmap = data
                .iter()
                .map(|(v, _)| (v.borrow(), vec![]))
                .collect::<HashMap<&T, Vec<&C>>>();

            for (t, inc) in &data {
                for v in inc {
                    if let Some(list) = dmap.get_mut(&v.borrow()) {
                        list.push(t);
                    }
                }
            }
            dmap
        };

        let mut features_base = Vec::with_capacity(data.len());
        let mut dependencies_masks = Vec::with_capacity(data.len());
        for (v, inc) in &data {
            let deps = dependents_map
                .get(&v.borrow())
                .map(|v| v.as_slice())
                .unwrap_or(&[]);
            let mut mask = FeatureCombinationIteratorInnerBits::ZERO;
            let mut indices = Vec::new();

            for dep in deps {
                let idx = map[&(*dep).borrow()];
                mask.set(idx, true);
                indices.push(idx);
            }
            features_base.push((indices, mask));

            // Compute dependencies mask
            let mut dep_mask = FeatureCombinationIteratorInnerBits::ZERO;
            for d in inc {
                if let Some(idx) = map.get(&d.borrow()) {
                    dep_mask.set(*idx, true);
                }
            }
            dependencies_masks.push(dep_mask);
        }

        let mut features_masks: Vec<FeatureCombinationIteratorInnerBits> =
            features_base.iter().map(|(_, m)| *m).collect();

        for _ in 0..data.len() {
            let mut changed = false;
            for i in 0..data.len() {
                let (indices, _) = &features_base[i];
                let mut mask = features_masks[i];
                for &dep_idx in indices {
                    mask |= features_masks[dep_idx];
                    if mask != features_masks[i] { // Check against current stored mask, not 'old' local var if we updated it?
                        // Logic: mask |= dep_mask.
                        // If mask grew, changed=true.
                    }
                }
                if mask != features_masks[i] {
                    features_masks[i] = mask;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }

        let features = features_masks
            .into_iter()
            .zip(dependencies_masks)
            .zip(data)
            .map(|((mask, dep_mask), (v, _))| (v, mask, dep_mask))
            .collect::<Vec<_>>();

        FeatureCombinationIterator {
            features,
            current: BitIterator::new(len as u8),
            __marker: std::marker::PhantomData,
        }
    }
}

impl<C: Borrow<T> + std::cmp::Eq + std::hash::Hash + Clone, T: ?Sized> Iterator
    for FeatureCombinationIterator<C, T>
{
    type Item = std::collections::HashSet<C>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let bits = self.current.now();

            // Check termination
            let count = core::mem::size_of::<FeatureCombinationIteratorInnerBits>() * 8
                - bits.trailing_zeros();
            if count > self.current.kind as usize {
                return None;
            }

            // Compute skip mask (dependents of absent features)
            let mut _mask = FeatureCombinationIteratorInnerBits::ZERO;
            for (i, (_, feature_bits, _)) in self.features.iter().enumerate() {
                if !bits[i] {
                    // Feature i is absent
                    _mask |= *feature_bits; // Dependents of i are forbidden
                }
            }

            // Compute skip mask (dependents of absent features)
            // AND resolve violations (features present without dependencies)
            let _mask = FeatureCombinationIteratorInnerBits::ZERO;

            // Check for Forbidden features (because dependency is missing)
            // This is equivalent to checking "Present features have all dependencies".

            // Strategy: Iterate all features.
            // If feature i is PRESENT:
            //    Check dependencies_masks[i].
            //    If ANY dependency d is ABSENT -> Violation.
            //    violation |= (1 << i).
            //    To resolve: Either Clear i (add 1<<i) OR Set d (add 1<<d).
            //    We should pick the one that adds the LEAST to current.
            //    If d < i: Set d.
            //    If d > i: Clear i.
            //    Also if Clear i, we might need to carry.

            // We can compute minimum jump.

            let mut min_jump = None;

            for (i, (_, _, dependencies)) in self.features.iter().enumerate() {
                if bits[i] {
                    // Feature i is Present.
                    // Dependencies must be Present.
                    // Missing = dependencies & !bits.
                    let missing = *dependencies & !bits;
                    if !missing.is_zero() {
                        // Violation! Feature i needs missing dependencies.
                        // Options:
                        // 1. Clear i (add 1<<i).
                        // 2. Set d (for each d in missing). (add distance to d).

                        // Option 1: Clear i.

                        // Option 2: Set d.
                        // For each d in missing:
                        //   d_pos = trailing_zeros(d)?
                        //   d_jump = 1<<d_pos - (bits & low_mask)?
                        //   Wait. next_valid(bits, d) = (bits | (1<<d)) & !((1<<d)-1).
                        //   jump = next_valid - bits.
                        //   Or simplistically: 1<<d?
                        //   If bits has lower bits set, 1<<d might not be enough or too much?
                        //   Actually, we want to reach the *next* state where d=1.
                        //   (bits | (1<<d)) & !((1<<d)-1).

                        // Let's implement calculate_jump(current, target_bit).
                        // But FeatureCombinationInnerBits doesn't expose arithmetic easily.
                        // It has `from_one_pos`.
                        // It has `+`.

                        // If d < i.
                        // Then bits[d]=0. bits[i]=1.
                        // We want d=1.
                        // Since d < i, d is a lower bit.
                        // If we increment, d will toggle soon.
                        // E.g. d=0. 0->1 is +1.
                        // d=1. 00->10 is +2 (if 00).

                        // If we just track "smallest bit that needs to change".
                        // If any d < i.
                        // Then we assume natural increment will handle it?
                        // But we want to SKIP invalid states.
                        // If d < i. We can jump to next d=1.
                        // If d > i. We MUST clear i. (Jump to next i=0).

                        // If d > i. Jump = 1<<i (Clear i).
                        // If d < i. Jump = Next d=1.
                        // Next d=1 is <= 1<<d (relative to cleared lower).

                        // Let's optimize:
                        // If ANY d > i: We MUST clear i.
                        // Jump = 1<<i.

                        // If ALL d < i:
                        // We can wait for d.
                        // Can we skip to d?
                        // Yes. Jump to smallest d.
                        // But we can't easily compute "Jump to d" with abstract bits.
                        // But if d < i, and we assume Lsb0.
                        // 1<<d is smaller than 1<<i.
                        // So jump is smaller?
                        // If we iterate violations and pick minimum 1<<pos.
                        // If we pick 1<<i (Clear i).
                        // If we pick 1<<d (Set d? No, 1<<d might not set d correctly if lower bits are messy).
                        // But generally, adding 1<<d will toggle d (0->1) and clear lower.
                        // So adding 1<<d IS correct to jump to next d=1.

                        // So:
                        // Candidates:
                        // 1. 1<<i.
                        // 2. 1<<d (for all d in missing).

                        // Pick the SMALLEST candidate (lowest index).
                        // And apply it.

                        // If we find MULTIPLE violations.
                        // We should pick the global minimum jump.

                        // So loop over all i.
                        // Collect candidates.
                        // Pick min.

                        // Candidate from i: i.
                        // Candidates from missing: d's.

                        // Wait. If d < i.
                        // Should we set d or clear i?
                        // If we set d (jump 1<<d), we keep i set. Result valid (i=1, d=1).
                        // If we clear i (jump 1<<i), we get i=0. Result valid (i=0, d=0).
                        // Which is next?
                        // 1<<d is smaller. So we set d.

                        // If d > i.
                        // Set d (jump 1<<d). Keep i set.
                        // Clear i (jump 1<<i).
                        // 1<<i is smaller. So we clear i.

                        // So strategy:
                        // Collect all `i` (violation bits) and all `d` (missing dependencies).
                        // Find the MINIMUM index `m` among them.
                        // Add `1 << m`.

                        // Example: i=1 (B). d=0 (A).
                        // Min(1, 0) = 0.
                        // Add 1<<0 (1).
                        // 0010 + 1 = 0011 (A=1, B=1). Correct.

                        // Example: i=0 (A). d=2 (C). (If A depended on C).
                        // Min(0, 2) = 0.
                        // Add 1<<0 (1).
                        // 0001 + 1 = 0010 (A=0). Correct.

                        // This logic is beautiful.
                        // Just find the lowest bit involved in any violation (either the feature itself or its missing dependency).
                        // And add 1 << that bit.

                        // Iterate bits of missing.

                        // Let's accumulate a "jump_mask".
                        // jump_mask |= (1 << i).
                        // jump_mask |= missing.

                        if min_jump.is_none() {
                            min_jump = Some(FeatureCombinationIteratorInnerBits::ZERO);
                        }
                        if let Some(ref mut j) = min_jump {
                            j.set(i, true);
                            *j |= missing;
                        }
                    }
                }
            }

            if let Some(jump_mask) = min_jump {
                // Found violations.
                // We want smallest bit in jump_mask.
                // Wait. trailing_zeros counts from MSB in BitArray (Lsb0)???
                // NO. I decided earlier it counted from MSB.
                // But leading_zeros counted from LSB?
                // `from_one_pos` uses index.
                // If I want index of lowest bit.
                // If `Lsb0`: Index 0 is lowest.
                // If `trailing_zeros` counts from End (127).
                // `leading_zeros` counts from Start (0).
                // So I want `leading_zeros`.

                // Let's use `leading_zeros`.
                let bit = jump_mask.leading_zeros(); // Index of first set bit (lowest index).
                self.current.current += FeatureCombinationIteratorInnerBits::from_one_pos(bit);

                // Continue loop
            } else {
                // Valid!
                // ... return result ...
                let mut result = std::collections::HashSet::new();
                for (i, (feature, _, _)) in self.features.iter().enumerate() {
                    if bits[i] {
                        result.insert(feature.clone());
                    }
                }
                self.current.current.increment();
                return Some(result);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, sync::Arc};

    use super::*;

    #[test]
    fn test_get_file_main_name() {
        let path = camino::Utf8Path::new("name.opt.adjusted.wasm");
        let file_name = path.get_file_main_name();
        assert_eq!(file_name.unwrap(), "name");
    }

    #[test]
    fn test_bit_generator() {
        let bits = BitIterator::new(3).collect::<Vec<_>>();

        assert_eq!(
            &bits
                .iter()
                .map(|b| b.as_raw_slice()[0])
                .collect::<Vec<u64>>(),
            &[0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111]
        );

        let count = BitIterator::new(10).count();
        assert_eq!(count, 1024);

        let mut generator = BitIterator::new(5);
        generator.register_skip(1);
        generator.register_skip(3);
        let bits = generator.collect::<Vec<_>>();

        assert_eq!(
            &bits
                .iter()
                .map(|b| b.as_raw_slice()[0])
                .collect::<Vec<u64>>(),
            &[
                0b00000, 0b00001, 0b00100, 0b00101, 0b10000, 0b10001, 0b10100, 0b10101
            ]
        );
    }

    #[test]
    fn test_feature_combination_iterator() {
        let data = vec![
            ("A", vec![]),
            ("B", vec!["A"]),
            ("C", vec!["A"]),
            ("D", vec!["B", "C"]),
        ];

        let iterator = data
            .into_iter()
            .collect::<FeatureCombinationIterator<_, str>>();

        println!("Iterator created: {:?}", iterator);

        let combinations = iterator.collect::<Vec<_>>();

        let expected = vec![
            HashSet::from([]),
            HashSet::from(["A"]),
            HashSet::from(["A", "B"]),
            HashSet::from(["A", "C"]),
            HashSet::from(["A", "B", "C"]),
            HashSet::from(["A", "B", "C", "D"]),
        ];

        assert_eq!(combinations, expected);

        let data = vec![
            (String::from("A"), vec![]),
            (String::from("B"), vec![String::from("A")]),
            (String::from("C"), vec![String::from("A")]),
            (
                String::from("D"),
                vec![String::from("B"), String::from("C")],
            ),
        ];
        let data_ref = data
            .iter()
            .map(|(v, inc)| (v, inc.iter().map(|s| s).collect::<Vec<_>>()))
            .collect::<Vec<_>>();

        let iterator = data
            .clone()
            .into_iter()
            .collect::<FeatureCombinationIterator<_, String>>();

        let iterator2 = data_ref
            .into_iter()
            .collect::<FeatureCombinationIterator<_, String>>();

        println!("Iterator created: {:?}", iterator);

        let combinations = iterator.collect::<Vec<_>>();
        let _combinations2 = iterator2.collect::<Vec<_>>();

        let expected = vec![
            HashSet::from([]),
            HashSet::from([String::from("A")]),
            HashSet::from([String::from("A"), String::from("B")]),
            HashSet::from([String::from("A"), String::from("C")]),
            HashSet::from([String::from("A"), String::from("B"), String::from("C")]),
            HashSet::from([
                String::from("A"),
                String::from("B"),
                String::from("C"),
                String::from("D"),
            ]),
        ];

        assert_eq!(combinations, expected);

        let data = data
            .iter()
            .map(|(u, v)| (Arc::new(u.clone()), v))
            .collect::<Vec<_>>();

        let iterator = data
            .into_iter()
            .collect::<FeatureCombinationIterator<_, String>>();

        println!("Iterator created: {:?}", iterator);

        let _combinations = iterator.collect::<Vec<_>>();
    }

    #[test]
    #[should_panic(expected = "Mutual reference detected!")]
    fn test_feature_combination_iterator_mutual_ref() {
        // A includes B
        // B includes A
        // Cycle!
        let data = vec![
            (String::from("A"), vec![String::from("B")]),
            (String::from("B"), vec![String::from("A")]),
        ];

        let _iterator = data
            .into_iter()
            .collect::<FeatureCombinationIterator<_, String>>();
    }

    #[test]
    #[should_panic(expected = "Mutual reference detected!")]
    fn test_feature_combination_iterator_complex_cycle() {
        // A -> B -> C -> A
        let data = vec![
            (String::from("A"), vec![String::from("B")]),
            (String::from("B"), vec![String::from("C")]),
            (String::from("C"), vec![String::from("A")]),
        ];

        let _iterator = data
            .into_iter()
            .collect::<FeatureCombinationIterator<_, String>>();
    }

    #[test]
    fn test_feature_combination_iterator_complex_valid() {
        // Valid graph:
        // A -> B, C
        // B -> D
        // C -> D
        // D -> []
        // Order should handle this (D comes first, then B/C, then A)

        let data = vec![
            (
                String::from("A"),
                vec![String::from("B"), String::from("C")],
            ),
            (String::from("B"), vec![String::from("D")]),
            (String::from("C"), vec![String::from("D")]),
            (String::from("D"), vec![]),
        ];

        let iterator = data
            .into_iter()
            .collect::<FeatureCombinationIterator<_, String>>();

        let combinations = iterator.collect::<Vec<_>>();
        // If it didn't panic and produced combinations, the topological sort/check worked for this valid case.
        assert!(combinations.len() > 0);
    }

    #[test]
    fn test_feature_combination_iterator_complex_diamond() {
        use std::collections::{HashMap, HashSet};
        // Diamond:
        // Root -> Left, Right
        // Left -> Base
        // Right -> Base
        // Base -> []

        let data = vec![
            (
                String::from("Root"),
                vec![String::from("Left"), String::from("Right")],
            ),
            (String::from("Left"), vec![String::from("Base")]),
            (String::from("Right"), vec![String::from("Base")]),
            (String::from("Base"), vec![]),
        ];

        let iterator = data
            .into_iter()
            .collect::<FeatureCombinationIterator<_, String>>();

        let combinations: Vec<HashSet<String>> = iterator.collect();

        // Verify that for every combination, if a feature is present, its dependencies are also present.
        // We can check this property for all combinations.

        // Dependency map for checking
        let deps: HashMap<String, Vec<String>> = HashMap::from([
            (
                String::from("Root"),
                vec![String::from("Left"), String::from("Right")],
            ),
            (String::from("Left"), vec![String::from("Base")]),
            (String::from("Right"), vec![String::from("Base")]),
            (String::from("Base"), vec![]),
        ]);

        for combo in &combinations {
            for feature in combo {
                if let Some(required) = deps.get(feature) {
                    for req in required {
                        assert!(
                            combo.contains(req),
                            "Combination {:?} invalid: {} requires {}",
                            combo,
                            feature,
                            req
                        );
                    }
                }
            }
        }

        // Also verify some known valid combinations are present
        assert!(combinations.contains(&HashSet::from([])));
        assert!(combinations.contains(&HashSet::from([String::from("Base")])));
        assert!(
            combinations.contains(&HashSet::from([String::from("Base"), String::from("Left")]))
        );
        assert!(combinations.contains(&HashSet::from([
            String::from("Base"),
            String::from("Left"),
            String::from("Right"),
            String::from("Root")
        ])));

        // Verify invalid ones are NOT present
        // Root only
        assert!(!combinations.contains(&HashSet::from([String::from("Root")])));
        // Left only
        assert!(!combinations.contains(&HashSet::from([String::from("Left")])));
    }
}
