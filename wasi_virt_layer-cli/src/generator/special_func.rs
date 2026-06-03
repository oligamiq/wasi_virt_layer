

use crate::util::WasmName;

/// Defines unique names associated with special life-cycle functions (startup, reset, main execution).
#[derive(Debug, strum::AsRefStr, strum::EnumCount, Hash, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum SpecialFuncUniqueName<'a> {
    /// Function that initializes memory resets.
    Resetter(&'a WasmName),
    /// Function that handles thread resets.
    ResetOnThread,
    /// Function that ensures thread reset occurs precisely once.
    ResetOnThreadOnce,
    /// Preserved original initialization function.
    StartInitOld,
    /// The primary start wrapper routine function.
    Start(&'a WasmName),
    /// Early application termination safe wrapper function.
    MainVoid(&'a WasmName),
    /// State reset function generated for given module.
    Reset(&'a WasmName),
}


