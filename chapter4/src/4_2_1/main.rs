use tokio;

#[tokio::main]
async fn main() {
    println!("start sub()");

    // 終了を受け取るためのfeature(チャンネルに変えたほうが良い？)
    let done = async move {
        println!("sub() is finished");
        true
    };
    // 終了を待つ
    let _done = done.await;

    println!("all tasks are finished");
}
