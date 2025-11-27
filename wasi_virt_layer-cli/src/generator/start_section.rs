use std::collections::HashMap;
use std::sync::Arc;
use walrus::FunctionId;

use crate::generator::GeneratorCtx;
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

impl StartOrigin {
    pub fn export_name(&self) -> String {
        match self {
            StartOrigin::ResetFunc => "__wasip1_vfs_origin_reset".to_string(),
            // StartOrigin::StartFunc => "__wasip1_vfs_origin_startfunc".to_string(),
            // StartOrigin::Threads => "__wasip1_vfs_origin_threads".to_string(),
            // StartOrigin::SharedGlobal => "__wasip1_vfs_origin_shared_global".to_string(),
            // StartOrigin::Debug => "__wasip1_vfs_origin_debug".to_string(),
        }
    }
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

#[derive(Debug)]
pub struct StartSectionGenerator {
    common: Arc<parking_lot::Mutex<StartSectionCommon>>,
}

impl StartSectionGenerator {
    pub fn new(
        module: &mut walrus::Module,
        wasm_names: impl IntoIterator<Item = WasmName>,
    ) -> Self {
        let func_ty = module.types.add(&[], &[]);
        let common = StartSectionCommon {
            map: HashMap::new(),
            start_alternatives: wasm_names
                .into_iter()
                .map(|name| {
                    let (new_fid, _) = module.add_import_func(
                        NAMESPACE,
                        &Self::unique_import_name(&name),
                        func_ty,
                    );
                    (name, new_fid)
                })
                .collect(),
        };
        Self {
            common: Arc::new(parking_lot::Mutex::new(common)),
        }
    }

    fn unique_import_name(wasm_name: &WasmName) -> String {
        format!("__wasip1_vfs_{wasm_name}__start_anchor")
    }

    pub fn builder(&self) -> StartSectionBuilder {
        StartSectionBuilder {
            common: self.common.clone(),
        }
    }

    pub fn build(&self) {}
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
