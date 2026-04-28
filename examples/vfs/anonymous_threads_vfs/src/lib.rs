use const_struct::const_struct;
use wasi_virt_layer::{file::*, plug_process, prelude::*};

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
        anonymous::_start();
    }

    fn main() {
        anonymous::_reset();
        anonymous::_start();
        // println!("Starting _main...");
        anonymous::_main();
        // println!("_main done.");
    }
}

#[cfg(not(test))]
export!(Starter);

import_wasm!(<anonymous>);

const FILE_COUNT: usize = 5;

type F = WasiEmbeddedFile<&'static str>;
type NormalFILES = StandardEmbeddedFiles<F, { FILE_COUNT }>;

#[const_struct]
const EMBEDDED_FILES: NormalFILES = EmbeddedFiles!([(
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

plug_thread!(
    { wasi_virt_layer::thread::DirectThreadPool::<ThreadAccessor>::new_const() },
    self,
    anonymous
);

plug_process!(
    wasi_virt_layer::process::StandardProcess,
    anonymous,
    self
);

#[const_struct]
const VIRTUAL_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
    environ: &[
        // "RUST_MIN_STACK=16777216",
        "HOME=~/",
        // "RUST_BACKTRACE=full",
    ],
};
plug_env!(@embedded, VirtualEnvTy, anonymous, self);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_files() {
        println!("Files: {:#?}", EMBEDDED_FILES);
    }
}

mod fs {
    use super::*;

    type LFS = StandardEmbeddedNormalLFS<EmbeddedFilesTy, F, FILE_COUNT, DefaultStdIO>;

    static VIRTUAL_FILE_SYSTEM: StandardEmbeddedFileSystem<LFS, FILE_COUNT> =
        StandardEmbeddedFileSystem::new_const(StandardEmbeddedNormalLFS::new_const());

    plug_fs!(&VIRTUAL_FILE_SYSTEM, anonymous, self);
}
