use const_struct::const_struct;
use wasip1_virtual_layer::{export_process, file::*, prelude::*, thread::DirectThreadPool};

wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "init",
});

struct Starter;

impl Guest for Starter {
    fn init() -> () {}

    fn start() {
        test_threads::_start();
    }

    fn main() {
        println!("Starting main... and resetting first.");
        test_threads::reset();
        println!("Reset done. Starting _start...");
        test_threads::_start();
        println!("_start done. Starting _main...");
        test_threads::_main();
        println!("_main done.");
    }
}

export!(Starter);

import_wasm!(test_threads);

const FILE_COUNT: usize = 5;

type F = WasiConstFile<&'static str>;
type NormalFILES = VFSConstNormalFiles<F, { FILE_COUNT }>;

#[const_struct]
const FILES: NormalFILES = ConstFiles!([(
    ".",
    [
        ("hey", F::new("Hey!")),
        (
            "hello",
            [
                ("world", F::new("Hello, world!")),
                ("everyone", F::new("Hello, everyone!")),
            ],
        ),
    ],
)]);

export_thread!(DirectThreadPool, self, test_threads);
export_process!(test_threads);
#[const_struct]
const VIRTUAL_ENV: VirtualEnvConstState = VirtualEnvConstState {
    environ: &["RUST_MIN_STACK=16777216", "HOME=~/"],
};
export_env!(@block, @const, VirtualEnvTy, test_threads);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_files() {
        println!("Files: {:#?}", FILES);
    }
}

mod fs {
    use super::*;

    type LFS = VFSConstNormalLFS<FilesTy, F, FILE_COUNT, DefaultStdIO>;

    static mut VIRTUAL_FILE_SYSTEM: Wasip1ConstVFS<LFS, FILE_COUNT> =
        Wasip1ConstVFS::new(VFSConstNormalLFS::new());

    export_fs!(@const, {
        #[allow(static_mut_refs)]
        unsafe { &mut VIRTUAL_FILE_SYSTEM }
    }, test_threads);
}
