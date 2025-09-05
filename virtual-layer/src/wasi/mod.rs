// https://github.com/bytecodealliance/wasmtime/blob/cff811b55e8b715e037226f2f3c36c65676d319a/crates/wasi-preview1-component-adapter/src/lib.rs#L1655

pub mod env;
pub mod file;
pub mod process;
#[cfg(feature = "threads")]
pub mod thread;

use crate::__private::wasip1::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wasip1_vfs_test_wasm_opt_proc_exit(rval: Exitcode) -> ! {
    #[cfg(feature = "std")]
    {
        std::process::exit(rval as i32);
    }
    #[cfg(not(feature = "std"))]
    {
        unsafe { wasip1::proc_exit(rval) };
        unreachable!();
    }
}
