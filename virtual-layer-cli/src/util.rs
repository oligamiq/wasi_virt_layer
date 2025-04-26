use std::path::{Path, PathBuf};

use eyre::Context as _;
use walrus::*;

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

    /// add fake function to the module
    /// and return the function id
    fn add_func(
        &mut self,
        params: &[ValType],
        results: &[ValType],
        fn_: impl FnOnce(&ModuleMemories, &mut FunctionBuilder, &Vec<LocalId>) -> eyre::Result<()>,
    ) -> eyre::Result<FunctionId>;

    /// get the memory id from target name
    /// and remove anchor
    fn get_target_memory_id(&mut self, name: impl AsRef<str>) -> eyre::Result<MemoryId>;
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

        self.exports
            .remove(export_name)
            .to_eyre()
            .wrap_err_with(|| eyre::eyre!("Failed to remove export"))?;

        Ok(())
    }

    fn add_func(
        &mut self,
        params: &[ValType],
        results: &[ValType],
        fn_: impl FnOnce(&ModuleMemories, &mut FunctionBuilder, &Vec<LocalId>) -> eyre::Result<()>,
    ) -> eyre::Result<FunctionId> {
        let mut builder = FunctionBuilder::new(&mut self.types, params, results);

        let args = params
            .iter()
            .map(|ty| self.locals.add(*ty))
            .collect::<Vec<_>>();

        fn_(&self.memories, &mut builder, &args)?;

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
