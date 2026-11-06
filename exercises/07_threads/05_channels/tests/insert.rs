// TODO: この演習が完了したと思ったら、`ready` 内の `move_forward` を `true` に設定してください。
//  解答を確認してもらうために、遠慮なくインストラクターに声をかけてください！
use channels::data::TicketDraft;
use channels::{launch, Command};
use std::time::Duration;
use ticket_fields::test_helpers::{ticket_description, ticket_title};

#[test]
fn a_thread_is_spawned() {
    let sender = launch();
    std::thread::sleep(Duration::from_millis(200));

    sender
        .send(Command::Insert(TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        }))
        // スレッドがすでに実行されていない場合、チャネルが閉じられるため
        // ここでパニックが発生します。
        .expect("Did you actually spawn a thread? The channel is closed!");
}

#[test]
fn ready() {
    // サーバーがいかなる**読み取り**アクションも公開していないため、
    // この演習で自動的にチェックできることはほとんどありません。
    // 挿入が実際に行われているかどうか、また正しく行われているかどうかを知る術がないのです。
    let move_forward = false;

    assert!(move_forward);
}
