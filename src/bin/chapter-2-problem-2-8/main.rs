use std::ops::{BitAnd, BitOr, BitXor};

fn main() {
    let a = 0b01001110;
    let b = 0b11100001;

    println!("{:b}", !a);
    println!("{:b}", !b);
    println!("{:b}", a.bitand(b));
    println!("{:b}", b.bitor(a));
    println!("{:b}", a.bitxor(b));
}
