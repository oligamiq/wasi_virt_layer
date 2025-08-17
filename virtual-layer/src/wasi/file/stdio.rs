use wasip1::Size;

use crate::memory::WasmAccess;
use crate::transporter::Wasip1Transporter;

pub struct DefaultStdIO;

impl StdIO for DefaultStdIO {
    fn read(buf: &mut [u8]) -> Result<Size, wasip1::Errno> {
        Wasip1Transporter::read_from_stdin(buf)
    }

    #[cfg(not(feature = "multi_memory"))]
    fn read_direct<Wasm: WasmAccess>(buf: *mut u8, len: usize) -> Result<Size, wasip1::Errno> {
        use crate::transporter::Wasip1Transporter;

        Wasip1Transporter::read_from_stdin_direct::<Wasm>(buf, len)
    }

    fn write(buf: &[u8]) -> Result<Size, wasip1::Errno> {
        Wasip1Transporter::write_to_stdout(buf)
    }

    #[cfg(not(feature = "multi_memory"))]
    fn write_direct<Wasm: WasmAccess>(buf: *const u8, len: usize) -> Result<Size, wasip1::Errno> {
        Wasip1Transporter::write_to_stdout_direct::<Wasm>(buf, len)
    }

    fn ewrite(buf: &[u8]) -> Result<Size, wasip1::Errno> {
        Wasip1Transporter::write_to_stderr(buf)
    }

    #[cfg(not(feature = "multi_memory"))]
    fn ewrite_direct<Wasm: WasmAccess>(buf: *const u8, len: usize) -> Result<Size, wasip1::Errno> {
        Wasip1Transporter::write_to_stderr_direct::<Wasm>(buf, len)
    }
}

pub trait StdIO {
    #[allow(unused_variables)]
    fn read(buf: &mut [u8]) -> Result<Size, wasip1::Errno> {
        Err(wasip1::ERRNO_NOSYS)
    }

    #[cfg(not(feature = "multi_memory"))]
    #[allow(unused_variables)]
    fn read_direct<Wasm: WasmAccess>(buf: *mut u8, len: usize) -> Result<Size, wasip1::Errno> {
        #[cfg(feature = "alloc")]
        {
            let mut buffer = {
                let mut vec = alloc::vec::Vec::with_capacity(len);
                unsafe { vec.set_len(len) };
                vec
            };
            Self::read(&mut buffer)?;
            Wasm::memcpy(buf, &buffer);
            Ok(buffer.len())
        }

        #[cfg(not(feature = "alloc"))]
        {
            // Stub implementation for non-std environments
            Err(wasip1::ERRNO_NOSYS)
        }
    }

    /// This function is called when the alloc feature is ON
    /// and write_direct is not implemented.
    /// If you are not familiar with Wasm memory, etc.,
    /// it is better to use this.
    #[allow(unused_variables)]
    fn write(buf: &[u8]) -> Result<Size, wasip1::Errno> {
        Err(wasip1::ERRNO_NOSYS)
    }

    /// This function is called,
    /// but if the write function is implemented
    /// and the alloc feature is ON,
    /// this function is automatically implemented.
    #[cfg(not(feature = "multi_memory"))]
    #[allow(unused_variables)]
    fn write_direct<Wasm: WasmAccess>(buf: *const u8, len: usize) -> Result<Size, wasip1::Errno> {
        #[cfg(feature = "alloc")]
        {
            Self::write(&Wasm::get_array(buf, len))
        }

        #[cfg(not(feature = "alloc"))]
        {
            // Stub implementation for non-std environments
            Err(wasip1::ERRNO_NOSYS)
        }
    }

    /// This function is called when the alloc feature is ON
    /// and ewrite_direct is not implemented.
    /// If you are not familiar with Wasm memory, etc.,
    /// it is better to use this.
    #[allow(unused_variables)]
    fn ewrite(buf: &[u8]) -> Result<Size, wasip1::Errno> {
        Err(wasip1::ERRNO_NOSYS)
    }

    /// This function is called,
    /// but if the ewrite function is implemented
    /// and the alloc feature is ON,
    /// this function is automatically implemented.
    #[cfg(not(feature = "multi_memory"))]
    #[allow(unused_variables)]
    fn ewrite_direct<Wasm: WasmAccess>(buf: *const u8, len: usize) -> Result<Size, wasip1::Errno> {
        #[cfg(feature = "alloc")]
        {
            Self::ewrite(&Wasm::get_array(buf, len))
        }

        #[cfg(not(feature = "alloc"))]
        {
            // Stub implementation for non-std environments
            Err(wasip1::ERRNO_NOSYS)
        }
    }
}
