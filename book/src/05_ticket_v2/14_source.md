# `Error::source`

`Error` トレイトについての説明を締めくくるために、もう1つだけ話しておくべきことがあります。それが `source` メソッドです。

```rust
// 今回は完全な定義です！
pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
```

`source` メソッドは、**エラーの原因 (error cause)** がある場合にそれにアクセスするための方法です。\
エラーはしばしば連鎖（チェーン）します。つまり、あるエラーが別のエラーの原因になっている状態です。例えば、高レベルのエラー（データベースに接続できないなど）が、低レベルのエラー（データベースのホスト名を解決できないなど）によって引き起こされているような場合です。
`source` メソッドを使用すると、エラーの連鎖全体を「辿る (walk)」ことができます。これはログにエラーのコンテキストを記録する際によく使われます。

## `source` の実装

`Error` トレイトは、常に `None`（つまり根本的な原因がない）を返すデフォルト実装を提供しています。これまでの演習で `source` を気にする必要がなかったのはそのためです。\
このデフォルト実装をオーバーライドして、自身のエラー型に対する原因を提供することができます。

```rust
use std::error::Error;

#[derive(Debug)]
struct DatabaseError {
    source: std::io::Error
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to connect to the database")
    }
}

impl std::error::Error for DatabaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}
```

この例では、`DatabaseError` はその原因（source）として `std::io::Error` をラップしています。
そして、呼び出されたときにこの原因を返すように `source` メソッドをオーバーライドしています。

## `&(dyn Error + 'static)`

この `&(dyn Error + 'static)` という型は何でしょうか？\
要素ごとに分解してみましょう：

- `dyn Error` は**トレイトオブジェクト (trait object)** です。これは、`Error` トレイトを実装する任意の型を参照するための方法です。
- `'static` は特別な**ライフタイム指定子 (lifetime specifier)** です。
  `'static` は、その参照が「必要な限りずっと」（つまりプログラムの実行期間全体にわたって）有効であることを意味します。

組み合わせると、`&(dyn Error + 'static)` は、`Error` トレイトを実装し、プログラムの実行期間全体にわたって有効なトレイトオブジェクトへの参照となります。

今のところは、これらの概念のどちらについてもあまり心配しないでください。今後の章でより詳しく説明します。

## `thiserror` を使った `source` の実装

`thiserror` は、エラー型に対して `source` を自動的に実装する3つの方法を提供しています：

- `source` という名前のフィールドは、自動的にエラーの原因として使用されます。
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          source: std::io::Error
      }
  }
  ```
- `#[source]` アトリビュートが付与されたフィールドは、自動的にエラーの原因として使用されます。
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          #[source]
          inner: std::io::Error
      }
  }
  ```
- `#[from]` アトリビュートが付与されたフィールドは、自動的にエラーの原因として使用される**だけでなく**、アノテートされた型をあなたのエラー型に変換するための `From` 実装も `thiserror` によって自動生成されます。
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          #[from]
          inner: std::io::Error
      }
  }
  ```

## `?` 演算子

`?` 演算子は、エラーを伝播（プロパゲート）するための省略記法です。\
`Result` を返す関数内で使用すると、対象の `Result` が `Err` である場合に、そのエラーを持って早期リターンします。

例えば：

```rust
use std::fs::File;

fn read_file() -> Result<String, std::io::Error> {
    let mut file = File::open("file.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

これは以下と等価です：

```rust
use std::fs::File;

fn read_file() -> Result<String, std::io::Error> {
    let mut file = match File::open("file.txt") {
        Ok(file) => file,
        Err(e) => {
            return Err(e);
        }
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => {
            return Err(e);
        }
    }
    Ok(contents)
}
```

`?` 演算子を使用すると、エラー処理のコードを大幅に短縮できます。\
特に、変換が可能である場合（つまり適切な `From` 実装が存在する場合）、`?` 演算子は失敗する可能性のある操作のエラー型を、関数の戻り値のエラー型に自動的に変換してくれます。
