fn main() {
    let memarg = wasm_encoder::MemArg {
        offset: 0,
        align: 0,
        memory_index: 0,
    };
    let _op = wasm_encoder::Instruction::I32Load(memarg);
    println!("Compiled!");
}
