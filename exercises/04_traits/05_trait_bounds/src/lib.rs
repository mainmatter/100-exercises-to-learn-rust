// TODO: Add the necessary trait bounds to `min` so that it compiles successfully.
//   Refer to the documentation of the `std::cmp` module for more information on the traits you might need.
//
// Note: there are different trait bounds that'll make the compiler happy, but they come with
// different _semantics_. We'll cover those differences later in the course when we talk about ordered
// collections (e.g. BTreeMap).

/// Return the minimum of two values.
pub fn min<T: Ord>(left: T, right: T) -> T {
    // `Ord` guarantees that the values can be compared.
    // `PartialOrd` would also make the compiler happy, but it would have different semantics:
    // it'd either return the minimum value or `right` if they can't be compared.
    if left <= right {
        left
    } else {
        right
    }
}
