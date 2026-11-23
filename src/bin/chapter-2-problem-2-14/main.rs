fn main() {
    let a: u8 = 0x55;
    let b: u8 = 0x46;
    println!("{:#x}", a & b);
    println!("{:#x}", a | b);
    println!("{:#x}", !a | !b);
    println!("{:#x}", a & !b);
    println!("{:#x}", (a != 0 && b != 0) as u8);
    println!("{:#x}", (a != 0 || b != 0) as u8);
    println!("{:#x}", (a == 0 || b == 0) as u8);
    println!("{:#x}", (a != 0 && !b != 0) as u8);
}
