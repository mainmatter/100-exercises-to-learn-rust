# Deriveマクロ (Derive macros)

`Ticket` に対して `PartialEq` を実装するのは少し面倒でしたよね？
構造体の各フィールドを手動で比較する必要がありました。

## 分割代入 (Destructuring) 構文

さらに、その実装は壊れやすい（脆弱な）ものでもあります。構造体の定義が変更された場合（新しいフィールドが追加された場合など）、忘れずに `PartialEq` の実装も更新しなければなりません。

構造体をそのフィールドに**分割代入 (destructuring)** することで、このリスクを軽減できます：

```rust
impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool {
        let Ticket {
            title,
            description,
            status,
        } = self;
        // [...]
    }
}
```

`Ticket` の定義が変更されると、コンパイラは分割代入が網羅的でなくなったとエラーを出してくれます。\
また、変数のシャドーイングを避けるために、構造体のフィールドに別名を付けることもできます：

```rust
impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool {
        let Ticket {
            title,
            description,
            status,
        } = self;
        let Ticket {
            title: other_title,
            description: other_description,
            status: other_status,
        } = other;
        // [...]
    }
}
```

分割代入は覚えておくと便利なパターンですが、これを行うさらに便利な方法があります。それが**deriveマクロ (derive macros)** です。

## マクロ (Macros)

これまでの演習でも、いくつかのマクロに遭遇してきました：

- テストケース内の `assert_eq!` と `assert!`
- コンソールに出力するための `println!`

Rustのマクロは**コードジェネレータ（コード生成器）**です。\
提供された入力に基づいて新しいRustコードを生成し、その生成されたコードがプログラムの他の部分と一緒にコンパイルされます。一部のマクロはRustの標準ライブラリに組み込まれていますが、独自のマクロを書くこともできます。このコースでは独自のマクロを作成しませんが、[「さらに詳しく知りたい方へ」セクション](#さらに詳しく知りたい方へ)に役立つリンクをいくつか紹介しています。

### 展開結果の検査

一部のIDEでは、マクロを展開して生成されたコードを検査できます。それができない場合は、[`cargo-expand`](https://github.com/dtolnay/cargo-expand) を使用できます。

### Deriveマクロ

**deriveマクロ (導出マクロ)** は、Rustマクロの特定の種類の1つです。構造体の上に**アトリビュート (属性)** として指定します。

```rust
#[derive(PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: String
}
```

deriveマクロは、カスタム型に対して一般的で（「自明な」）トレイトの実装を自動化するために使用されます。
上記の例では、`Ticket` に対して `PartialEq` トレイトが自動的に実装されます。
マクロを展開すると、少し読みにくいものの、生成されたコードが手動で書いたものと機能的に等価であることがわかります：

```rust
#[automatically_derived]
impl ::core::cmp::PartialEq for Ticket {
    #[inline]
    fn eq(&self, other: &Ticket) -> bool {
        self.title == other.title 
            && self.description == other.description
            && self.status == other.status
    }
}
```

コンパイラは、可能な場合にはトレイトを `derive` するよう促してくれます。

## さらに詳しく知りたい方へ

- [The little book of Rust macros](https://veykril.github.io/tlborm/)
- [Proc macro workshop](https://github.com/dtolnay/proc-macro-workshop)
