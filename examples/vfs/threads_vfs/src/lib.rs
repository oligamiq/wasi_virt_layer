use const_struct::const_struct;
use wasip1_virtual_layer::{file::*, plug_process, prelude::*, thread::DirectThreadPool};

wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "init",
});

struct Starter;

impl Guest for Starter {
    fn init() {
        fn print_loop() {
            for i in 0..1000 {
                println!("Hello from a thread spawned in the `init` function! {i}");
            }
        }

        let handle = std::thread::spawn(|| {
            print_loop();
        });

        print_loop();

        handle.join().unwrap();

        println!("`init` function done.");
    }

    fn start() {
        test_threads::_start();
    }

    fn main() {
        // test_threads::reset();
        // test_threads::_start();
        // println!("Starting _main...");
        test_threads::_main();
        // println!("_main done.");
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

plug_thread!(DirectThreadPool, self, test_threads);
plug_process!(test_threads);
#[const_struct]
const VIRTUAL_ENV: VirtualEnvConstState = VirtualEnvConstState {
    environ: &[
        // "RUST_MIN_STACK=16777216",
        "HOME=~/",
        // "RUST_BACKTRACE=full",
    ],
};
plug_env!(@const, VirtualEnvTy, test_threads);

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

    plug_fs!(@const, {
        #[allow(static_mut_refs)]
        unsafe { &mut VIRTUAL_FILE_SYSTEM }
    }, test_threads, self);
}
