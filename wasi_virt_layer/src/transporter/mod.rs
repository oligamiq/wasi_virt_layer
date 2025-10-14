#![allow(dead_code)]

use crate::__private::wasip1;

pub struct Wasip1Transporter;

#[cfg(not(feature = "multi_memory"))]
use crate::prelude::WasmAccess;

unsafe fn non_recursive_fd_read(
    fd: wasip1::Fd,
    iovs: wasip1::IovecArray<'_>,
) -> Result<wasip1::Size, wasip1::Errno> {
    let mut rp0 = core::mem::MaybeUninit::<wasip1::Size>::uninit();

    let fd = fd as i32;
    let iovs_ptr = iovs.as_ptr() as i32;
    let iovs_len = iovs.len() as i32;
    let rp0_ptr = rp0.as_mut_ptr() as i32;

    let ret = crate::non_recursive_wasi_snapshot_preview1!(
        fd_read(
            fd: i32,
            iovs_ptr: i32,
            iovs_len: i32,
            rp0_ptr: i32
        ) -> i32
    );

    match ret {
        0 => Ok(unsafe { core::ptr::read(rp0.as_mut_ptr() as i32 as *const wasip1::Size) }),
        _ => Err(unsafe { core::mem::transmute::<u16, wasip1::Errno>(ret as u16) }),
    }
}

unsafe fn non_recursive_fd_write(
    fd: wasip1::Fd,
    iovs: wasip1::CiovecArray<'_>,
) -> Result<wasip1::Size, wasip1::Errno> {
    let mut rp0 = core::mem::MaybeUninit::<wasip1::Size>::uninit();

    let fd = fd as i32;
    let iovs_ptr = iovs.as_ptr() as i32;
    let iovs_len = iovs.len() as i32;
    let rp0_ptr = rp0.as_mut_ptr() as i32;

    let ret = crate::non_recursive_wasi_snapshot_preview1!(
        fd_write(
            fd: i32,
            iovs_ptr: i32,
            iovs_len: i32,
            rp0_ptr: i32
        ) -> i32
    );

    match ret {
        0 => Ok(unsafe { core::ptr::read(rp0.as_mut_ptr() as i32 as *const wasip1::Size) }),
        _ => Err(unsafe { core::mem::transmute::<u16, wasip1::Errno>(ret as u16) }),
    }
}

unsafe fn non_recursive_proc_exit(rval: wasip1::Exitcode) -> ! {
    let rval = rval as i32;

    crate::non_recursive_wasi_snapshot_preview1!(
        proc_exit(rval: i32) -> ()
    );

    unreachable!("proc_exit should not return");
}

impl Wasip1Transporter {
    #[allow(unused_variables)]
    pub fn read_from_stdin(buf: &mut [u8]) -> Result<wasip1::Size, wasip1::Errno> {
        #[cfg(target_os = "wasi")]
        {
            let iovec_arr = [wasip1::Iovec {
                buf: buf.as_mut_ptr() as *mut u8,
                buf_len: buf.len(),
            }];

            unsafe { non_recursive_fd_read(wasip1::FD_STDIN, &iovec_arr) }
        }

        #[cfg(not(target_os = "wasi"))]
        {
            unimplemented!("this is not supported on this architecture");
        }
    }

    #[cfg(not(feature = "multi_memory"))]
    #[allow(unused_variables)]
    pub fn read_from_stdin_direct<Wasm: WasmAccess>(
        buf: *mut u8,
        len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        #[cfg(target_os = "wasi")]
        {
            let iovec_arr = [wasip1::Iovec {
                buf: Wasm::memory_director_mut(buf),
                buf_len: len,
            }];

            unsafe { non_recursive_fd_read(wasip1::FD_STDIN, &iovec_arr) }
        }

        #[cfg(not(target_os = "wasi"))]
        {
            unimplemented!("this is not supported on this architecture");
        }
    }

    #[allow(unused_variables)]
    pub fn write_to_stdout(data: &[u8]) -> Result<wasip1::Size, wasip1::Errno> {
        #[cfg(target_os = "wasi")]
        {
            let ciovec_arr = [wasip1::Ciovec {
                buf: data.as_ptr() as *const u8,
                buf_len: data.len(),
            }];

            unsafe { non_recursive_fd_write(wasip1::FD_STDOUT, &ciovec_arr) }
        }

        #[cfg(not(target_os = "wasi"))]
        {
            unimplemented!("this is not supported on this architecture");
        }
    }

    #[cfg(not(feature = "multi_memory"))]
    #[allow(unused_variables)]
    pub fn write_to_stdout_direct<Wasm: WasmAccess>(
        buf: *const u8,
        len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        let ciovec_arr = [wasip1::Ciovec {
            buf: Wasm::memory_director(buf),
            buf_len: len,
        }];

        #[cfg(target_os = "wasi")]
        unsafe {
            non_recursive_fd_write(wasip1::FD_STDOUT, &ciovec_arr)
        }

        #[cfg(not(target_os = "wasi"))]
        {
            unimplemented!("this is not supported on this architecture");
        }
    }

    #[allow(unused_variables)]
    pub fn write_to_stderr(data: &[u8]) -> Result<wasip1::Size, wasip1::Errno> {
        #[cfg(target_os = "wasi")]
        {
            let ciovec_arr = [wasip1::Ciovec {
                buf: data.as_ptr() as *const u8,
                buf_len: data.len(),
            }];

            unsafe { non_recursive_fd_write(wasip1::FD_STDERR, &ciovec_arr) }
        }

        #[cfg(not(target_os = "wasi"))]
        {
            unimplemented!("this is not supported on this architecture");
        }
    }

    #[cfg(not(feature = "multi_memory"))]
    #[allow(unused_variables)]
    pub fn write_to_stderr_direct<Wasm: WasmAccess>(
        buf: *const u8,
        len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        #[cfg(target_os = "wasi")]
        {
            let ciovec_arr = [wasip1::Ciovec {
                buf: Wasm::memory_director(buf),
                buf_len: len,
            }];

            unsafe { non_recursive_fd_write(wasip1::FD_STDERR, &ciovec_arr) }
        }

        #[cfg(not(target_os = "wasi"))]
        {
            unimplemented!("this is not supported on this architecture");
        }
    }

    #[allow(unused_variables)]
    pub fn process_abort(rval: wasip1::Exitcode) {
        #[cfg(not(target_os = "wasi"))]
        unimplemented!("this is not supported on this architecture");

        #[cfg(target_os = "wasi")]
        {
            // This is a no-op in wasm, as wasm does not support unwinding.
            // If you need to handle unwinding, you should use a different mechanism.
            // For example, you can use `wasip1::exit` to exit the process.
            unsafe { non_recursive_proc_exit(rval) };
        }
    }
}
