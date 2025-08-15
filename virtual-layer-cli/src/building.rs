use std::{
    collections::VecDeque,
    io::{BufRead, Write as _},
    sync::{LazyLock, mpsc::Receiver},
};

use eyre::Context as _;

use crate::{down_color, is_valid, util::ResultUtil as _};

struct CustomReadIterator<const T: usize, R: BufRead> {
    r: R,
    chars: [char; T],
}

impl<const T: usize, R: BufRead> CustomReadIterator<T, R> {
    fn new(r: R, chars: [char; T]) -> Self {
        Self { r, chars }
    }
}

impl<const T: usize, R: BufRead> Iterator for CustomReadIterator<T, R> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = Vec::new();
        loop {
            let mut one_buffer = [0; 1];
            let read = self.r.read(&mut one_buffer).unwrap();
            if read == 0 {
                return None;
            }
            if self.chars.contains(&(one_buffer[0] as char)) {
                let line = String::from_utf8(buf).unwrap();
                return Some(line);
            } else {
                buf.push(one_buffer[0]);
            }
        }
    }
}

pub fn build_vfs(
    manifest_path: Option<String>,
    package: &Option<String>,
    building_crate: cargo_metadata::Package,
) -> Option<camino::Utf8PathBuf> {
    let mut ret = None;

    let mut command = std::process::Command::new("cargo")
        .args({
            let mut args = vec![
                "build",
                "--target",
                "wasm32-wasip1",
                "--release",
                "--message-format=json-render-diagnostics",
                "--color",
                "always",
            ];
            if let Some(package_name) = package {
                args.push("--package");
                args.push(package_name);
            }
            if let Some(ref manifest_path) = manifest_path {
                args.push("--manifest-path");
                args.push(&manifest_path);
            }
            args
        })
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    // Capture the output
    let reader = std::io::BufReader::new(command.stdout.take().unwrap());

    // Compiling etc.
    let err_reader = std::io::BufReader::new(command.stderr.take().unwrap());

    let mut before_len = 0;
    let term = console::Term::stdout();

    let mut last_lines = VecDeque::with_capacity(3);

    let (msg_sender, msg_receiver) = std::sync::mpsc::channel();
    let (parse_sender, parse_receiver) = std::sync::mpsc::channel();

    let msg_thread = std::thread::spawn(move || {
        for line in CustomReadIterator::new(err_reader, ['\n', '\r']) {
            msg_sender.send(line).unwrap();
        }
    });

    let parse_thread = std::thread::spawn(move || {
        for message in cargo_metadata::Message::parse_stream(reader) {
            let message = message.unwrap();

            parse_sender.send(message).unwrap();
        }
    });

    let mut before_msgs: Vec<String> = Vec::new();

    fn process_msg(
        msg_receiver: &Receiver<String>,
        last_lines: &mut VecDeque<String>,
        before_msgs: &mut Vec<String>,
        before_len: &mut usize,
        term: &console::Term,
    ) {
        let mut is_change = false;
        'inner: loop {
            if let Some(line) = msg_receiver.try_recv().ok() {
                fn check_finish_or_compiling(line: &str) -> bool {
                    static NON_ANSI: LazyLock<regex::Regex> =
                        LazyLock::new(|| regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap());

                    let line = NON_ANSI.replace_all(line, "");

                    if line.contains("Finished") || line.contains("Compiling") {
                        let index = line
                            .find("Finished")
                            .or_else(|| line.find("Compiling"))
                            .unwrap();

                        let line = &line[..index];

                        line.as_bytes().iter().all(|&b| b == b' ')
                    } else {
                        return false;
                    }
                }

                if check_finish_or_compiling(&line) {
                    // Skip lines with carriage return
                    if line.contains("\r") {
                        last_lines.pop_back();
                        last_lines.push_back(line.to_string());
                    } else {
                        last_lines.push_back(line.to_string());
                        if last_lines.len() > 3 {
                            last_lines.pop_front();
                        }
                    }
                } else {
                    before_msgs.push(line.to_string());
                }
                is_change = true;
            } else {
                break 'inner;
            }
        }

        if is_change || !before_msgs.is_empty() {
            term.clear_last_lines(*before_len).unwrap();

            for msg in before_msgs.iter() {
                term.write_line(msg).unwrap();
            }
            before_msgs.clear();

            *before_len = last_lines.len();
            for line in last_lines.iter() {
                term.write_line(&down_color::reduce_saturation(line, 0.5))
                    .unwrap();
            }
            term.flush().unwrap();
        }
    }

    let finished = 'outer: loop {
        process_msg(
            &msg_receiver,
            &mut last_lines,
            &mut before_msgs,
            &mut before_len,
            &term,
        );

        'inner: loop {
            if let Some(message) = parse_receiver.try_recv().ok() {
                match message {
                    cargo_metadata::Message::CompilerArtifact(artifact) => {
                        if building_crate.id == artifact.package_id {
                            // let mut file = std::fs::OpenOptions::new()
                            //     .append(true)
                            //     .create(true)
                            //     .open("output_artifact.txt")
                            //     .unwrap();
                            // file.write_all(format!("{:?}", artifact).as_bytes())
                            //     .unwrap();
                            // file.write_all(b"\n").unwrap();

                            if let Some(wasm) = artifact
                                .filenames
                                .iter()
                                .filter(|f| f.extension() == Some("wasm"))
                                .next()
                                .cloned()
                            {
                                ret = Some(wasm);
                            }
                        }
                    }
                    cargo_metadata::Message::CompilerMessage(msg) => {
                        if let Some(ref rendered) = msg.message.rendered {
                            before_msgs.push(rendered.to_string());
                        }
                    }
                    cargo_metadata::Message::BuildFinished(finished) => {
                        // Handle the build finished message
                        // println!("Build Finished: {:?}", finished);

                        break 'outer finished;
                    }
                    _ => {}
                }
            } else {
                break 'inner;
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    };

    msg_thread.join().unwrap();
    parse_thread.join().unwrap();
    command.wait().unwrap();

    process_msg(
        &msg_receiver,
        &mut last_lines,
        &mut before_msgs,
        &mut before_len,
        &term,
    );

    print!("\x1b[39m");

    if finished.success {
        println!("Build succeeded!");
    } else {
        println!("Build failed!");
    }

    ret
}

pub fn get_building_crate(
    metadata: &cargo_metadata::Metadata,
    package: &Option<String>,
) -> cargo_metadata::Package {
    let building_crate = {
        let packages = metadata.packages.clone();

        if let Some(package_name) = package {
            packages
                .iter()
                .filter(|package| *package.name == *package_name)
                .cloned()
                .collect::<Vec<_>>()
        } else {
            let workspace = metadata.workspace_members.clone();
            let workspace_default_packages = metadata.workspace_default_packages();

            if workspace_default_packages.is_empty() {
                packages
                    .iter()
                    .filter(|package| {
                        workspace
                            .iter()
                            .any(|workspace_package| package.id == *workspace_package)
                    })
                    .cloned()
                    .collect::<Vec<_>>()
            } else {
                packages
                    .iter()
                    .filter(|package| {
                        workspace_default_packages
                            .iter()
                            .any(|workspace_package| package.id == workspace_package.id)
                    })
                    .cloned()
                    .collect::<Vec<_>>()
            }
        }
    }
    .into_iter()
    .next()
    .unwrap();

    building_crate
}

pub fn optimize_wasm(
    wasm_path: &camino::Utf8PathBuf,
    add_args: &[&str],
    require_update: bool,
) -> eyre::Result<camino::Utf8PathBuf> {
    let mut before_path = wasm_path.clone();

    let mut first = true;

    loop {
        let output_path = before_path.with_extension("opt.wasm");
        if output_path.exists() {
            std::fs::remove_file(&output_path)?;
        }

        let command = std::process::Command::new("wasm-opt")
            .args(add_args)
            .args(["-Oz", wasm_path.as_str()])
            .args(["--output", output_path.as_str()])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        let output = command.wait_with_output()?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            println!("err: {err}");
            Err(eyre::eyre!("wasm-opt failed."))?;
        }

        let before_size = std::fs::metadata(&before_path)?.len();
        let after_size = std::fs::metadata(&output_path)?.len();

        if before_size <= after_size {
            if first {
                if !require_update {
                    std::fs::remove_file(&output_path)?;
                    std::fs::copy(&before_path, &output_path)?;
                    before_path = output_path.clone();
                }
            } else {
                // remove
                std::fs::remove_file(&output_path)?;
            }

            break;
        }

        first = false;

        before_path = output_path.clone();
    }

    Ok(before_path)
}

pub fn wasm_to_component(
    wasm_path: &camino::Utf8PathBuf,
    wasm_names: &[impl AsRef<str>],
) -> eyre::Result<camino::Utf8PathBuf> {
    let output_path = wasm_path.with_extension("component.wasm");
    if output_path.exists() {
        std::fs::remove_file(&output_path)?;
    }

    // https://github.com/bytecodealliance/wasm-tools/blob/main/src/bin/wasm-tools/component.rs#L259
    let wasm = std::fs::read(wasm_path)?;

    is_valid::is_valid_wasm_for_component(&wasm, wasm_names)?;

    let mut encoder = wit_component::ComponentEncoder::default()
        .validate(true)
        .reject_legacy_names(false);

    encoder = encoder
        .module(&wasm)
        .to_eyre()
        .wrap_err_with(|| eyre::eyre!("failed to add module"))?;

    encoder = encoder.realloc_via_memory_grow(true);

    let bytes = encoder
        .encode()
        .to_eyre()
        .wrap_err_with(|| eyre::eyre!("failed to encode a component"))?;

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&output_path)?;
    file.write_all(&bytes)?;
    file.sync_data()?;

    Ok(output_path)
}
