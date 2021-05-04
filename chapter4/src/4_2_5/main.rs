use tokio;
use tokio::sync::oneshot;

struct WithCancel {
    cancel_rx: oneshot::Receiver<()>,
}

impl WithCancel {
    pub fn new() -> (Self, Canceler) {
        let (tx, rx) = oneshot::channel();

        (Self { cancel_rx: rx }, Canceler(tx))
    }

    pub async fn done(self) -> Result<(), oneshot::error::RecvError> {
        self.cancel_rx.await
    }
}

struct Canceler(oneshot::Sender<()>);

impl Canceler {
    fn cancel(self) -> Result<(), ()> {
        self.0.send(())
    }
}

#[tokio::main]
async fn main() {
    println!("start sub()");

    let (ctx, canceler) = WithCancel::new(); // context.Background()に関してあまり理解できていない

    tokio::spawn(async move {
        println!("sub() is finished");
        canceler.cancel().unwrap();
    });

    ctx.done().await.unwrap();
    println!("all tasks are finished");
}
