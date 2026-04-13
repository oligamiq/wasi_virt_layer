use std::{
    fs::File,
    io::{Read as _, Seek as _},
    path::Path,
};

use eyre::Context as _;
use fs2::FileExt as _;
use tempfile::Builder as TempFileBuilder;

/// A command that can fall back to a Rust function if the binary is not found.
pub struct FallbackCommand<F>
where
    F: FnOnce(&[String]) -> i32 + Send + 'static,
{
    bin: String,
    args: Vec<String>,
    func: Option<F>,
}

const DISABLE_FALLBACK: bool = true;

/// A file-based lock to prevent concurrent execution of commands.
pub struct CommandLock(File);

impl<F> FallbackCommand<F>
where
    F: FnOnce(&[String]) -> i32 + Send + 'static,
{
    /// Creates a new `FallbackCommand` with the specified binary and fallback function.
    pub fn new(bin: impl AsRef<str>, func: F) -> Self {
        Self {
            bin: bin.as_ref().to_string(),
            args: Vec::new(),
            func: Some(func),
        }
    }

    /// Adds an argument to the command.
    pub fn arg(&mut self, arg: impl AsRef<str>) -> &mut Self {
        self.args.push(arg.as_ref().to_string());
        self
    }

    /// Adds multiple arguments to the command.
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

    /// Spawns the command, either as a child process or as a fallback thread.
    pub fn spawn(&mut self) -> std::io::Result<FallbackChild> {
        let mut cmd = std::process::Command::new(&self.bin);
        cmd.args(&self.args);
        let piped_out = std::process::Stdio::piped();
        let piped_err = std::process::Stdio::piped();
        cmd.stdout(piped_out);
        cmd.stderr(piped_err);
        match cmd.spawn() {
            Ok(child) => Ok(FallbackChild::new_process(child)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound && DISABLE_FALLBACK => {
                let _ = self.func.take();
                Err(e)
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // Fallback to the provided function
                let args = self.args.clone();
                let func = self.func.take().expect("Function already taken");
                let handle = std::thread::spawn(move || {
                    let log = std::fs::OpenOptions::new()
                        .truncate(true)
                        .read(true)
                        .create(true)
                        .write(true)
                        .open(get_temp_filepath())
                        .unwrap();

                    let print_redirect = gag::Redirect::stdout(log).unwrap();

                    let result = (func)(&args);

                    // Extract redirect
                    let mut log = print_redirect.into_inner();

                    let mut buf = String::new();
                    log.seek(std::io::SeekFrom::Start(0)).unwrap();
                    log.read_to_string(&mut buf).unwrap();

                    FallbackOutput {
                        stdout: buf.into_bytes(),
                        stderr: Vec::new(),
                        success: result == 0,
                    }
                });
                Ok(FallbackChild::new_thread(handle))
            }
            Err(e) => Err(e),
        }
    }
}

/// A handle to a spawned fallback command, which could be a process or a thread.
pub enum FallbackChild {
    /// A child process handle.
    Process(std::process::Child),
    /// A handle to a thread running the fallback function.
    Thread(std::thread::JoinHandle<FallbackOutput>),
}

impl FallbackChild {
    fn new_process(child: std::process::Child) -> Self {
        FallbackChild::Process(child)
    }

    fn new_thread(handle: std::thread::JoinHandle<FallbackOutput>) -> Self {
        FallbackChild::Thread(handle)
    }

    /// Waits for the command to finish and returns its output.
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

/// The output of a finished fallback command.
pub struct FallbackOutput {
    /// The standard output of the command.
    pub stdout: Vec<u8>,
    /// The standard error of the command.
    pub stderr: Vec<u8>,
    /// Whether the command exited successfully.
    pub success: bool,
}

impl CommandLock {
    /// Acquires the command lock.
    pub fn acquire() -> eyre::Result<Self> {
        let lock_path = get_temp_lock_filepath();

        if let Some(parent) = Path::new(&lock_path).parent() {
            std::fs::create_dir_all(parent).wrap_err_with(|| {
                format!("Failed to create temp lock dir: {}", parent.display())
            })?;
        }

        let lock_file = File::create(&lock_path)
            .wrap_err_with(|| format!("Failed to create lock file: {lock_path}"))?;
        lock_file
            .lock_exclusive()
            .wrap_err_with(|| format!("Failed to lock command file: {lock_path}"))?;

        Ok(Self(lock_file))
    }
}

impl Drop for CommandLock {
    fn drop(&mut self) {
        let _ = self.0.unlock();
    }
}

fn get_temp_filepath() -> String {
    let mut builder = TempFileBuilder::new();
    let prefix = format!("tmp_{}_", env!("CARGO_PKG_NAME"));
    builder.prefix(&prefix);
    builder.suffix(".log");

    #[cfg(windows)]
    let builder = builder.tempfile_in(dirs::data_local_dir().unwrap().join("Temp"));

    #[cfg(unix)]
    let builder = builder.tempfile_in("/tmp");

    let file = builder.expect("Failed to create temp log file");
    let (_file, path) = file.keep().expect("Failed to persist temp log file");

    path.to_string_lossy().into_owned()
}

fn get_temp_lock_filepath() -> String {
    #[cfg(windows)]
    return dirs::data_local_dir()
        .unwrap()
        .join("Temp")
        .join(env!("CARGO_PKG_NAME"))
        .join("command.lock")
        .to_string_lossy()
        .into();

    #[cfg(unix)]
    return Path::new("/tmp")
        .join(env!("CARGO_PKG_NAME"))
        .join("command.lock")
        .to_string_lossy()
        .into_owned();
}

/// require mutex
pub fn check_gag() -> bool {
    pub fn check_gag() -> Option<()> {
        let gag = gag::Redirect::stdout(
            std::fs::OpenOptions::new()
                .truncate(true)
                .read(true)
                .create(true)
                .write(true)
                .open(get_temp_filepath())
                .ok()?,
        )
        .ok()?;

        const WHITE_SPACE: &str = " \t\n\r";

        print!("{WHITE_SPACE}");
        std::io::Write::flush(&mut std::io::stdout()).ok()?;

        let mut stdout = gag.into_inner();
        let mut buf = String::new();
        stdout.seek(std::io::SeekFrom::Start(0)).ok()?;
        stdout.read_to_string(&mut buf).ok()?;

        Some(if buf.contains(WHITE_SPACE) {
            ()
        } else {
            return None;
        })
    }

    check_gag().is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    static MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    /// if not use nocapture arg, skip test.
    /// because gag crate require it.
    fn test_fallback_command() {
        if DISABLE_FALLBACK {
            return;
        }

        let _lock = MUTEX.lock().unwrap();
        if !check_gag() {
            return;
        }

        let mut cmd = FallbackCommand::new("non_existent_command", |args: &[String]| {
            println!("Fallback function called with args: {:?}", args);
            0
        });
        cmd.arg("arg1").arg("arg2");

        let child = cmd.spawn().expect("Failed to spawn command");
        let output = child.wait_with_output().expect("Failed to get output");

        assert!(output.success);
        let stdout_str = String::from_utf8_lossy(&output.stdout);
        assert!(stdout_str.contains("Fallback function called with args: [\"arg1\", \"arg2\"]"));

        drop(_lock);
    }

    #[cfg(feature = "fallback")]
    #[test]
    fn test_fallback_wasm_merge() {
        let _lock = MUTEX.lock().unwrap();
        if !check_gag() {
            return;
        }

        let mut cmd = FallbackCommand::new("non_existent_command", |args: &[String]| {
            wasm_merge_sys::run_wasm_merge(&args)
        });
        cmd.arg("--help");

        let child = cmd.spawn().expect("Failed to spawn command");
        let output = child.wait_with_output().expect("Failed to get output");

        assert!(output.success);
        let stdout_str = String::from_utf8_lossy(&output.stdout);
        panic!("Output: {}", stdout_str);

        drop(_lock);
    }
}
