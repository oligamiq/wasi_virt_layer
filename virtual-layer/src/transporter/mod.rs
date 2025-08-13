use crate::memory::WasmAccess;

pub struct Wasip1Transporter;

#[cfg(feature = "alloc")]
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
unsafe extern "C" fn allocator(size: usize) -> *mut u8 {
    let layout = alloc::alloc::Layout::from_size_align(size, 0).unwrap();
    unsafe { alloc::alloc::alloc(layout) }
}

impl Wasip1Transporter {
    pub fn write_to_stdout(data: &[u8]) -> Result<wasip1::Size, wasip1::Errno> {
        let ciovec_arr = [wasip1::Ciovec {
            buf: data.as_ptr() as *const u8,
            buf_len: data.len(),
        }];

        unsafe { wasip1::fd_write(wasip1::FD_STDOUT, &ciovec_arr) }
    }

    pub fn write_to_stdout_direct<Wasm: WasmAccess>(
        buf: *const u8,
        len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        let ciovec_arr = [wasip1::Ciovec {
            buf: Wasm::memory_directer(buf),
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

    pub fn write_to_stderr_direct<Wasm: WasmAccess>(
        buf: *const u8,
        len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        let ciovec_arr = [wasip1::Ciovec {
            buf: Wasm::memory_directer(buf),
            buf_len: len,
        }];

        unsafe { wasip1::fd_write(wasip1::FD_STDERR, &ciovec_arr) }
    }

    pub fn process_unwind(rval: wasip1::Exitcode) {
        #[cfg(not(target_arch = "wasm32"))]
        unimplemented!("this is not supported on this architecture");

        #[cfg(target_arch = "wasm32")]
        {
            // This is a no-op in wasm, as wasm does not support unwinding.
            // If you need to handle unwinding, you should use a different mechanism.
            // For example, you can use `wasip1::exit` to exit the process.
            unsafe { wasip1::proc_exit(rval) };
        }
    }
}
