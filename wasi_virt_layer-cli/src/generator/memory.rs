use eyre::{Context as _, ContextCompat as _};
use walrus::ir;

use crate::{
    args::TargetMemoryType,
    generator::{Generator, GeneratorCtx},
    unique_name::UniqueName,
    util::{ResultUtil as _, WalrusFID, WalrusUtilExport, WasmName},
};

/// Encapsulates naming variants reserved for VFS memory interactions.
#[derive(Debug, strum::AsRefStr, strum::EnumCount, Hash, PartialEq, Eq, strum::VariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum MemoryUniqueName<'a> {
    /// Copies contents from Target Memory to VFS Memory.
    MemoryCopyFrom(&'a WasmName),
    /// Copies contents from VFS Memory to Target Memory.
    MemoryCopyTo(&'a WasmName),
    /// Emplaces a runtime trap to deduce pointer arithmetic limits.
    MemoryTrap(&'a WasmName),
    /// Serves as a linked anchor ensuring `MemoryTrap` preserves its compiled identity.
    MemoryTrapAnchor(&'a WasmName),
    /// Wraps dynamic function dispatch controlling pointers to merged targets.
    MemoryDirector(&'a WasmName),
    /// Serves as a linked anchor preserving the compiled `MemoryDirector` identity.
    MemoryDirectorAnchor(&'a WasmName),
    /// The fundamental reference identifying the exported memory location of the underlying module.
    Memory(&'a WasmName),
}

/// ABI Generator: Manages memory configurations temporarily during transpilation.
/// When exchanging data via Wasip1ABI,
/// there are operations involving writing to
/// and reading from memory.
/// However, as these cannot be accessed during compilation,
/// alternative functions are employed. These shall be replaced.
///
/// **Why this is needed:**
/// Since the VFS layer needs to transfer data between its own memory and the guest's memory
/// when processing ABI calls, it defines `memory_copy_from` and `memory_copy_to` as external imports.
/// Since `walrus` does not allow generating components with unknown raw imports, this generator
/// patches those structural import declarations with native `memory.copy` WebAssembly instructions
/// executing over the correct `multi-memory` tables.
#[derive(Debug, Default)]
pub struct MemoryBridge;

macro_rules! assert_ptr {
    ($ptr:expr) => {
        if { $ptr } != walrus::ValType::I32 {
            let ptr = $ptr;
            eyre::bail!("Invalid pointer type, expected i32. Got {ptr}");
        }
    };
}

macro_rules! assert_len {
    ($len:expr) => {
        if { $len } != walrus::ValType::I32 {
            let len = $len;
            eyre::bail!("Invalid length type, expected i32. Got {len}");
        }
    };
}

macro_rules! check_len {
    ($params:expr, $len:expr) => {
        if { $params.len() } != { $len } {
            let len = $len;
            eyre::bail!(
                "Invalid params length, expected {len}. Got {}",
                { $params }.len()
            );
        }
    };
}

impl Generator for MemoryBridge {
    fn post_combine(
        &mut self,
        _module: &mut walrus::Module,
        _ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        Ok(())
    }
}

/// The final wasm, due to ABI constraints,
/// only exposes vfs memory.
/// Therefore, when calling the ABI from non-vfs memory,
/// data must be copied. However,
/// when ultimately consolidating memory into a single pool,
/// data can be passed externally by directly passing pointers.
/// To implement this optimization,
/// a function is provided to determine the pointer bias
/// before memory consolidation.
///
/// **Why this is needed:**
/// If running in `single_memory` mode (without the `multi_memory` WASM feature), all modules'
/// memories are merged into one large address space. The VFS needs to know where the guest
/// module's memory segment actually resides within this merged pool. This generator uses a trap
/// mechanism to securely determine the actual pointer bias at runtime so external callers and
/// the VFS can translate addresses correctly.
#[derive(Debug, Default)]
pub struct MemoryTrap;

impl Generator for MemoryTrap {
    fn post_combine(
        &mut self,
        _module: &mut walrus::Module,
        _ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        Ok(())
    }
}
