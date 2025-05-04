// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

#![allow(unused)]

pub struct WrappingU32 {
    value: u32,
}

// The `Into` trait is automatically implemented for types that implement the `From` trait.

impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        WrappingU32 { value }
    }
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
