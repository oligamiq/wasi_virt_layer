use wasm_encoder::{CodeSection, Function, Instruction};
fn main() {
    let mut f = Function::new(vec![]);
    f.instruction(&Instruction::Nop);
    let mut c = CodeSection::new();
    c.function(&f);
    println!("bytes: {:?}", c);
}
