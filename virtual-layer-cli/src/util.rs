use std::path::{Path, PathBuf};

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
    ) -> anyhow::Result<()>;

    /// add fake function to the module
    /// and return the function id
    fn add_func(
        &mut self,
        params: &[ValType],
        results: &[ValType],
        fn_: impl FnOnce(&ModuleMemories, &mut FunctionBuilder, &Vec<LocalId>) -> anyhow::Result<()>,
    ) -> anyhow::Result<FunctionId>;
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
    ) -> anyhow::Result<()> {
        let fid = self.imports.get_func(import_module, import_name)?;

        let export_id = self.exports.get_func(&export_name)?;

        self.replace_imported_func(fid, |(builder, arg_locals)| {
            let mut func_body = builder.func_body();

            for local in arg_locals {
                func_body.local_get(*local);
            }
            func_body.call(export_id);
            func_body.return_();
        })?;

        self.exports.remove(export_name)?;

        Ok(())
    }

    fn add_func(
        &mut self,
        params: &[ValType],
        results: &[ValType],
        fn_: impl FnOnce(&ModuleMemories, &mut FunctionBuilder, &Vec<LocalId>) -> anyhow::Result<()>,
    ) -> anyhow::Result<FunctionId> {
        let mut builder = FunctionBuilder::new(&mut self.types, params, results);

        let args = params
            .iter()
            .map(|ty| self.locals.add(*ty))
            .collect::<Vec<_>>();

        fn_(&self.memories, &mut builder, &args)?;

        Ok(builder.finish(vec![], &mut self.funcs))
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
