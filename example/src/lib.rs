// Use a procedural macro to generate bindings for the world we specified in
// `host.wit`
// wit_bindgen::generate!({
//     // the name of the world in the `*.wit` input file
//     world: "virtual-file-system",
// });
// cargo binstall wit-bindgen-cli -y
// wit-bindgen rust wit
pub mod virtual_file_system;

// Define a custom type and implement the generated `Guest` trait for it which
// represents implementing all the necessary exported interfaces for this
// component.
struct VirtualFileSystem;

impl Guest for VirtualFileSystem {
    fn run() {}

    fn fd_write_export(fd: i32, iovs: i32, iovs_len: i32, written: i32) -> i32 {
        #[inline(always)]
        fn fd_write(fd: u32, iovs: *const *const u8, iovs_len: u32, written: *mut u32) -> u32 {
            0
        }

        fd_write(
            fd as u32,
            iovs as *const *const u8,
            iovs_len as u32,
            written as *mut u32,
        ) as i32
    }

    fn environ_sizes_get_export(environ_count: i32, environ_size: i32) -> i32 {
        #[inline(always)]
        fn environ_sizes_get(environ_count: *mut u32, environ_size: *mut u32) -> u32 {
            0
        }

        environ_sizes_get(environ_count as *mut u32, environ_size as *mut u32) as i32
    }

    fn environ_get_export(environ: i32, environ_buf: i32) -> i32 {
        #[inline(always)]
        fn environ_get(environ: *mut *mut u8, environ_buf: *mut u8) -> u32 {
            0
        }

        environ_get(environ as *mut *mut u8, environ_buf as *mut u8) as i32
    }

    fn proc_exit_export(exit_code: i32) {
        #[inline(always)]
        fn proc_exit(exit_code: u32) -> ! {
            std::process::exit(exit_code as i32);
        }

        proc_exit(exit_code as u32)
    }

    fn fd_write_import_wrap(fd: i32, iovs: i32, iovs_len: i32, written: i32) -> i32 {
        Wasip1::fd_write_import(fd, iovs, iovs_len, written)
    }

    fn environ_sizes_get_import_wrap(environ_count: i32, environ_size: i32) -> i32 {
        Wasip1::environ_sizes_get_import(environ_count, environ_size)
    }

    fn environ_get_import_wrap(environ: i32, environ_buf: i32) -> i32 {
        Wasip1::environ_get_import(environ, environ_buf)
    }

    fn proc_exit_import_wrap(code: i32) -> () {
        Wasip1::proc_exit_import(code)
    }
}

// export! defines that the `VirtualFileSystem` struct defined below is going to define
// the exports of the `world`, namely the `run` function.
export!(VirtualFileSystem);

use virtual_file_system::*;
pub(crate) use wasip1_virtual_layer::*;

// #[unsafe(export_name = "environ_sizes_get")]
// pub unsafe extern "C" fn environ_sizes_get(environ_count: *mut u32, environ_size: *mut u32) -> u32 {
//     0
// }

// #[unsafe(export_name = "environ_get")]
// pub unsafe extern "C" fn environ_get(environ: *const *mut u8, environ_buf: *mut u8) -> u32 {
//     0
// }

// #[unsafe(export_name = "fd_write")]
// pub unsafe extern "C" fn fd_write(
//     fd: u32,
//     iovs: *const *const u8,
//     iovs_len: u32,
//     written: *mut u32,
// ) -> u32 {
//     0
// }

// #[unsafe(export_name = "proc_exit")]
// pub unsafe extern "C" fn proc_exit(exit_code: u32) -> ! {
//     std::process::exit(exit_code as i32);
// }

// #[wasm_bindgen::prelude::wasm_bindgen]
// pub fn greet(a: &str) -> String {
//     format!("Hello, {}!", a)
// }
