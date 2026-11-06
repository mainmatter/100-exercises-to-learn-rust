//! TODO: `example` 関数内の文を**並べ替える**ことで、コードがコンパイルできるようにしてください。
//!  `spawner` 関数や、`example` 内の各行が行う処理自体を変更することは許可されていません。
//!   必要であれば、既存の文をブロック `{}` で囲むことは可能です。
use std::rc::Rc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
    let non_send = Rc::new(1);
    yield_now().await;
    println!("{}", non_send);
}
