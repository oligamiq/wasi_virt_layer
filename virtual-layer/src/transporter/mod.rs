use crate::memory::WasmAccess;

pub struct Wasip1Transporter;

impl Wasip1Transporter {
    pub fn read_from_stdin(buf: &mut [u8]) -> Result<wasip1::Size, wasip1::Errno> {
        let iovec_arr = [wasip1::Iovec {
            buf: buf.as_mut_ptr() as *mut u8,
            buf_len: buf.len(),
        }];

        unsafe { wasip1::fd_read(wasip1::FD_STDIN, &iovec_arr) }
    }

    #[cfg(not(feature = "multi_memory"))]
    pub fn read_from_stdin_direct<Wasm: WasmAccess>(
        buf: *mut u8,
        len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        let iovec_arr = [wasip1::Iovec {
            buf: Wasm::memory_director_mut(buf),
            buf_len: len,
        }];

        unsafe { wasip1::fd_read(wasip1::FD_STDIN, &iovec_arr) }
    }

    pub fn write_to_stdout(data: &[u8]) -> Result<wasip1::Size, wasip1::Errno> {
        let ciovec_arr = [wasip1::Ciovec {
            buf: data.as_ptr() as *const u8,
            buf_len: data.len(),
        }];

        unsafe { wasip1::fd_write(wasip1::FD_STDOUT, &ciovec_arr) }
    }

    #[cfg(not(feature = "multi_memory"))]
    pub fn write_to_stdout_direct<Wasm: WasmAccess>(
        buf: *const u8,
        len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        let ciovec_arr = [wasip1::Ciovec {
            buf: Wasm::memory_director(buf),
            buf_len: len,
        }];

        unsafe { wasip1::fd_write(wasip1::FD_STDOUT, &ciovec_arr) }
    }

    pub fn write_to_stderr(data: &[u8]) -> Result<wasip1::Size, wasip1::Errno> {
        let ciovec_arr = [wasip1::Ciovec {
            buf: data.as_ptr() as *const u8,
            buf_len: data.len(),
        }];

        unsafe { wasip1::fd_write(wasip1::FD_STDERR, &ciovec_arr) }
    }

    #[cfg(not(feature = "multi_memory"))]
    pub fn write_to_stderr_direct<Wasm: WasmAccess>(
        buf: *const u8,
        len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        let ciovec_arr = [wasip1::Ciovec {
            buf: Wasm::memory_director(buf),
            buf_len: len,
        }];

        unsafe { wasip1::fd_write(wasip1::FD_STDERR, &ciovec_arr) }
    }

    pub fn process_abort(rval: wasip1::Exitcode) {
        #[cfg(not(target_os = "wasi"))]
        unimplemented!("this is not supported on this architecture");

        #[cfg(target_os = "wasi")]
        {
            // This is a no-op in wasm, as wasm does not support unwinding.
            // If you need to handle unwinding, you should use a different mechanism.
            // For example, you can use `wasip1::exit` to exit the process.
            unsafe { wasip1::proc_exit(rval) };
        }
    }
}
