fn f_n(n: u32) {
    let p = 2_i32.pow(n);
    println!("{} {} 0x{:x}", n, p, p);
}

fn f_p(p: i32) {
    let mut n: i32 = 1;
    while 2_i32.pow(n as u32) < p {
        n += 1;
    }
    if p - 2_i32.pow(n as u32) > 0 {
        panic!("Invalid input for f_p: {}", p)
    }

    println!("{} {} 0x{:x}", n, p, p);
}

fn main() {
    f_n(5);
    f_n(23);
    f_p(32_768);
    f_p(0x2000);
    f_n(12);
    f_p(64);
    f_p(0x100);
}
