use crate::wasm_stream::pipeline::StreamPass;
use wasm_encoder::{ExportKind, ExportSection, Module, RawSection};

pub struct StartsPreStreamPass {
    pub is_vfs: bool,
    pub is_library: bool,
    pub start_export_name: String,
    /// Additional export name for the Wasm start section function.
    ///
    /// When set (non-VFS targets only), the Wasm start section function is
    /// exported under this name in addition to `start_export_name`. This
    /// allows the VFS to call the start section initializer independently
    /// for reused pool worker threads.
    pub thread_start_export_name: Option<String>,
}

impl StartsPreStreamPass {
    pub fn new(is_vfs: bool, is_library: bool, start_export_name: String) -> Self {
        Self {
            is_vfs,
            is_library,
            start_export_name,
            thread_start_export_name: None,
        }
    }

    /// Sets the additional thread-start export name for target modules.
    ///
    /// When set, the Wasm start section function is also exported under
    /// this name so that `VirtualThreadPool` can call it before reusing
    /// a worker thread for a new logical thread.
    pub fn with_thread_start_export_name(mut self, name: String) -> Self {
        self.thread_start_export_name = Some(name);
        self
    }
}

impl StreamPass for StartsPreStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> eyre::Result<Vec<u8>> {
        let mut module = Module::new();
        let parser = wasmparser::Parser::new(0);

        let mut start_func_id = None;
        let mut has_export_section = false;

        // Pass 1: Find start section
        for payload in wasmparser::Parser::new(0).parse_all(input_wasm) {
            if let wasmparser::Payload::StartSection { func, range: _ } = payload? {
                start_func_id = Some(func);
            }
        }

        for payload in parser.parse_all(input_wasm) {
            let payload = payload?;
            match payload {
                wasmparser::Payload::StartSection { .. } => {
                    // Remove start section by not emitting it
                }
                wasmparser::Payload::ExportSection(s) => {
                    has_export_section = true;
                    let mut exports = ExportSection::new();
                    for export in s {
                        let export = export?;
                        let kind = match export.kind {
                            wasmparser::ExternalKind::Func => ExportKind::Func,
                            wasmparser::ExternalKind::FuncExact => ExportKind::Func,
                            wasmparser::ExternalKind::Table => ExportKind::Table,
                            wasmparser::ExternalKind::Memory => ExportKind::Memory,
                            wasmparser::ExternalKind::Global => ExportKind::Global,
                            wasmparser::ExternalKind::Tag => ExportKind::Tag,
                            _ => unimplemented!(),
                        };

                        if export.name == "_start" && matches!(kind, ExportKind::Func) {
                            start_func_id = Some(export.index);
                        } else {
                            exports.export(export.name, kind, export.index);
                        }
                    }
                    if let Some(func_id) = start_func_id {
                        exports.export(&self.start_export_name, ExportKind::Func, func_id);
                        // For target modules, also export the Wasm start section
                        // function under a separate name so that VirtualThreadPool
                        // can call it when reusing a worker thread.
                        if let Some(ref thread_start_name) = self.thread_start_export_name {
                            exports.export(thread_start_name, ExportKind::Func, func_id);
                        }
                    } else if !self.is_library {
                        // If there is no start function and it's not a library, we'd need to generate a dummy start.
                        // Generating a dummy start here is complex because it requires modifying Function/Code sections.
                        // We will defer dummy start generation to the post-combine pass where it's easier, or just let post-combine handle missing starts.
                        log::warn!(
                            "Module has no start section or _start export but is not marked as a library. Deferred dummy start generation."
                        );
                    }
                    module.section(&exports);
                }
                wasmparser::Payload::CustomSection(s) => {
                    module.section(&wasm_encoder::CustomSection {
                        name: s.name().into(),
                        data: std::borrow::Cow::Borrowed(s.data()),
                    });
                }
                _ => {
                    if let Some((id, range)) = payload.as_section() {
                        module.section(&RawSection {
                            id,
                            data: &input_wasm[range.clone()],
                        });
                    }
                }
            }
        }

        if !has_export_section && start_func_id.is_some() {
            let mut exports = ExportSection::new();
            exports.export(
                &self.start_export_name,
                ExportKind::Func,
                start_func_id.unwrap(),
            );
            if let Some(ref thread_start_name) = self.thread_start_export_name {
                exports.export(thread_start_name, ExportKind::Func, start_func_id.unwrap());
            }
            module.section(&exports);
        }

        Ok(module.finish())
    }
}
