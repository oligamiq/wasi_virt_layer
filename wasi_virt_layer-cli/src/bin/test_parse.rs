use wasmparser::{Parser, Payload};

fn main() {
    let bytes = std::fs::read("/home/oligami/projects/wasi_virt_layer/DEBUG_INPUT.wasm").unwrap();
    for payload in Parser::new(0).parse_all(&bytes) {
        if let Ok(Payload::ImportSection(s)) = payload {
            println!("Found ImportSection!");
            let mut count = 0;
            for group in s {
                for i in group.unwrap().into_iter() {
                    let (_, i) = i.unwrap();
                    println!("  Import: {}::{}", i.module, i.name);
                    count += 1;
                }
            }
            println!("Count: {}", count);
        }
    }
}
