use walrus::{Function, Import, ImportKind, ModuleImports};

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
}
