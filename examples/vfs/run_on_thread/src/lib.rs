use const_struct::const_struct;
use wasi_virt_layer::{file::*, plug_thread, prelude::*, process::*, thread::VirtualThreadPool};

use crate::arg::set_rustc_opt_args;

struct ComponentABI;

wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "component-abi",
});

import_wasm!(<anonymous>);

impl Guest for ComponentABI {
    fn init() {
        // initialization logic if needed
    }

    fn start() {
        anonymous::_start();
    }

    fn main() {
        unsafe {
            THREAD_POOL.init();
            THREAD_POOL.flush_capacity().wait();
        }

        std::thread::spawn(|| {
            for _ in 0..2 {
                set_rustc_opt_args(&["rustc", "###"]);
                anonymous::_reset();
                anonymous::_start();
                anonymous::_main();
            }
        })
        .join()
        .unwrap();
    }
}

#[cfg(not(test))]
export!(ComponentABI);

static THREAD_POOL: VirtualThreadPool<ThreadAccessor> = unsafe { VirtualThreadPool::new_const(8) };

plug_thread!({ &THREAD_POOL }, anonymous, self);

mod env {
    use super::*;

    #[const_struct]
    const HOST_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
        environ: &["HOME=~/"],
    };

    plug_env!(@embedded, HostEnvTy, anonymous, self);
}

mod arg {
    use super::*;
    use parking_lot::Mutex;
    use std::sync::LazyLock;
    use wasi_virt_layer::plug_args;

    pub struct VirtualArgsState {
        pub args: Vec<String>,
    }

    impl<'a> VirtualArgs<'a> for VirtualArgsState {
        type Str = String;
        fn get_args(&mut self) -> &[Self::Str] {
            &self.args
        }
    }

    pub static VIRTUAL_ARGS: LazyLock<Mutex<VirtualArgsState>> =
        LazyLock::new(|| Mutex::new(VirtualArgsState { args: vec![] }));

    pub fn set_rustc_opt_args(args: &[impl AsRef<str>]) {
        VIRTUAL_ARGS.lock().args = args.iter().map(|s| s.as_ref().to_string()).collect();
    }

    plug_args!(@dynamic, { &mut VIRTUAL_ARGS.lock() }, anonymous, self);
}

mod fs {
    use super::*;

    const FILE_COUNT: usize = 10;

    #[const_struct]
    const EMBEDDED_FILES: StandardEmbeddedFiles<WasiEmbeddedFile<&'static str>, { FILE_COUNT }> =
        EmbeddedFiles!([
            (
                "/root",
                [("root.txt", WasiEmbeddedFile::new("This is root"))]
            ),
            (
                ".",
                [
                    ("hey", WasiEmbeddedFile::new("Hey!")),
                    (
                        "hello",
                        [
                            ("world", WasiEmbeddedFile::new("Hello, world!")),
                            ("everyone", WasiEmbeddedFile::new("Hello, everyone!")),
                        ]
                    )
                ]
            ),
            (
                "~",
                [
                    ("home", WasiEmbeddedFile::new("This is home")),
                    ("user", WasiEmbeddedFile::new("This is user")),
                ]
            )
        ]);

    type LFS = StandardEmbeddedNormalLFS<
        EmbeddedFilesTy,
        WasiEmbeddedFile<&'static str>,
        FILE_COUNT,
        DefaultStdIO,
    >;

    static VIRTUAL_FILE_SYSTEM: StandardEmbeddedFileSystem<LFS, FILE_COUNT> =
        StandardEmbeddedFileSystem::new_const(StandardEmbeddedNormalLFS::new_const());

    plug_fs!(&VIRTUAL_FILE_SYSTEM, anonymous, self);
}

mod process {
    use super::*;
    use wasi_virt_layer::memory::WasmAccessName;
    use wasi_virt_layer::prelude::*;
    use wasi_virt_layer::wasi::wrap_unreachable::WrapUnreachable;

    plug_random!(StandardRandom, anonymous, self);

    pub const SUCCESS_FLAG: i32 = 999;

    pub struct CustomProcess;

    impl ProcessExit for CustomProcess {
        fn proc_exit<Wasm: WasmAccess + WasmAccessName + 'static>(code: i32) {
            if code == 0 {
                match Wasm::NAME {
                    anonymous::NAME => WrapUnreachableAnonymous::set_flag(SUCCESS_FLAG),
                    _ => unreachable!(),
                }
            }
        }
    }

    wasi_virt_layer::plug_process!(CustomProcess, anonymous);

    pub struct UnreachableHandler;

    impl WrapUnreachable for UnreachableHandler {
        fn fix_main_raw_exit_code<Wasm: WasmAccess + WasmAccessName + 'static>(code: i32) -> i32 {
            if code == 0 || code == SUCCESS_FLAG {
                0
            } else {
                code
            }
        }
    }

    wasi_virt_layer::wrap_unreachable!(UnreachableHandler, anonymous);
}
