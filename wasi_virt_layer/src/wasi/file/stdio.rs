use crate::__private::wasip1;
use crate::__private::wasip1::Size;

#[allow(unused_imports)]
use crate::memory::WasmAccess;
use crate::transporter::Wasip1Transporter;

#[cfg(not(feature = "multi_memory"))]
use crate::memory::WasmAccessDynCompatible;

/// Default implementation of `StdIO` using the system's standard I/O.
#[derive(Debug)]
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

    #[cfg(not(feature = "multi_memory"))]
    fn read_direct_dyn_compatible(
        access: &impl WasmAccessDynCompatible,
        buf: *mut u8,
        len: usize,
    ) -> Result<Size, wasip1::Errno> {
        use crate::transporter::Wasip1Transporter;

        Wasip1Transporter::read_from_stdin_direct_dyn_compatible(access, buf, len)
    }

    fn write(buf: &[u8]) -> Result<Size, wasip1::Errno> {
        Wasip1Transporter::write_to_stdout(buf)
    }

    #[cfg(not(feature = "multi_memory"))]
    fn write_direct<Wasm: WasmAccess>(buf: *const u8, len: usize) -> Result<Size, wasip1::Errno> {
        Wasip1Transporter::write_to_stdout_direct::<Wasm>(buf, len)
    }

    #[cfg(not(feature = "multi_memory"))]
    fn write_direct_dyn_compatible(
        access: &impl WasmAccessDynCompatible,
        buf: *const u8,
        len: usize,
    ) -> Result<Size, wasip1::Errno> {
        Wasip1Transporter::write_to_stdout_direct_dyn_compatible(access, buf, len)
    }

    fn ewrite(buf: &[u8]) -> Result<Size, wasip1::Errno> {
        Wasip1Transporter::write_to_stderr(buf)
    }

    #[cfg(not(feature = "multi_memory"))]
    fn ewrite_direct<Wasm: WasmAccess>(buf: *const u8, len: usize) -> Result<Size, wasip1::Errno> {
        Wasip1Transporter::write_to_stderr_direct::<Wasm>(buf, len)
    }

    #[cfg(not(feature = "multi_memory"))]
    fn ewrite_direct_dyn_compatible(
        access: &impl WasmAccessDynCompatible,
        buf: *const u8,
        len: usize,
    ) -> Result<Size, wasip1::Errno> {
        Wasip1Transporter::write_to_stderr_direct_dyn_compatible(access, buf, len)
    }
}

/// Trait for handling standard I/O operations.
pub trait StdIO: core::fmt::Debug {
    /// Reads data from stdin into the provided buffer.
    #[allow(unused_variables)]
    fn read(buf: &mut [u8]) -> Result<Size, wasip1::Errno> {
        Err(wasip1::ERRNO_NOSYS)
    }

    /// Reads data from stdin directly into WASM memory.
    #[cfg(not(feature = "multi_memory"))]
    #[allow(unused_variables)]
    fn read_direct<Wasm: WasmAccess>(buf: *mut u8, len: usize) -> Result<Size, wasip1::Errno> {
        #[cfg(feature = "alloc")]
        {
            use crate::utils::alloc_buff;

            let (_, size) = unsafe {
                alloc_buff(len, |b| {
                    let size = Self::read(b)?;
                    Wasm::memcpy(buf, &b[..size]);
                    Ok(size)
                })
            };
            size
        }

        #[cfg(not(feature = "alloc"))]
        {
            // Stub implementation for non-std environments
            Err(wasip1::ERRNO_NOSYS)
        }
    }

    #[cfg(not(feature = "multi_memory"))]
    #[allow(unused_variables)]
    fn read_direct_dyn_compatible(
        access: &impl WasmAccessDynCompatible,
        buf: *mut u8,
        len: usize,
    ) -> Result<Size, wasip1::Errno> {
        #[cfg(feature = "alloc")]
        {
            use crate::utils::alloc_buff;

            let (_, size) = unsafe {
                alloc_buff(len, |b| {
                    let size = Self::read(b)?;
                    access.memcpy_with(buf, &b[..size]);
                    Ok(size)
                })
            };
            size
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
    /// Writes data to stdout from the provided buffer.
    #[allow(unused_variables)]
    fn write(buf: &[u8]) -> Result<Size, wasip1::Errno> {
        Err(wasip1::ERRNO_NOSYS)
    }

    /// Writes data to stdout directly from WASM memory.
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

    #[cfg(not(feature = "multi_memory"))]
    #[allow(unused_variables)]
    fn write_direct_dyn_compatible(
        access: &impl WasmAccessDynCompatible,
        buf: *const u8,
        len: usize,
    ) -> Result<Size, wasip1::Errno> {
        #[cfg(feature = "alloc")]
        {
            Self::write(&access.get_array_with(buf, len))
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
    /// Writes data to stderr from the provided buffer.
    #[allow(unused_variables)]
    fn ewrite(buf: &[u8]) -> Result<Size, wasip1::Errno> {
        Err(wasip1::ERRNO_NOSYS)
    }

    /// Writes data to stderr directly from WASM memory.
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

    #[cfg(not(feature = "multi_memory"))]
    #[allow(unused_variables)]
    fn ewrite_direct_dyn_compatible(
        access: &impl WasmAccessDynCompatible,
        buf: *const u8,
        len: usize,
    ) -> Result<Size, wasip1::Errno> {
        #[cfg(feature = "alloc")]
        {
            Self::ewrite(&access.get_array_with(buf, len))
        }

        #[cfg(not(feature = "alloc"))]
        {
            // Stub implementation for non-std environments
            Err(wasip1::ERRNO_NOSYS)
        }
    }
}
