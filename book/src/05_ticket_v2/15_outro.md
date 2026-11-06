# まとめ

ドメインモデリングにおいては、神は細部に宿ります（細部こそが重要です）。\
Rustは、ドメインの制約を型システム内で直接表現するのに役立つ幅広いツールを提供していますが、それを適切に行い、イディオマティックなコードを書けるようになるには少し練習が必要です。

最後に `Ticket` モデルの最終的な改良を行って、この章を締めくくりましょう。\
それぞれの制約をカプセル化するために、`Ticket` の各フィールドに対して新しい型を導入します。\
誰かが `Ticket` のフィールドにアクセスするたびに、有効であることが保証された値（つまり `String` ではなく `TicketTitle`）が返されます。コードの他の場所でタイトルが空でないか心配する必要はもうありません。`TicketTitle` を持っている限り、それは**構築された時点で (by construction)** 有効であることがわかっているからです。

これは、Rustの型システムを使用してコードをより安全で表現力豊かなものにする方法のほんの一例です。

## さらに詳しく知りたい方へ

- [Parse, don't validate (検証するのではなく、パースせよ)](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/)
- [Using types to guarantee domain invariants (型を使ってドメインの不変条件を保証する)](https://www.lpalmieri.com/posts/2020-12-11-zero-to-production-6-domain-modelling/)
