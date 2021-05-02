use chrono::Local;
use serde_json;
use serde_json::json;

fn main() -> serde_json::Result<()> {
    //{:?} はDebugを導出しているものは、なんでも表示できる
    println!("Write with local datetime at {:?}", Local::now());

    serde_json::to_writer_pretty(
        std::io::stdout(),
        &json!({
            "example": "encoding/json",
            "hello": "world",
        }),
    )?;
    Ok(())
}
