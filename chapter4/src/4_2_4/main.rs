use tokio;
use tokio::sync::{mpsc, oneshot};
use tokio::time::{sleep, Duration};

async fn multi_channel(
    chint: mpsc::Sender<i64>,
    chstr: mpsc::Sender<String>,
    end: oneshot::Sender<()>,
) {
    for i in 0..10 {
        if i % 2 == 0 {
            println!("ch1へ送信");
            chint.send(i).await;
        } else {
            println!("ch2へ送信");
            chstr.send(format!("test{}", i)).await;
        }

        sleep(Duration::from_secs(1)).await;
    }
    end.send(());
}

#[tokio::main]
async fn main() {
    println!("開始");

    let (tx1, mut ch1) = mpsc::channel(100);
    let (tx2, mut ch2) = mpsc::channel(100);
    let (tx_exit, mut exit) = oneshot::channel();

    tokio::spawn(multi_channel(tx1, tx2, tx_exit));

    loop {
        tokio::select! {
            Some(val) = ch1.recv() => {
                println!("ch1から受信:{}", val);
            }
            Some(s) = ch2.recv() => {
                println!("ch2から受信:{}", s);
            }
            _ = &mut exit => {
                println!("終了");
                break;
            }
        }
    }
}
