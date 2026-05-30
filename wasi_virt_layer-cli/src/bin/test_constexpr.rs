use wasm_encoder::{ConstExpr, Instruction};

fn main() {
    let expr1 = ConstExpr::i32_const(42);

    let expr2 = ConstExpr::extended(vec![Instruction::I32Const(42)]);
    let expr3 = ConstExpr::extended(vec![Instruction::I32Const(42), Instruction::End]);

    println!("expr1: {:?}", expr1);
    println!("expr2: {:?}", expr2);
    println!("expr3: {:?}", expr3);
}
