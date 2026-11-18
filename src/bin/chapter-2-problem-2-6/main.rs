fn main() {
    let a: i32 = 2607352;
    let b: f32 = 3510593.0;
    println!("{:#b}", a); // 0b1001111100100011111000
    println!("{:#x}", a); // 0x27c8f8
    println!("{:#b}", b.to_bits()); // 0b1001010010101100100010100000100
    println!("{:#x}", b.to_bits()); // 0x4a564504
}
