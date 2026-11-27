fn fun_1(word: u32) -> i32 {
    ((word << 24) >> 24) as i32
    // "Clear" the first 24 bits/6 bytes
}

fn fun_2(word: u32) -> i32 {
    (word << 24) as i32 >> 24
    // Fill the first 6 bytes with:
    //
    // - `0xf` if the 7th byte is >= `0x8`/`0b1000`
    // - `0x0` otherwise (clearing like `fun_1`)
}

fn main() {
    for w in [
        0x00000076, 0x87654321, 0x000000C9, 0xEDCBA987, 0x000000A0, 0x00000060,
    ] {
        println!("{:#010x} {:#010x} {:#010x}", w, fun_1(w), fun_2(w));
    }
}
