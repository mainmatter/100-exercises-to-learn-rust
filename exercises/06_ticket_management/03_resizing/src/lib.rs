#[cfg(test)]
mod tests {
    #[test]
    fn resizing() {
        let mut v = Vec::with_capacity(2);
        v.push(1);
        v.push(2); // 最大容量に到達
        assert_eq!(v.capacity(), 2);

        v.push(3); // 容量を超えたため、リサイズが必要

        // 新しい容量がいくつになるか推測できますか？
        // 標準ライブラリはベクタのリサイズに使用するアルゴリズムについていかなる保証もしていないため、
        // 将来変更される可能性があることに注意してください。
        assert_eq!(v.capacity(), todo!());
    }
}
