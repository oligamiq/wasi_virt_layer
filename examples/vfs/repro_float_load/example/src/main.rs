fn main() {
    let arr = [1.5f32, 2.5f32];
    let val = std::hint::black_box(arr[0]);
    println!("Value: {}", val);
}
