pub mod wasip1_with_memory;

pub struct Wasip1Transporter;

impl Wasip1Transporter {
    pub fn write_to_stdout(data: &[u8]) -> Result<wasip1::Size, wasip1::Errno> {
        let ciovec_arr = [wasip1::Ciovec {
            buf: data.as_ptr() as *const u8,
            buf_len: data.len(),
        }];

        unsafe { wasip1::fd_write(wasip1::FD_STDOUT, &ciovec_arr) }
    }

    pub fn write_to_stdout_direct(
        buf: *const u8,
        len: usize,
    ) -> Result<wasip1::Size, wasip1::Errno> {
        let ciovec_arr = [wasip1::Ciovec { buf, buf_len: len }];

        unsafe { wasip1::fd_write(wasip1::FD_STDOUT, &ciovec_arr) }
    }

    pub fn write_to_stderr(data: &[u8]) -> Result<wasip1::Size, wasip1::Errno> {
        let ciovec_arr = [wasip1::Ciovec {
            buf: data.as_ptr() as *const u8,
            buf_len: data.len(),
        }];

        unsafe { wasip1::fd_write(wasip1::FD_STDERR, &ciovec_arr) }
    }
}
