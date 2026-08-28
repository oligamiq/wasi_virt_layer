fn main() {
    let wasm = [0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x00, 0x04, 0x01, 0x61, 0x01, 0x02];
    for payload in wasmparser::Parser::new(0).parse_all(&wasm) {
        let payload = payload.unwrap();
        println!("{:?}", payload.as_section().map(|(id, _)| id));
    }
}
