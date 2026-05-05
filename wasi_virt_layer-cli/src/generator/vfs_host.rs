use crate::util::WalrusUtilModule as _;
use super::{Generator, GeneratorCtx};

/// Generator that connects imports from `__wasip1_vfs-host` to identical exports if they exist.
///
/// This is used when the VFS (or another module) wants to call a function that is expected
/// to be provided by the host or another merged module, using a specific "host" module name.
#[derive(Debug, Default)]
pub struct ConnectVfsHost;

impl Generator for ConnectVfsHost {
    fn post_combine(
        &mut self,
        module: &mut walrus::Module,
        _ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        let imports_to_resolve = module
            .imports
            .iter()
            .filter_map(|import| {
                if import.module == "__wasip1_vfs-host" {
                    if let walrus::ImportKind::Function(import_fid) = import.kind {
                        return Some((import.id(), import.name.clone(), import_fid));
                    }
                }
                None
            })
            .collect::<Vec<_>>();

        for (_, name, import_fid) in imports_to_resolve {
            let export_fid = module.exports.iter().find_map(|export| {
                if export.name == name {
                    if let walrus::ExportItem::Function(export_fid) = export.item {
                        return Some(export_fid);
                    }
                }
                None
            });

            if let Some(export_fid) = export_fid {
                log::info!("Connecting __wasip1_vfs-host import '{}' to matching export.", name);
                // Connect the import to the export without removing the export.
                module.connect_func_without_remove(import_fid, export_fid)?;
            }
        }
        Ok(())
    }
}
