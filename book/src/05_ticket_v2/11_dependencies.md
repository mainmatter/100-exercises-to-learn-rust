# 依存関係 (Dependencies)

パッケージは、その `Cargo.toml` ファイルの `[dependencies]` セクションに他のパッケージをリストすることで、それらに依存することができます。\
依存関係を指定する最も一般的な方法は、その名前とバージョンを指定することです：

```toml
[dependencies]
thiserror = "1"
```

これにより、**最小**バージョン `1.0.0` の `thiserror` がパッケージの依存関係として追加されます。
`thiserror` は、Rustの公式パッケージレジストリである [crates.io](https://crates.io) から取得されます。
`cargo build` を実行すると、`cargo` は以下のいくつかの段階を経ます：

- 依存関係の解決 (Dependency resolution)
- 依存関係のダウンロード
- プロジェクト（自身のコードと依存関係）のコンパイル

プロジェクトに `Cargo.lock` ファイルがあり、マニフェストファイルが変更されていない場合、依存関係の解決はスキップされます。
ロックファイルは、依存関係の解決が成功した後に `cargo` によって自動的に生成されます。これにはプロジェクトで使用されているすべての依存関係の正確なバージョンが含まれており、異なるビルド間（CIなど）で同じバージョンが一貫して使用されることを保証するために使われます。複数の開発者でプロジェクトに取り組んでいる場合は、`Cargo.lock` ファイルをバージョン管理システムにコミットすべきです。

すべての依存関係の最新の（互換性のある）バージョンで `Cargo.lock` ファイルを更新するには、`cargo update` を使用できます。

### パス依存関係 (Path dependencies)

**パス (path)** を使用して依存関係を指定することもできます。これは、複数のローカルパッケージで作業している場合に便利です。

```toml
[dependencies]
my-library = { path = "../my-library" }
```

パスは、依存関係を宣言しているパッケージの `Cargo.toml` ファイルからの相対パスになります。

### その他の取得元

どこから依存関係を取得できるか、および `Cargo.toml` ファイルでの指定方法の詳細については、[Cargoのドキュメント](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)を確認してください。

## 開発用依存関係 (Dev dependencies)

開発時にのみ必要な依存関係（つまり、`cargo test` を実行しているときにのみ引き込まれる依存関係）を指定することもできます。\
これらは `Cargo.toml` ファイルの `[dev-dependencies]` セクションに記述します：

```toml
[dev-dependencies]
static_assertions = "1.1.0"
```

この教材でも、テストを短く書くためにこれらのいくつかをすでに使用しています。
