fn main() {
    let a: i32 = 0x123456EE;
    println!("{:0X?}", a.to_le_bytes());
    println!("{:0X?}", a.to_be_bytes());
}
