fn f(x: u8, w: u8) -> i8 {
    ((x << (8 - w)) as i8) >> (8 - w)
}


fn main() {
    println!("{}", f(0b1100, 4));
    println!("{}", f(0b11100, 5));
    println!("{}", f(0b111100, 6));
}
