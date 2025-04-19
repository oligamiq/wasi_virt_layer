use std::{
    io::{BufRead, BufReader, Write as _},
    process::ChildStdout,
};

use clap::parser;

use crate::down_color;

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
            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open("output_one.txt")
                .unwrap();
            file.write_all(&one_buffer).unwrap();
            file.write_all(b"\n").unwrap();

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

    let mut last_lines = std::collections::VecDeque::with_capacity(3);

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

    loop {
        if let Some(line) = msg_receiver.try_recv().ok() {
            // line print to file
            {
                let mut file = std::fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open("output.txt")
                    .unwrap();
                file.write_all(line.as_bytes()).unwrap();
                file.write_all(b"\n").unwrap();
            }

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

            term.clear_last_lines(before_len).unwrap();

            for msg in before_msgs.iter() {
                term.write_line(msg).unwrap();
            }
            before_msgs.clear();

            before_len = last_lines.len();
            for line in last_lines.iter() {
                term.write_line(&down_color::reduce_saturation(line, 0.5))
                    .unwrap();
            }
            term.flush().unwrap();
        } else {
            for msg in before_msgs.iter() {
                term.write_line(msg).unwrap();
            }
            before_msgs.clear();
            term.flush().unwrap();
        }

        if let Some(message) = parse_receiver.try_recv().ok() {
            match message {
                cargo_metadata::Message::CompilerArtifact(artifact) => {
                    if building_crate.id == artifact.package_id {
                        let mut file = std::fs::OpenOptions::new()
                            .append(true)
                            .create(true)
                            .open("output_artifact.txt")
                            .unwrap();
                        file.write_all(format!("{:?}", artifact).as_bytes())
                            .unwrap();
                        file.write_all(b"\n").unwrap();

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
                    print!("\x1b[39m");

                    // Handle the build finished message
                    // println!("Build Finished: {:?}", finished);

                    if finished.success {
                        println!("Build succeeded!");
                    } else {
                        println!("Build failed!");
                    }

                    break;
                }
                _ => {}
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    msg_thread.join().unwrap();
    parse_thread.join().unwrap();
    command.wait().unwrap();

    ret
}

pub fn get_building_crate(metadata: &cargo_metadata::Metadata) -> cargo_metadata::Package {
    let building_crate = {
        let packages = metadata.packages.clone();
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
    .into_iter()
    .next()
    .unwrap();

    building_crate
}
