// TODO: いわゆる「ドロップ爆弾 (Drop bomb)」（特定の操作が実行されない限り、
//  ドロップされたときにパニックを起こす型）を実装してください。
//  期待されるAPIは以下のテストで確認できます。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // 爆弾はドロップされたときにパニックを起こすべきです
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // 爆弾は信管が抜かれた（defuseされた）ため、
        // ドロップされてもパニックを起こすべきではありません
    }
}
