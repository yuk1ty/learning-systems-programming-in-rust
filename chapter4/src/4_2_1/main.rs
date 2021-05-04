use tokio;

#[tokio::main]
async fn main() {
    println!("start sub()");

    // 終了を受け取るためのchannel
    let (done_tx, mut done_rx) = tokio::sync::mpsc::channel(1);

    tokio::spawn(async move {
        println!("sub() is finished");
        done_tx.send(true).await.unwrap();
    });

    // 終了を待つ
    let _done = done_rx.recv().await;

    println!("all tasks are finished");
}
