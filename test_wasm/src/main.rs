fn main() {
    println!("Hello, world!");

    let envs = std::env::vars()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>();
    println!("Environ: {:?}", envs);
}
