use tokio;
use tokio::sync::mpsc;

fn prime_number() -> mpsc::Receiver<u64> {
    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(async move {
        tx.send(2);
        for i in (3..100000).step_by(2) {
            let l = (i as f64).sqrt() as u64;
            let found = (3..=l).step_by(2).into_iter().any(|j| i % j == 0);
            if !found {
                tx.send(i).await.unwrap();
            }
        }
    });

    rx
}

#[tokio::main]
async fn main() {
    let mut rx = prime_number();

    while let Some(x) = rx.recv().await {
        println!("{}", x);
    }
}
