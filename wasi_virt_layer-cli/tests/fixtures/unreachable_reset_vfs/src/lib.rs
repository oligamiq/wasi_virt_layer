use const_struct::const_struct;
use wasi_virt_layer::file::*;
use wasi_virt_layer::memory::WasmAccessRaw;
use wasi_virt_layer::prelude::*;

struct ComponentABI;

wit_bindgen::generate!({
    world: "component-abi",
});

import_wasm!(test_unreachable_target);
wasi_virt_layer::own_memory!(test_unreachable_target);
wasi_virt_layer::wrap_unreachable!(test_unreachable_target);
plug_process!(StandardProcess, test_unreachable_target, self);

#[const_struct]
const HOST_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState { environ: &[] };

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

impl Guest for ComponentABI {
    fn main() {
        test_unreachable_target::_reset();

        let initial = WrapUnreachableTestUnreachableTarget::get_flag();
        println!("unreachable reset initial flag = {initial}");
        assert_eq!(initial, 0);

        let _ = test_unreachable_target::_main_raw();
        let after_first = WrapUnreachableTestUnreachableTarget::get_flag();
        println!("unreachable reset after-first flag = {after_first}");
        assert_eq!(after_first, 1);

        test_unreachable_target::_reset();
        let after_reset = WrapUnreachableTestUnreachableTarget::get_flag();
        println!("unreachable reset after-reset flag = {after_reset}");
        assert_eq!(after_reset, 0);

        let _ = test_unreachable_target::_main_raw();
        let after_second = WrapUnreachableTestUnreachableTarget::get_flag();
        println!("unreachable reset after-second flag = {after_second}");
        assert_eq!(after_second, 1);

        println!("unreachable reset flag test passed");
    }
}

#[cfg(not(test))]
export!(ComponentABI);
