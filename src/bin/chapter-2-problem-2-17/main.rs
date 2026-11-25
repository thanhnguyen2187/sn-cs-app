fn f(x: u8) {
    println!("{:#04x} {:#010b} {:3} {:3}", x, x, x, ((x << 4) as i8) >> 4);
}

fn main() {
    f(0xA);
    f(0x1);
    f(0xB);
    f(0x2);
    f(0x7);
    f(0xC);
}
