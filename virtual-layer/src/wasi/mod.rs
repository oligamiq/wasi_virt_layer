// https://github.com/bytecodealliance/wasmtime/blob/cff811b55e8b715e037226f2f3c36c65676d319a/crates/wasi-preview1-component-adapter/src/lib.rs#L1655

pub mod env;
pub mod file;
pub mod process;
#[cfg(feature = "threads")]
pub mod thread;
