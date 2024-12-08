// TODO: this is an example of an orphan rule violation.
//  We're implementing a foreign trait (`PartialEq`, from `std`) on
//  a foreign type (`u32`, from `std`).
//  Look at the compiler error to get familiar with what it looks like.
//  Then delete the code below and move on to the next exercise.

/*
    Extension trait:
    An extension trait is a trait whose primary purpose is to attach new methods to
    foreign types, such as u32. That's exactly the pattern you deployed in the previous 
    exercise, by defining the IsEven trait and the implementing it for i32 and u32. You 
    are then free to call is_even on those types as long as the IsEven trait is in scope.

    One implementation:
    You can't implement the same trait twice, in a crate, for the same type.

    Orphan rule:
    At least one of the following must be true:
    - The trait is defined in the current crate
    - The implementor type is defined in the current crate

*/

// impl PartialEq for u32 {
//     fn eq(&self, _other: &Self) -> bool {
//         todo!()
//     }
// }

/*
    This gives an error:
    only traits defined in the current crate can be implemented for primitive types
    impl doesn't use only types from inside the current crate
    u32 is not defined in the current crate
    note: define and implement types from inside the current crate

*/
