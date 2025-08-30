use const_struct::const_struct;
use wasip1_virtual_layer::{self, wasi::file::constant::lfs_raw::*, *};

// mod lib_ex;

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
const FILES: VFSConstNormalFiles<F, 5> = ConstFiles!([(
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
        (
            // file_name
            ".",
            // dir(start, end), parent
            Dir((1, 3), None),
        ),
        (
            // file_name
            "hey",
            // file(content), parent
            File(F::new("Hey!"), 0),
        ),
        ("hello", Dir((3, 5), Some(0))),
        ("world", File(F::new("Hello, world!"), 2)),
        ("everyone", File(F::new("Hello, everyone!"), 2)),
    ],
    pre_open: &[0],
};

type NormalFILES2 = VFSConstNormalFiles<F, 10>;

#[const_struct]
const FILES2: NormalFILES2 = ConstFiles!([
    ("/root", [("root.txt", F::new("This is root"))]),
    (
        ".",
        [
            ("hey", F::new("Hey!")),
            (
                "hello",
                [
                    ("world", F::new("Hello, world!")),
                    ("everyone", F::new("Hello, everyone!")),
                ]
            )
        ]
    ),
    (
        "~",
        [
            ("home", F::new("This is home")),
            ("user", F::new("This is user")),
        ]
    )
]);

#[allow(dead_code)]
const FILE2_EX: NormalFILES2 = VFSConstNormalFiles {
    files: [
        ("/root", Dir((3, 4), None)),
        (".", Dir((4, 6), None)),
        ("~", Dir((6, 8), None)),
        ("root.txt", File(F::new("This is root"), 0)),
        ("hey", File(F::new("Hey!"), 1)),
        ("hello", Dir((8, 10), Some(1))),
        ("home", File(F::new("This is home"), 2)),
        ("user", File(F::new("This is user"), 2)),
        ("world", File(F::new("Hello, world!"), 5)),
        ("everyone", File(F::new("Hello, everyone!"), 5)),
    ],
    pre_open: &[0, 1, 2],
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
