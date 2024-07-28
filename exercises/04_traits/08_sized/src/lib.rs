pub fn example() -> usize {
    // Trying to get the size of a str (or any other DST)
    // via `std::mem::size_of` will result in a compile-time error.
    //
    // TODO: Comment out the following line and move on to the next exercise.
    // str은 컴파일타임에 사이즈 알 수 없음 (DST)
    // &str은 알 수 있음(&str)
    let res = std::mem::size_of::<&str>();
    res
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test() {
//         let res = example();
//         print!("{:?}", res);
//     }
// }