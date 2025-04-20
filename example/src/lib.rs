use wasip1_virtual_layer::{wasi::env::VirtualEnvConstState, *};

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

use const_struct::*;

#[const_struct]
const VIRTUAL_ENV: VirtualEnvConstState = VirtualEnvConstState {
    environ: &["PATH=/usr/local/bin:/usr/bin:/bin"],
};

export_env!(VirtualEnvTy);
