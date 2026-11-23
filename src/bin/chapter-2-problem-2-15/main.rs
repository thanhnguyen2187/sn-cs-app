fn equal(x: u8, y: u8) -> bool {
    x ^ y == 0
}

#[cfg(test)]
mod tests {
    use super::*; // Brings parent functions (even private ones) into scope

    #[test]
    fn test_equal() {
        assert!(!equal(0, 100));
        assert!(equal(1, 1));
        assert!(!equal(1, 2));
    }
}

fn main() {
}
