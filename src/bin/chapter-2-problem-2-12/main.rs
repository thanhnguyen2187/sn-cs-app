use std::ops::Not;
use num::traits::{FromBytes, ToBytes};

fn f_a<T>(x: T) -> T
where
    T: ToBytes + FromBytes<Bytes = <T as ToBytes>::Bytes>,
{
    let mut bytes = x.to_le_bytes();
    bytes.as_mut()[1..].fill(0);
    T::from_le_bytes(&bytes)
}

fn f_b<T>(x: T) -> T
where
    T: ToBytes + FromBytes<Bytes = <T as ToBytes>::Bytes>,
{
    let mut bytes = x.to_le_bytes();
    bytes.as_mut()[1..]
        .iter_mut()
        .for_each(|b| *b = b.not());
    T::from_le_bytes(&bytes)
}

fn f_c<T>(x: T) -> T
where
    T: ToBytes + FromBytes<Bytes = <T as ToBytes>::Bytes>,
{
    let mut bytes = x.to_le_bytes();
    bytes.as_mut()[0] = 0xFF;
    T::from_le_bytes(&bytes)
}

fn main() {
    let x: u32 = 0x8765_4321;
    println!("{:#x?}", f_a(x));
    println!("{:#x?}", f_b(x));
    println!("{:#x?}", f_c(x));
}
