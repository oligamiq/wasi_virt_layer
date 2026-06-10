#[test]
fn test_strum() {
    println!("{:?}", <crate::abi::Wasip1ABIFunc as strum::VariantNames>::VARIANTS);
    panic!("show variants");
}
