use itertools::Itertools;
use std::collections::HashMap;
use std::panic;
use std::sync::Arc;
use walrus::FunctionId;

use crate::{
    generator::{Generator, GeneratorCtx},
    unique_name::UniqueName,
    util::{WalrusFID, WalrusUtilExport, WalrusUtilFuncs, WalrusUtilModule, WasmName},
};

#[derive(Debug, Clone)]
pub enum StartFnPriority {
    AfterMemoryReset,
    AfterAll,
}

#[derive(Debug)]
pub struct StartFnInfo {
    pub priority: StartFnPriority,
    pub source: StartSource,
}

pub enum StartSource {
    ExportFunc(String),
    Rewrite(Option<Box<dyn FnOnce(&mut walrus::Module, &GeneratorCtx) -> eyre::Result<()>>>),
}

impl core::fmt::Debug for StartSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StartSource::ExportFunc(name) => {
                write!(f, "ExportFunc({})", name)
            }
            StartSource::Rewrite(_) => {
                write!(f, "Rewrite(<function>)")
            }
        }
    }
}

#[derive(Debug, Clone, strum::AsRefStr, strum::EnumCount, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
pub enum StartAlternativeName {
    /// Initialize each target wasm module
    WasmName(WasmName),
    /// This is the VFS initialization function
    VFS(WasmName),
    /// This should call after resetting the memory state
    /// Conclusion: All initialization function must be done here
    AfterMemoryReset,
}

#[derive(Debug, Default)]
pub struct StartSectionCommon {
    /// Additional start functions.
    map: Vec<StartFnInfo>,
    /// The function body is an import fn and is replaced during the Build phase.
    start_alternatives: HashMap<StartAlternativeName, FunctionId>,
    is_builded: bool,
}

impl StartSectionCommon {
    pub fn vfs_start_fid(&self) -> (WasmName, FunctionId) {
        self.start_alternatives
            .iter()
            .find_map(|(name, fid)| match name {
                StartAlternativeName::VFS(wasm_name) => Some((wasm_name.clone(), *fid)),
                _ => None,
            })
            .expect("VFS start alternative must exist")
    }

    pub fn iter(&self) -> impl Iterator<Item = (&StartAlternativeName, FunctionId)> {
        self.start_alternatives.iter().map(|(a, b)| (a, *b))
    }

    pub fn target_wasm_fids(&self) -> impl Iterator<Item = (&WasmName, FunctionId)> {
        self.start_alternatives
            .iter()
            .filter_map(move |(name, fid)| match name {
                StartAlternativeName::WasmName(wasm_name) => Some((wasm_name, *fid)),
                _ => None,
            })
    }

    pub fn check_is_builded(&self) {
        if self.is_builded {
            panic!("StartSectionCommon has already been built");
        }
    }
}

#[derive(Debug, Default)]
pub struct StartSectionGenerator {
    common: Option<Arc<parking_lot::Mutex<StartSectionCommon>>>,
}

impl StartSectionGenerator {
    pub fn init(
        &mut self,
        module: &mut walrus::Module,
        vfs_name: WasmName,
        wasm_names: &[WasmName],
    ) {
        if let Some(_) = &self.common {
            // let mut common = common.lock();
            // common.start_alternatives.clear();
            // for name in wasm_names {
            //     let unique_name = Self::unique_import_name(name);
            //     // if (UniqueName::NAMESPACE, &unique_name).get_fid(&module.imports).is_ok() {
            //     //     panic!("Import function for start alternative '{name}' already exists");
            //     // }
            //     let fid = (UniqueName::NAMESPACE, &unique_name)
            //         .get_fid(&module.imports)
            //         .unwrap_or_else(|_| {
            //             let func_ty = module.types.add(&[], &[]);
            //             let (new_fid, _) = module.add_import_func(UniqueName::NAMESPACE, &unique_name, func_ty);
            //             new_fid
            //         });
            //     common.start_alternatives.insert(name.clone(), fid);
            // }
            panic!("Re-initialization of StartSectionGenerator is not supported");
        } else {
            let func_ty = module.types.add(&[], &[]);
            let common = StartSectionCommon {
                map: Vec::new(),
                is_builded: false,
                start_alternatives: core::iter::once(StartAlternativeName::VFS(vfs_name.clone()))
                    .chain(core::iter::once(StartAlternativeName::AfterMemoryReset))
                    .chain(
                        wasm_names
                            .iter()
                            .cloned()
                            .map(StartAlternativeName::WasmName),
                    )
                    .map(|name| {
                        let unique_name = UniqueName::StartAlternative(&name);
                        if (UniqueName::NAMESPACE, &unique_name)
                            .get_fid(&module.imports)
                            .is_ok()
                        {
                            panic!(
                                "Import function for start alternative '{name:?}' already exists"
                            );
                        }
                        let (new_fid, _) = module.add_import_func(
                            UniqueName::NAMESPACE,
                            &unique_name.to_string(),
                            func_ty,
                        );
                        (name, new_fid)
                    })
                    .collect(),
            };
            self.common = Some(Arc::new(parking_lot::Mutex::new(common)));
        }
    }

    pub fn builder(&self) -> StartSectionBuilder {
        match &self.common {
            None => panic!("StartSectionGenerator is not initialized"),
            Some(common) => {
                if common.lock().is_builded {
                    panic!("StartSectionGenerator has already been built");
                }
            }
        }

        StartSectionBuilder {
            common: self.common.as_ref().unwrap().clone(),
        }
    }

    pub fn build(self, module: &mut walrus::Module, ctx: &GeneratorCtx) -> eyre::Result<()> {
        let common = self.common.unwrap();
        let mut common = common.lock();
        common.is_builded = true;

        // todo!();

        for info in common
            .map
            .iter_mut()
            .filter(|info| matches!(info.priority, StartFnPriority::AfterAll))
        {
            match &mut info.source {
                StartSource::ExportFunc(name) => {
                    unimplemented!();
                }
                StartSource::Rewrite(f) => {
                    if let Some(f) = f.take() {
                        f(module, ctx)?;
                    } else {
                        panic!("StartSource::Rewrite function has already been taken");
                    }
                }
            }
        }

        Ok(())
    }
}

impl Generator for StartSectionGenerator {}

#[derive(Debug, Clone)]
pub struct StartSectionBuilder {
    common: Arc<parking_lot::Mutex<StartSectionCommon>>,
}

impl StartSectionBuilder {
    pub fn iter(&self) -> Vec<(StartAlternativeName, FunctionId)> {
        self.common
            .lock()
            .start_alternatives
            .iter()
            .map(|(name, fid)| (name.clone(), *fid))
            .collect::<Vec<_>>()
    }

    pub fn vfs_start_fid(&self) -> (WasmName, FunctionId) {
        self.common.lock().vfs_start_fid()
    }

    pub fn after_memory_reset_fid(&self) -> FunctionId {
        self.common
            .lock()
            .start_alternatives
            .get(&StartAlternativeName::AfterMemoryReset)
            .copied()
            .expect("AfterMemoryReset start alternative must exist")
    }

    pub fn target_wasm_fids(&self) -> Vec<(WasmName, FunctionId)> {
        self.common
            .lock()
            .target_wasm_fids()
            .map(|(name, fid)| (name.clone(), fid))
            .collect::<Vec<_>>()
    }

    pub fn add_start_fn(&self, info: StartFnInfo) {
        self.common.lock().check_is_builded();

        self.common.lock().map.push(info);
    }
}
