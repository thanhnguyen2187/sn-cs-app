use std::cmp::PartialEq;

#[derive(Debug, PartialEq, Eq)]
pub enum Format {
    Binary,
    Hexadecimal,
    Decimal,
}

const LOOKUP_TUPLES: [[&str; 3]; 16] = [
    ["0", "0", "0000"],
    ["1", "1", "0001"],
    ["2", "2", "0010"],
    ["3", "3", "0011"],
    ["4", "4", "0100"],
    ["5", "5", "0101"],
    ["6", "6", "0110"],
    ["7", "7", "0111"],
    ["8", "8", "1000"],
    ["9", "9", "1001"],
    ["10", "A", "1010"],
    ["11", "B", "1011"],
    ["12", "C", "1100"],
    ["13", "D", "1101"],
    ["14", "E", "1110"],
    ["15", "F", "1111"],
];

pub fn convert(value: String, from: Format, to: Format) -> String {
    if from == Format::Hexadecimal && !value.starts_with("0x") {
        panic!("A hexadecimal string should start with \"0x\"");
    }
    if from == Format::Hexadecimal && value.len() % 4 != 2 {
        panic!("A hexadecimal string should start with \"0x\"");
    }

    match (from, to) {
        (Format::Binary, Format::Binary) => value,
        (Format::Binary, Format::Hexadecimal) => value,
        (Format::Binary, Format::Decimal) => value,
        (Format::Hexadecimal, Format::Binary) => value,
        (Format::Hexadecimal, Format::Hexadecimal) => value,
        (Format::Hexadecimal, Format::Decimal) => value,
        (Format::Decimal, Format::Binary) => value,
        (Format::Decimal, Format::Hexadecimal) => value,
        (Format::Decimal, Format::Decimal) => value,
    }
}
