use std::process::Command;

pub struct FallbackCommand<F>
where
    F: FnOnce(&[String]) -> i32 + Send + 'static,
{
    bin: String,
    args: Vec<String>,
    func: Option<F>,
}

impl<F> FallbackCommand<F>
where
    F: FnOnce(&[String]) -> i32 + Send + 'static,
{
    pub fn new(bin: impl AsRef<str>, func: F) -> Self {
        Self {
            bin: bin.as_ref().to_string(),
            args: Vec::new(),
            func: Some(func),
        }
    }

    pub fn arg(&mut self, arg: impl AsRef<str>) -> &mut Self {
        self.args.push(arg.as_ref().to_string());
        self
    }

    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for arg in args {
            self.arg(arg.as_ref());
        }
        self
    }

    pub fn spawn(&mut self) -> std::io::Result<FallbackChild> {
        let mut cmd = Command::new(&self.bin);
        cmd.args(&self.args);
        let piped_out = std::process::Stdio::piped();
        let piped_err = std::process::Stdio::piped();
        cmd.stdout(piped_out);
        cmd.stderr(piped_err);
        match cmd.spawn() {
            Ok(child) => Ok(FallbackChild::new_process(child)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // Fallback to the provided function
                let args = self.args.clone();
                let func = self.func.take().expect("Function already taken");
                let handle = std::thread::spawn(move || {
                    let captured_output = CapturedOutput::new();
                    let result = (func)(&args);
                    let mut output = captured_output.into_captured();
                    output.success = result == 0;
                    output
                });
                Ok(FallbackChild::new_thread(handle))
            }
            Err(e) => Err(e),
        }
    }
}

pub enum FallbackChild {
    Process(std::process::Child),
    Thread(std::thread::JoinHandle<FallbackOutput>),
}

impl FallbackChild {
    fn new_process(child: std::process::Child) -> Self {
        FallbackChild::Process(child)
    }

    fn new_thread(handle: std::thread::JoinHandle<FallbackOutput>) -> Self {
        FallbackChild::Thread(handle)
    }

    pub fn wait_with_output(self) -> std::io::Result<FallbackOutput> {
        match self {
            FallbackChild::Process(child) => {
                let output = child.wait_with_output()?;
                Ok(FallbackOutput {
                    stdout: output.stdout,
                    stderr: output.stderr,
                    success: output.status.success(),
                })
            }
            FallbackChild::Thread(handle) => {
                let out = handle.join().expect("Thread panicked");
                Ok(out)
            }
        }
    }
}

pub struct FallbackOutput {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub success: bool,
}

#[cfg(any(unix, target_os = "wasi"))]
mod unix {
    use camino::Utf8PathBuf;
    use std::{
        env::temp_dir,
        io::{Read as _, Seek as _, SeekFrom, Write as _},
        os::fd::AsRawFd as _,
    };

    use super::*;

    pub struct CapturedOutput {
        out_file: std::fs::File,
        out_path: Utf8PathBuf,
        err_file: std::fs::File,
        err_path: Utf8PathBuf,
        is_released: bool,
    }

    impl CapturedOutput {
        pub fn new() -> Self {
            let time_stamp = chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
            let out_path = temp_dir().join(format!("wasi_virt_layer_cli_out_{time_stamp}.txt"));
            let err_path = temp_dir().join(format!("wasi_virt_layer_cli_err_{time_stamp}.txt"));
            let out = std::fs::File::create(&out_path).unwrap();
            let err = std::fs::File::create(&err_path).unwrap();

            // swap stdout
            Self::swap_fds(out.as_raw_fd(), libc::STDOUT_FILENO).unwrap();
            Self::swap_fds(err.as_raw_fd(), libc::STDERR_FILENO).unwrap();

            Self {
                out_file: out,
                out_path: Utf8PathBuf::from_path_buf(out_path).unwrap(),
                err_file: err,
                err_path: Utf8PathBuf::from_path_buf(err_path).unwrap(),
                is_released: false,
            }
        }

        fn swap_fds(fd1: i32, fd2: i32) -> std::io::Result<()> {
            #[cfg(target_os = "wasi")]
            fn fd_renumber(from: i32, to: i32) {
                let r = unsafe { libc::__wasilibc_fd_renumber(from, to) };
                if r != 0 {
                    panic!("Failed to renumber fd from {from} to {to}: code: {r}");
                }
            }

            #[cfg(unix)]
            fn fd_renumber(from: i32, to: i32) {
                let r = unsafe { libc::dup2(from, to) };
                if r == -1 {
                    panic!(
                        "Failed to renumber fd from {from} to {to}: code: {}",
                        std::io::Error::last_os_error()
                    );
                }
            }

            let path = format!(
                "{}/wasi_virt_layer_cli_temp_{}.txt",
                std::env::temp_dir().to_string_lossy(),
                chrono::Utc::now().timestamp()
            );
            let temp_file = std::fs::File::open(&path)?;
            let temp_fd = temp_file.as_raw_fd();

            fd_renumber(temp_fd, fd1);
            fd_renumber(fd1, fd2);
            fd_renumber(fd2, temp_fd);

            core::mem::drop(temp_file);
            std::fs::remove_file(&path)?;

            Ok(())
        }

        fn dropper(&mut self) -> Option<(Vec<u8>, Vec<u8>)> {
            if self.is_released {
                return None;
            }

            std::io::stdout().flush().expect("Failed to flush stdout");
            std::io::stderr().flush().expect("Failed to flush stderr");

            // reset stdout and stderr
            Self::swap_fds(self.out_file.as_raw_fd(), libc::STDOUT_FILENO).unwrap();
            Self::swap_fds(self.err_file.as_raw_fd(), libc::STDERR_FILENO).unwrap();

            self.out_file
                .seek(SeekFrom::Start(0))
                .expect("Failed to seek stdout capture file");
            self.err_file
                .seek(SeekFrom::Start(0))
                .expect("Failed to seek stderr capture file");
            let mut err_bytes = Vec::new();
            let mut out_bytes = Vec::new();
            self.out_file
                .read_to_end(&mut out_bytes)
                .expect("Failed to read captured stdout");
            self.err_file
                .read_to_end(&mut err_bytes)
                .expect("Failed to read captured stderr");

            std::fs::remove_file(&self.out_path).ok();
            std::fs::remove_file(&self.err_path).ok();

            self.is_released = true;

            Some((out_bytes, err_bytes))
        }

        pub fn into_captured(mut self) -> FallbackOutput {
            let (out_bytes, err_bytes) = self.dropper().unwrap_or((Vec::new(), Vec::new()));
            FallbackOutput {
                stdout: out_bytes,
                stderr: err_bytes,
                success: true,
            }
        }
    }

    impl Drop for CapturedOutput {
        fn drop(&mut self) {
            self.dropper();
        }
    }
}
#[cfg(any(unix, target_os = "wasi"))]
use unix::CapturedOutput;

#[cfg(windows)]
mod windows {
    use std::io::Write;

    use super::*;
    use camino::Utf8PathBuf;
    use libc::{O_WRONLY, close, dup, dup2, open_osfhandle};
    use std::env::temp_dir;
    use std::os::windows::io::AsRawHandle as _;
    use windows_sys::Win32::Foundation::HANDLE;
    use windows_sys::Win32::System::Console::{
        GetStdHandle, STD_ERROR_HANDLE, STD_OUTPUT_HANDLE, SetStdHandle,
    };
    const STDOUT_FILENO: i32 = 1;
    const STDERR_FILENO: i32 = 2;

    pub struct CapturedOutput {
        out_file: std::fs::File,
        out_path: Utf8PathBuf,
        err_file: std::fs::File,
        err_path: Utf8PathBuf,
        original_stdout: HANDLE,
        original_stderr: HANDLE,
        original_stdout_fd: i32,
        original_stderr_fd: i32,
        is_released: bool,
    }

    impl CapturedOutput {
        pub fn new() -> Self {
            let time_stamp = chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
            let out_path = temp_dir().join(format!("wasi_virt_layer_cli_out_{time_stamp}.txt"));
            let err_path = temp_dir().join(format!("wasi_virt_layer_cli_err_{time_stamp}.txt"));
            let out = std::fs::File::create(&out_path).unwrap();
            let err = std::fs::File::create(&err_path).unwrap();

            unsafe {
                std::io::stdout().flush().unwrap();
                std::io::stderr().flush().unwrap();

                let original_stdout = GetStdHandle(STD_OUTPUT_HANDLE);
                let original_stderr = GetStdHandle(STD_ERROR_HANDLE);

                let original_stdout_fd = dup(STDOUT_FILENO);
                let original_stderr_fd = dup(STDERR_FILENO);

                if SetStdHandle(STD_OUTPUT_HANDLE, out.as_raw_handle() as HANDLE) == 0 {
                    panic!("Failed to set stdout handle");
                }
                if SetStdHandle(STD_ERROR_HANDLE, err.as_raw_handle() as HANDLE) == 0 {
                    panic!("Failed to set stderr handle");
                }

                Self {
                    out_file: out,
                    out_path: Utf8PathBuf::from_path_buf(out_path).unwrap(),
                    err_file: err,
                    err_path: Utf8PathBuf::from_path_buf(err_path).unwrap(),
                    original_stdout,
                    original_stderr,
                    is_released: false,
                }
            }
        }

        fn dropper(&mut self) -> Option<(Vec<u8>, Vec<u8>)> {
            if self.is_released {
                return None;
            }

            std::io::stdout().flush().ok();
            std::io::stderr().flush().ok();

            unsafe {
                // Restore original stdout and stderr
                SetStdHandle(STD_OUTPUT_HANDLE, self.original_stdout);
                SetStdHandle(STD_ERROR_HANDLE, self.original_stderr);
            }

            // Close the file handles to ensure everything is flushed
            drop(std::mem::replace(
                &mut self.out_file,
                std::fs::File::create("NUL").unwrap(),
            ));
            drop(std::mem::replace(
                &mut self.err_file,
                std::fs::File::create("NUL").unwrap(),
            ));

            // Read the captured output
            let out_bytes = std::fs::read(&self.out_path).expect("Failed to read captured stdout");
            let err_bytes = std::fs::read(&self.err_path).expect("Failed to read captured stderr");

            // Clean up temp files
            std::fs::remove_file(&self.out_path).ok();
            std::fs::remove_file(&self.err_path).ok();

            self.is_released = true;

            Some((out_bytes, err_bytes))
        }

        pub fn into_captured(mut self) -> FallbackOutput {
            let (out_bytes, err_bytes) = self.dropper().unwrap();
            FallbackOutput {
                stdout: out_bytes,
                stderr: err_bytes,
                success: true,
            }
        }
    }

    impl Drop for CapturedOutput {
        fn drop(&mut self) {
            self.dropper();
        }
    }

    mod redirect {
        use super::*;

        use std::ffi::CString;
        use std::fs::File;
        use std::io;
        use std::io::Write;
        use std::os::windows::io::{AsRawHandle, FromRawHandle};
        use std::sync::Arc;
        use std::thread;
        use windows_sys::Win32::Foundation::*;
        use windows_sys::Win32::Security::SECURITY_ATTRIBUTES;
        use windows_sys::Win32::Storage::FileSystem::*;
        use windows_sys::Win32::System::Console::*;
        use windows_sys::Win32::System::Diagnostics::Debug::*;
        use windows_sys::Win32::System::IO::*;
        use windows_sys::Win32::System::Pipes::CreatePipe;
        use windows_sys::Win32::System::Threading::*;

        #[derive(Clone, Copy)]
        enum StdHandleToRedirect {
            Stdout,
            Stderr,
        }

        struct StdRedirect {
            readable: HANDLE,
            writable: HANDLE,
            _thread: Option<std::thread::JoinHandle<()>>,
        }

        impl StdRedirect {
            fn new(
                h: StdHandleToRedirect,
                callback: Arc<dyn Fn(u8) + Send + Sync + 'static>,
            ) -> io::Result<Self> {
                unsafe {
                    let mut readable: HANDLE = HANDLE::default();
                    let mut writable: HANDLE = HANDLE::default();

                    if CreatePipe(&mut readable, &mut writable, std::ptr::null(), 0) == 0 {
                        return Err(io::Error::last_os_error());
                    }

                    // Set std handle
                    let handle_id = match h {
                        StdHandleToRedirect::Stdout => STD_OUTPUT_HANDLE,
                        StdHandleToRedirect::Stderr => STD_ERROR_HANDLE,
                    };
                    SetStdHandle(handle_id, writable);

                    // Redirect libc stdout/stderr
                    let writable_file_stream = open_osfhandle(writable as _, 0);

                    let writable_file =
                        libc::fdopen(writable_file_stream, "wt".as_ptr() as *const _);

                    // Duplicate the handle for C stdout compatibility
                    let _ = unsafe {
                        libc::dup2(
                            libc::fileno(writable_file),
                            match h {
                                StdHandleToRedirect::Stdout => STDOUT_FILENO,
                                StdHandleToRedirect::Stderr => STDERR_FILENO,
                            },
                        )
                    };

                    // Launch a reading thread
                    let readable_clone = readable;
                    let cb = callback.clone();

                    let handle = thread::Builder::new()
                        .spawn_unchecked::<_, ()>(move || {
                            let mut buf = [0u8; 1];
                            loop {
                                let mut read: u32 = 0;
                                let ok = ReadFile(
                                    readable_clone,
                                    buf.as_mut_ptr() as *mut _,
                                    1,
                                    &mut read,
                                    std::ptr::null_mut(),
                                );
                                if ok == 0 {
                                    panic!(
                                        "Failed to read from pipe: {}",
                                        io::Error::last_os_error()
                                    );
                                }
                                if read == 1 {
                                    cb(buf[0]);
                                }
                                std::thread::yield_now();
                            }
                        })
                        .unwrap();

                    Ok(Self {
                        readable,
                        writable,
                        _thread: Some(handle),
                    })
                }
            }
        }
    }
}
#[cfg(windows)]
use windows::CapturedOutput;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_captured_output() {
        let captured = CapturedOutput::new();
        print!("This is a ");
        println!("test output");
        eprint!("This is a ");
        eprintln!("test error");
        let output = captured.into_captured();

        let stdout_str = String::from_utf8(output.stdout).unwrap();
        let stderr_str = String::from_utf8(output.stderr).unwrap();

        println!("Captured stdout: {}", stdout_str);
        println!("Captured stderr: {}", stderr_str);

        assert!(stdout_str.contains("This is a test output"));
        assert!(stderr_str.contains("This is a test error"));
    }
}
