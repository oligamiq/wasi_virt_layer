#![cfg_attr(target_arch = "wasm32", no_main)]
#![allow(non_snake_case)]

use wasi_virt_layer::prelude::*;
use wasi_virt_layer::wasi::wrap_unreachable::WrapUnreachable;

struct UnreachableHandler;

impl WrapUnreachable for UnreachableHandler {
    fn fix_main_raw_exit_code<Wasm: WasmAccess>(code: i32) -> i32 {
        if code == 1 {
            println!("Unreachable occurred");
            return 42;
        }
        code
    }
}

import_wasm!(test_unreachable_target1);
import_wasm!(test_unreachable_target2);

wrap_unreachable!(
    UnreachableHandler,
    test_unreachable_target1,
    test_unreachable_target2
);

#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub fn main() {
    println!("Starting Multi-Target Unreachable VFS");
    test_unreachable_target1::_main();
    test_unreachable_target2::_main();
    println!("Done!");
}
