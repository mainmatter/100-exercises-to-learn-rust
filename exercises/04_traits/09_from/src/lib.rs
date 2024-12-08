// From trait

/*
    From and Into
    The Rust std library defines two traits for infallible conversions:
    From and Into, in the std::convert module

    pub trait From<T>: Sized {
        fn from(value: T) -> Self;
    }

    pub trait Into<T>: Sized {
        fn into(self) -> T;
    }

    These trait definitions showcase: supertraits and implicit trait bounds. 

    Supertrait/subtrait
    From: Sized syntax implies that From is a subtrait of Sized: any type that implements
    From must also implement Sized. Alternatively, you could say that Sized is a 
    supertrait of From. 

    Implicit trait bounds
    Everytime you have a generic type parameter, the compiler implicitley assums that it's 
    Sized.

    Negative trait bounds
    You can opt out of the implicit Sized bound with a negative trait bound:

    pub struct Foo<T: ?Sized> {
        //            ^^^^^^^
        //            This is a negative trait bound
        inner: T,
    }

    This syntax reads as "T may or may not be Sized", and it allows you to bind T to a 
    DST. It is a special case, though: negative trait bounds are exclusive to Sized,
    you can't use them with other traits. 

*/


// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

pub struct WrappingU32 {
    value: u32,
}

impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        WrappingU32 { value }
    }
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
