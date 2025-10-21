use std::{
    io::{Read as _, Seek as _},
    process::Command,
};

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

fn get_temp_filepath() -> String {
    let now = std::time::SystemTime::now();
    let timestamp = now
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    #[cfg(windows)]
    return dirs::data_local_dir()
        .unwrap()
        .join("Temp")
        .join(format!("tmp_{}_{timestamp}.log", env!("CARGO_PKG_NAME")))
        .to_string_lossy()
        .into();

    #[cfg(unix)]
    return format!("/tmp/tmp_{}_{timestamp}.log", env!("CARGO_PKG_NAME"));
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
