use std::collections::HashMap;
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
    fn init(&mut self, module: &mut walrus::Module, wasm_names: &[WasmName]) {
        let func_ty = module.types.add(&[], &[]);
        let common = StartSectionCommon {
            map: HashMap::new(),
            start_alternatives: wasm_names
                .iter()
                .map(|name| {
                    let unique_name = Self::unique_import_name(name);
                    let new_fid = (NAMESPACE, &unique_name)
                        .get_fid(&module.imports)
                        .unwrap_or_else(|_| {
                            module.add_import_func(NAMESPACE, &unique_name, func_ty).0
                        });
                    (name.clone(), new_fid)
                })
                .collect(),
        };
        self.common = Some(Arc::new(parking_lot::Mutex::new(common)));
    }

    fn unique_import_name(wasm_name: &WasmName) -> String {
        format!("__wasip1_vfs_{wasm_name}__start_anchor")
    }

    pub fn builder(&self) -> StartSectionBuilder {
        StartSectionBuilder {
            common: self.common.as_ref().unwrap().clone(),
        }
    }

    pub fn build(&self) {}
}

impl Generator for StartSectionGenerator {
    fn pre_vfs(&mut self, module: &mut walrus::Module, ctx: &GeneratorCtx) -> eyre::Result<()> {
        Self::init(self, module, &ctx.target_names_with_self);

        println!("Start Section Generator:");

        for (name, fid) in self
            .common
            .as_ref()
            .unwrap()
            .lock()
            .start_alternatives
            .iter()
        {
            println!("Added start alternative: {name}");
        }
        Ok(())
    }
}

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
