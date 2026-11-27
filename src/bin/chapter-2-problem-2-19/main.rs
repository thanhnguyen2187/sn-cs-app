fn t2u_4(x: i8) -> u8 {
    ((x << 4) as u8) >> 4
    // x as u8
}

fn main() {
    for x in [-1, -5, -6, -4, 1, 8] {
        println!("{}", t2u_4(x));
    }
}
