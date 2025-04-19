use clap::{Parser, command};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    // --manifest-path
    #[arg(long)]
    manifest_path: Option<String>,
}

impl Args {
    pub fn new() -> Self {
        Args::parse()
    }

    pub fn get_manifest_path(&self) -> &'_ Option<String> {
        &self.manifest_path
    }
}
