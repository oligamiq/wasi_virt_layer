use wasm_encoder::{ConstExpr, GlobalSection, GlobalType, Instruction, Module, ValType};

fn main() {
    let mut s1 = GlobalSection::new();
    s1.global(
        GlobalType {
            val_type: ValType::I32,
            mutable: false,
            shared: false,
        },
        &ConstExpr::i32_const(42),
    );

    let mut s2 = GlobalSection::new();
    s2.global(
        GlobalType {
            val_type: ValType::I32,
            mutable: false,
            shared: false,
        },
        &ConstExpr::extended(vec![Instruction::I32Const(42)]),
    );

    let mut s3 = GlobalSection::new();
    s3.global(
        GlobalType {
            val_type: ValType::I32,
            mutable: false,
            shared: false,
        },
        &ConstExpr::extended(vec![Instruction::I32Const(42), Instruction::End]),
    );

    let m1 = {
        let mut m = Module::new();
        m.section(&s1);
        m.finish()
    };
    let m2 = {
        let mut m = Module::new();
        m.section(&s2);
        m.finish()
    };
    let m3 = {
        let mut m = Module::new();
        m.section(&s3);
        m.finish()
    };

    println!("s1 length: {}", m1.len());
    println!("s2 length: {}", m2.len());
    println!("s3 length: {}", m3.len());
    println!("s1 bytes: {:?}", m1);
    println!("s2 bytes: {:?}", m2);
    println!("s3 bytes: {:?}", m3);
}
