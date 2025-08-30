use const_struct::const_struct;
use wasip1_virtual_layer::{self, wasi::file::constant::lfs_raw::*, *};

// wit_bindgen::generate!({
//     // the name of the world in the `*.wit` input file
//     world: "init",
// });

// struct Starter;

// impl Guest for Starter {
//     fn init() -> () {}

//     fn start() -> () {
//         println!("Files: {:?}", FILES);

//         todo!()
//     }
// }

// export!(Starter);

// import_wasm!(test_threads);

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

use wasip1_virtual_layer::wasi::file::constant::lfs_raw::VFSConstNormalInode::Dir;
use wasip1_virtual_layer::wasi::file::constant::lfs_raw::VFSConstNormalInode::File;

#[allow(dead_code)]
const FILE_EX: NormalFILES = VFSConstNormalFiles {
    files: [
        (".", Dir((1, 3), None)),
        ("hey", File(WasiConstFile { file: "Hey!" }, 0)),
        ("hello", Dir((3, 5), Some(0))),
        (
            "world",
            File(
                WasiConstFile {
                    file: "Hello, world!",
                },
                2,
            ),
        ),
        (
            "everyone",
            File(
                WasiConstFile {
                    file: "Hello, everyone!",
                },
                2,
            ),
        ),
    ],
    pre_open: &[0],
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_files() {
        println!("Files: {:?}", FILES);
    }
}

// mod fs {
//     use super::*;
//     use wasip1_virtual_layer::{
//         export_fs,
//         wasi::file::{
//             constant::{lfs::VFSConstNormalLFS, vfs::Wasip1ConstVFS},
//             stdio::DefaultStdIO,
//         },
//     };

//     type LFS = VFSConstNormalLFS<FilesTy, F, FILE_COUNT, DefaultStdIO>;

//     static mut VIRTUAL_FILE_SYSTEM: Wasip1ConstVFS<LFS, FILE_COUNT> =
//         Wasip1ConstVFS::new(VFSConstNormalLFS::new());

//     export_fs!(@const, {
//         #[allow(static_mut_refs)]
//         unsafe { &mut VIRTUAL_FILE_SYSTEM }
//     }, test_threads);
// }
