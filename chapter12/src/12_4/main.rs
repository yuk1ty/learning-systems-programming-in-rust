use futures::stream::StreamExt;
use signal_hook::consts::signal::*;
use signal_hook_tokio::Signals;
use std::io::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let signals: Signals = Signals::new(&[SIGINT, SIGTERM])?;
    let handle = signals.handle();
    let mut signals = signals.fuse();

    tokio::spawn(async move {
        while let Some(signal) = signals.next().await {
            match signal {
                SIGINT => println!("SIGINT"),
                SIGTERM => println!("SIGTERM"),
                _ => unreachable!(),
            }
        }
    });

    handle.close();

    Ok(())
}
