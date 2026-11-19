#[repr(u8)]
#[derive(Debug, PartialEq)]
enum Colors {
    Black = 0b000,
    Blue = 0b001,
    Green = 0b010,
    Cyan = 0b011,
    Red = 0b100,
    Magenta = 0b101,
    Yellow = 0b110,
    White = 0b111,
}

impl TryFrom<u8> for Colors {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b000 => Ok(Colors::Black),
            0b001 => Ok(Colors::Blue),
            0b010 => Ok(Colors::Green),
            0b011 => Ok(Colors::Cyan),
            0b100 => Ok(Colors::Red),
            0b101 => Ok(Colors::Magenta),
            0b110 => Ok(Colors::Yellow),
            0b111 => Ok(Colors::White),
            _ => Err("Unknown color"),
        }
    }
}

fn main() {
    println!(
        "{:?}",
        Colors::try_from((Colors::Blue as u8) | (Colors::Green as u8))
    ); // Ok(Cyan)
    println!(
        "{:?}",
        Colors::try_from((Colors::Yellow as u8) | (Colors::Cyan as u8))
    ); // Ok(White)
    println!(
        "{:?}",
        Colors::try_from((Colors::Red as u8) ^ (Colors::Magenta as u8))
    ); // Ok(Blue)
}
