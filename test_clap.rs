use clap::{Arg, Command, ArgAction};

fn main() {
    let cmd = Command::new("test")
        .arg(Arg::new("no_opt").long("no-opt").action(ArgAction::Count))
        .arg(Arg::new("wasm").action(ArgAction::Append));
    
    let matches = cmd.get_matches_from(vec!["test", "--no-opt", "wasm1"]);
    println!("count: {:?}", matches.get_count("no_opt"));
    println!("indices: {:?}", matches.indices_of("no_opt").map(|i| i.collect::<Vec<_>>()));
    
    let matches = cmd.get_matches_from(vec!["test"]);
    println!("count: {:?}", matches.get_count("no_opt"));
    println!("indices: {:?}", matches.indices_of("no_opt").map(|i| i.collect::<Vec<_>>()));
}