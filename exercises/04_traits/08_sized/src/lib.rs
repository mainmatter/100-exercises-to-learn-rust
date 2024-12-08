// Sized

/*
    &str (string slice reference) stores a pointer to the heap and some metadata (the length
    of the slice it poitns to)

    Dynamically sized types
    str is a dynamically sized type (DST).
    A DST is a type whose size is not known at compile time. Whenever you have a reference
    to a DST, like &str, it has to include additional information about the data it 
    points to. It is a 'fat pointer'. In the case of &str, it stores the length of the
    slice it points to. 

    The Sized trait
    Rust's std library defines a trait called Sized.
    A type is Sized if its size is known at compile time. 
    In other words, it's NOT a DST

    Auto traits
    In particular, Sized is also an auto trait. You don't need to implement it explicitly;
    the compiler implements it automatically for you based on the type's definition.
*/

pub fn example() {
    // Trying to get the size of a str (or any other DST)
    // via `std::mem::size_of` will result in a compile-time error.
    //
    // TODO: Comment out the following line and move on to the next exercise.
    // std::mem::size_of::<str>();
}
