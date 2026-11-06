# ベクタ (`Vec`)

配列の強みは、同時にその弱みでもあります。配列のサイズはコンパイル時にあらかじめわかっていなければなりません。
実行時にしかわからないサイズで配列を作成しようとすると、コンパイルエラーになります：

```rust
let n = 10;
let numbers: [u32; n];
```

```text
error[E0435]: attempt to use a non-constant value in a constant
 --> src/main.rs:3:20
  |
2 | let n = 10;
3 | let numbers: [u32; n];
  |                    ^ non-constant value
```

コンパイル時には保存する必要があるチケットの数がわからないため、チケット管理システムには配列は使えません。
そこで登場するのが `Vec`（ベクタ）です。

## `Vec`

`Vec` は、標準ライブラリによって提供される可変長（伸長可能）の配列型です。\
`Vec::new` 関数を使用して空のベクタを作成できます：

```rust
let mut numbers: Vec<u32> = Vec::new();
```

その後、`push` メソッドを使用してベクタに要素を追加（プッシュ）します：

```rust
numbers.push(1);
numbers.push(2);
numbers.push(3);
```

新しい値はベクタの末尾に追加されます。\
作成時に値がわかっている場合は、`vec!` マクロを使用して初期化済みのベクタを作成することもできます：

```rust
let numbers = vec![1, 2, 3];
```

## 要素へのアクセス

要素にアクセスするための構文は、配列と同じです：

```rust
let numbers = vec![1, 2, 3];
let first = numbers[0];
let second = numbers[1];
let third = numbers[2];
```

インデックスの型は `usize` でなければなりません。\
また、`Option<&T>` を返す `get` メソッドを使用することもできます：

```rust
let numbers = vec![1, 2, 3];
assert_eq!(numbers.get(0), Some(&1));
// 範囲外のインデックスにアクセスしようとしても、
// パニックではなく `None` が返されます。
assert_eq!(numbers.get(3), None);
```

配列の要素アクセスと同様に、アクセス時には境界チェックが行われます。計算量は `O(1)` です。

## メモリレイアウト

`Vec` はヒープ割り当てされるデータ構造です。\
`Vec` を作成すると、要素を保存するためにヒープ上にメモリが割り当てられます。

次のコードを実行すると：

```rust
let mut numbers = Vec::with_capacity(3);
numbers.push(1);
numbers.push(2);
```

次のようなメモリレイアウトになります：

```text
         +---------+--------+----------+
スタック | pointer | length | capacity | 
         |  |      |   2    |    3     |
         +--|------+--------+----------+
            |
            |
            v
         +---+---+---+
ヒープ:  | 1 | 2 | ? |
         +---+---+---+
```

`Vec` は3つの情報を追跡します：

- 予約したヒープ領域への**ポインタ (pointer)**。
- ベクタの**長さ (length)**、つまりベクタ内にいくつの要素があるか。
- ベクタの**容量 (capacity)**、つまりヒープ上に予約されたスペースに収まる要素の数。

このレイアウトには見覚えがあるはずです。`String` とまったく同じですね！\
これは偶然ではありません。`String` は内部的にはバイトのベクタ、つまり `Vec<u8>` として定義されているのです：

```rust
pub struct String {
    vec: Vec<u8>,
}
```
