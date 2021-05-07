use std::io;

use tokio::signal::unix::{signal, SignalKind};

#[tokio::main]
async fn main() -> io::Result<()> {
    let (sigint_tx, mut sigint_rx) = tokio::sync::mpsc::channel::<()>(1);

    tokio::spawn(async move {
        let mut sig_stream = signal(SignalKind::interrupt())?;

        println!("[signal thread] Waiting for SIGINT (CTRL+C)");
        sig_stream.recv().await;

        sigint_tx.send(()).await.unwrap();
        Ok::<(), io::Error>(())
    });

    println!("[main thread] Waiting for a message from channel");
    let _ = sigint_rx.recv().await;
    println!("[main thread] Got a message");

    Ok(())
}
