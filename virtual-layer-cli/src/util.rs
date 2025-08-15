use std::path::{Path, PathBuf};

use eyre::Context as _;
use walrus::{ir::InstrSeqId, *};

use crate::instrs::InstrRead;

pub(crate) trait WalrusUtilImport {
    fn find_mut(&mut self, module: impl AsRef<str>, name: impl AsRef<str>) -> Option<&mut Import>;
}

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

    fn connect_func_inner(&mut self, fid: FunctionId, export_id: FunctionId) -> eyre::Result<()>;

    /// add fake function to the module
    /// and return the function id
    fn add_func(
        &mut self,
        params: &[ValType],
        results: &[ValType],
        fn_: impl FnOnce(&mut FunctionBuilder, &Vec<LocalId>) -> eyre::Result<()>,
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
    fn get_using_func(&self, fid: FunctionId)
    -> eyre::Result<Vec<(FunctionId, InstrSeqId, usize)>>;
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
    fn connect_func_inner(&mut self, fid: FunctionId, export_id: FunctionId) -> eyre::Result<()> {
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

        self.exports.delete(export_id);

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
            .get_func(import_module, import_name.as_ref())
            .to_eyre()
            .wrap_err_with(|| eyre::eyre!("import {} not found", import_name.as_ref()))?;

        let export_id = self
            .exports
            .get_func(&export_name)
            .to_eyre()
            .wrap_err_with(|| eyre::eyre!("export {} not found", export_name.as_ref()))?;

        self.connect_func_inner(fid, export_id)
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
            // environ_sizes_get
            let import_id = self
                .imports
                .get_func("wasi_snapshot_preview1", "environ_sizes_get")
                .to_eyre()
                .wrap_err_with(|| eyre::eyre!("Failed to get environ_sizes_get"))?;

            let using_funcs = self.get_using_func(import_id)?;

            let ret_mem_id = std::sync::Arc::new(std::sync::Mutex::new(None));

            for (fid, _, _) in using_funcs {
                let arg_ptr = std::sync::Arc::new(std::sync::Mutex::new(Option::<Vec<u32>>::None));
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
                *mem_id
            } else {
                return Err(eyre::eyre!("Memory not found"));
            }
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
        fid: FunctionId,
    ) -> eyre::Result<Vec<(FunctionId, InstrSeqId, usize)>> {
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
            if *name == "opt" || *name == "adjusted" || *name == "wasm" {
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
