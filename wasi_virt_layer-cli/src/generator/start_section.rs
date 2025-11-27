use std::collections::HashMap;
use std::panic;
use std::sync::Arc;
use walrus::FunctionId;

use crate::generator::{Generator, GeneratorCtx};
use crate::util::{
    NAMESPACE, WalrusFID, WalrusUtilExport, WalrusUtilFuncs, WalrusUtilModule, WasmName,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StartOrigin {
    ResetFunc,
    // StartFunc,
    // Threads,
    // SharedGlobal,
    // Debug,
}

#[derive(Debug, Clone)]
pub struct StartSource(String);

#[derive(Debug, Default)]
pub struct StartSectionCommon {
    /// Additional start functions.
    map: HashMap<StartOrigin, StartSource>,
    /// The function body is an import fn and is replaced during the Build phase.
    start_alternatives: HashMap<WasmName, FunctionId>,
}

#[derive(Debug, Default)]
pub struct StartSectionGenerator {
    common: Option<Arc<parking_lot::Mutex<StartSectionCommon>>>,
}

impl StartSectionGenerator {
    pub fn init(&mut self, module: &mut walrus::Module, wasm_names: &[WasmName]) {
        if let Some(common) = &self.common {
            let mut common = common.lock();
            common.start_alternatives.clear();
            for name in wasm_names {
                let unique_name = Self::unique_import_name(name);
                // if (NAMESPACE, &unique_name).get_fid(&module.imports).is_ok() {
                //     panic!("Import function for start alternative '{name}' already exists");
                // }
                let fid = (NAMESPACE, &unique_name)
                    .get_fid(&module.imports)
                    .unwrap_or_else(|_| {
                        let func_ty = module.types.add(&[], &[]);
                        let (new_fid, _) = module.add_import_func(NAMESPACE, &unique_name, func_ty);
                        new_fid
                    });
                common.start_alternatives.insert(name.clone(), fid);
            }
        } else {
            let func_ty = module.types.add(&[], &[]);
            let common = StartSectionCommon {
                map: HashMap::new(),
                start_alternatives: wasm_names
                    .iter()
                    .map(|name| {
                        let unique_name = Self::unique_import_name(name);
                        if (NAMESPACE, &unique_name).get_fid(&module.imports).is_ok() {
                            panic!("Import function for start alternative '{name}' already exists");
                        }
                        let (new_fid, _) = module.add_import_func(NAMESPACE, &unique_name, func_ty);
                        (name.clone(), new_fid)
                    })
                    .collect(),
            };
            self.common = Some(Arc::new(parking_lot::Mutex::new(common)));
        }
    }

    fn unique_import_name(wasm_name: &WasmName) -> String {
        format!("__wasip1_vfs_{wasm_name}__start_anchor")
    }

    pub fn builder(&self) -> StartSectionBuilder {
        StartSectionBuilder {
            common: self.common.as_ref().unwrap().clone(),
        }
    }

    pub fn build(self, module: &mut walrus::Module) -> eyre::Result<()> {
        Ok(())
    }
}

impl Generator for StartSectionGenerator {}

#[derive(Debug, Clone)]
pub struct StartSectionBuilder {
    common: Arc<parking_lot::Mutex<StartSectionCommon>>,
}

impl StartSectionBuilder {
    pub fn iter(&self) -> Vec<(WasmName, FunctionId)> {
        self.common
            .lock()
            .start_alternatives
            .iter()
            .map(|(name, fid)| (name.clone(), *fid))
            .collect::<Vec<_>>()
    }
}
