use std::io;

use signal_hook::{consts::SIGINT, iterator::Signals};

#[tokio::main]
async fn main() -> io::Result<()> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);

    tokio::spawn(async move {
        let mut signals = Signals::new(&[SIGINT])?;

        println!("[signal thread] Waiting for SIGINT (CTRL+C)");
        if signals.into_iter().next().is_some() {
            tx.send(()).await.unwrap();
        }
        Ok::<(), io::Error>(())
    });

    println!("[main thread] Waiting for a message from channel");
    let _ = rx.recv().await;
    println!("[main thread] Got a message");

    Ok(())
}
