use std::ops::{BitAnd, BitOr};

fn bis(x: u8, m: u8) -> u8 {
    x.bitor(m)
}

fn bic(x: u8, m: u8) -> u8 {
    x.bitand(!m)
}

fn bool_not(x: u8) -> u8 {
    bic(u8::MAX, x)
}

fn bool_or(x: u8, y: u8) -> u8 {
    bis(x, y)
}

fn bool_and(x: u8, y: u8) -> u8 {
    bic(x, bool_not(y))
}

fn bool_xor(x: u8, y: u8) -> u8 {
    bool_and(bool_or(x, y), bool_not(bool_and(x, y)))
}

#[cfg(test)]
mod tests {
    use super::*; // Brings parent functions (even private ones) into scope

    const X: u8 = 0b10101101;
    const Y: u8 = 0b11101100;

    #[test]
    fn test_bis() {
        //      0b10101101
        //      0b11101100
        // BIS: 0b11101101
        assert_eq!(bis(X, Y), 0b11101101);
    }

    #[test]
    fn test_bic() {
        //      0b10101101
        //      0b11101100
        // BIC: 0b00000001
        assert_eq!(bic(X, Y), 0b00000001);
        //      0b11111111
        //      0b11101100
        // BIC: 0b00010001
        assert_eq!(bic(u8::MAX, X), 0b00000001);
    }

    #[test]
    fn test_not() {
        //      0b10101101
        // NOT: 0b01010010
        assert_eq!(bool_not(X), 0b01010010);
    }

    #[test]
    fn test_xor() {
        //      0b10101101
        //      0b11101100
        // XOR: 0b01000001
        assert_eq!(bool_xor(X, Y), 0b01000001);
    }

}

fn main() {
    //      0b10101101
    // NOT: 0b01010010

    // println!("{:#08b}", bool_not(x));
    // println!("{:#08b}", bool_or(0b10101101, 0b11101100));
    // println!("{:#08b}", bool_and(0b10101101, 0b11101100));
    // println!("{:#08b}", bool_xor(0b10101101, 0b11101100));
    //      0b10101101
    //      0b11101100
    // XOR: 0b01000001
    // AND: 0b10101100
    // OR:  0b
}
