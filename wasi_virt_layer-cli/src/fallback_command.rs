use std::{
    fs::File,
    io::{Read as _, Seek as _, Write as _},
    path::Path,
};

use eyre::Context as _;
use fs2::FileExt as _;
use tempfile::Builder as TempFileBuilder;

pub(crate) const COMMAND_ALTERNATE_ENV_VAR: &str = "WASI_VIRT_LAYER_FALLBACK_ALTERNATE_COMMAND";

#[allow(unused)]
pub(crate) fn wasm_merge(args: &[String]) -> i32 {
    #[cfg(feature = "fallback")]
    {
        wasm_merge_sys::run_wasm_merge(args)
    }
    #[cfg(not(feature = "fallback"))]
    {
        eprintln!("wasm-merge fallback is not enabled");
        1
    }
}

#[allow(unused)]
pub(crate) fn wasm_opt(args: &[String]) -> i32 {
    #[cfg(feature = "fallback")]
    {
        let mut command = wasm_opt::integration::Command::new("wasm-opt");
        command.args(args.iter().skip(1));

        match wasm_opt::integration::run_from_command_args(command) {
            Ok(()) => 0,
            Err(err) => {
                eprintln!("wasm-opt fallback failed: {err}");
                1
            }
        }
    }
    #[cfg(not(feature = "fallback"))]
    {
        eprintln!("wasm-opt fallback is not enabled");
        1
    }
}

fn fake_fallback(args: &[String]) -> i32 {
    unimplemented!("This is a fake fallback function. It should never be called. Args: {args:?}");
}

/// Gets a `FallbackCommand` for the specified binary, with a fallback function that will be called if the binary is not found.
pub fn get_fallback_command(
    bin: impl AsRef<str>,
) -> FallbackCommand<impl FnOnce(&[String]) -> i32 + Send + 'static> {
    match bin.as_ref() {
        "wasm-merge" => FallbackCommand::new("wasm-merge", fake_fallback),
        "wasm-opt" => FallbackCommand::new("wasm-opt", fake_fallback),
        _ => panic!("Unsupported fallback command specified: {}", bin.as_ref()),
    }
}

/// A command that can fall back to a Rust function if the binary is not found.
pub struct FallbackCommand<F>
where
    F: FnOnce(&[String]) -> i32 + Send + 'static,
{
    /// The binary name.
    pub bin: String,
    /// The arguments to the command.
    pub args: Vec<String>,
    /// The optional fallback function.
    pub func: Option<F>,
}

const DISABLE_FALLBACK: bool = false;

/// A file-based lock to prevent concurrent execution of commands.
pub struct CommandLock {
    locks: Vec<(File, String)>,
}

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
        let setter = |cmd: &mut std::process::Command| {
            cmd.args(&self.args);
            let piped_out = std::process::Stdio::piped();
            let piped_err = std::process::Stdio::piped();
            cmd.stdout(piped_out);
            cmd.stderr(piped_err);
        };
        setter(&mut cmd);
        match cmd.spawn() {
            Ok(child) => Ok(FallbackChild::new_process(child)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound && DISABLE_FALLBACK => {
                let _ = self.func.take();
                Err(e)
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // // Fallback to the provided function
                // let args = self.args.clone();
                // let func = self.func.take().expect("Function already taken");
                // let handle = std::thread::spawn(move || {
                //     let log = std::fs::OpenOptions::new()
                //         .truncate(true)
                //         .read(true)
                //         .create(true)
                //         .write(true)
                //         .open(get_temp_filepath())
                //         .unwrap();

                //     let print_redirect = gag::Redirect::stdout(log).unwrap();

                //     let result = (func)(&args);

                //     // Extract redirect
                //     let mut log = print_redirect.into_inner();

                //     let mut buf = String::new();
                //     log.seek(std::io::SeekFrom::Start(0)).unwrap();
                //     log.read_to_string(&mut buf).unwrap();

                //     FallbackOutput {
                //         stdout: buf.into_bytes(),
                //         stderr: Vec::new(),
                //         success: result == 0,
                //     }
                // });
                // Ok(FallbackChild::new_thread(handle))

                // Fallback to self call with env var
                let mut cmd = std::process::Command::new(std::env::current_exe().unwrap());
                setter(&mut cmd);
                cmd.env(COMMAND_ALTERNATE_ENV_VAR, &self.bin);
                let child = cmd.spawn()?;
                Ok(FallbackChild::new_process(child))
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

    #[allow(unused)]
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
    /// Acquires the command locks for the specified identifiers using a master-lock strategy to prevent deadlocks.
    pub fn acquire(ids: &[String]) -> eyre::Result<Self> {
        let mut sorted_ids = ids.to_vec();
        sorted_ids.sort();
        sorted_ids.dedup();

        // 1. Ensure temp dir exists
        let temp_dir = std::env::temp_dir().join(env!("CARGO_PKG_NAME"));
        std::fs::create_dir_all(&temp_dir).wrap_err("Failed to create temp lock directory")?;

        // 2. Acquire Master Lock to serialize acquisition phase
        let master_path = temp_dir.join("master.lock");
        let master_file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(&master_path)
            .wrap_err_with(|| format!("Failed to open master lock file: {}", master_path.display()))?;

        master_file
            .lock_exclusive()
            .wrap_err("Failed to acquire master lock")?;

        let mut locks = Vec::new();

        // 3. Acquire individual locks while holding the master lock
        for id in sorted_ids {
            let hash = {
                use std::hash::{Hash, Hasher};
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                id.hash(&mut hasher);
                format!("{:x}", hasher.finish())
            };

            let lock_path = get_temp_lock_filepath(&hash);
            let lock_file = File::options()
                .read(true)
                .write(true)
                .create(true)
                .open(&lock_path)
                .wrap_err_with(|| format!("Failed to open lock file: {lock_path}"))?;

            if let Err(_) = lock_file.try_lock_exclusive() {
                let mut pid_str = String::new();
                let mut file = &lock_file;
                let _ = file.read_to_string(&mut pid_str);
                let pid_info = if pid_str.is_empty() {
                    String::new()
                } else {
                    format!(" (held by PID {})", pid_str.trim())
                };

                eprintln!("Waiting for command lock for {} {}...", id, pid_info);

                // We are holding the master lock, so we serialize all acquisitions.
                // If we block here, other processes waiting for DIFFERENT locks will also be blocked
                // at the master lock. This is acceptable for a CLI tool and prevents deadlock.
                lock_file
                    .lock_exclusive()
                    .wrap_err_with(|| format!("Failed to lock command file: {lock_path}"))?;
            }

            // Write current PID to the lock file
            let mut file = &lock_file;
            let _ = file.set_len(0);
            let _ = file.seek(std::io::SeekFrom::Start(0));
            let _ = write!(file, "{}", std::process::id());
            let _ = file.flush();

            locks.push((lock_file, lock_path));
        }

        // 4. Release master lock
        let _ = master_file.unlock();

        Ok(Self { locks })
    }
}

impl Drop for CommandLock {
    fn drop(&mut self) {
        for (file, _) in &mut self.locks {
            let _ = file.unlock();
        }
    }
}

fn get_temp_filepath() -> String {
    let mut builder = TempFileBuilder::new();
    let prefix = format!("tmp_{}_", env!("CARGO_PKG_NAME"));
    builder.prefix(&prefix);
    builder.suffix(".log");

    let builder = builder.tempfile_in(std::env::temp_dir());

    let file = builder.expect("Failed to create temp log file");
    let (_file, path) = file.keep().expect("Failed to persist temp log file");

    path.to_string_lossy().into_owned()
}

fn get_temp_lock_filepath(hash: &str) -> String {
    std::env::temp_dir()
        .join(env!("CARGO_PKG_NAME"))
        .join(format!("command_{}.lock", hash))
        .to_string_lossy()
        .into_owned()
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

    #[cfg_attr(not(feature = "fallback"), ignore = "fallback feature disabled")]
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
