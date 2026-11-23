fn f_1(a: u8) {
    println!("{:#x} {:#010b}", a, a);
}

fn f_2(a: u8) {
    let shifted = a << 2;
    println!("{:#010b} {:#x}", shifted, shifted);
}

fn f_3(a: u8) {
    // Logical shift for unsigned int
    let shifted = a >> 3;
    println!("{:#010b} {:#x}", shifted, shifted);
}

fn f_4(a: u8) {
    // Arithmetic shift for unsigned int
    let shifted = ((a as i8) >> 3) as u8;
    println!("{:#010b} {:#x}", shifted, shifted);
}

fn main() {
    let nums: [u8; 4] = [0xD4, 0x64, 0x72, 0x44];

    for f in [f_1, f_2, f_3, f_4].iter() {
       for num in nums {
           f(num);
       }
        println!();
    }
}
