use std::sync::{LazyLock, Mutex};

use wasip1_virtual_layer::{
    wasi::env::{VirtualEnv, VirtualEnvConstState},
    *,
};

wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "hello",
});

struct Hello;

impl Guest for Hello {
    fn world() {
        println!("Hello, world!");
    }
}

export!(Hello);

import_wasm!(test_wasm);

// Use const to optimize as much as possible at compile time
use const_struct::*;

#[const_struct]
const VIRTUAL_ENV: VirtualEnvConstState = VirtualEnvConstState {
    environ: &["RUST_MIN_STACK=16777216"],
};

export_env!(@const, VirtualEnvTy, test_wasm);

// struct VirtualEnvState {
//     environ: Vec<String>,
// }

// impl<'a> VirtualEnv<'a> for VirtualEnvState {
//     type Str = String;

//     fn get_environ(&mut self) -> &[Self::Str] {
//         &self.environ
//     }
// }

// static VIRTUAL_ENV: LazyLock<Mutex<VirtualEnvState>> = LazyLock::new(|| {
//     let mut environ = Vec::<String>::new();
//     environ.push("RUST_MIN_STACK=16777216".into());
//     environ.push("HOME=~/".into());
//     Mutex::new(VirtualEnvState { environ })
// });

// export_env!(@static, &mut VIRTUAL_ENV.lock().unwrap(), test_wasm);
