pub fn example() {
    // `std::mem::size_of` を介して `str`（またはその他のDST）のサイズを取得しようとすると、
    // コンパイルエラーが発生します。
    //
    // TODO: 以下の行をコメントアウトして、次の演習に進んでください。
    std::mem::size_of::<str>();
}
