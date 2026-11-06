// TODO: `example` がコンパイルできるように、`WrappingU32` 型に対して `From` トレイトを実装してください。

pub struct WrappingU32 {
    value: u32,
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
