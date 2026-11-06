# ようこそ

**「100 Exercises To Learn Rust（1問ずつ学ぶRust）」**へようこそ！

このコースでは、演習問題を1問ずつ解きながらRustのコアコンセプトを学びます。\
Rustの構文、型システム、標準ライブラリ、そしてエコシステムについて学習していきます。

Rustの事前知識は前提としていませんが、少なくとも他のプログラミング言語を1つ知っていることを前提としています。
また、システムプログラミングやメモリ管理の事前知識も必要ありません。これらのトピックはコースの中で扱います。

つまり、ゼロからスタートします！\
小さく取り組みやすいステップで、Rustの知識を積み上げていきます。
コースの終わりまでに約100問の演習を解き終える頃には、小〜中規模のRustプロジェクトに自信を持って取り組めるようになっているはずです。

## 学習方法

このコースは「実践して学ぶ（learn by doing）」という原則に基づいています。\
インタラクティブかつ実践的になるよう設計されています。

[Mainmatter](https://mainmatter.com/rust-consulting/) は、このコースを4日間の講義形式で提供するために開発しました。各受講者は自分のペースでレッスンを進め、経験豊富なインストラクターがガイダンスを提供し、質問に答え、必要に応じてトピックをより深く掘り下げます。\
次回の講師付きセッションには[私たちのウェブサイト](https://ti.to/mainmatter/rust-from-scratch-jan-2025)から申し込むことができます。
企業向けのプライベートセッションの開催をご希望の場合は、[お問い合わせ](https://mainmatter.com/contact/)ください。

また、独学でこのコースを進めることも可能ですが、行き詰まったときに助けてくれる友人やメンターを見つけることをお勧めします。すべての演習問題の解答は、
[GitHubリポジトリの `solutions` ブランチ](https://github.com/mainmatter/100-exercises-to-learn-rust/tree/solutions) にあります。

## フォーマット

コースの教材は[ブラウザ上](https://rust-exercises.com/100-exercises/)で読み進めることも、オフラインで読むために[PDFファイルとしてダウンロード](https://rust-exercises.com/100-exercises-to-learn-rust.pdf)することもできます。\
印刷された教材がお好みの場合は、[Amazonでペーパーバック版を購入](https://www.amazon.com/dp/B0DJ14KQQG/)できます。

## 構成

画面の左側を見るとわかるように、このコースは複数のセクションに分かれています。
各セクションでは、Rust言語の新しい概念や機能を導入します。\
理解度を確認するため、各セクションには解くべき演習問題がペアとして用意されています。

演習問題は[コンパニオンGitHubリポジトリ](https://github.com/mainmatter/100-exercises-to-learn-rust)にあります。\
コースを始める前に、リポジトリをローカルマシンにクローンしてください：

```bash
# GitHubにSSHキーを設定している場合
git clone git@github.com:mainmatter/100-exercises-to-learn-rust.git
# そうでない場合は、HTTPS URLを使用してください：
#   https://github.com/mainmatter/100-exercises-to-learn-rust.git
```

また、進捗状況を簡単に追跡し、必要に応じてメインリポジトリから更新を取り込めるように、ブランチを作成して作業することをお勧めします：

```bash
cd 100-exercises-to-learn-rust
git checkout -b my-solutions
```

すべての演習問題は `exercises` フォルダ内にあります。
各演習はRustパッケージとして構成されています。
パッケージには、演習問題そのもの、やるべきことの説明（`src/lib.rs` 内）、そして解答を自動的に検証するためのテストスイートが含まれています。

### ツール

このコースを進めるには、以下のものが必要です：

- [**Rust**](https://www.rust-lang.org/tools/install)。
  すでにシステムに `rustup` がインストールされている場合は、`rustup update`（またはシステムへのRustのインストール方法に応じた適切なコマンド）を実行して、最新の安定版（stable）で動作していることを確認してください。
- _（任意ですが推奨）_ Rustの自動補完をサポートするIDE。
  以下のいずれかを推奨します：
  - [RustRover](https://www.jetbrains.com/rust/)
  - [Visual Studio Code](https://code.visualstudio.com) と [`rust-analyzer`](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) 拡張機能の組み合わせ

### ワークショップランナー `wr`

解答を検証し、コースをガイドするためのツールとして、`wr` CLI（「workshop runner」の略）も提供しています。
`wr` の[ウェブサイト](https://mainmatter.github.io/rust-workshop-runner/)の手順に従ってインストールしてください。

`wr` をインストールしたら、新しいターミナルを開き、リポジトリのトップレベルフォルダに移動します。
`wr` コマンドを実行してコースを開始します：

```bash
wr
```

`wr` は現在の演習の解答を検証します。\
現在のセクションの演習を解き終えるまでは、次のセクションに進まないでください。

> コースを進めながら、解答をGitにコミットすることをお勧めします。
> そうすることで、進捗状況を簡単に追跡でき、必要に応じて既知の時点から「再開」することができます。

コースを楽しんでください！

## 著者

このコースは、[Mainmatter](https://mainmatter.com/rust-consulting/) のプリンシパルエンジニアリングコンサルタントである [Luca Palmieri](https://www.lpalmieri.com/) によって執筆されました。\
Lucaは2018年からRustに携わっており、最初はTrueLayerで、その後AWSで経験を積みました。\
Lucaは、Rustでバックエンドアプリケーションを構築する方法を学ぶための定番リソースである[『Zero to Production in Rust』](https://zero2prod.com)の著者です。\
また、[`cargo-chef`](https://github.com/LukeMathWalker/cargo-chef)、[Pavex](https://pavex.dev)、[`wiremock`](https://github.com/LukeMathWalker/wiremock-rs) など、さまざまなオープンソースRustプロジェクトの作者およびメンテナでもあります。

## ライセンス

Copyright © 2024- Mainmatter GmbH (https://mainmatter.com), released under the
[PolyForm-Noncommercial-1.0.0](https://polyformproject.org/licenses/noncommercial/1.0.0).
