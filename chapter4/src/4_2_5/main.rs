use tokio;
use tokio::sync::oneshot;

struct WithCancel {
    cancel_rx: oneshot::Receiver<()>,
}

impl WithCancel {
    pub fn new() -> (Self, oneshot::Sender<()>) {
        let (tx, rx) = oneshot::channel();

        (Self { cancel_rx: rx }, tx)
    }

    pub async fn done(self) -> Result<(), oneshot::error::RecvError> {
        self.cancel_rx.await
    }
}

#[tokio::main]
async fn main() {
    println!("start sub()");

    let (ctx, cancel) = WithCancel::new(); // context.Background()に関してあまり理解できていない

    tokio::spawn(async move {
        println!("sub() is finished");
        cancel.send(());
    });

    ctx.done().await.unwrap();
    println!("all tasks are finished");
}
