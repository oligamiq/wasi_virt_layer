use wasi_virt_layer::memory::WasmAccessRaw;
use wasi_virt_layer::wasi::wrap_unreachable::WrapUnreachable;
use wasi_virt_layer::prelude::*;
use wasi_virt_layer::file::*;
use wasi_virt_layer::process::*;
use const_struct::const_struct;

struct UnreachableHandler;

impl WrapUnreachable for UnreachableHandler {
    fn fix_main_raw_exit_code(code: i32) -> i32 {
        if code == 1 {
            println!("Unreachable occurred, rewriting exit code to 42");
            return 42;
        }
        code
    }
}

import_wasm!(test_unreachable_target);

wasi_virt_layer::wrap_unreachable!(test_unreachable_target, UnreachableHandler);

plug_process!(StandardProcess, test_unreachable_target, self);

#[const_struct]
const HOST_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
    environ: &["HOME=~/"],
};

plug_env!(@embedded, HostEnvTy, test_unreachable_target, self);

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

plug_fs!(&VIRTUAL_FILE_SYSTEM, test_unreachable_target);


#[unsafe(no_mangle)]
pub fn main() {
    println!("Starting VFS...");
    let code = WrapUnreachableTestUnreachableTarget::get_flag();
    println!("Initial flag: {}", code);

    // Call the target wasm
    let raw_code = test_unreachable_target::_main_raw();
    println!("Target exited with raw_code: {:?}", raw_code);

    let flag = WrapUnreachableTestUnreachableTarget::get_flag();
    println!("Flag after execution: {}", flag);

    if flag != 0 {
        let fixed = WrapUnreachableTestUnreachableTarget::fix_main_raw_exit_code(flag);
        println!("Fixed exit code: {}", fixed);
        std::process::exit(fixed);
    }

    std::process::exit(unsafe { core::mem::transmute::<_, u16>(raw_code) } as i32);
}
