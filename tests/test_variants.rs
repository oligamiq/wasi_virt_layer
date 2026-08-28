#[test]
fn test_variants() {
    println!("{:?}", <crate::abi::Wasip1ABIFunc as strum::VariantNames>::VARIANTS);
    panic!("Show me the variants");
}
